use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;
mod config;

struct Rem {
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Note {
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Notes {
    notes: Vec<Note>,
}

impl Rem {
    fn new() -> Rem {
        Rem {
            path: Rem::notes_path(),
        }
    }

    fn new_with_path(path: String) -> Rem {
        Rem { path: path }
    }

    fn notes_path() -> String {
        match env::var("REM_CLI_NOTES_PATH") {
            Ok(notes_path) => notes_path,
            Err(_) => {
                let home = std::env::var("HOME").unwrap();
                format!("{}/rem_notes.txt", home)
            }
        }
    }

    fn cat(&self, numbered: bool) -> Result<(), Box<dyn Error>> {
        let contents = self.read_note_file()?;
        if contents.notes.len() == 0 {
            println!("No notes found! Try adding a note!");
        }

        for (i, note) in contents.notes.iter().enumerate() {
            if numbered {
                print!("{}: {}\n", i, note.text)
            } else {
                println!("{}", note.text.trim());
            }
        }

        Ok(())
    }

    fn write_note(&self, note: Vec<String>) -> Result<(), Box<dyn Error>> {
        let whole_note = format!("{}", note.join(" "));
        let n = Note { text: whole_note };

        let mut notes = self.read_note_file()?;
        notes.notes.push(n);

        self.write_all_notes(notes)?;

        Ok(())
    }

    fn write_all_notes(&self, notes: Notes) -> Result<(), Box<dyn Error>> {
        let serialized = serde_yaml::to_string(&notes)?;

        let notes_path = PathBuf::from(&self.path);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(notes_path)?;

        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    fn delete_line(&self, line: u32, force: bool) -> Result<(), Box<dyn Error>> {
        let mut n = self.read_note_file()?;

        if n.notes.is_empty() {
            println!("No notes found! Try adding a note!");
        } else if n.notes.len() - 1 < line as usize {
            println!("Line specified not in notes!");
        } else {
            if !force && !self.confirm()? {
                println!("Deletion stopped.");
                return Ok(());
            }

            n.notes.remove(line as usize);
            println!("{}", n.notes.len());
            self.write_all_notes(n)?;

            println!("Removed: {}", line);
        }

        Ok(())
    }

    fn read_note_file(&self) -> Result<Notes, Box<dyn Error>> {
        let notes_path = PathBuf::from(&self.path);
        if !notes_path.exists() {
            // Treat a missing notes file as no notes since the user can
            // Create notes are any time in this file.
            // This is more user friendly than spitting out an error.
            Ok(Notes { notes: vec![] })
        } else {
            let contents = fs::read_to_string(&self.path)?;
            println!("{} {}", contents.len(), contents);
            if contents.len() == 0 || contents == String::from("\n") {
                // the file only contains a new line or is empty
                Ok(Notes { notes: vec![] })
            } else {
                let deserialized_contents: Notes = serde_yaml::from_str(&contents)?;
                Ok(deserialized_contents)
            }
        }
    }

    fn confirm(&self) -> io::Result<bool> {
        let mut input = String::new();

        loop {
            print!("Are you sure you want to continue? (y/n) ");
            let _ = io::stdout().flush();

            io::stdin().read_line(&mut input)?;

            let c = match &*input.trim().to_lowercase() {
                "y" | "yes" => true,
                "n" | "no" => false,
                _ => {
                    continue;
                }
            };

            return Ok(c);
        }
    }
}

fn main() {
    let opts = config::Opt::from_args();

    let rem = Rem::new();

    match opts {
        config::Opt::Cat { numbered } => rem.cat(numbered).expect("Cound not read notes!"),
        config::Opt::Add { note } => rem.write_note(note).expect("Could not add note!"),
        config::Opt::Del { line, force } => rem
            .delete_line(line, force)
            .expect("Could not delete line!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_notes_location() {
        env::remove_var("REM_CLI_NOTES_PATH");
        let home = env::var("HOME").unwrap();
        assert_eq!(format!("{}/rem_notes.txt", home), Rem::notes_path());
    }

    #[test]
    fn test_env_overrides_notes_location() {
        env::set_var("REM_CLI_NOTES_PATH", "/cloud_drive/rem_notes.txt");

        assert_eq!("/cloud_drive/rem_notes.txt", Rem::notes_path());
    }

    #[test]
    fn test_read_rem_notes_file() {
        let file = NamedTempFile::new().unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path);

        rem.write_note(vec![String::from("new note who dis")])
            .unwrap();

        let n = rem.read_note_file().unwrap();
        assert_eq!(1, n.notes.len());
        assert_eq!("new note who dis", n.notes[0].text);
    }

    #[test]
    fn test_rem_formats_file_in_yaml() {
        let file = NamedTempFile::new().unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path.clone());

        rem.write_note(vec![String::from("new note who dis")])
            .unwrap();

        let yaml = fs::read_to_string(path).unwrap();
        assert_eq!(
            String::from("---\nnotes:\n  - text: new note who dis"),
            yaml
        );
    }

    #[test]
    fn test_rem_adds_multiple_notes() {
        let file = NamedTempFile::new().unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path.clone());

        rem.write_note(vec![String::from("first")]).unwrap();
        rem.write_note(vec![String::from("second")]).unwrap();

        let yaml = fs::read_to_string(path).unwrap();
        assert_eq!(
            String::from("---\nnotes:\n  - text: first\n  - text: second"),
            yaml
        );
    }
}
