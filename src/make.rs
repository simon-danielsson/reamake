use crate::make_modules::*;
use chrono::Local;
use progress_bar::*;

#[derive(Clone)]
pub struct ProjectOptions {
	pub client_name: Option<String>,
	pub project_name: Option<String>,
	pub bpm: Option<u32>,
	pub rpp_templ: Option<String>,
	pub yaml_templ: Option<String>,
	pub dest_dir: String,
}

impl ProjectOptions {
	pub fn item_as_string(&self, field: &str) -> String {
		match field {
			"client_name" => self.client_name.clone().unwrap_or_default(),
			"project_name" => self.project_name.clone().unwrap_or_default(),
			"bpm" => self.bpm.map(|v| v.to_string()).clone().unwrap_or_default(),
			"rpp_templ" => self.rpp_templ.clone().unwrap_or_default(),
			"yaml_templ" => self.yaml_templ.clone().unwrap_or_default(),
			"dest_dir" => self.dest_dir.clone(),
			_ => panic!(
				"ProjectOptions::item_as_string attempted to match an invalid field: {}",
				field
			),
		}
	}
}

pub fn make(input_opts: ProjectOptions) {
	init_progress_bar(4);

	let opts = validate_opt_inputs::run(input_opts.clone());

	let dest_dir = opts.0.item_as_string("dest_dir");
	let project_name = opts.0.item_as_string("project_name");
	let client_name = opts.0.item_as_string("client_name");
	let bpm = opts.0.item_as_string("bpm");
	let date_str = Local::now().format("%d-%m-%Y").to_string();

	print_progress_bar_info("Creating", &project_name, Color::Green, Style::Bold);

	inc_progress_bar();

	let file_struct_vec = parse_yaml::run(&opts.2);

	inc_progress_bar();

	let file_entries_modified =
		normalize_names::run(file_struct_vec, &project_name, &client_name, &date_str);

	inc_progress_bar();

	let master_folder_path: String = create_structure::run(
		file_entries_modified,
		&dest_dir,
		&project_name,
		&client_name,
		&date_str,
	);

	inc_progress_bar();

	// replace contents of main.RPP project with the given template contents
	// modify BPM in all reaper projects in the destination folder recursively
	modify_rpp_contents::run(master_folder_path, opts.1, bpm, date_str);

	finalize_progress_bar();
}
