use crate::common::*;
use crate::{Config, Docs};

// there are 2 approaches: root first and config first
// with root first we specify the root (e.g. current dir) and load config from there
// with config first we load the config and then determine the root path

// root contains directories with documents

pub struct App {
    config: Config,
    root: PathBuf,
    // mode: Mode,
    docs: Docs,
}

impl App {
    pub fn new(root: PathBuf) -> Self {
        // try to load docs from the path
        let docs = todo!();
        let config = Config::load().unwrap();

        // let mode = config.mode;

        let root = config.default_path.unwrap_or(root);

        Self { root, config, docs }
    }

    pub fn run(&mut self, args: impl Iterator<Item = String>) {
        todo!();
    }
}

pub enum Mode {
    // Startup,
    // Command,
    // Interactive
}
