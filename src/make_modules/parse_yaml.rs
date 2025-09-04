use crate::make_modules::file_entry::FileEntry;
use yaml_rust2::Yaml;

pub fn run(yaml_contents: &String) -> Vec<FileEntry> {
	let docs = yaml_rust2::YamlLoader::load_from_str(&yaml_contents).expect("Invalid YAML");
	let doc = &docs[0];
	let mut entries: Vec<FileEntry> = Vec::new();
	if let Yaml::Hash(map) = doc {
		if let Some((root_key, root_value)) = map.iter().next() {
			let root_name = root_key.as_str().unwrap_or("default_root");
			flatten_yaml(root_value, root_name, &mut entries); // Start *inside* root
		}
	}
	entries
}

fn flatten_yaml(yaml: &Yaml, parent_path: &str, entries: &mut Vec<FileEntry>) {
	match yaml {
		Yaml::String(name) => {
			let path = format!("{}/{}", parent_path, name);
			let content_type = if name == "main.RPP" {
				Some("main".to_string())
			} else if name.ends_with(".RPP") {
				Some("other".to_string())
			} else {
				None
			};
			entries.push(FileEntry {
				path,
				is_folder: false,
				original_name: name.clone(),
				content_type,
			});
		}
		Yaml::Hash(map) => {
			for (k, v) in map {
				if let Yaml::String(folder_name) = k {
					let path = format!("{}/{}", parent_path, folder_name);
					entries.push(FileEntry {
						path: path.clone(),
						is_folder: true,
						original_name: folder_name.clone(),
						content_type: None,
					});
					flatten_yaml(v, &path, entries);
				}
			}
		}
		Yaml::Array(arr) => {
			for item in arr {
				flatten_yaml(item, parent_path, entries);
			}
		}
		_ => {}
	}
}
