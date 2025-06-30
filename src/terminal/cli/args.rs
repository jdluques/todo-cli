use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long, help = "Update configuration values")]
    pub config: Option<String>,

    #[arg(short = 'v', long = "visual", help = "Print the to-do list without interaction")]
    pub visual: bool,
}
