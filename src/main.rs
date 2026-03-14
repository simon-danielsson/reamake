// *brakoll - d: initial setup, p: 0, t: feature, s: closed

use std::{fmt, io};

use crate::utils::parser::Node;

mod subcommands;
mod utils;

fn main() -> io::Result<()> {
    // === get args ===
    let args = utils::args::parse()?;

    if args.help {
        subcommands::help::print();
        return Ok(());
    }

    let mut r = Reamake::new();

    // get path to reamake file
    let path = "../test/mix.reamake";

    // parse batch file
    let mut project = r.parse_reamake_file(path.to_string())?;

    project.debug_print();

    Ok(())
}

#[derive(Debug)]
enum DateFormat {
    US,
    EU,
    ISO,
}
impl fmt::Display for DateFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DateFormat::US => "US",
            DateFormat::EU => "EU",
            DateFormat::ISO => "ISO",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
struct ProjectBlock {
    // variables
    var_client: String,
    var_project: String,
    var_service: String,
    // sources
    src_rpp: String,
    // settings
    set_kebab: bool,
    set_date: DateFormat,
    // hierarchy
    hierarchy: Vec<Node>,
}

impl ProjectBlock {
    fn new() -> Self {
        Self {
            // variables
            var_client: String::new(),
            var_project: String::new(),
            var_service: String::new(),
            // sources
            src_rpp: String::new(),
            // settings
            set_date: DateFormat::EU,
            set_kebab: false,
            // hierarchy
            hierarchy: Vec::new(),
        }
    }

    fn debug_print(&mut self) {
        println!("[variables]");
        println!("client: {}", self.var_client);
        println!("project: {}", self.var_project);
        println!("service: {}", self.var_service);
        println!("[sources]");
        println!("rpp: {}", self.src_rpp);
        println!("[settings]");
        println!("date: {}", self.set_date);
        println!("text: {}", self.set_kebab);
        println!("[hierarchy]");
        println!("{:#?}", self.hierarchy);
    }
}

#[allow(dead_code)]
struct Reamake {
    project_blocks: Vec<ProjectBlock>,
}

impl Reamake {
    fn new() -> Self {
        Self {
            project_blocks: Vec::new(),
        }
    }
}
