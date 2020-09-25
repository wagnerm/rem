use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

mod config;

struct Rem {
    rem_config_path: String,
}

impl Rem {
    fn new(path: String) -> Rem {
        Rem { path: path }
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

    // set config
    fn set_config(&self, config_type: config::ConfigType) -> Result<(), Box<dyn Error>> {
        match config_type {
            config::ConfigType::SetNotePath { note_path } => self.set_note_path(note_path),
        }
    }

    fn set_note_path(&self, note_path: String) -> Result<(), Box<dyn Error>> {
        let notes_path = PathBuf::from(&self.path);
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

    let home = std::env::var("HOME").unwrap();
    let rem_config_path = format!("{}/.rem", home);
    let rem = Rem::new(rem_config_path);

    match opts {
        config::Opt::Cat { numbered } => rem.cat(numbered),
        config::Opt::Add { note } => rem.write_note(note).expect("Could not add note!"),
        config::Opt::Del { line, force } => rem
            .delete_line(line, force)
            .expect("Could not delete line!"),
        config::Opt::Config { config_type } => {
            rem.set_config(config_type).expect("Counld not set config!")
        }
    }
}
