use clap::{Parser, Subcommand};
mod generate;
mod init;
use init::init;
use generate::generate;

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct AppArg {
    #[clap(subcommand)]
    commands: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    Init,
    #[clap(arg_required_else_help = true)]
    Generate {
        name: String,
        path: String
    },
}

fn main() {
    let cli = AppArg::parse();
    match cli.commands {
        Subcommands::Init => {
            init();
        },
        Subcommands::Generate { name, path } => {
            generate(name, path).unwrap();
        }
    }
}
