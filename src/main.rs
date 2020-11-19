use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use tempfile::NamedTempFile;
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

impl Note {
    fn new(text: String) -> Note {
        Note { text: text }
    }
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
        if whole_note.is_empty() || whole_note.trim().is_empty() {
            println!("Your note is empty, try adding some content.");
            return Ok(());
        }
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
            self.write_all_notes(n)?;

            println!("Removed: {}", line);
        }

        Ok(())
    }

    fn edit_note(&self, line: u32) -> Result<(), Box<dyn Error>> {
        if let Ok(editor) = self.get_editor() {
            let mut n = self.read_note_file()?;

            if n.notes.is_empty() {
                println!("No notes found! Try adding a note!");
            } else if n.notes.len() - 1 < line as usize {
                println!("Line specified not in notes!");
            } else {
                let raw = self.edit(editor, &n.notes[line as usize].text)?;
                let trimmed_text = String::from(raw.trim());

                n.notes[line as usize] = Note::new(trimmed_text.clone());
                self.write_all_notes(n)?;

                println!("Note committed! {}", trimmed_text);
            }
        } else {
            println!("EDITOR is not set!");
            return Ok(());
        }

        Ok(())
    }

    fn edit(&self, editor: String, initial_text: &String) -> Result<String, Box<dyn Error>> {
        let mut f = NamedTempFile::new()?;
        f.write_all(initial_text.as_bytes())?;

        // Close the file, but persist it for reading from the editor
        let temp_path = f.into_temp_path();
        let path = String::from(temp_path.to_str().unwrap());

        Command::new(editor).arg(&path).status()?;

        let raw_path = PathBuf::from(&path);
        let raw = fs::read_to_string(&raw_path)?;

        Ok(raw)
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

    fn get_editor(&self) -> Result<String, env::VarError> {
        match env::var("EDITOR") {
            Ok(editor) => Ok(editor),
            Err(e) => Err(e),
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
        config::Opt::Edit { line } => rem.edit_note(line).expect("Cound not edit note!"),
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

    #[test]
    fn test_rem_deletes_note() {
        let file = NamedTempFile::new().unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path.clone());

        rem.write_note(vec![String::from("first")]).unwrap();
        rem.write_note(vec![String::from("second")]).unwrap();
        rem.write_note(vec![String::from("third")]).unwrap();

        rem.delete_line(1, true).unwrap();

        let yaml = fs::read_to_string(path).unwrap();
        assert_eq!(
            String::from("---\nnotes:\n  - text: first\n  - text: third"),
            yaml
        );
    }

    #[test]
    fn test_rem_deletes_from_empty_note() {
        let file = NamedTempFile::new().unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path.clone());

        rem.delete_line(0, true).unwrap();

        let yaml = fs::read_to_string(path).unwrap();
        assert!(yaml.is_empty());
    }

    #[test]
    fn test_rem_delete_skips_out_of_bounds() {
        let file = NamedTempFile::new().unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path.clone());

        rem.write_note(vec![String::from("first")]).unwrap();

        rem.delete_line(1, true).unwrap();

        let yaml = fs::read_to_string(path).unwrap();
        assert_eq!(String::from("---\nnotes:\n  - text: first"), yaml);
    }

    #[test]
    fn test_rem_writes_note() {
        let file = NamedTempFile::new().unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path.clone());

        rem.write_note(vec![String::from("new note who dis")])
            .unwrap();

        let n = rem.read_note_file().unwrap();
        assert_eq!(1, n.notes.len());
        assert_eq!("new note who dis", n.notes[0].text);
    }

    #[test]
    fn test_rem_rejects_empty_note() {
        let file = NamedTempFile::new().unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path.clone());

        rem.write_note(vec![String::from("")]).unwrap();

        let n = rem.read_note_file().unwrap();
        assert_eq!(0, n.notes.len());
    }

    #[test]
    fn test_rem_rejects_whitespace_note() {
        let file = NamedTempFile::new().unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path.clone());

        rem.write_note(vec![String::from("                ")])
            .unwrap();

        let n = rem.read_note_file().unwrap();
        assert_eq!(0, n.notes.len());
    }

    #[test]
    fn test_get_editor() {
        env::remove_var("EDITOR");
        env::set_var("EDITOR", "foo");
        let rem = Rem::new();
        let editor = rem.get_editor().unwrap();
        assert_eq!("foo", editor);
    }
}
