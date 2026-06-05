use std::process::Command;

#[test]
fn git_tools_reports_package_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_git-tools"))
        .arg("--version")
        .output()
        .unwrap();

    assert!(output.status.success(), "git-tools failed: {output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("git-tools {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(output.stderr, b"");
}
