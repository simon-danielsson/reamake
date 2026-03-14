// *brakoll - d: initial setup, p: 0, t: feature, s: closed

use std::io;

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

    // parse batch file
    let path = "../test/mix.reamake";
    r.batch_parser(path.to_string())?;

    Ok(())
}

#[derive(Debug)]
enum DateFormat {
    US,
    EU,
    ISO,
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
    hierarchy: String,
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
            hierarchy: String::new(),
        }
    }
}

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
