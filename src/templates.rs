use crate::config::{Config, ParsedBookmark};
use crate::themes::catppuccin::{
	CATPPUCCIN_FRAPPE, CATPPUCCIN_LATTE, CATPPUCCIN_MACCHIATO, CATPPUCCIN_MOCHA,
};
use crate::themes::dracula::{DRACULA_ALUCARD, DRACULA_DEFAULT};

use askama::Template;

use serde::Deserialize;

/// Homepage template variables
#[derive(Debug, Deserialize, Template)]
#[template(path = "home.html", escape = "none")]
pub(crate) struct Homepage {
	pub(crate) title: Option<String>,
	pub(crate) theme: &'static str,
	pub(crate) search_engine: Option<String>,
	pub(crate) footer: bool,
	pub(crate) bookmarks: Option<Vec<ParsedBookmark>>,
}

impl Homepage {
	/// Create Homepage with a given config
	pub(crate) fn new(config: &Config) -> Self {
		let converted_bookmarks;
		if let Some(bookmarks) = &config.bookmarks {
			converted_bookmarks = Some(ParsedBookmark::convert_all(bookmarks));
		} else {
			converted_bookmarks = None;
		}
		let theme = match &config.theme {
			Some(theme_name) => match theme_name.to_ascii_lowercase().as_str() {
				"catppuccin-latte" | "catppuccin_latte" | "latte" => &CATPPUCCIN_LATTE,
				"catppuccin" | "catppuccin-frappe" | "catppuccin_frappe" | "frappe" => &CATPPUCCIN_FRAPPE,
				"catppuccin-macchiato" | "catppuccin_macchiato" | "macchiato" => &CATPPUCCIN_MACCHIATO,
				"catppuccin-mocha" | "catppuccin_mocha" | "mocha" => &CATPPUCCIN_MOCHA,
				"dracula" => &DRACULA_DEFAULT,
				"alucard" => &DRACULA_ALUCARD,
				"" => "",
				_ => {
					eprintln!("WARNING: Unreconized theme in config file!");
					""
				}
			},
			None => "",
		};
		let footer = match &config.footer {
			Some(footer) => *footer,
			None => true,
		};

		Self {
			title: config.title.clone(),
			theme,
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
