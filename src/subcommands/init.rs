use std::{
    env,
    fs::File,
    io::{self, Write},
};

use crate::utils::args::Arguments;

const INIT_REAMAKE: &str = include_str!("../static/init/init.reamake");

// *brakoll - d: implement 'init' subcommand that gen_inits template files (perhaps a .reamake file and a stock .rpp file) in current directory (or opt_path), p: 100, t: feature, s: closed
pub fn gen_init(args: Arguments) -> io::Result<()> {
    let mut target = env::current_dir()?;
    if args.opt_target.exists() && args.opt_target.is_dir() {
        target = args.opt_target;
    }

    let filename = "init.reamake";
    target = target.join(filename);

    let mut file = File::create(&target)?;
    file.write_all(INIT_REAMAKE.as_bytes())?;

    println!(
        "Initialized reamake file has been gen_initd into:\n{}",
        target.display()
    );

    Ok(())
}