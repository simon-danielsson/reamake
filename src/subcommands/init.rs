use crate::constants::DEF_CSV;
use crate::constants::DEF_YAM;
use clap::ArgMatches;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn run(matches: &ArgMatches) -> std::io::Result<()> {
	let destination = matches.get_one::<String>("destination").unwrap();
	println!("Generating init files at {}...", destination);

	match add_file_to_dir(destination, "default.yaml", DEF_YAM) {
		Ok(_) => println!("default.yaml added to the chosen directory!"),
		Err(e) => println!("Error while adding default.yaml to directory: {}", e),
	}
	match add_file_to_dir(destination, "default.csv", DEF_CSV) {
		Ok(_) => println!("default.csv added to the chosen directory!"),
		Err(e) => println!("Error while adding default.csv to directory: {}", e),
	}

	Ok(())
}

fn add_file_to_dir(destination: &str, filename: &str, contents: &str) -> std::io::Result<()> {
	let path = PathBuf::from(destination).join(filename);
	let mut file = File::create(path)?;
	file.write_all(contents.as_bytes())?;
	Ok(())
}
