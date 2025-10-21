# Config Documentation

To edit your RustyPage page, you edit a [TOML](https://toml.io/) file.
This program using the [directories crate](https://docs.rs/directories/latest/directories/) to find your config file.

## Example Config File

```toml
title = "RustyPage" # Sets the title in the head (this is what shows on the tab)
theme = "Frappe" # Select the Catppuccin Frappe theme for the color scheme
search_engine = "https://duckduckgo.com/?q=%q" # Optional search bar engine (replaces the '%q' with the query)
footer = true # Enable/disable footer

[[bookmarks]] # Create a bookmark for the page
link = "https://youtube.com" # Link of the bookmark (Required if creating a bookmark)
name = "YouTube" # Name of the bookmark (Required if creating bookmark)
shortcut = "y" # Optional shortcut you can press to open bookmark in new tab
```

There are more examples in the [`examples/`](./examples/) directory.

## Config File Location

On **Linux**, it uses the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications.
This defaults to `~/.config/rustypage`.

On **Windows**, it uses the [Known Folder](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb776911(v=vs.85)?redirectedfrom=MSDN) system.

On **MacOS**, it uses the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6).

> [!NOTE]
> Please see the [directories crate](https://docs.rs/directories/latest/directories/) for more infomation.
