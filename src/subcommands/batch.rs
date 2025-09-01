use crate::constants::DEF_BPM;
use crate::make::{self, ProjectOptions};
use clap::ArgMatches;
use std::io::Read;

pub fn run(matches: &ArgMatches) -> std::io::Result<()> {
	let batchfile = matches.get_one::<String>("batchfile").unwrap();
	println!("Processing batch {}\n", batchfile);

	let projects = parse_lines(&batchfile)?;
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
			client_name: Some(chunk[0].1.clone()),
			project_name: Some(chunk[1].1.clone()),
			bpm: Some(bpm),
			rpp_templ: Some(chunk[3].1.clone()),
			yaml_templ: Some(chunk[4].1.clone()),
			dest_dir: chunk[5].1.clone(),
		};

		make::make(opts);
	}
}

fn parse_lines(csv_file_path: &str) -> std::io::Result<Vec<Vec<(String, String)>>> {
	let mut contents = String::new();
	std::fs::File::open(csv_file_path)?.read_to_string(&mut contents)?;
	let mut rows = Vec::new();
	for line in contents.lines() {
		let line = line.trim();
		if line.is_empty() {
			continue;
		}

		let fields: Vec<&str> = line.split(',').map(|f| f.trim()).collect();

		// Skip the line if any field contains '#'
		if fields.iter().any(|f| f.contains('#')) {
			continue;
		}

		if fields.len() == 2 {
			rows.push((fields[0].to_string(), fields[1].to_string()));
		} else {
			eprintln!("Warning! Malformed line: {}", line);
		}
	}

	let mut projects: Vec<Vec<(String, String)>> = Vec::new();
	for chunk in rows.chunks(6) {
		projects.push(chunk.to_vec());
	}

	Ok(projects)
}
