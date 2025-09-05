use crate::make_modules::file_entry::FileEntry;
use progress_bar::*;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

pub fn run(
	file_entries_modified: Vec<FileEntry>,
	dest_dir: &String,
	project_name: &String,
	client_name: &String,
	date_str: &String,
) -> String {
	let master_folder = format!(
		"{}_{}_{}_{}",
		file_entries_modified[0].path.split("/").next().unwrap(),
		project_name
			.trim()
			.to_lowercase()
			.replace('_', "_")
			.replace(' ', "_")
			.replace('-', "_"),
		client_name
			.trim()
			.to_lowercase()
			.replace('_', "_")
			.replace(' ', "_")
			.replace('-', "_"),
		date_str
	);

	let path_vectors: (Vec<String>, Vec<String>) =
		create_path_vectors(&master_folder, &file_entries_modified);

	for vector in [path_vectors.0, path_vectors.1] {
		match write_to_sys(vector, dest_dir) {
			Ok(_) => continue,
			Err(e) => {
				let error: String =
					format!("Error occured when writing to system : {}", e);
				print_progress_bar_info("Error", &error, Color::Red, Style::Bold);
				panic!();
			}
		}
	}
	format!("{}/{}/", dest_dir.trim_end_matches("/"), master_folder)
}

fn write_to_sys(mut path_vector: Vec<String>, dest_dir: &String) -> std::io::Result<()> {
	path_vector.sort_by_key(|s| s.matches("/").count());
	for path in path_vector {
		let full_path = format!("{}/{}", dest_dir.trim_end_matches("/"), path);
		match write_path_auto(&full_path) {
			Ok(_) => continue,
			Err(e) => {
				let error: String = format!(
					"Error occured while creating file '{}': {}",
					path, e
				);
				print_progress_bar_info("Error", &error, Color::Red, Style::Bold);
				panic!();
			}
		}
	}
	Ok(())
}

fn write_path_auto(path: &str) -> io::Result<()> {
	if path.ends_with('/') {
		fs::create_dir_all(path)?;
	} else {
		if let Some(parent) = Path::new(path).parent() {
			fs::create_dir_all(parent)?;
		}
		File::create(path)?;
	}
	Ok(())
}

fn create_path_vectors(
	master_folder: &String,
	file_entries_modified: &Vec<FileEntry>,
) -> (Vec<String>, Vec<String>) {
	let mut directory_paths: Vec<String> = Vec::new();
	let mut file_paths = Vec::new();

	for entry in file_entries_modified {
		let relative_path = entry
			.path_as_str()
			.splitn(2, "/")
			.nth(1)
			.unwrap()
			.to_string();

		if entry.is_folder {
			let path_name = format!("{}/{}/", master_folder, relative_path);
			directory_paths.push(path_name);
		} else {
			let path_name = format!("{}/{}", master_folder, relative_path);
			file_paths.push(path_name);
		}
	}
	directory_paths.push(format!("{}/", master_folder));
	(directory_paths, file_paths)
}
