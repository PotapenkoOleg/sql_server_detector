use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long, short, default_value = "./FINAL/")]
    pub input_directory: String,
}