use std::path::PathBuf;

use clap::Parser;

/// Commandline arguments for RustyPage
#[derive(Parser)]
pub(crate) struct Args {
	/// Manually set the config file used to create RustyHome
	#[arg(short, long)]
	pub(crate) config_file: Option<PathBuf>,

	/// Set where you want the output file to be
	pub(crate) output_file: Option<PathBuf>,
}
