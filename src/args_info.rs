pub fn args_info() -> Vec<(char, &'static str)> {
	let mut helper = Helper::new();

	let about = format!(
		"\nreamake v{}\nCreate Reaper project structures!\nBuilt by Simon Danielsson.\nhttps://github.com/simon-danielsson/reamake",
		env!("CARGO_PKG_VERSION")
	);
	let about_static: &'static str = Box::leak(about.into_boxed_str());

	// about [0]
	helper.add('h', about_static);

	// client [1]
	helper.add(
		'c',
		"Set the client name.\nOptional; defaults to a generic name if omitted.\n(All words are normalized to lower case with underscores, e.g 'cool_client.)\nExample usage: -c 'cool client'\n",
	);

	// project [2]
	helper.add(
		'p',
		"Set the project name.\nOptional; defaults to a generic name if omitted.\n(All words are normalized to lower case with underscores, e.g 'cool_project.)\nExample usage: -p 'cool project'\n",
	);

	// bpm [3]
	helper.add(
		'b',
		"Set the bpm.\nOptional; defaults to 120 BPM if omitted.\nExample usage: -b 114\n",
	);

	// template file [4]
	helper.add(
		't',
                "Sets the absolute path to a reaper project template file (.RPP).\nExample usage: -p 'Users/user/Desktop/music/mixing-projects/templates/mixing.RPP'\n",
	);

	// structure file [5]
	helper.add(
		's',
                "Sets the absolute path to a folder/file structure template (.yaml).\nOptional; defaults to a standard structure if omitted.\nExample usage: -s 'Users/user/Desktop/music/mixing-projects/templates/structure.yaml'\n",
	);

	// destination directory [6]
	helper.add('d', "Absolute path to a destination folder.");

	// batch [7]
	helper.add(
		'b',
                "Provide all flags through a .csv file.\nCreate several structures at once using a single command. Add path to .csv, and then the destination path. \nExample: reamake batch 'Users/user/Desktop/music/mixing-projects/templates/batch.csv' 'Users/user/Desktop/music/mixing-projects'\n",
	);

	// initialized file creation [8]
	helper.add(
		'i', "Create initialized batch.csv and structure.yaml files in chosen directory for further customization.\nExample: reamake init 'Users/user/Desktop/music/mixing-projects/templates/)\n",
        );

	helper.into_vec()
}

struct Helper(Vec<(char, &'static str)>);
impl Helper {
	fn new() -> Self {
		Self(Vec::new())
	}
	fn add(&mut self, flag: char, descr: &'static str) {
		self.0.push((flag, descr))
	}
	fn into_vec(self) -> Vec<(char, &'static str)> {
		self.0
	}
}
