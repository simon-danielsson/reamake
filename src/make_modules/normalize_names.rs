use crate::make_modules::file_entry::FileEntry;

pub fn run(
	file_struct_vec: Vec<FileEntry>,
	project_name: &String,
	client_name: &String,
	date_str: &String,
) -> Vec<FileEntry> {
	let mut entries = file_struct_vec.clone();
	normalize_file_entries(&mut entries, project_name, client_name, date_str);
	entries
}

pub fn normalize_file_entries(
	entries: &mut Vec<FileEntry>,
	project_name: &str,
	_client_name: &str,
	date_str: &String,
) {
	for entry in entries.iter_mut() {
		let segments: Vec<String> = entry
			.path
			.split('/')
			.map(|s| normalize_segment(s))
			.collect();
		let parent_path = segments[..segments.len() - 1]
			.iter()
			.map(|s| normalize_segment(s))
			.collect::<Vec<_>>()
			.join("/");
		let mut normalized_path = segments.join("/");
		let original_file = normalize_segment(
			segments.last()
				.unwrap()
				.trim_end_matches(".RPP")
				.trim_end_matches(".rpp"),
		);
		let project_name = normalize_segment(project_name);
		if let Some(content_type) = &entry.content_type {
			if content_type == "main" {
				normalized_path = format!(
					"{}/{}_{}.RPP",
					parent_path, project_name, date_str
				);
			} else if content_type == "other" {
				normalized_path = format!(
					"{}/{}_{}.RPP",
					parent_path, project_name, original_file,
				);
			}
		}
		entry.path = normalized_path;
		if !entry.is_folder {
			entry.original_name = normalize_segment(&entry.original_name);
		}
	}
}

fn normalize_segment(segment: &str) -> String {
	segment.trim()
		.to_lowercase()
		.replace('_', "_")
		.replace(' ', "_")
		.replace('-', "_")
}
