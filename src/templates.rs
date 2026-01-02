use crate::config::{Clock, Config, ParsedBookmark};
use crate::themes::catppuccin::{
	CATPPUCCIN_FRAPPE, CATPPUCCIN_LATTE, CATPPUCCIN_MACCHIATO, CATPPUCCIN_MOCHA,
};
use crate::themes::default::DEFAULT;
use crate::themes::dracula::{DRACULA_ALUCARD, DRACULA_DEFAULT};

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
