use crate::constants::DEF_CSV;
use crate::constants::DEF_YAM;
use clap::ArgMatches;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn run(matches: &ArgMatches) -> std::io::Result<()> {
	let destination = matches.get_one::<String>("destination").unwrap();
	println!("Generating init files at {}...", destination);
	match add_to_dir(destination) {
		Ok(_) => println!("Init files added to the chosen directory!"),
		Err(e) => println!("Error while adding to directory: {}", e),
	}
	Ok(())
}

fn add_to_dir(destination: &String) -> std::io::Result<()> {
	let yaml_path = PathBuf::from(destination).join("default.yaml");
	let mut yaml_file = File::create(yaml_path)?;
	yaml_file.write_all(DEF_YAM.as_bytes())?;
	let csv_path = PathBuf::from(destination).join("default.csv");
	let mut csv_file = File::create(csv_path)?;
	csv_file.write_all(DEF_CSV.as_bytes())?;
	Ok(())
}
