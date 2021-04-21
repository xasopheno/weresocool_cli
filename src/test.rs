#[cfg(test)]
mod test {
    use assert_cmd::Command;

    #[test]
    fn it_plays_a_file() {
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
    }

    // let buffered = BufReader::new(input);

    // for line in buffered.lines() {
    // println!("{}", line?);
    // }
}
