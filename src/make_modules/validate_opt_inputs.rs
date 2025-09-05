use crate::constants::*;
use crate::make::ProjectOptions;
use progress_bar::*;
use std::fs;
use std::path::Path;

/// Validates input, replaces omitted options, and panics if the dest dir is missing - returns opts, rpp_contents and yaml_contents
pub fn run(mut opts: ProjectOptions) -> (ProjectOptions, String, String) {
	path_validation_helper(&opts.dest_dir, true);
	if opts.client_name.is_none() {
		print_progress_bar_info(
			"Info",
			"Client not set, defaulting to fallback.",
			Color::Blue,
			Style::Bold,
		);
		opts.client_name = Some(DEF_CLI.to_string());
	}
	if opts.project_name.is_none() {
		print_progress_bar_info(
			"Info",
			"Project not set, defaulting to fallback.",
			Color::Blue,
			Style::Bold,
		);
		opts.project_name = Some(DEF_PRO.to_string());
	}
	if opts.bpm.is_none() {
		print_progress_bar_info(
			"Info",
			"BPM not set, defaulting to fallback.",
			Color::Blue,
			Style::Bold,
		);
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
				print_progress_bar_info(
					"Info",
					"RPP template and/or .yaml template could not be read, defaulting to fallback.",
					Color::Blue,
					Style::Bold,
				);
				default_contents.to_string()
			}
		},
		None => {
			print_progress_bar_info(
				"Info",
				"RPP template and/or .yaml template not set, defaulting to fallback.",
				Color::Blue,
				Style::Bold,
			);
			default_contents.to_string()
		}
	}
}

/// Check if a path exists and panic if the path is both required and missing
fn path_validation_helper(path: &str, panic_if_missing: bool) {
	if Path::new(path).exists() {
		let error: String = format!("Valid destination path: '{}'", path);
		print_progress_bar_info("Info", &error, Color::Blue, Style::Bold);
	} else if panic_if_missing {
		let error: String =
			format!("Required path is invalid: {} Terminating program...", path);
		print_progress_bar_info("Error", &error, Color::Blue, Style::Bold);
		panic!();
	} else {
		let error: String = format!("Path does not exist: {}", path);
		print_progress_bar_info("Error", &error, Color::Blue, Style::Bold);
	}
}
