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
        if contents.is_empty() {
            println!("No notes found! Try adding a note!");
        }

        for (i, line) in contents.lines().enumerate() {
            if numbered {
                print!("{}: {}\n", i, line)
            } else {
                println!("{}", line.trim());
            }
        }

        Ok(())
    }

    fn write_note(&self, note: Vec<String>) -> std::io::Result<()> {
        let whole_note = format!("{}\n", note.join(" "));

        let notes_path = PathBuf::from(&self.path);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(notes_path)?;
        file.write_all(whole_note.as_bytes())?;

        Ok(())
    }

    fn delete_line(&self, line: u32, force: bool) -> Result<(), Box<dyn Error>> {
        let notes_path = PathBuf::from(&self.path);

        let contents = self.read_note_file()?;
        let mut lines: Vec<&str> = contents.lines().collect();

        if lines.is_empty() {
            println!("No notes found! Try adding a note!");
        } else if lines.len() - 1 < line as usize {
            println!("Line specified not in notes!");
        } else {
            if !force && !self.confirm()? {
                println!("Deletion stopped.");
                return Ok(());
            }

            lines.remove(line as usize);

            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(notes_path)?;

            // Ensure the last line is a new line
            let last_line = lines.pop().unwrap();
            let new_last_line = format!("{}\n", last_line);
            lines.push(new_last_line.as_str());

            file.write_all(lines.join("\n").as_bytes())?;

            println!("Remove: {}", line);
        }

        Ok(())
    }

    fn read_note_file(&self) -> Result<String, Box<dyn Error>> {
        let notes_path = PathBuf::from(&self.path);
        if !notes_path.exists() {
            // Treat a missing notes file as no notes since the user can
            // Create notes are any time in this file.
            // This is more user friendly than spitting out an error.
            Ok(String::from(""))
        } else {
            let contents = fs::read_to_string(&self.path)?;
            Ok(contents)
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
        let mut file = NamedTempFile::new().unwrap();
        file.write_all("new note who dis".as_bytes()).unwrap();
        let path = String::from(file.path().to_str().unwrap());
        let rem = Rem::new_with_path(path);

        assert_eq!("new note who dis", rem.read_note_file().unwrap());
    }
}
