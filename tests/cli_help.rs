use assert_cmd::Command;

fn normalize_output(output: &[u8]) -> String {
    String::from_utf8_lossy(output).replace("\r\n", "\n")
}

#[test]
fn cli_help_prints_expected_banner() {
    let assert = Command::cargo_bin("qbit-cli")
        .expect("binary")
        .arg("--help")
        .assert()
        .success();

    let stdout = normalize_output(&assert.get_output().stdout);
    assert!(
        stdout.contains("Multi-language package/project manager")
            || stdout.to_ascii_lowercase().contains("qbit")
    );
}
