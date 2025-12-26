//! Tests the RustyPage commands are successful.

#[cfg(test)]
mod tests {
	use std::str;

	use assert_cmd::cargo::cargo_bin_cmd;

	use clap::crate_version;

	#[test]
	fn basic_cmd_test() {
		let mut command = cargo_bin_cmd!("rustypage");
		let _ = command.unwrap();
	}

	#[test]
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
}

