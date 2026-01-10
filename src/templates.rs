use crate::config::{Clock, Config, ParsedBookmark};
use crate::themes::catppuccin::{
	CATPPUCCIN_FRAPPE, CATPPUCCIN_LATTE, CATPPUCCIN_MACCHIATO, CATPPUCCIN_MOCHA,
};
use crate::themes::default::DEFAULT;
use crate::themes::dracula::{DRACULA_ALUCARD, DRACULA_DEFAULT};
use crate::themes::gruvbox::GRUVBOX;

use askama::Template;

use serde::Deserialize;

/// Homepage template variables
#[derive(Debug, Deserialize, Template)]
#[template(path = "home.html", escape = "none")]
pub(crate) struct Homepage {
	pub(crate) title: Option<String>,
	pub(crate) favicon: String,
	pub(crate) theme: &'static str,
	pub(crate) clock: Option<Clock>,
	pub(crate) search_engine: Option<String>,
	pub(crate) footer: bool,
	pub(crate) bookmarks: Option<Vec<ParsedBookmark>>,
}

impl Homepage {
	/// Create Homepage with a given config
	pub(crate) fn new(config: &Config) -> Self {
		let mut favicon = String::new();
		if let Some(favicon_input) = &config.favicon {
			let filtered_favicon = &favicon_input.as_str();

			if filtered_favicon.starts_with("http") {
				favicon = format!(
					"<link href=\"{}\" rel=\"icon\" type=\"image/x-icon\">",
					favicon_input
				);
			} else {
				favicon = format!(
					"<link href=\"data:image/x-icon;base64,{}\" rel=\"icon\" type=\"image/x-icon\">",
					favicon_input
				);
			}
		}

		let theme = match &config.theme {
			Some(theme_name) => match theme_name.to_ascii_lowercase().as_str() {
				"catppuccin-latte" | "catppuccin_latte" | "latte" => &CATPPUCCIN_LATTE,
				"catppuccin" | "catppuccin-frappe" | "catppuccin_frappe" | "frappe" => {
					&CATPPUCCIN_FRAPPE
				}
				"catppuccin-macchiato" | "catppuccin_macchiato" | "macchiato" => {
					&CATPPUCCIN_MACCHIATO
				}
				"catppuccin-mocha" | "catppuccin_mocha" | "mocha" => &CATPPUCCIN_MOCHA,
				"dracula" => &DRACULA_DEFAULT,
				"alucard" => &DRACULA_ALUCARD,
				"gruvbox" => &GRUVBOX,
				"" => &DEFAULT,
				_ => {
					eprintln!("WARNING: Unreconized theme in config file!");
					&DEFAULT
				}
			},
			None => &DEFAULT,
		};

		let footer = match &config.footer {
			Some(footer) => *footer,
			None => true,
		};

		let converted_bookmarks;
		if let Some(bookmarks) = &config.bookmarks {
			converted_bookmarks = Some(ParsedBookmark::convert_all(bookmarks));
		} else {
			converted_bookmarks = None;
		}

		Self {
			title: config.title.clone(),
			favicon,
			theme,
			clock: config.clock.clone(),
			search_engine: config.search_engine.clone(),
			footer,
			bookmarks: converted_bookmarks,
		}
	}

	/// Convert Homepage to a String
	pub(crate) fn render(template: &Self) -> Result<String, askama::Error> {
		template.render()
	}
}

#[cfg(test)]
mod tests {
	use crate::config::Config;
	use crate::templates::Homepage;

	use std::fs::{self, File};
	use std::io::Read;
    use std::path::PathBuf;

	use diff;

	/// Check the difference between 2 strings
	fn check_difference(left: String, right: String) -> bool {
		let mut not_different = true;

		for difference in diff::lines(&left, &right) {
			match difference {
				diff::Result::Left(left) => {
					not_different = false;
					println!("l-{}", left);
				},
				diff::Result::Right(right) => {
					not_different = false;
					println!("r-{}", right);
				},
				_ => (),
			}
		}

		return not_different;
	}

	// TODO: Handle error `toml::from_str()`/Result error better
	/// Get config from a given file
	fn get_config_from_file(config_file: &PathBuf) -> Result<Config, ()> {
		let toml_string = fs::read_to_string(&config_file).expect("error getting config file string");

		match toml::from_str(&toml_string) {
			Ok(config) => Ok(config),
			Err(error) => {
				eprintln!("ERROR: {}", error);
				Err(())
			},
		}
	}

	#[test]
	/// Test rendering the default config file
	fn default_config_diff_test() {
		let config_file_path: PathBuf = "./docs/config/examples/default.toml".into();
		let config = get_config_from_file(&config_file_path);

		let homepage = Homepage::new(&config.expect("error getting config file"));

		let mut rendered = Homepage::render(&homepage).expect("error rendering homepage");

		let mut wanted = File::open("./tests/data/default.html").expect("error getting test file");
		let mut contents = String::new();
		let _ = wanted.read_to_string(&mut contents);

		let not_different = check_difference(rendered, contents);

		assert!(not_different)
	}
}
