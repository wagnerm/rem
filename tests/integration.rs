use assert_cmd::Command;
use tempfile::NamedTempFile;

fn rem_cmd(path: &String) -> Command {
    let mut cmd = Command::cargo_bin("rem-cli").unwrap();
    cmd.env_remove("REM_CLI_NOTES_PATH");
    cmd.env("REM_CLI_NOTES_PATH", path);
    cmd
}

#[test]
fn test_simple() {
    let file = NamedTempFile::new().unwrap();
    let path = String::from(file.path().to_str().unwrap());

    let mut cmd_for_add = rem_cmd(&path);
    cmd_for_add.arg("add").arg("new note who dis").assert();

    let mut cmd_for_cat = rem_cmd(&path);
    cmd_for_cat.arg("cat").assert().stdout("new note who dis\n");
}

#[test]
fn test_add_many_notes() {
    let file = NamedTempFile::new().unwrap();
    let path = String::from(file.path().to_str().unwrap());

    for i in 0..3 {
        let mut cmd_for_add = rem_cmd(&path);
        cmd_for_add.arg("add").arg(format!("{}-hobbit", i)).assert();
    }

    let mut cmd_for_cat = rem_cmd(&path);
    cmd_for_cat
        .arg("cat")
        .assert()
        .stdout("0-hobbit\n1-hobbit\n2-hobbit\n");
}

#[test]
fn test_add_with_name() {
    let file = NamedTempFile::new().unwrap();
    let path = String::from(file.path().to_str().unwrap());

    let mut cmd_for_add = rem_cmd(&path);
    cmd_for_add
        .arg("add")
        .arg("-n")
        .arg("hobbits")
        .arg("frodo")
        .assert();

    let mut cmd_for_cat = rem_cmd(&path);
    cmd_for_cat
        .arg("cat")
        .arg("-n")
        .assert()
        .stdout("0: hobbits ~ frodo\n");
}

#[test]
fn test_delete() {
    let file = NamedTempFile::new().unwrap();
    let path = String::from(file.path().to_str().unwrap());

    let mut cmd_for_add = rem_cmd(&path);
    cmd_for_add
        .arg("add")
        .arg("-n")
        .arg("hobbits")
        .arg("frodo")
        .assert();

    let mut cmd_for_cat = rem_cmd(&path);
    cmd_for_cat
        .arg("cat")
        .arg("-n")
        .assert()
        .stdout("0: hobbits ~ frodo\n");

    let mut cmd_for_del = rem_cmd(&path);
    cmd_for_del
        .arg("del")
        .arg("0")
        .arg("-f")
        .assert()
        .stdout("Removed: 0\n");

    let mut cmd_for_verify = rem_cmd(&path);
    cmd_for_verify
        .arg("cat")
        .arg("-n")
        .assert()
        .stdout("No notes found! Try adding a note!\n");
}

#[test]
fn test_delete_out_of_bounds() {
    let file = NamedTempFile::new().unwrap();
    let path = String::from(file.path().to_str().unwrap());

    let mut cmd_for_add = rem_cmd(&path);
    cmd_for_add.arg("add").arg("new note who dis").assert();

    let mut cmd_for_del = rem_cmd(&path);
    cmd_for_del
        .arg("del")
        .arg("10")
        .assert()
        .stdout("Line specified not in notes!\n");
}
