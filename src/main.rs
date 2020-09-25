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

    fn notes_path() -> String {
        match env::var("REM_CLI_NOTES_PATH") {
            Ok(notes_path) => notes_path,
            Err(_) => {
                let home = std::env::var("HOME").unwrap();
                format!("{}/rem_notes.txt", home)
            }
        }
    }

    fn cat(&self, numbered: bool) {
        let notes_path = PathBuf::from(&self.path);
        if !notes_path.exists() {
            println!("No notes found! Try adding a note! `rem add Is mayonnaise an instrument?`")
        } else {
            let contents = fs::read_to_string(&self.path).expect("Could not read notes!");
            for (i, line) in contents.lines().enumerate() {
                if numbered {
                    print!("{}: {}\n", i, line)
                } else {
                    println!("{}", line.trim());
                }
            }
        }
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

        if !notes_path.exists() {
            println!("No notes found! Try adding a note! `rem add Is mayonnaise an instrument?`");
            Ok(())
        } else {
            let contents = fs::read_to_string(&self.path).expect("Could not read notes!");
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
                file.write_all(lines.join("\n").as_bytes())?;

                println!("Remove: {}", line);
            }

            Ok(())
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
        config::Opt::Cat { numbered } => rem.cat(numbered),
        config::Opt::Add { note } => rem.write_note(note).expect("Could not add note!"),
        config::Opt::Del { line, force } => rem
            .delete_line(line, force)
            .expect("Could not delete line!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
