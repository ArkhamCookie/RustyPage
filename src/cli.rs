use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
	/// Manually set the config file used to create RustyHome
	pub(crate) config_file: Option<PathBuf>,
}
