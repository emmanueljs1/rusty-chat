use structopt::StructOpt;

/// stracer argument parser
#[derive(StructOpt, Debug)]
#[structopt(name = "chat server client", about = "A client that connects to a chat server")]
pub struct Opt {
    #[structopt(name = "ip_addr")]
    /// run server remotely
    pub ip_addr: Option<String>,
}
