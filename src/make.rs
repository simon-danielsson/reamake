use crate::constants::*;
use std::fs;
use std::path::Path;

pub struct ProjectOptions {
	pub client_name: Option<String>,
	pub project_name: Option<String>,
	pub bpm: Option<u32>,
	pub rpp_templ: Option<String>,
	pub yaml_templ: Option<String>,
	pub dest_dir: String,
}

pub fn make(input_opts: ProjectOptions) {
	let _opts = validate_opts(input_opts);
	println!("All paths have been validated! Generating structure...\n");
	// 1. parse yaml file and retrieve folder/file structure
	// 2. function to adjust BPM in every reaper project that exists in the given folder structure
}

/// Validate integrity of user input
pub fn validate_opts(mut opts: ProjectOptions) -> (ProjectOptions, String, String) {
	path_validation_helper(&opts.dest_dir, true);
	if opts.client_name.is_none() {
		println!("Client not set; defaulting to fallback '{}'.", DEF_CLI);
		opts.client_name = Some(DEF_CLI.to_string());
	}
	if opts.project_name.is_none() {
		println!("Project not set; defaulting to fallback '{}'.", DEF_PRO);
		opts.project_name = Some(DEF_PRO.to_string());
	}
	if opts.bpm.is_none() {
		println!("BPM not set; defaulting to fallback {}.", DEF_BPM);
		opts.bpm = Some(DEF_BPM);
	}
	let rpp_contents = load_file_or_default(opts.rpp_templ.as_ref(), DEF_RPP);
	let yaml_contents = load_file_or_default(opts.yaml_templ.as_ref(), DEF_YAM);
	(opts, rpp_contents, yaml_contents)
}

/// Returns contents of file on user path, or the default if omitted or invalid
fn load_file_or_default(user_path: Option<&String>, default_contents: &str) -> String {
	match user_path {
		Some(path) => match fs::read_to_string(path) {
			Ok(contents) => contents,
			Err(_) => {
				eprintln!("Could not read '{}'. Defaulting to fallback.", path);
				default_contents.to_string()
			}
		},
		None => {
			eprintln!(
				"RPP template and/or .yaml template not set. Defaulting to fallback.",
			);
			default_contents.to_string()
		}
	}
}

/// Check if a path exists and panic if the path is both required and missing
fn path_validation_helper(path: &str, panic_if_missing: bool) {
	if Path::new(path).exists() {
		println!("Valid path: {}", path);
	} else if panic_if_missing {
		panic!("Required path is invalid: {}\nTerminating program...", path);
	} else {
		eprintln!("Warning!: path '{}' does not exist.", path);
	}
}
