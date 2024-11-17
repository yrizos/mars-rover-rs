use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_multiple_rovers() {
    let mut child = Command::new("cargo")
        .arg("run")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cargo run");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        let input = b"5 5\n1 2 N\nLMLMLMLMM\n3 3 E\nMMRMMRMRRM\n";
        stdin.write_all(input).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    let expected_output = "1 3 N\n5 1 E\n";
    let actual_output = String::from_utf8_lossy(&output.stdout);

    assert_eq!(actual_output, expected_output);
}
