use crate::watch::watch;
use crate::Error;
use clap::ArgMatches;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use weresocool::interpretable::InputType::Filename;
use weresocool::manager::prepare_render_outside;
use weresocool::manager::RenderManager;
use weresocool::portaudio::real_time_render_manager;

pub fn play(play_args: Option<&ArgMatches>, cwd: PathBuf) -> Result<(), Error> {
    let filename = play_args
        .ok_or(Error::Message("No play args".to_string()))?
        .values_of("file")
        .ok_or(Error::Message("No value of file".to_string()))?
        .collect::<Vec<_>>()
        .first()
        .expect("No filename")
        .to_string();
    play_file(filename, cwd)?;
    Ok(())
}

fn play_file(filename: String, working_path: PathBuf) -> Result<(), Error> {
    let (tx, rx) = std::sync::mpsc::channel::<bool>();
    let render_voices = prepare_render_outside(Filename(&filename), Some(working_path.clone()));

    let render_manager = Arc::new(Mutex::new(RenderManager::init(
        render_voices?,
        Some(tx),
        true,
    )));
    watch(filename, working_path, render_manager.clone())?;
    let mut stream = real_time_render_manager(Arc::clone(&render_manager))?;

    stream.start()?;

    match rx.recv() {
        Ok(v) => {
            dbg!(v);
            Ok(())
        }
        Err(e) => {
            dbg!(e);
            Err(Error::Message("error".to_string()))
        }
    }
}
