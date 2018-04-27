use std::sync::mpsc;
use notify::{self, RecursiveMode, RecommendedWatcher};
use vulkano::device::Device;
use std::{thread, fs::{self, File}};
use glsl_to_spirv::{self, ShaderType};
use common::*;
use shader;

pub struct Watcher {
  #[allow(dead_code)]
  /// Keeps inner watcher alive for the lifetime
  /// of the outer watcher.
  file_watcher: RecommendedWatcher,
  rx:           mpsc::Receiver<shader::fs::Shader>,
}

impl Watcher {
  pub fn new<P: AsRef<Path>>(path: P, device: Arc<Device>) -> notify::Result<Watcher> {
    let watched_path = fs::canonicalize(path)
      .map_err(|io_error| notify::Error::Io(io_error))?;

    info!("Watcher::new: watching {}", watched_path.display());

    let (notify_tx, notify_rx) = mpsc::channel();

    let file_watcher = {
      use notify::Watcher;

      let mut file_watcher: notify::RecommendedWatcher =
        notify::Watcher::new(notify_tx, Duration::from_millis(1))?;
      file_watcher.watch(&watched_path, RecursiveMode::Recursive)?;

      file_watcher
    };

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
      loop {
        let event = match notify_rx.recv() {
          Ok(event) => event,
          Err(mpsc::RecvError) => {
            error!("notify channel disconnected. stopping resource watcher.");
            return;
          }
        };

        use notify::DebouncedEvent::*;
        match event {
          Write(path) |
          Create(path) => {
            info!("resource changed: {}", path.display());
            match path.strip_prefix(&watched_path) {
              Ok(name) => if let Some(resource) = Watcher::process(&path, name.as_os_str(), &device) {
                if let Err(mpsc::SendError(_)) = tx.send(resource) {
                  error!("failed to send resource. stopping resource watcher. T_T");
                  return;
                }
              },
              Err(_) => {
                error!("failed to strip prefix from event path. stopping resource watcher. >_<");
                return;
              }
            }
          }
          Error(error, None) => error!("notify error: {}", error),
          Error(error, Some(path)) => error!("notify error: {} for: {}", error, path.display()),
          _ => trace!("notify event: {:?}", event),
        }
      }
    });

    Ok(Watcher {
      file_watcher,
      rx,
    })
  }

  fn process(path: &Path, name: &OsStr, device: &Arc<Device>) -> Option<shader::fs::Shader> {
    if name == "fs.glsl" {
      info!("recompiling fragment shader");
      let mut source = String::new();
      File::open(path).unwrap().read_to_string(&mut source).unwrap();
      let mut output_file = glsl_to_spirv::compile(&source, ShaderType::Fragment).unwrap();
      let mut binary = Vec::new();
      output_file.read_to_end(&mut binary).unwrap();
      unsafe {
        Some(shader::fs::Shader::load_binary(device.clone(), &binary).expect("failed to create shader module"))
      }
    } else {
      None
    }
  }

  pub fn try_recv(&self) -> Option<shader::fs::Shader> {
    self.rx.try_recv().ok()
  }
}
