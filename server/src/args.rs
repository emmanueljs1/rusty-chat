use structopt::StructOpt;

/// stracer argument parser
#[derive(StructOpt, Debug)]
#[structopt(name = "chat server", about = "A chat server written in Rust")]
pub struct Opt {
    #[structopt(short = "r", long = "remote")]
    /// run server remotely, otherwise run on localhost
    pub remote: bool,
}
