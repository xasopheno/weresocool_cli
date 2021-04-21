use crate::Error;
use clap::ArgMatches;
use weresocool::generation::{RenderType, WavType};
use weresocool::interpretable::InputType;
use weresocool::interpretable::Interpretable;

pub fn print(print_args: Option<&ArgMatches>) -> Result<(), Error> {
    let args = print_args.ok_or(Error::Message("No print args".to_string()))?;

    let mut printed: Vec<&str> = vec![];
    let should_print = |target: &[&str]| -> bool {
        let result = target.iter().any(|arg| args.is_present(arg));
        result
    };
    let filename = args
        .values_of("file")
        .ok_or(Error::Message(
            "Filename required. Usage: weresocool print [FILENAME] [FLAGS]".to_string(),
        ))?
        .collect::<Vec<_>>()
        .first()
        .expect("No Filename")
        .to_string();

    println!("Filename: {}", filename);
    if should_print(&["all", "wav", "sound"]) {
        println!("{}", "printing .wav...");

        InputType::Filename(&filename).make(RenderType::Wav(WavType::Wav { cli: true }), None)?;
        printed.push("wav")
    }
    if should_print(&["all", "mp3", "sound"]) {
        println!("{}", "printing .mp3...");

        InputType::Filename(&filename).make(RenderType::Wav(WavType::Mp3 { cli: true }), None)?;
        printed.push("mp3")
    }
    if should_print(&["all", "csv"]) {
        println!("{}", "printing .csv...");
        InputType::Filename(&filename).make(RenderType::Csv1d, None)?;
        printed.push("csv")
    }
    if should_print(&["all", "json"]) {
        println!("{}", "printing .json...");
        InputType::Filename(&filename).make(RenderType::Json4d, None)?;
        printed.push("json")
    }
    if printed.is_empty() {
        InputType::Filename(&filename).make(RenderType::Wav(WavType::Wav { cli: true }), None)?;
        println!("{}", "printing .wav (default)...");
    }

    println!("\t{}", "done");
    Ok(())
}
