use askama::Template;

pub(crate) struct Bookmark {
	// icon: String,
	id: String,
	link: String,
	name: String,
	shortcut: Option<String>,
}

impl Bookmark {
	pub(crate) fn new(link: String, name: String, shortcut: Option<String>) -> Self {
		// let id = format!("bookmark-{}", foo); // Do something to get number in list of bookmarks
		let id = String::from("foo");

		Self {
			id,
			link,
			name,
			shortcut
		}
	}
}

#[derive(Template)]
#[template(path = "home.html")]
pub(crate) struct HomePage {
	pub(crate) title: Option<String>,
	pub(crate) bookmarks: Vec<Bookmark>,
}
