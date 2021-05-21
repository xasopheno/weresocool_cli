use crate::Error;
use notify::{watcher, RecursiveMode, Watcher};
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use weresocool::interpretable::InputType::Filename;
use weresocool::manager::prepare_render_outside;
use weresocool::manager::RenderManager;

pub fn watch(
    filename: String,
    working_path: PathBuf,
    render_manager: Arc<Mutex<RenderManager>>,
) -> Result<(), Error> {
    std::thread::spawn(move || -> Result<(), Error> {
        loop {
            let (tx, rx) = channel();

            let mut watcher = watcher(tx, Duration::from_millis(10))?;

            let path = Path::new(&working_path).join(Path::new(&filename));

            watcher.watch(path, RecursiveMode::NonRecursive)?;
            if let Ok(_event) = rx.recv() {
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
        }
    });
    Ok(())
}
