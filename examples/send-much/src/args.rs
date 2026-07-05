use clap::Parser;

/// Command line arguments parser for this application.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Start as client, give address to connect.
    #[arg(short, long, default_value = "127.0.0.1:20000")]
    connect_addr: String,

    /// Start as server, give address to listen for connections.
    #[arg(short, long, default_value = "")]
    server_addr: String,

    /// When operating as client, number of bytes to write.
    #[arg(short, long, default_value_t = 100000)]
    bytes: usize,
}

impl Args {
    pub fn new() -> Args {
        let args = Args::parse();

        args
    }

    pub fn connect_addr(&self) -> &String {
        &self.connect_addr
    }

    pub fn server_addr(&self) -> &String {
        &self.server_addr
    }

    pub fn bytes(&self) -> usize {
        self.bytes
    }
}
