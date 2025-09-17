#[test]
fn test_single_scan_google() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--target", "google.com", "--ports", "80"])
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn test_scan_localhost() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--target", "127.0.0.1", "--ports", "80"])
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn test_scan_invalid_ip() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--target", "256.256.256.256", "--ports", "80"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn test_display_invalid_format() {
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "--target",
            "localhost",
            "--ports",
            "80",
            "--format",
            "invalid",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
