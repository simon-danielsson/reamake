pub fn run(rpp_contents: &String, new_bpm: &String) -> String {
	let new_bpm_as_string: &str = new_bpm.as_str();

	let mut original_bpm: String = String::new();
	let mut new_line: String = String::new();
	let mut lines_as_vec: Vec<String> = Vec::new();

	for line in rpp_contents.lines() {
		lines_as_vec.push(line.to_string());
		if line.contains("TEMPO") {
			for word in line.split_ascii_whitespace() {
				if word.parse::<u32>().is_ok() {
					original_bpm = word.to_string();
					break;
				}
			}
			new_line = line.replace(&original_bpm, new_bpm_as_string);
		}
	}

	for i in 0..lines_as_vec.len() {
		if lines_as_vec[i].contains("TEMPO") {
			lines_as_vec[i] = new_line.clone();
			break;
		}
	}
	let new_rpp_contents: String = lines_as_vec.join("\n");

	new_rpp_contents
}
