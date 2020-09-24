use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

mod config;

struct Rem {
    path: String,
}

impl Rem {
    fn new(path: String) -> Rem {
        Rem { path: path }
    }

    fn cat(&self) {
        let contents = fs::read_to_string(&self.path).expect("Could not read rem notes!");
        print!("{}", contents);
    }

    fn write_note(&self, note: Vec<String>) -> std::io::Result<()> {
        let whole_note = format!("{}\n", note.join(" "));

        let notes_path = PathBuf::from(&self.path);
        let mut file = OpenOptions::new().append(true).open(notes_path)?;
        file.write_all(whole_note.as_bytes())?;

        Ok(())
    }
}

fn main() {
    let opts = config::Opt::from_args();

    let home = std::env::var("HOME").unwrap();
    let notes_path = format!("{}/notes.rem", home);
    let rem = Rem::new(notes_path);

    match opts {
        config::Opt::Cat {} => rem.cat(),
        config::Opt::Add { note } => rem.write_note(note).expect("Could not add note!"),
    }
}
