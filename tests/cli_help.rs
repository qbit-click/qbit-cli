fn normalize_output(output: &[u8]) -> String {
    String::from_utf8_lossy(output).replace("\r\n", "\n")
}

#[test]
fn cli_help_prints_expected_banner() {
    let assert = assert_cmd::cargo::cargo_bin_cmd!("qbit-cli")
        .arg("--help")
        .assert()
        .success();

    let stdout = normalize_output(&assert.get_output().stdout);
    assert!(
        stdout.contains("Multi-language package/project manager")
            || stdout.to_ascii_lowercase().contains("qbit")
    );
}
