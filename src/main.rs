use crate::cli::{Args, get_path};
use crate::config::get_config;
use crate::templates::Homepage;

use std::fs;
use std::process::exit;

use clap::Parser;

mod cli;
mod config;
mod templates;
mod themes;

fn main() {
	let args = Args::parse();

	let config = get_config(&args);
	let homepage = Homepage::new(&config);
	let rendered = match Homepage::render(&homepage) {
		Ok(rendered) => rendered,
		Err(error) => {
			eprintln!("ERROR: {}", error);
			exit(1);
		}
	};

	let output_path = get_path(&args.output_file);

	let _ = match fs::write(output_path, rendered) {
		Ok(output) => output,
		Err(error) => {
			eprintln!("ERROR: {}", error.kind());
			exit(1);
		}
	};
}
