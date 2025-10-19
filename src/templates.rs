use crate::config::{Config, ParsedBookmark};
use crate::themes::catppuccin::{CATPPUCCIN_FRAPPE, CATPPUCCIN_LATTE, CATPPUCCIN_MACCHIATO};

use askama::Template;

use serde::Deserialize;

#[derive(Debug, Deserialize, Template)]
#[template(path = "home.html", escape = "none")]
pub(crate) struct Homepage {
	pub(crate) title: Option<String>,
	pub(crate) theme: &'static str,
	pub(crate) bookmarks: Vec<ParsedBookmark>,
}

impl Homepage {
	pub(crate) fn new(config: &Config) -> Self {
		let converted_bookmarks = ParsedBookmark::convert_all(&config.bookmarks);
		let theme = match &config.theme {
			Some(theme_name) => match theme_name.to_ascii_lowercase().as_str() {
				"catppuccin-latte" | "catppuccin_latte" | "latte" => {
					&CATPPUCCIN_LATTE
				},
				"catppuccin" | "catppuccin-frappe" | "catppuccin_frappe" | "frappe" => {
					&CATPPUCCIN_FRAPPE
				},
				"cattpuccin-macchiato" | "catppuccin_macchiato" | "macchiato" => {
					&CATPPUCCIN_MACCHIATO
				},
				"" => "",
				_ => {
					eprintln!("WARNING: Unreconized theme in config file!");
					""
				}
			},
			None => "",
		};

		Self {
			title: config.title.clone(),
			theme,
			bookmarks: converted_bookmarks,
		}
	}

	pub(crate) fn render(template: &Self) -> Result<String, askama::Error> {
		template.render()
	}
}
