use crate::cli::{Args, get_path};
use crate::config::get_config;
use crate::templates::Homepage;

use std::fs;
use std::process::exit;

use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};

#[cfg(feature = "minify")]
use minify_html::{Cfg, minify};

mod cli;
mod config;
mod templates;
mod themes;

fn main() {
	let args = Args::parse();

	if args.version {
		println!("{}: v{}", crate_name!(), crate_version!());
		println!("{}", crate_authors!());
		println!("\n{}", crate_description!());

		exit(0);
	}

	let config = get_config(&args);
	let homepage = Homepage::new(&config);
	let mut rendered = match Homepage::render(&homepage) {
		Ok(rendered) => rendered,
		Err(error) => {
			eprintln!("ERROR: {}", error);
			exit(1);
		}
	};

	#[cfg(feature = "minify")]
	if args.minify {
		let mut minify_config = Cfg::new();
		minify_config.minify_css = true;
		minify_config.minify_js = true;

		let minified = minify(rendered.as_bytes(), &minify_config);
		rendered = String::from_utf8(minified).expect("error converting minified html to string");
	}

	let output_path = get_path(&args.output_file);

	let _ = match fs::write(output_path, rendered) {
		Ok(output) => output,
		Err(error) => {
			eprintln!("ERROR: {}", error.kind());
			exit(1);
		}
	};
}
