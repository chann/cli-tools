use std::process::Command;

#[test]
fn zzz_reports_package_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_zzz"))
        .arg("--version")
        .output()
        .unwrap();

    assert!(output.status.success(), "zzz failed: {output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("zzz {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(output.stderr, b"");
}
