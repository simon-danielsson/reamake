use std::{collections::HashMap, io, path::PathBuf};

// *brakoll - d: support for overriding any variables that the user desires, p: 100, t: feature, s: closed
#[derive(PartialEq, Clone)]
pub struct Arguments {
    pub help: bool,
    pub init: bool,
    pub reamake_file_path: String,
    pub opt_target: PathBuf,
    pub overrides: HashMap<String, String>,
}
impl Arguments {
    fn new() -> Self {
        Self {
            help: false,
            init: false,
            reamake_file_path: String::new(),
            opt_target: PathBuf::new(),
            overrides: HashMap::new(),
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

            "-var" | "--var" | "-v" => {
                let arg = it.next().expect(
                    "No key-value pair was given after the \"-S\" flag.",
                );
                // println!("{}", arg);
                let (k, v): (String, String) = get_kv_from_var(arg);
                a.overrides.insert(k, v);
            }

            "init" => {
                a.init = true;
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

// --var
fn get_kv_from_var(arg: String) -> (String, String) {
    let mut k = String::new();
    let mut v = String::new();

    let iter = arg.split_terminator('=').into_iter();
    for (i, field) in iter.enumerate() {
        if i == 0 {
            k = field.trim().to_string();
        }
        if i == 1 {
            v = field.trim().to_string();
        }
    }

    // println!("{} == {}", k, v);
    (k, v)
}