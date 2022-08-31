use crate::common::*;
use crate::{Config, Index};

// there are 2 approaches: root first and config first
// with root first we specify the root (e.g. current dir) and load config from there
// with config first we load the config and then determine the root path

// root contains directories with documents

pub struct App {
    config: Config,
    root: PathBuf,
    // mode: Mode,
    index: Index,
}

impl App {
    pub fn new(root: PathBuf) -> Self {
        // try to load docs from the path
        let config = Config::load().unwrap();

        let root = config.default_path.to_owned().unwrap_or(root);

        let mut index = Index::new(root.clone());
        // let mode = config.mode;

        assert!(
            std::env::set_current_dir(&root).is_ok(),
            "unable to set current directory, abort"
        );
        index.scan(&root).unwrap();

        Self {
            root,
            config,
            index,
        }
    }

    pub fn run(&mut self, args: impl Iterator<Item = String>) {
        self.config.write().unwrap();
    }
}

pub enum Mode {
    // Startup,
    // Command,
    // Interactive
}
