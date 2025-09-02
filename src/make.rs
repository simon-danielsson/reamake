use crate::make_modules;
use crate::make_modules::*;

pub struct ProjectOptions {
	pub client_name: Option<String>,
	pub project_name: Option<String>,
	pub bpm: Option<u32>,
	pub rpp_templ: Option<String>,
	pub yaml_templ: Option<String>,
	pub dest_dir: String,
}

pub fn make(input_opts: ProjectOptions) {
	// 1. validate input options
	let opts = validate_opt_inputs::run(input_opts);
	println!("All paths have been validated!\n");

	// 2. parse yaml file and retrieve folder/file structure
	let file_struct_vec = make_modules::parse_yaml::run(opts.2);
	println!("Yaml file has been parsed successfully!");
	println!("{:?}", file_struct_vec);

	// 3. function to adjust BPM in every reaper project that exists in the given folder structure
}
