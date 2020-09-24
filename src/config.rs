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
}
