//! Tests the RustyPage commands are successful.

#[cfg(test)]
mod tests {
	use assert_cmd::cargo::cargo_bin_cmd;

	#[test]
	fn basic_cmd_test() {
		let mut command = cargo_bin_cmd!("rustypage");
		let _ = command.unwrap();
	}
}

