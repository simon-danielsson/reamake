use crate::constants::*;
use crate::make_modules::*;
use progress_bar::*;
use std::fs;
use std::path::PathBuf;

pub fn run(master_folder_path: String, rpp_contents: String, bpm: String, date_str: String) {
	let mut matches = Vec::new();
	match collect_matching_files(master_folder_path.as_str(), ".RPP", &mut matches) {
		Ok(_) => (),
		Err(e) => {
			let error: String = format!(
				"Error occured while collecting .RPP projects in project directory: {}",
				e
			);
			print_progress_bar_info("Error", &error, Color::Red, Style::Bold);
			panic!();
		}
	}
	let rpp_contents: String = change_bpm_of_rpp::run(&rpp_contents, &bpm);
	let def_contents: String = change_bpm_of_rpp::run(&DEF_RPP.to_string(), &bpm);
	overwrite_rpps(&matches, rpp_contents, def_contents, date_str);
}
pub fn overwrite_rpps(
	matches: &Vec<PathBuf>,
	rpp_contents: String,
	alt_contents: String,
	date_str: String,
) {
	for m in matches {
		let file_name = m.file_name().and_then(|n| n.to_str()).unwrap_or_default();

		// Decide which contents to write
		let contents_to_write = if file_name.contains(&date_str) {
			&rpp_contents
		} else {
			&alt_contents
		};

		// Write
		if let Err(e) = fs::write(m, contents_to_write) {
			let error = format!("Failed to overwrite '{}': {}", m.display(), e);
			print_progress_bar_info("Error", &error, Color::Red, Style::Bold);
		}
	}
}

fn collect_matching_files(
	dir: &str,
	pattern: &str,
	matches: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
	let dir_path = PathBuf::from(dir);
	for entry in fs::read_dir(&dir_path)? {
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			if let Some(subdir) = path.to_str() {
				collect_matching_files(subdir, pattern, matches)?;
			}
		} else {
			if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
				if name.to_lowercase().ends_with(&pattern.to_lowercase()) {
					matches.push(path.clone());
				}
			}
		}
	}
	Ok(())
}
