use crate::config::{Config, ParsedBookmark};

use askama::Template;

use serde::Deserialize;

#[derive(Debug, Deserialize, Template)]
#[template(path = "home.html")]
pub(crate) struct Homepage {
	pub(crate) title: Option<String>,
	pub(crate) bookmarks: Vec<ParsedBookmark>,
}

impl Homepage {
	pub(crate) fn new(config: &Config) -> Self {
		let converted_bookmarks = ParsedBookmark::convert_all(&config.bookmarks);

		Self {
			title: config.title.clone(),
			bookmarks: converted_bookmarks,
		}
	}

	pub(crate) fn render(template: &Self) -> Result<String, askama::Error> {
		template.render()
	}
}
