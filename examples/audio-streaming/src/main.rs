//! Sends input audio samples as UDP datagrams.

use clap::Parser;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, OutputCallbackInfo, StreamConfig,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use opus::{Application, Channels, Decoder, Encoder};
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapProd, HeapRb,
};
use std::error::Error;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Default)]
struct SamplingState {
    started_at: Option<Instant>,
    generation: u64,
    datagrams: u64,
    bytes: u64,
}

impl SamplingState {
    fn start(&mut self) {
        self.datagrams = 0;
        self.bytes = 0;
        self.generation = self.generation.wrapping_add(1);
        self.started_at = Some(Instant::now());
    }

    fn stop(&mut self) {
        if let Some(started_at) = self.started_at.take() {
            let seconds = started_at.elapsed().as_secs_f64();
            let per_second = |count: u64| {
                if seconds > 0.0 {
                    count as f64 / seconds
                } else {
                    0.0
                }
            };
            println!("Microphone OFF\r");
            println!(
                "Sent {} datagrams, {} bytes in {:.3} seconds ({:.2} datagrams/s, {:.2} bytes/s)\r",
                self.datagrams,
                self.bytes,
                seconds,
                per_second(self.datagrams),
                per_second(self.bytes),
            );
        }
    }
}

struct TerminalMode;

impl Drop for TerminalMode {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

#[derive(Parser, Debug)]
#[command(version, about = "Send CPAL input audio as UDP datagrams", long_about = None)]
struct Opt {
    /// Audio backend to use (for example, pulseaudio or alsa on Linux)
    #[arg(long, value_name = "HOST")]
    audio: Option<cpal::HostId>,

    /// The input audio device to use
    #[arg(short, long, value_name = "IN")]
    input_device: Option<String>,

    /// The output audio device to use
    #[arg(short, long, value_name = "OUT")]
    output_device: Option<String>,

    /// UDP address to send audio datagrams to
    #[arg(
        short,
        long,
        value_name = "HOST:PORT",
        default_value = "127.0.0.1:5000"
    )]
    destination: SocketAddr,

    /// Local UDP address to bind from
    #[arg(long, short, value_name = "HOST:PORT", default_value = "0.0.0.0:0")]
    bind: SocketAddr,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    let host = select_host(opt.audio)?;
    println!("Using audio host: {}", host.id());

    // Find devices.
    let input_device = if let Some(device) = opt.input_device {
        let id = &device
            .parse()
            .map_err(|err| format!("failed to parse input device id: {err}"))?;
        host.device_by_id(id)
    } else {
        host.default_input_device()
    }
    .ok_or("failed to find input device")?;

    let output_device = if let Some(device) = opt.output_device {
        let id = &device
            .parse()
            .map_err(|err| format!("failed to parse output device id: {err}"))?;
        host.device_by_id(id)
    } else {
        host.default_output_device()
    }
    .ok_or("failed to find output device")?;

    println!("Using input device: \"{}\"", input_device.id()?);
    println!("Using output device: \"{}\"", output_device.id()?);

    let input_configs: Vec<_> = input_device.supported_input_configs()?.collect();
    let output_configs: Vec<_> = output_device.supported_output_configs()?.collect();
    let (config, output_config) = select_stream_config(&input_configs, &output_configs).ok_or(
        "input and output have no f32 mono/stereo configurations sharing an Opus-supported \
         rate (48000, 24000, 16000, 12000, or 8000 Hz); select different audio devices \
         or use an audio server that supports resampling",
    )?;
    let socket = UdpSocket::bind(opt.bind)
        .map_err(|err| io::Error::new(err.kind(), format!("failed to bind {}: {err}", opt.bind)))?;
    socket.connect(opt.destination).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!("failed to connect UDP socket to {}: {err}", opt.destination),
        )
    })?;

    let channels = match config.channels {
        1 => Channels::Mono,
        2 => Channels::Stereo,
        channels => {
            return Err(
                format!("Opus encoding supports mono or stereo, got {channels} channels").into(),
            );
        }
    };
    let sample_rate = config.sample_rate;

    // Create an Opus encoder for the input audio stream.
    // Opus converts audio samples into compressed packets that are sent over the network.
    let encoder = Encoder::new(sample_rate, channels, Application::Voip)
        .map_err(|err| format!("failed to create Opus encoder at {sample_rate} Hz: {err}"))?;

    // Calculate the frame size and number of samples per frame based on the sample rate and channel count.
    // frame_size is used to determine how many samples to encode into each Opus packet,
    // and samples_per_frame is used to manage the input buffer.
    let frame_size = (sample_rate / 50) as usize;
    let samples_per_frame = frame_size * config.channels as usize;

    let recv_socket = socket.try_clone().map_err(|err| {
        io::Error::new(
            err.kind(),
            format!("failed to clone UDP socket for receiving: {err}"),
        )
    })?;
    let ring = HeapRb::<f32>::new(sample_rate as usize * output_config.channels as usize);
    let (decoded_producer, mut decoded_consumer) = ring.split();
    let decoder_channels = output_config.channels as usize;
    let decode_channels = if decoder_channels == 1 {
        Channels::Mono
    } else {
        Channels::Stereo
    };
    let decoded_samples_per_frame = frame_size * decoder_channels;

    println!(
        "Sending and receiving Opus audio from {} to {} ({} Hz, {} channel(s), 20 ms frames)",
        socket.local_addr()?,
        opt.destination,
        sample_rate,
        config.channels
    );

    // Synchronize toggles with callbacks so muted audio and partial frames never leak
    // into the next transmission.
    let sampling = Arc::new(Mutex::new(SamplingState::default()));

    // Create the input callback function that will be called by CPAL when audio data is available.
    let input_data_fn = input_callback(socket, encoder, samples_per_frame, Arc::clone(&sampling));

    // Spawn a thread to receive and decode audio from the UDP socket.
    std::thread::spawn(move || {
        decode_audio(
            recv_socket,
            sample_rate,
            decode_channels,
            decoded_samples_per_frame,
            decoder_channels,
            decoded_producer,
        );
    });

    // Create the output callback function that will be called by CPAL when audio data is needed.
    // The callback will fill the output buffer with decoded audio samples from the ring buffer.
    let output_data_fn = move |data: &mut [f32], _: &OutputCallbackInfo| {
        for sample in data {
            *sample = decoded_consumer.try_pop().unwrap_or(0.0);
        }
    };

    // Set up the input and output audio streams using the selected configurations and callback functions.
    let input_stream = input_device.build_input_stream(config, input_data_fn, err_fn, None)?;
    let output_stream =
        output_device.build_output_stream(output_config, output_data_fn, err_fn, None)?;

    // Enable raw mode to read keyboard input without waiting for Enter key.
    enable_raw_mode().map_err(|err| {
        io::Error::new(
            err.kind(),
            format!("failed to enable terminal keyboard input: {err}"),
        )
    })?;
    let terminal_mode = TerminalMode;

    // Start the input and output audio streams. They will run in the background
    // and call the provided callback functions when audio data is available or needed.
    input_stream.play()?;
    output_stream.play()?;

    println!("Microphone OFF. Press Space to toggle sampling; Q key or Ctrl+C to quit.\r");
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                KeyCode::Char(' ') => {
                    let mut state = sampling.lock().unwrap();
                    if state.started_at.is_some() {
                        state.stop();
                    } else {
                        state.start();
                        println!("Microphone ON\r");
                    }
                }
                _ => {}
            }
        }
    }
    sampling.lock().unwrap().stop();
    drop(input_stream);
    drop(output_stream);
    drop(terminal_mode);
    println!("Done!");
    Ok(())
}

// Returns a closure that will be called by CPAL when audio data is available.
fn input_callback(
    socket: UdpSocket,
    mut encoder: Encoder,
    samples_per_frame: usize,
    callback_sampling: Arc<Mutex<SamplingState>>,
) -> impl FnMut(&[f32], &InputCallbackInfo) + Send + 'static {
    let mut pcm_buffer = Vec::<i16>::with_capacity(samples_per_frame * 2);
    let mut opus_packet = vec![0_u8; 4_000];
    let mut last_generation = 0;

    // what follows is the returned closure that will be called by CPAL when audio data is available.
    move |data: &[f32], _: &InputCallbackInfo| {
        let mut state = callback_sampling.lock().unwrap();
        if state.generation != last_generation || state.started_at.is_none() {
            pcm_buffer.clear();
            last_generation = state.generation;
        }
        if state.started_at.is_none() {
            return;
        }
        for sample in data {
            let sample = sample.clamp(-1.0, 1.0);
            // convert float sample into 16-bit integer PCM sample for Opus encoding.
            pcm_buffer.push((sample * i16::MAX as f32) as i16);
        }

        while pcm_buffer.len() >= samples_per_frame {
            // Compress a 20 ms frame of PCM audio into an Opus packet for UDP transmission.
            match encoder.encode(&pcm_buffer[..samples_per_frame], &mut opus_packet) {
                Ok(packet_len) => match socket.send(&opus_packet[..packet_len]) {
                    Ok(bytes_sent) => {
                        state.datagrams += 1;
                        state.bytes += bytes_sent as u64;
                    }
                    Err(err) => eprintln!("failed to send UDP audio datagram: {err}"),
                },
                Err(err) => eprintln!("failed to encode Opus audio: {err}"),
            }
            pcm_buffer.drain(..samples_per_frame);
        }
    }
}

// Receives and decodes audio from the UDP socket, and pushes the decoded samples into a ring buffer.
fn decode_audio(
    recv_socket: UdpSocket,
    sample_rate: u32,
    channels: Channels,
    samples_per_frame: usize,
    decoder_channels: usize,
    mut decoded_producer: HeapProd<f32>,
) {
    let mut decoder = match Decoder::new(sample_rate, channels) {
        Ok(decoder) => decoder,
        Err(err) => {
            eprintln!("failed to create Opus decoder: {err}");
            return;
        }
    };
    let mut packet = vec![0_u8; 4_000];
    let mut decoded = vec![0_i16; samples_per_frame];

    loop {
        let packet_len = match recv_socket.recv(&mut packet) {
            Ok(packet_len) => packet_len,
            Err(err) => {
                eprintln!("failed to receive UDP audio datagram: {err}");
                continue;
            }
        };

        let decoded_frames = match decoder.decode(&packet[..packet_len], &mut decoded, false) {
            Ok(decoded_frames) => decoded_frames,
            Err(err) => {
                eprintln!("failed to decode Opus audio: {err}");
                continue;
            }
        };

        for sample in &decoded[..decoded_frames * decoder_channels] {
            if decoded_producer
                .try_push(*sample as f32 / i16::MAX as f32)
                .is_err()
            {
                eprintln!("decoded audio buffer is full; dropping sample");
                break;
            }
        }
    }
}

fn err_fn(err: cpal::Error) {
    eprintln!("an error occurred on stream: {err}");
}

fn select_stream_config(
    input: &[cpal::SupportedStreamConfigRange],
    output: &[cpal::SupportedStreamConfigRange],
) -> Option<(StreamConfig, StreamConfig)> {
    // Devices must share a rate, but may have different channel layouts.
    // Opus decodes mono packets to stereo (or stereo to mono) as needed.
    for sample_rate in [48_000, 24_000, 16_000, 12_000, 8_000] {
        let select = |ranges: &[cpal::SupportedStreamConfigRange]| {
            [1, 2].into_iter().find_map(|channels| {
                ranges
                    .iter()
                    .any(|range| {
                        range.sample_format() == cpal::SampleFormat::F32
                            && range.channels() == channels
                            && range.contains_rate(sample_rate)
                    })
                    .then_some(StreamConfig {
                        channels,
                        sample_rate,
                        buffer_size: cpal::BufferSize::Default,
                    })
            })
        };
        if let (Some(input), Some(output)) = (select(input), select(output)) {
            return Some((input, output));
        }
    }
    None
}

fn select_host(requested: Option<cpal::HostId>) -> Result<cpal::Host, Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    {
        let server = std::env::var("PULSE_SERVER").ok();
        let use_pulse = requested == Some(cpal::HostId::PulseAudio)
            || (requested.is_none() && server.as_ref().is_some_and(|s| !s.is_empty()));
        if use_pulse {
            // libpulse accepts bare absolute socket paths (as supplied by WSLg),
            // but CPAL's Rust PulseAudio client only recognizes the unix: form.
            if let Some(path) = server.as_ref().filter(|s| s.starts_with('/')) {
                // Startup only: no application threads or audio hosts exist yet.
                std::env::set_var("PULSE_SERVER", format!("unix:{path}"));
            }
            return cpal::host_from_id(cpal::HostId::PulseAudio).map_err(|err| {
                format!(
                    "failed to initialize PulseAudio (PULSE_SERVER={:?}): {err}. \
                     Run `pactl info` from the same shell to check the server connection.",
                    std::env::var_os("PULSE_SERVER")
                )
                .into()
            });
        }
    }

    match requested {
        Some(id) => cpal::host_from_id(id)
            .map_err(|err| format!("failed to initialize audio host {id}: {err}").into()),
        None => Ok(cpal::default_host()),
    }
}
