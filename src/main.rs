extern crate uuid;

use std::ffi::OsStr;
use std::path::Path;
use uuid::Uuid;

/// 新しい名前を生成します。
///
/// # Returns
/// 文字列
fn generate_new_name() -> String {
	let uuid = Uuid::new_v4();
	return uuid.hyphenated().to_string();
}

/// 標準入力から一行の入力を得ます。
///
/// # Returns
/// 入力(改行の手前まで)
fn input() -> String {
	let mut line = String::new();
	let ret = std::io::stdin().read_line(&mut line);
	if ret.is_err() {
		println!("[ERROR] {}", ret.err().unwrap());
		return String::new();
	}
	if ret.unwrap() == 0 {
		return String::new();
	}
	return (*line.trim()).to_string();
}

/// 確認
fn confirm_rename(left: &Path, right: &Path) -> bool {
	println!("CONTINUE? {:?} >> {:?}", &left, &right);
	let line = input().to_uppercase();
	if line == "Y" {
		return true;
	}
	if line == "YES" {
		return true;
	}
	return false;
}

/// ファイルハンドラーの定義
///
/// # Arguments
/// * `e` ファイルのパス
fn on_file_found(e: &Path) -> Result<(), Box<dyn std::error::Error>> {
	let parent = match e.parent() {
		Some(d) => d,
		None => Path::new(""),
	};

	// 新しい名前
	let name = generate_new_name();

	// もとの拡張子
	let ext = match e.extension() {
		Some(s) => s,
		None => OsStr::new(""),
	};

	let new_path = parent.join(&name).with_extension(ext);
	println!("{}", new_path.as_os_str().to_str().unwrap());

	if !confirm_rename(&e, &new_path) {
		return Ok(());
	}

	std::fs::rename(e, new_path)?;

	return Ok(());
}

/// ディレクトリ走査
///
/// # Arguments
/// * `e` パス
/// * `handler` ファイルハンドラー
fn enumerate(
	e: &Path,
	handler: &dyn Fn(&Path) -> Result<(), Box<dyn std::error::Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
	if !e.exists() {
		println!("[TRACE] invalid path {}", e.to_str().unwrap());
		return Ok(());
	} else if e.is_dir() {
		let it = std::fs::read_dir(e)?;
		for e in it {
			let entry = e.unwrap();
			let path = entry.path();
			enumerate(&path, handler)?;
		}
		return Ok(());
	} else {
		return handler(e);
	}
}

/// Rust アプリケーションのエントリーポイント
fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("path?");
		return;
	}

	for e in &args[1..] {
		let path = Path::new(e);
		let result = enumerate(path, &on_file_found);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("ERROR: {:?}", error);
			break;
		}
	}
}
