use crate::cli::Args;

use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Config {
	pub(crate) title: Option<String>,
	pub(crate) theme: Option<String>,
	pub(crate) search_engine: Option<String>,
	pub(crate) footer: Option<bool>,
	pub(crate) bookmarks: Vec<Bookmark>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Bookmark {
	pub(crate) link: String,
	pub(crate) name: String,
	pub(crate) shortcut: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ParsedBookmark {
	pub(crate) id: String,
	pub(crate) link: String,
	pub(crate) name: String,
	pub(crate) shortcut: String,
}

impl ParsedBookmark {
	fn parse(bookmark: &Bookmark, id: i32) -> Self {
		let parsed_shortcut: String;

		if let Some(shortcut) = &bookmark.shortcut {
			parsed_shortcut = shortcut.clone();
		} else {
			parsed_shortcut = String::new();
		}

		Self {
			id: format!("bookmark-{}", id),
			link: bookmark.link.clone(),
			name: bookmark.name.clone(),
			shortcut: parsed_shortcut,
		}
	}

	pub(crate) fn convert_all(bookmarks: &Vec<Bookmark>) -> Vec<Self> {
		let mut id = 0;
		let mut current_bookmark;
		let mut parsed_bookmarks: Vec<Self> = Vec::new();

		for bookmark in bookmarks {
			current_bookmark = ParsedBookmark::parse(&bookmark, id);
			parsed_bookmarks.push(current_bookmark);
			id += 1;
		}

		return parsed_bookmarks;
	}
}

pub(crate) fn get_config(args: &Args) -> Config {
	if let Some(config_file) = &args.config_file {
		return get_config_from_file(&config_file);
	}

	get_config_from_dirs()
}

fn get_config_from_dirs() -> Config {
	let project_dirs = ProjectDirs::from("com", "arkhamcookie", "rustypage")
		.expect("error couldn't get project directory");
	let config_dirs = project_dirs.config_dir();

	fs::create_dir_all(&config_dirs).expect("error creating config directories");

	let config_path = &config_dirs.join("config.toml");

	let default_toml = String::from(
		"title = \"RustyPage\"

[[bookmarks]]
link = \"https://github.com\"
name = \"GitHub\"
shortcut = \"g\"

[[bookmarks]]
link = \"https://arkhamcookie.com\"
name = \"ArkhamCookie\"
	",
	);

	if !config_path.exists() {
		fs::write(&config_path, default_toml).expect("error couldn't create default config file");
	}

	get_config_from_file(&config_path)
}

fn get_config_from_file(config_file: &PathBuf) -> Config {
	let toml_string = fs::read_to_string(config_file).expect("Error reading file");

	toml::from_str(&toml_string).expect("Error parsing config file")
}

#[cfg(test)]
mod tests {
	use crate::config::{Bookmark, Config, ParsedBookmark, get_config_from_file};

	use std::path::PathBuf;

	#[test]
	fn parse_bookmark_with_shortcut() {
		let want = ParsedBookmark {
			id: String::from("bookmark-0"),
			link: String::from("https://github.com"),
			name: String::from("GitHub"),
			shortcut: String::from("g"),
		};

		let bookmark = Bookmark {
			link: String::from("https://github.com"),
			name: String::from("GitHub"),
			shortcut: Some(String::from("g")),
		};
		let got = ParsedBookmark::parse(&bookmark, 0);

		assert_eq!(want, got)
	}


	#[test]
	fn parse_bookmark_without_shortcut() {
		let want = ParsedBookmark {
			id: String::from("bookmark-0"),
			link: String::from("https://arkhamcookie.com"),
			name: String::from("ArkhamCookie"),
			shortcut: String::from(""),
		};

		let bookmark = Bookmark {
			link: String::from("https://arkhamcookie.com"),
			name: String::from("ArkhamCookie"),
			shortcut: None,
		};
		let got = ParsedBookmark::parse(&bookmark, 0);

		assert_eq!(want, got)
	}

	#[test]
	fn parse_all_bookmarks() {
		let want = vec![
			ParsedBookmark {
				link: String::from("https://github.com"),
				id: String::from("bookmark-0"),
				name: String::from("GitHub"),
				shortcut: String::from("g"),
			},
			ParsedBookmark {
				link: String::from("https://arkhamcookie.com"),
				id: String::from("bookmark-1"),
				name: String::from("ArkhamCookie"),
				shortcut: String::from(""),
			},
		];

		let bookmarks = vec![
			Bookmark {
				link: String::from("https://github.com"),
				name: String::from("GitHub"),
				shortcut: Some(String::from("g")),
			},
			Bookmark {
				link: String::from("https://arkhamcookie.com"),
				name: String::from("ArkhamCookie"),
				shortcut: None,
			},
		];
		let got = ParsedBookmark::convert_all(&bookmarks);

		assert_eq!(want, got)
	}

	#[test]
	fn get_full_config() {
		let want_bookmarks = vec![
			Bookmark {
				link: String::from("https://github.com"),
				name: String::from("GitHub"),
				shortcut: Some(String::from("g")),
			},
			Bookmark {
				link: String::from("https://arkhamcookie.com"),
				name: String::from("ArkhamCookie"),
				shortcut: None,
			},
		];
		let want = Config {
			title: Some(String::from("ArkhamCookie's Homepage")),
			theme: Some(String::from("catppuccin")),
			search_engine: Some(String::from("https://duckduckgo.com/?q=%q")),
			footer: Some(true),
			bookmarks: want_bookmarks,
		};
		let got = get_config_from_file(&PathBuf::from("./docs/config/examples/full.toml"));

		assert_eq!(want, got)
	}
}
