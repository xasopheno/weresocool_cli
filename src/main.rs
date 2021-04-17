mod test;
use clap::App;
use clap::Arg;
use clap::SubCommand;
use notify::{watcher, Error as NotifyError, RecursiveMode, Watcher};
use std::env;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use thiserror::Error;
use weresocool::manager::prepare_render_outside;
use weresocool::portaudio::real_time_render_manager;
use weresocool::{interpretable::InputType::Filename, manager::RenderManager};
use weresocool_error::Error as WscError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("WereSoCoolError: `{0}`")]
    WereSoCoolError(#[from] WscError),
    #[error("PortAudioError: `{0}`")]
    PortAudioError(#[from] portaudio::error::Error),
    #[error("NotifyError: `{0}`")]
    NotifyError(#[from] NotifyError),
    #[error("IoError: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("Message: `{0}`")]
    Message(String),
}

// use weresocool::ui::were_so_cool_logo;

fn main() -> Result<(), Error> {
    // were_so_cool_logo();
    let cwd = env::current_dir()?;

    let matches = App::new("WereSoCool CLI")
        .version("1.0")
        .author("Danny Meyer")
        .about("Does cool things")
        .subcommand(
            SubCommand::with_name("play")
                .about("Play a song")
                .help("play .socool file")
                .arg(
                    Arg::with_name("file")
                        .multiple(false)
                        .number_of_values(1)
                        .index(1)
                        .help("play .socool file"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("play", play_args) => {
            let filenames = play_args
                .ok_or(Error::Message("No play args".to_string()))?
                .values_of("file")
                .ok_or(Error::Message("No value of file".to_string()))?
                .collect::<Vec<_>>();
            play_file(filenames[0].to_string(), cwd.display().to_string())?;
        }
        _ => unimplemented!(),
    }
    Ok(())
}

fn watch(
    filename: String,
    working_path: String,
    render_manager: Arc<Mutex<RenderManager>>,
) -> Result<(), Error> {
    thread::spawn(move || -> Result<(), Error> {
        loop {
            let (tx, rx) = channel();

            let mut watcher = watcher(tx, Duration::from_millis(10))?;
            let path = Path::new(&working_path).join(Path::new(&filename));

            watcher.watch(path, RecursiveMode::NonRecursive)?;
            match rx.recv() {
                Ok(event) => {
                    // println!("{:?}", event);
                    let render_voices = match prepare_render_outside(
                        Filename(&filename),
                        Some(working_path.clone()),
                    ) {
                        Ok(result) => Some(result),
                        Err(error) => {
                            println!("{}", error);
                            None
                        }
                    };
                    if let Some(voices) = render_voices {
                        render_manager.lock().unwrap().push_render(voices);
                    }
                }
                Err(_) => {}
            }
        }
    });
    Ok(())
}

fn play_file(filename: String, working_path: String) -> Result<(), Error> {
    let render_voices = prepare_render_outside(Filename(&filename), Some(working_path.clone()));

    let render_manager = Arc::new(Mutex::new(RenderManager::init(render_voices?)));
    watch(filename, working_path, render_manager.clone())?;
    let mut stream = real_time_render_manager(Arc::clone(&render_manager))?;

    stream.start()?;
    loop {}
}