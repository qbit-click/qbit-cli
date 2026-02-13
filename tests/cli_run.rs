use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn normalize_output(output: &[u8]) -> String {
    String::from_utf8_lossy(output).replace("\r\n", "\n")
}

#[test]
fn run_named_script_from_temp_project_succeeds() {
    let tmp = tempdir().expect("tempdir");
    let config = r#"scripts:
  hello: "echo hello-qbit"
"#;
    fs::write(tmp.path().join("qbit.yml"), config).expect("write qbit.yml");

    Command::cargo_bin("qbit-cli")
        .expect("binary")
        .current_dir(tmp.path())
        .args(["run", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-qbit"));
}

#[test]
fn run_missing_script_returns_actionable_error() {
    let tmp = tempdir().expect("tempdir");
    let config = r#"scripts:
  hello: "echo hello-qbit"
"#;
    fs::write(tmp.path().join("qbit.yml"), config).expect("write qbit.yml");

    let assert = Command::cargo_bin("qbit-cli")
        .expect("binary")
        .current_dir(tmp.path())
        .args(["run", "missing_script"])
        .assert()
        .code(1);

    let stderr = normalize_output(&assert.get_output().stderr);
    assert!(stderr.contains("Script"));
    assert!(stderr.contains("not found"));
}
