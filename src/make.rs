pub struct ProjectOptions<'a> {
	pub client: Option<&'a str>,
	pub project: Option<&'a str>,
	pub bpm: Option<u32>,
	pub template: &'a str,
	pub structure: Option<&'a str>,
	pub destin: &'a str,
}

pub fn create_project(opts: ProjectOptions) {
	if let Some(client) = opts.client {
		println!("Client name: {}", client);
	}

	if let Some(project) = opts.project {
		println!("Project name: {}", project);
	}

	if let Some(bpm) = opts.bpm {
		println!("BPM: {}", bpm);
	}

	println!("Template: {}", opts.template);
	if let Some(structure) = opts.structure {
		println!("Structure: {}", structure);
	}
	println!("Destination: {}", opts.destin);

	// Insert main project creation logic here
	println!("Creating project structure...");
}
