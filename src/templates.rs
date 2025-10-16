use crate::config::ParsedBookmark;

use askama::Template;

use serde::Deserialize;

#[derive(Debug, Deserialize, Template)]
#[template(path = "home.html")]
pub(crate) struct Homepage {
	pub(crate) title: Option<String>,
	pub(crate) bookmarks: Vec<ParsedBookmark>,
}
