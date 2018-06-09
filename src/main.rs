extern crate uuid;

use std::path::Path;
use std::ffi::OsStr;
use uuid::Uuid;
use std::io::Result;

fn generate_new_name() -> String {

	let uuid = Uuid::new_v4();
	return uuid.hyphenated().to_string();
}

fn on_file_found(e: &Path) -> Result<()> {

	let parent = match e.parent() {
		Some(d) => d,
		None => Path::new("")
	};

	let name = generate_new_name();

	let ext = match e.extension() {
		Some(s) => s,
		None => OsStr::new("")
	};

	let new_path = parent.join(&name).with_extension(ext);
	println!("{}", new_path.as_os_str().to_str().unwrap());

	std::fs::rename(e, new_path)?;

	return Ok(());
}

#[allow(unused)]
fn enumerate(e: &Path, handler: &Fn(&Path) -> Result<()>) -> Result<()> {

	if !e.exists() {
		println!("[TRACE] invalid path {}", e.to_str().unwrap());
		return Ok(());
	}
	else if e.is_dir() {
		let it = std::fs::read_dir(e)?;
		for e in it {
			let entry = e.unwrap();
			let path = entry.path();
			enumerate(&path, handler);
		}
		return Ok(());
	}
	else {
		return handler(e);
	}
}

fn main() {

	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("path?");
		return;
	}

	for e in &args[1..] {
		let path = Path::new(e);
		let _ = enumerate(path, &on_file_found);
	}
}
