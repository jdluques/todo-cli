use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long, help = "Update configuration values")]
    pub config: Option<String>,
}
