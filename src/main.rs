use crate::cli::Args;
use crate::config::get_config;
use crate::templates::Homepage;

use std::fs;
use std::path::PathBuf;

use clap::Parser;

mod cli;
mod config;
mod templates;
mod themes;

fn main() {
	let args = Args::parse();
	let config = get_config(&args);
	let homepage = Homepage::new(&config);
	let rendered = Homepage::render(&homepage).expect("error rendering template");

	let mut output_path = &PathBuf::from("./index.html");
	if let Some(path) = &args.output_file {
		output_path = path;
	}

	fs::write(output_path, rendered).expect("error couldn't create RustyPage file");
}
