use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Oleg Potapenko",
    version,
    about = "Utility for detection Sql Server specific keywords and functions is translated SQL code"
)]
pub struct Args {
    #[arg(long, short /*, default_value = "./FINAL/"*/)]
    pub input_directory: String,

    #[arg(long, short)]
    pub keywords_file_name: Option<String>,

    #[arg(long, short, default_value = "true")]
    pub load_sql_server_keywords: bool,

    #[arg(long, short, default_value = "false")]
    pub suppress_banner: bool,
}
