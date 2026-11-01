# voicechat-test

Send and receive Opus audio over UDP, using CPAL library. Press Space to toggle
the microphone, and Q or Ctrl+C to quit.

## WSL / Ubuntu

If you try this on Windows Subsystem for Linux, you'll need to install audio
developement librarias and PulseAudio tools:

```sh
sudo apt update
sudo apt install build-essential pkg-config cmake libasound2-dev pulseaudio-utils
cargo run
```

The example should select a proper default audio backend automatically. If this
does not work, you may need to explicitly select the audio backend. In WSL, use
`cargo run -- --host pulseaudio`. In some cases you may also try `cargo run --
--host alsa` on Linux.
