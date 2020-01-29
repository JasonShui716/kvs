use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Set the value of key")]
    Set {
        #[structopt(help = "Key string")]
        key: String,
        #[structopt(help = "Value string")]
        value: String,
    },
    #[structopt(about = "Get the value of key")]
    Get {
        #[structopt(help = "Key string")]
        key: String,
    },
    #[structopt(name = "rm", about = "Remove a key")]
    Remove {
        #[structopt(help = "Key string")]
        key: String,
    },
}

fn exit_err() {
    eprintln!("unimplemented");
    std::process::exit(1);
}

fn main() {
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Get { key: _k } => exit_err(),
        Command::Set { key: _k, value: _v } => exit_err(),
        Command::Remove { key: _k } => exit_err(),
    }
}
