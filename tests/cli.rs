use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_flag_prints_usage() {
	let mut cmd = Command::cargo_bin("reamake").unwrap();
	cmd.arg("-c Bob")
		.assert()
		.success()
		.stdout(predicate::str::contains("Usage"));
}
