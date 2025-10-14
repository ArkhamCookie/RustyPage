pub(crate) struct Bookmark {
	// icon: String,
	pub(crate) id: String,
	pub(crate) link: String,
	pub(crate) name: String,
	pub(crate) shortcut: Option<String>,
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
