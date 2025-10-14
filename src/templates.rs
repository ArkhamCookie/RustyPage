use crate::config::Bookmark;

use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub(crate) struct Homepage {
	pub(crate) title: Option<String>,
	pub(crate) bookmarks: Vec<Bookmark>,
}
