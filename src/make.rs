pub struct ProjectOptions {
	pub client: Option<String>,
	pub project: Option<String>,
	pub bpm: Option<u32>,
	pub template: Option<String>,
	pub structure: Option<String>,
	pub destin: String,
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
	if let Some(template) = opts.template {
		println!("Template: {}", template);
	}

	if let Some(structure) = opts.structure {
		println!("Structure: {}", structure);
	}
	println!("Destination: {}", opts.destin);

	println!("Creating project structure...");
	// Insert main project creation logic here
}
