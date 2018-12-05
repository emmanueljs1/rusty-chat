use structopt::StructOpt;

/// stracer argument parser
#[derive(StructOpt, Debug)]
#[structopt(name = "chat server client", about = "A client that connects to a chat server")]
pub struct Opt {
    #[structopt(name = "ip_addr")]
    /// IP address to connect to (defaults to localhost)
    pub ip_addr: Option<String>,
}
