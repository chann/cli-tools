use std::process::Command;

#[test]
fn code_cost_reports_package_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_code-cost"))
        .arg("--version")
        .output()
        .unwrap();

    assert!(output.status.success(), "code-cost failed: {output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("code-cost {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(output.stderr, b"");
}
