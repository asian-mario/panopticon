use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Integration test: run the validate binary and expect success on the sample repo
#[test]
fn validate_runs_and_succeeds() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("validate")?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Validation complete"));
    Ok(())
}
