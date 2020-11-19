use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rem")]
pub enum Opt {
    Add {
        note: Vec<String>,

        #[structopt(short, long)]
        name: Option<String>,
    },
    Cat {
        #[structopt(short, long)]
        numbered: bool,

        #[structopt(short, long)]
        without_names: bool,
    },
    Del {
        line: u32,

        #[structopt(short, long)]
        force: bool,
    },
    Edit {
        line: u32,
    },
}
