use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
	let destination = matches.get_one::<String>("destination").unwrap();
	println!("Initializing project at {}", destination);

	// Put your init logic here, e.g., creating template files
}
