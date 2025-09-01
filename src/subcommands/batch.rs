use crate::constants::DEF_BPM;
use crate::make::{self, ProjectOptions};
use clap::ArgMatches;
use std::io::Read;

pub fn run(matches: &ArgMatches) -> std::io::Result<()> {
	let batchfile = matches.get_one::<String>("batchfile").unwrap();
	println!("Processing batch {}", batchfile);
	let rows_vec = parse_lines(&batchfile)?;
	let mut projects: Vec<Vec<(String, String)>> = Vec::new();
	for chunk in rows_vec.chunks(6) {
		projects.push(chunk.to_vec());
	}
	send_to_make(projects);
	Ok(())
}

fn send_to_make(projects: Vec<Vec<(String, String)>>) {
	for chunk in projects {
		let bpm: u32 = match chunk[2].1.parse() {
			Ok(val) => val,
			Err(_) => {
				eprintln!(
					"Invalid BPM '{}' for project '{}'. Using default BPM {}.",
					chunk[2].1, chunk[1].1, DEF_BPM
				);
				DEF_BPM
			}
		};
		let opts = ProjectOptions {
			client: Some(chunk[0].1.clone()),
			project: Some(chunk[1].1.clone()),
			bpm: Some(bpm),
			template: Some(chunk[3].1.clone()),
			structure: Some(chunk[4].1.clone()),
			destin: chunk[5].1.clone(),
		};
		make::create_project(opts);
	}
}

fn parse_lines(csv_file_path: &str) -> std::io::Result<Vec<(String, String)>> {
	let mut contents = String::new();
	std::fs::File::open(csv_file_path)?.read_to_string(&mut contents)?;
	let mut rows = Vec::new();
	for line in contents.lines() {
		let fields: Vec<&str> = line.split(',').collect();
		if fields.len() == 2 {
			rows.push((fields[0].trim().to_string(), fields[1].trim().to_string()));
		} else {
			eprintln!("Warning! Malformed line: {}", line);
		}
	}
	Ok(rows)
}
