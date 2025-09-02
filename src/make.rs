use crate::make_modules::*;

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
			"bpm" => self.bpm.map(|v| v.to_string()).unwrap_or_default(),
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
	// validate input
	let mut opts = validate_opt_inputs::run(input_opts.clone());
	println!("All paths have been validated!\n");

	// parse yaml file and retrieve folder/file structure
	let file_struct_vec = parse_yaml::run(&opts.2);
	println!("Yaml file has been parsed successfully!");
	println!("{:?}", file_struct_vec);

	// modify BPM in a reaper project (should be done at the end after the files have been created)
	let new_bpm = input_opts.item_as_string("bpm").clone();
	change_bpm_of_rpp::run(reaper_file_contents, new_bpm);
}
