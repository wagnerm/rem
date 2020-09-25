use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rem")]
pub enum Opt {
    Add {
        note: Vec<String>,
    },
    Cat {
        #[structopt(short, long)]
        numbered: bool,
    },
    Config {
        #[structopt(subcommand)]
        config_type: ConfigType,
    },
    Del {
        line: u32,

        #[structopt(short, long)]
        force: bool,
    },
}

#[derive(StructOpt, Debug)]
pub enum ConfigType {
    SetNotePath { note_path: String },
}
