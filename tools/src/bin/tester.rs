use clap::Parser;
use std::{
    ffi::OsString,
    process::{self, Stdio},
};
use tools::interact;

#[derive(Parser, Debug)]
struct Args {
    #[clap(help = "Command to run your program")]
    command: OsString,
    #[clap(help = "Arguments to pass to your program")]
    args: Vec<OsString>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut process = process::Command::new(&args.command)
        .args(&args.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let score = match interact(&mut process) {
        Ok(score) => score,
        Err(err) => {
            if let Ok(Some(status)) = process.try_wait() {
                if !status.success() {
                    std::process::exit(1);
                }
            }

            process.kill()?;
            eprintln!("{:?}", err);
            0
        }
    };

    eprintln!("Score = {}", score);

    Ok(())
}
