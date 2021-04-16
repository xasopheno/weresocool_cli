#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use std::env;

    #[test]
    fn it_plays_a_file() {
        let mut cwd = env::current_dir().unwrap();
        let mut cmd = Command::new("cargo");

        cwd.push("song.socool");
        // let expected_filepath = cwd.display();

        let assert = cmd
            .arg("run")
            .arg("--release")
            .arg("--")
            .arg("play")
            .arg("song.socool")
            .assert();

        assert.success();
        // .stdout(format!("Playing: {}\n", expected_filepath.to_string()));
    }
}
