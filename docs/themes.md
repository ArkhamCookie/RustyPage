# Themes

You can choose a theme by adding this to your config file.

```toml
theme = "theme_name"
```

## Theme List

- [Catppuccin](https://catppuccin.com)
  - Latte
  - Frappe
  - Macchiato
  - Mocha
- [Dracula](https://draculatheme.com)
  - Alucard
  - Dracula
- [gruvbox](https://github.com/morhetz/gruvbox)

## Adding a Theme

Adding a theme for RustyPage is pretty simple;
it just requires some basic knowledge about CSS and a tiny bit of Rust.

> [!NOTE]
> If you want to just handle the css side, feel free to open an issue and ask for help.
> I don't mind doing the little bit of Rust there is for this.

1. First create a file in [src/themes/](/src/themes/) with the name of your theme.
2. Add your file to the [theme mod file](/src/themes/mod.rs).
3. Add a doc comment (`//!` at the top) at the top of your file saying what your theme is.
**Make sure to give credit!**
4. Create a const &str with the name of your theme or variation, if applicable.
(`pub(crate) const YOUR_THEME: &str = "";`)
5. Add the required variables to `:root`.
6. Add your theme to the match statement used for selecting the theme.
It is in the `impl Homepage` new function.
7. Test it out.
8. Open a pull request!
