// *brakoll - d: initial setup, p: 0, t: feature, s: closed
// *brakoll - d: support for macos linux and windows this time around, p: 100, t: feature, s: open
// *brakoll - d: custom config file parser instead of csv, p: 100, t: feature, s: open

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

    println!("hello world");

    Ok(())
}