use crate::make::{self, ProjectOptions};
use clap::ArgMatches;
use std::str::FromStr;

pub fn run(matches: &ArgMatches) {
	let batchfile = matches.get_one::<String>("batchfile").unwrap();
	let destination = matches.get_one::<String>("destination").unwrap();

	println!("Processing batch '{}' to '{}'", batchfile, destination);

	let rows_vec = parse_lines(&batchfile);

	let bpm: u32 = FromStr::from_str(rows_vec[2].1).unwrap();

	let projects = vec![
		ProjectOptions {
			client: Some(rows_vec[0].1),
			project: Some(rows_vec[1].1),
			bpm: Some(bpm),
			template: rows_vec[3].1,
			structure: Some(rows_vec[4].1),
			destin: rows_vec[5].1,
		},
		{ //make this entire projects variable dynamic depending on how many rows of structures are in the given csv file},
	];

	for opts in projects {
		make::create_project(opts);
	}
}

fn parse_lines(csv_file: &str) -> Vec<(&str, &str)> {
	let mut rows: Vec<(&str, &str)> = Vec::new();
	for line in csv_file.lines() {
		let fields: Vec<&str> = line.split(',').collect();
		if fields.len() == 2 {
			rows.push((fields[0].trim(), fields[1].trim()));
		}
	}
	rows
}
