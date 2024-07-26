use assert_cmd::Command;

fn get_cmd() -> Command {
    Command::cargo_bin("msf-hasher").unwrap()
}

#[test]
fn test_cli_checksum() {
    let mut cmd = get_cmd();
    let assert = cmd.arg("--check").arg("checksum.txt").assert();

    assert.success().code(0).stdout("lorem ipsum.txt: OK\n");
}

#[test]
fn test_cli_hashing() {
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("lorem ipsum.txt")
        .arg("-")
        .write_stdin("Hello world")
        .assert();

    assert
        .success()
        .code(0)
        .stderr("")
        .stdout("3003c17ef87d64554b5dcb53571b44739e3e29d79903081bcf37222f0935bb7c  lorem ipsum.txt\n64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c  -\n");
}

#[test]
fn test_cli_strict() {
    let mut cmd = get_cmd();

    let assert = cmd
        .arg("--check")
        .arg("--strict")
        .arg("bad_checksum.txt")
        .assert();

    assert.failure().code(1);
}

#[test]
fn test_cli_strict_warn() {
    let mut cmd = get_cmd();

    let assert = cmd
        .arg("--check")
        .arg("--strict")
        .arg("--warn")
        .arg("bad_checksum.txt")
        .assert();

    assert
        .failure()
        .code(1)
        .stderr("Improperly formatted line at \"bad_checksum.txt:1\"\n");
}
