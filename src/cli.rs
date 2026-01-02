use std::path::PathBuf;

use clap::{ArgAction, Parser};

/// Commandline arguments for RustyPage
#[derive(Clone, Parser)]
pub(crate) struct Args {
	/// Manually set the config file used to create RustyHome
	#[arg(short, long = "config")]
	pub(crate) config_file: Option<PathBuf>,

	#[cfg(feature = "minify")]
	/// Minify the output
	#[arg(short, long, action = ArgAction::SetTrue)]
	pub(crate) minify: bool,

	/// Set where you want the output file to be
	pub(crate) output_file: Option<PathBuf>,

	/// Print version and exit
	#[arg(short = 'V', long, action = ArgAction::SetTrue)]
	pub(crate) version: bool,
}

pub(crate) fn get_path(path_arg: &Option<PathBuf>) -> PathBuf {
	if let Some(path) = path_arg {
		if path.is_dir() {
			let mut path_dir = path.clone();
			path_dir.push("index.html");

			return path_dir;
		}
		return path.to_path_buf();
	}

	return PathBuf::from("index.html");
}
