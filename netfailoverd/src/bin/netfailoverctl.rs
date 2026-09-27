use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "netfailoverctl", about = "Control netfailoverd")]
struct Args {
    #[arg(long, default_value = "/var/run/netfailover.state")]
    state: PathBuf,
    #[arg(long, default_value = "/var/run/netfailover.disabled")]
    lock: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Status,
    Disable,
    Resume,
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Cmd::Status => {
            let state = fs::read_to_string(&args.state).unwrap_or_else(|_| "unknown\n".to_string());
            let disabled = args.lock.exists();
            println!("disabled: {disabled}");
            print!("{state}");
        }
        Cmd::Disable => {
            let _ = fs::write(&args.lock, "disabled\n");
            println!("disabled");
        }
        Cmd::Resume => {
            let _ = fs::remove_file(&args.lock);
            println!("resumed");
        }
    }
}
