use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn run_program(input: &str) -> String {
    let pkg = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is not set");
    let exe =
        env::var("CARGO_BIN_EXE_".to_string() + &pkg).expect("CARGO_BIN_EXE_<crate> is not set");

    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run the program");

    child
        .stdin
        .unwrap()
        .write(input.as_bytes())
        .expect("failed to write to stdin");
    child.stdin = None;

    let output = child
        .wait_with_output()
        .expect("failed to read program output");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[test]
fn existing_file_success() {
    assert_eq!(run_program("/etc/hosts\n"), "success");
}

#[test]
fn another_existing_file_success() {
    assert_eq!(run_program("/etc/passwd\n"), "success");
}

#[test]
fn os_release_file_success() {
    assert_eq!(run_program("/etc/os-release\n"), "success");
}

#[test]
fn system_binary_file_success() {
    assert_eq!(run_program("/bin/sh\n"), "success");
}

#[test]
fn coreutils_binary_file_success() {
    assert_eq!(run_program("/usr/bin/env\n"), "success");
}

#[test]
fn executable_script_file_success() {
    assert_eq!(run_program("/bin/true\n"), "success");
}

#[test]
fn python_binary_file_success() {
    assert_eq!(run_program("/usr/bin/python3\n"), "success");
}

#[test]
fn device_file_success() {
    assert_eq!(run_program("/dev/null\n"), "success");
}

#[test]
fn path_with_trailing_newline() {
    assert_eq!(run_program("/etc/hosts\n\n"), "success");
}

#[test]
fn path_with_leading_spaces() {
    assert_eq!(run_program("  /etc/hosts\n"), "success");
}

#[test]
fn path_with_surrounding_spaces() {
    assert_eq!(run_program("  /etc/hosts  \n"), "success");
}

#[test]
fn nonexistent_path_failure() {
    assert_eq!(
        run_program("/strange_folder/strange_file.ganteli\n"),
        "failure"
    );
}

#[test]
fn another_nonexistent_path_failure() {
    assert_eq!(
        run_program("/nonexistent_dir/nonexistent_file\n"),
        "failure"
    );
}

#[test]
fn deep_nonexistent_path_failure() {
    assert_eq!(run_program("/this/does/not/exist\n"), "failure");
}

#[test]
fn trailing_slash_on_file_failure() {
    assert_eq!(run_program("/etc/hosts/\n"), "failure");
}

#[test]
fn directory_is_not_readable_failure() {
    assert_eq!(run_program("/etc\n"), "failure");
}

#[test]
fn another_directory_is_not_readable_failure() {
    assert_eq!(run_program("/bin\n"), "failure");
}

#[test]
fn root_directory_is_not_readable_failure() {
    assert_eq!(run_program("/\n"), "failure");
}

#[test]
fn empty_input_failure() {
    assert_eq!(run_program(""), "failure");
}
