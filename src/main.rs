mod app;
mod play;
mod print;
mod test;
mod watch;
use crate::play::{
    play,
    Play::{Once, Watch},
};
use notify::Error as NotifyError;
use std::env;
use thiserror::Error;
use weresocool::core::ui::were_so_cool_logo;
use weresocool::error::Error as WscError;

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
    #[error("`{0}")]
    Message(String),
}

fn main() -> Result<(), Error> {
    were_so_cool_logo();
    let cwd = env::current_dir()?;

    let matches = app::app().get_matches();

    match matches.subcommand() {
        ("play", play_args) => play(play_args, cwd, Once)?,
        ("watch", play_args) => play(play_args, cwd, Watch)?,
        ("print", print_args) => print::print(print_args)?,
        _ => unimplemented!(),
    }
    Ok(())
}
