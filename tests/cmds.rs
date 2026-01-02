//! Tests the RustyPage commands are successful.

#[cfg(test)]
mod tests {
	use std::default::Default;
	use std::fs;
	use std::path::PathBuf;
	use std::process::ExitStatus;
	use std::str;

	use assert_cmd::cargo::cargo_bin_cmd;

	use clap::crate_version;

	/// Removes output of rustypage
	fn clean(path: &str) {
		let path: PathBuf = path.into();
		let result = fs::remove_file(path);

		if result.is_err() {
			println!("`./index.html` not found. Continuing...")
		}
	}

	#[test]
	/// Test that cargo command (rustypage) runs.
	fn basic_cmd_test() {
		let mut command = cargo_bin_cmd!("rustypage");
		let _ = command.unwrap();

		clean("./index.html");
	}

	#[test]
	/// Test getting the version of rustypage.
	fn version_test() {
		let mut command = cargo_bin_cmd!("rustypage");
		let output = command
			.arg("-V")
			.unwrap()
			.stdout;
		let string_output = str::from_utf8(&output).unwrap();
		let expected = format!(
			"RustyPage: v{}
ArkhamCookie <cargo@mail.arkhamcookie.com>

RustyPage is a simple startpage configurable with a simple TOML file.
",
			crate_version!()
		);

		assert_eq!(string_output, expected)
	}

	#[test]
	/// Test the exit status of base command.
	fn exit_status_test() {
		let mut command = cargo_bin_cmd!("rustypage");
		let output = command.unwrap();

		assert_eq!(output.status, <ExitStatus as Default>::default());

		clean("./index.html");
	}
}

