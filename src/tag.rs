use audiotags::{Tag};
use std::path::Path;

pub fn tag_title(file: &Path, title: &str) {
	let mut tag = Tag::new().read_from_path(file).unwrap();

	tag.set_title(title);

	tag
		.write_to_path(file.to_str().unwrap_or("file.mp3"))
		.expect("Fail to save");
}
