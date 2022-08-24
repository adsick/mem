# mem

### a simple cli app that allows for keeping track of your learning experience.

### things you can do with mem include:
- adding links to read list. mem then reminds you to read them.
- checking them out when you're done
- creating and managing todo's
- building your own knowledge base with documents and links between them.
- query for content by using powerful indexing and filtering queries (e.g. todo#1-5 with(math, university) without(boring))

check out [features](features.md) list and create an issue in case you need something more.

## crates selection
we will pick some of them in the process of development.
### CLI
- [bpaf](https://lib.rs/crates/bpaf)
- [clap](https://github.com/clap-rs/clap)
- [inquire](https://github.com/mikaelmello/inquire)
- [crossterm](https://github.com/crossterm-rs/crossterm)


### parsing&serialization
- [serde](https://github.com/serde-rs/serde)
- [nom](https://github.com/geal/nom)

### GUI
- [dioxus](https://github.com/dioxuslabs/dioxus)
- [egui](https://github.com/emilk/egui)
- [tauri](https://tauri.app/)

### Web stuff (separate backend and frontend later)
- [axum](https://github.com/tokio-rs/axum)
- [rocket](https://rocket.rs)
- ...

### other
- [chrono](https://github.com/chronotope/chrono)
- strum
- some utils for working with text
- [notify](https://github.com/notify-rs/notify) (detects file changes)
- [rhai](https://github.com/rhaiscript/rhai) in case we will need scripting/plugins
- maybe some hashing libs