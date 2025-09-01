use crate::constants::*;

pub fn args_info() -> Vec<(char, &'static str)> {
	let mut info = Info::new();

	let about = format!(
		"\nreamake v{}\nBuilt by Simon Danielsson.\nhttps://github.com/simon-danielsson/reamake/",
		RMK_VER.to_string()
	);
	let about_st: &'static str = Box::leak(about.into_boxed_str());

	// about [0]
	info.add('h', about_st);

	// client [1]
	info.add(
		'c',
		"Set the client name.\nOptional; defaults to a generic name if omitted.\n(All files and folders are normalized to snake case, e.g 'cool_client'.)\nExample usage: -c 'cool client'\n",
	);

	// project [2]
	info.add(
		'p',
		"Set the project name.\nOptional; defaults to a generic name if omitted.\n(All files and folders are normalized to snake case, e.g 'cool_project'.)\nExample usage: -p 'cool project'\n",
	);

	let bpm = format!(
		"Set the bpm.\nOptional; defaults to {} BPM if omitted.\nExample usage: -b 114\n",
		DEF_BPM
	);
	let bpm_st: &'static str = Box::leak(bpm.into_boxed_str());

	// bpm [3]
	info.add('b', bpm_st);

	// template file [4]
	info.add(
		't',
                "Sets the absolute path to a reaper project template file (.RPP).\nOptional; defaults to an empty project if omitted.\nExample usage: -p 'Users/user/Desktop/music/mixing-projects/templates/mixing.RPP'\n",
	);

	// structure file [5]
	info.add(
		's',
                "Sets the absolute path to a folder/file structure template (.yaml).\nOptional; defaults to a standard structure if omitted.\nExample usage: -s 'Users/user/Desktop/music/mixing-projects/templates/structure.yaml'\n",
	);

	// destination directory [6]
	info.add('d', "Absolute path to a destination folder.");

	// batch [7]
	info.add(
		'b',
                "Provide all flags through a .csv file.\nCreate several structures at once using a single command. \nExample: reamake batch 'Users/user/Desktop/music/mixing-projects/templates/batch.csv'\n",
	);

	// initialized file creation [8]
	info.add(
		'i', "Create initialized batch.csv and structure.yaml files in target directory for further customization.\nExample: reamake init 'Users/user/Desktop/music/mixing-projects/templates/'\n",
        );

	info.into_vec()
}

struct Info(Vec<(char, &'static str)>);
impl Info {
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
