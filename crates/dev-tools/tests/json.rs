use std::process::{Command, Output};

fn run_json(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dev-tools"))
        .arg("json")
        .args(args)
        .output()
        .expect("dev-tools should run")
}

#[test]
fn check_accepts_valid_json_without_echoing_the_document() {
    let output = run_json(&[r#"{"b":2,"a":1}"#, "--check"]);

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Valid JSON\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn check_rejects_invalid_json_with_location_context() {
    let output = run_json(&["{\n  \"a\":\n}", "--check"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("Invalid JSON:"));
    assert!(stderr.contains("line 3 column 1"));
    assert!(output.stdout.is_empty());
}

#[test]
fn format_pretty_prints_json_explicitly() {
    let output = run_json(&[r#"{"b":2,"a":1}"#, "--format"]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\n  \"a\": 1,\n  \"b\": 2\n}\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn minify_emits_one_compact_line() {
    let output = run_json(&[r#"{ "b": 2, "a": 1 }"#, "--minify"]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"a\":1,\"b\":2}\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn sort_asc_recurses_into_objects_without_reordering_arrays() {
    let input = r#"{"z":{"b":2,"a":1},"items":[{"d":4,"c":3},0]}"#;
    let output = run_json(&[input, "--sort", "asc", "--minify"]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"items\":[{\"c\":3,\"d\":4},0],\"z\":{\"a\":1,\"b\":2}}\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn sort_desc_recurses_into_objects_without_reordering_arrays() {
    let input = r#"{"a":{"x":1,"z":3},"m":[{"a":1,"b":2},0],"z":0}"#;
    let output = run_json(&[input, "--sort", "desc", "--minify"]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"z\":0,\"m\":[{\"b\":2,\"a\":1},0],\"a\":{\"z\":3,\"x\":1}}\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn sort_applies_after_jsonpath_querying() {
    let input = r#"{"payload":{"a":1,"b":2}}"#;
    let output = run_json(&[input, "--query", "$.payload", "--sort", "desc", "--minify"]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "[{\"b\":2,\"a\":1}]\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn check_rejects_output_transform_flags() {
    let output = run_json(&[r#"{"a":1}"#, "--check", "--minify"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("cannot be used with"));
    assert!(output.stdout.is_empty());
}

#[test]
fn format_and_minify_are_mutually_exclusive() {
    let output = run_json(&[r#"{"a":1}"#, "--format", "--minify"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("cannot be used with"));
    assert!(output.stdout.is_empty());
}

#[test]
fn sort_rejects_non_json_conversion_modes() {
    let output = run_json(&[r#"{"a":1}"#, "--sort", "asc", "--yaml"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("cannot be used with"));
    assert!(output.stdout.is_empty());
}

#[test]
fn sort_rejects_unknown_order() {
    let output = run_json(&[r#"{"a":1}"#, "--sort", "sideways"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("invalid value 'sideways'"));
    assert!(output.stdout.is_empty());
}
