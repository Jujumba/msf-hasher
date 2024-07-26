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
fn test_cli_quiet() {
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("--check")
        .arg("--quiet")
        .arg("checksum.txt")
        .assert();

    assert.success().code(0).stderr("").stdout("");
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
fn test_cli_check_strict_warn() {
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

#[test]
fn test_md5_hashing() {
    let mut cmd = get_cmd();

    let assert = cmd
        .arg("--hash")
        .arg("MD5")
        .write_stdin("Hello world")
        .assert();
    assert
        .success()
        .code(0)
        .stderr("")
        .stdout("3e25960a79dbc69b674cd4ec67a72c62  -\n");
}

#[test]
fn test_sha1_hashing() {
    let mut cmd = get_cmd();

    let assert = cmd
        .arg("--hash")
        .arg("SHA1")
        .write_stdin("Hello world")
        .assert();
    assert
        .success()
        .code(0)
        .stderr("")
        .stdout("7b502c3a1f48c8609ae212cdfb639dee39673f5e  -\n");
}

#[test]
fn test_sha512_hashing() {
    let mut cmd = get_cmd();

    let assert = cmd
        .arg("--hash")
        .arg("SHA512")
        .write_stdin("Hello world")
        .assert();
    assert
        .success()
        .code(0)
        .stderr("")
        .stdout("b7f783baed8297f0db917462184ff4f08e69c2d5e5f79a942600f9725f58ce1f29c18139bf80b06c0fff2bdd34738452ecf40c488c22a7e3d80cdf6f9c1c0d47  -\n");
}
