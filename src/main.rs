mod test;
use clap::App;
use clap::Arg;
use clap::SubCommand;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use weresocool::manager::prepare_render_outside;
use weresocool::portaudio::real_time_render_manager;
use weresocool::{error::Error, interpretable::InputType::Filename, manager::RenderManager};

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// use weresocool::ui::were_so_cool_logo;

fn main() {
    // were_so_cool_logo();
    let cwd = env::current_dir().unwrap();

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
                .unwrap()
                .values_of("file")
                .unwrap()
                .collect::<Vec<_>>();
            play_file(
                filenames.first().unwrap().to_string(),
                cwd.display().to_string(),
            )
            .unwrap();
        }
        _ => unimplemented!(),
    }
}

fn watch(filename: String, working_path: String, render_manager: Arc<Mutex<RenderManager>>) -> () {
    thread::spawn(move || loop {
        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_millis(1000)).unwrap();

        watcher
            .watch(
                "/Users/user1/code/weresocool_cmd/song.socool",
                RecursiveMode::Recursive,
            )
            .unwrap();
        match rx.recv() {
            Ok(_event) => {
                // println!("{:?}", event);
                let render_voices =
                    match prepare_render_outside(Filename(&filename), Some(working_path.clone())) {
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
    });
}

fn play_file(filename: String, working_path: String) -> Result<(), Error> {
    let render_voices = prepare_render_outside(Filename(&filename), Some(working_path.clone()));

    let render_manager = Arc::new(Mutex::new(RenderManager::init(render_voices?)));
    watch(filename, working_path, render_manager.clone());
    let mut stream = real_time_render_manager(Arc::clone(&render_manager)).unwrap();

    stream.start()?;
    loop {}
    // Ok(())
}
