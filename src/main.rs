use clap::{Arg, Command};
pub mod args_info;
pub mod make;
pub mod subcommands;

fn main() {
	let args_info_vec = args_info::args_info();

	let matches = Command::new("reamake")
		.about(args_info_vec[0].1)
		.arg_required_else_help(true)
		.arg(Arg::new("client")
			.short(args_info_vec[1].0)
			.help(args_info_vec[1].1)
			.required(false))
		.arg(Arg::new("project")
			.short(args_info_vec[2].0)
			.help(args_info_vec[2].1)
			.required(false))
		.arg(Arg::new("bpm")
			.short(args_info_vec[3].0)
			.help(args_info_vec[3].1)
			.required(false))
		.arg(Arg::new("template")
			.short(args_info_vec[4].0)
			.help(args_info_vec[4].1)
			.required(false))
		.arg(Arg::new("structure")
			.short(args_info_vec[5].0)
			.help(args_info_vec[5].1)
			.required(false))
		.arg(Arg::new("destin").help(args_info_vec[6].1).required(false))
		.subcommand(
			Command::new("batch")
				.about(args_info_vec[7].1)
				.arg(Arg::new("batchfile").help("batch.csv").required(true))
				.arg(Arg::new("destination")
					.help("Destination folder")
					.required(true)),
		)
		.subcommand(
			Command::new("init")
				.about(args_info_vec[8].1)
				.arg(Arg::new("destination")
					.help("Destination folder")
					.required(true)),
		)
		.get_matches();

	// Dispatch subcommands
	if let Some(init_matches) = matches.subcommand_matches("init") {
		subcommands::init::run(init_matches);
		return;
	}

	if let Some(batch_matches) = matches.subcommand_matches("batch") {
		subcommands::batch::run(batch_matches);
		return;
	}

	// No subcommand -> run main project creation
	let template = matches
		.get_one::<String>("template")
		.expect("Template required");
	let destin = matches
		.get_one::<String>("destin")
		.expect("Destination required");

	make::create_project(make::ProjectOptions {
		client: matches.get_one::<String>("client").map(|s| s.as_str()),
		project: matches.get_one::<String>("project").map(|s| s.as_str()),
		bpm: matches
			.get_one::<String>("bpm")
			.and_then(|s| s.parse::<u32>().ok()),
		template,
		structure: matches.get_one::<String>("structure").map(|s| s.as_str()),
		destin,
	});
}
