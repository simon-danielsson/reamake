#[derive(Debug)]
pub struct FileEntry {
	pub path: String,
	pub is_folder: bool,
	pub original_name: String,
	pub content_type: Option<String>,
}
