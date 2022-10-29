use assert_cmd::Command;
#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("cli").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}
