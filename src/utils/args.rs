use std::{io, path::PathBuf};

// use crate::IssueStatus;

#[derive(PartialEq, Clone)]
pub struct Arguments {
    pub help: bool,
    pub reamake_file_path: String,
    pub opt_target: PathBuf,
    pub client: String,
    pub project: String,
    pub service: String,
}
impl Arguments {
    fn new() -> Self {
        Self {
            help: false,
            reamake_file_path: String::new(),
            opt_target: PathBuf::new(),
            client: String::new(),
            project: String::new(),
            service: String::new(),
        }
    }
}

pub fn parse() -> io::Result<Arguments> {
    let mut a = Arguments::new();
    let mut it = std::env::args().skip(1); // skip program name

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-f" => {
                let arg = it.next().expect(
                    "No reamake file path was given after the \"-f\" flag.",
                );
                a.reamake_file_path = arg.trim().to_string();
            }

            "-s" => {
                let arg = it
                    .next()
                    .expect("No service name was given after the \"-s\" flag.");
                a.service = arg.trim().to_string();
            }

            "-p" => {
                let arg = it
                    .next()
                    .expect("No project name was given after the \"-p\" flag.");
                a.project = arg.trim().to_string();
            }

            "-c" => {
                let arg = it
                    .next()
                    .expect("No client name was given after the \"-c\" flag.");
                a.client = arg.trim().to_string();
            }

            "help" => {
                a.help = true;
            }

            other => {
                a.opt_target = PathBuf::from(other);
                break;
            }
        }
    }
    Ok(a)
}
