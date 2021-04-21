#[cfg(test)]
mod test {
    use assert_cmd::Command;

    #[test]
    fn it_plays_a_cool_file() {
        let mut cmd = Command::new("cargo");

        cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("play")
            .arg("test_data/play.socool")
            .assert()
            .success();
    }

    #[test]
    fn it_prints_a_csv() {
        let mut cmd = Command::new("cargo");

        cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("print")
            .arg("test_data/play.socool")
            .arg("--csv")
            .assert()
            .success();

        let expected_filename = "test_data/play.socool.csv";
        let written_filename = "play.socool.csv";
        assert_same_file_contents(expected_filename, written_filename)
    }

    #[test]
    fn it_prints_a_json() {
        let mut cmd = Command::new("cargo");

        cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("print")
            .arg("test_data/play.socool")
            .arg("--json")
            .assert()
            .success();

        let expected_filename = "test_data/play.socool.json";
        let written_filename = "play.socool.json";
        assert_same_file_contents(expected_filename, written_filename)
    }

    fn assert_same_file_contents(expected_filename: &str, written_filename: &str) {
        let expected = std::fs::read_to_string(expected_filename)
            .expect("Something went wrong reading the file");
        let written = std::fs::read_to_string(written_filename)
            .expect("Something went wrong reading the file");

        assert!(expected == written);
    }
}
