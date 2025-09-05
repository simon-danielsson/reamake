pub fn run(rpp_contents: &str, new_bpm: &str) -> String {
	let mut new_rpp_lines = Vec::with_capacity(rpp_contents.lines().count());
	let mut tempo_updated = false;
	for line in rpp_contents.lines() {
		if !tempo_updated && line.trim_start().starts_with("TEMPO ") {
			let mut parts = line.trim_start().splitn(2, char::is_whitespace);
			let tempo_keyword = parts.next().unwrap_or("TEMPO");
			let rest_of_line = parts.next().unwrap_or("").trim_start();
			let mut rest_parts = rest_of_line.splitn(2, char::is_whitespace);
			let _old_bpm = rest_parts.next().unwrap_or("");
			let remaining = rest_parts.next().unwrap_or("");
			let new_line = if remaining.is_empty() {
				format!("{} {}", tempo_keyword, new_bpm)
			} else {
				format!("{} {} {}", tempo_keyword, new_bpm, remaining)
			};
			new_rpp_lines.push(new_line);
			tempo_updated = true;
		} else {
			new_rpp_lines.push(line.to_string());
		}
	}
	new_rpp_lines.join("\n")
}
