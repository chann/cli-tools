use std::process::Command;

#[test]
fn work_summary_reports_package_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_work-summary"))
        .arg("--version")
        .output()
        .unwrap();

    assert!(output.status.success(), "work-summary failed: {output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("work-summary {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(output.stderr, b"");
}
