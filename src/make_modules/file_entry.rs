#[derive(Debug, Clone)]
pub struct FileEntry {
	pub path: String,
	pub is_folder: bool,
	pub original_name: String,
	pub content_type: Option<String>,
}
impl FileEntry {
	pub fn path_as_str(&self) -> String {
		self.path.clone()
	}
}
