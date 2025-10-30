use crate::cli::Args;

use std::fs;
use std::path::PathBuf;
use std::process::exit;

use directories::ProjectDirs;

use serde::Deserialize;

/// RustyPage config struct
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Config {
	pub(crate) title: Option<String>,
	pub(crate) favicon: Option<String>,
	pub(crate) theme: Option<String>,
	pub(crate) clock: Option<Clock>,
	pub(crate) search_engine: Option<String>,
	pub(crate) footer: Option<bool>,
	pub(crate) bookmarks: Option<Vec<Bookmark>>,
}

/// Clock from Config
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Clock {
	pub(crate) twelve_hour: Option<bool>,
}

/// Bookmark from Config before being parsed
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Bookmark {
	pub(crate) link: String,
	pub(crate) name: String,
	pub(crate) shortcut: Option<String>,
}

/// Bookmark from Config after being parsed
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ParsedBookmark {
	pub(crate) id: String,
	pub(crate) link: String,
	pub(crate) name: String,
	pub(crate) shortcut: String,
}

impl ParsedBookmark {
	/// Convert a Bookmark into a ParsedBookmark
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

	/// Convert all a vec of Bookmark into a vec of ParsedBookmarks
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

/// Get config file either from config file arg or from config directory
pub(crate) fn get_config(args: &Args) -> Config {
	if let Some(config_file) = &args.config_file {
		return get_config_from_file(&config_file);
	}

	get_config_from_dirs()
}

/// Get config from config directory file (creates one if needed)
fn get_config_from_dirs() -> Config {
	let project_dirs = match ProjectDirs::from("com", "arkhamcookie", "rustypage") {
		Some(project_dirs) => project_dirs,
		None => {
			eprintln!("ERROR: Project directories not found (ProjectDirs crate issue).");
			exit(1);
		}
	};
	let config_dirs = project_dirs.config_dir();

	let _ = match fs::create_dir_all(&config_dirs) {
		Ok(result) => result,
		Err(error) => {
			eprintln!("ERROR: {}", error);
			exit(1);
		}
	};

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
		let _ = match fs::write(&config_path, default_toml) {
			Ok(result) => result,
			Err(error) => {
				eprintln!("ERROR: {}", error);
				exit(1);
			}
		};
	}

	get_config_from_file(&config_path)
}

/// Get config from a given file
fn get_config_from_file(config_file: &PathBuf) -> Config {
	let toml_string = match fs::read_to_string(config_file) {
		Ok(toml_string) => toml_string,
		Err(error) => {
			eprintln!("ERROR: {}", error);
			exit(1)
		}
	};

	match toml::from_str(&toml_string) {
		Ok(config) => config,
		Err(error) => {
			eprintln!("ERROR: {}", error);
			exit(1);
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::config::{Bookmark, Clock, Config, ParsedBookmark, get_config_from_file};

	use std::path::PathBuf;

	/// Test parsing a single bookmark with a shortcut
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

	/// Test parsing a single bookmark without a shortcut
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

	/// Test parsing a vec of bookmarks
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

	/// Test that config matches what we want
	#[test]
	fn get_full_config() {
		let wanted_clock = Clock {
			twelve_hour: Some(true),
		};
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
			favicon: Some(String::from(
				"AAABAAEAEBAAAAEAIABoBAAAFgAAACgAAAAQAAAAIAAAAAEAIAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIAsLBjsXEcI3FQ77NBIM/zIQCv8wDQj/Lg0H/ywKBf8rCgT/KggD/yoIA/8qCAP6LQkEwRgGA00AAAAAHQ4OBDQUDv8zEg3/MRAK/y8PCf8tDAf/KwoF/ygJA/8mBwL/JQYB/yQFAf8kBAD/JAUB/yQFAP8lBQD/IAAABEIeGMQ2FQ//NRMO/zMSDP8xEAv/Lg4I/ywMBv8qCgX/KAgD/yYHAv8lBgH/JQUB/yUFAP8kBQD/JQYB/y0LBcJCHhjhOBYQ/zcVD/81FA3/E33D/wp/1f8LhtL/Co7J/wmQuP8Ilp3/CJiG/wiVd/8kEAr/JQUB/yYFAf8sCQTgRSAZ+zsZEv86FxD/OBUO/w1/4/8ZNJn/IC1+/yUva/8qOFv/Ljc7/zI/Kv8MomX/Hz4e/yYGAf8nBwL/LQoF+kkkHP8/HBX/PhoT/zwZEv8QZ+H/Ixt//zMVWv9WJFP/flBV/z8fGf8/KBD/Hpc3/yhEEv8oCAL/KAgD/y4MBf1PKCD/Qx4X/0MfF/9CHRf/GEnf/y8Wef9GE0//fipQ/6plMP9RJg//RigM/z+WGv8zRw//KwsF/ysLBv8xDgn/Uioi/0ciG/9HIRr/SCId/yIy3P89E3T/bB1e/51DSP+LRyT/ek4K/04qCv9okQr/Sk4L/y8PCf8vDgj/MxEL/1YtJf9KIxz/SiUf/y421f8rIN3/SRFw/7JDpP+aQkf/gkQf/5d1Ev9lQwr/gIwI/4+uBv9VRAv/MhEM/zcVD/9ZMCf/SyQc/00mHv9NJyr/QxzW/1gVoP9bEkD/fi47/4JIIf9eJhD/YzgP/5qWCv96dAv/NhUP/zQUDv86FxH/XDIp/0wlHv9NJR7/TSUe/08mI/96GKj/ghll/1kSJv9VFBn/bSsX/6hsE/91WhH/ORgS/zcXEf83FhD/PBoT/14zK/5NJR7/TSUf/00lH/9OJR//TiYf/64jav+iIy3/eSEd/7FGGv9rPRj/PhwV/zsZE/85GBL/OBcR/z0bFPxfNSzyTSUe/00mH/9NJR//TiYf/00lHv9NJR7/nCoy/700KP9hLR7/Qh8Y/z4cFf87GhP/OhgS/zgYEf8+HBXvZzoxxE0lHv9NJR7/TSYf/00lH/9NJR7/TCUd/0skHf9RJR7/RSAZ/0EeF/8+HBX/OxkT/zkYEv84GBH/QR4XwzghHUpMJR7/TSUd/04mH/9OJh7/TSUe/0skHf9KJBz/RyEa/0QgGf9BHhf/PRsV/zsZE/85GBL/OBgS/yUSAAMAAAAANyIdTmU5McFfNCz7XjMr/1wxKf9ZMCf/Vy0l/1IrIv9OJyD/SiUd/0ciGv9DHxjfQyAZwiMRDVAAAAAAwAMAAIABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAQAAwAMAAA==",
			)),
			theme: Some(String::from("catppuccin")),
			clock: Some(wanted_clock),
			search_engine: Some(String::from("https://duckduckgo.com/?q=%q")),
			footer: Some(true),
			bookmarks: Some(want_bookmarks),
		};
		let got = get_config_from_file(&PathBuf::from("./docs/config/examples/full.toml"));

		assert_eq!(want, got)
	}
}
