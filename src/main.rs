use clap::{Args, Parser, Subcommand};
pub use minino::Path;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List all the paths in the registry
    List,
    /// Push a string to the registry
    Push(PushArg),
    /// Remove an element that matches the input - Ex: "Alacritty" will remove "C:\Program Files\Alacritty\"
    Remove(RemoveArg),
}

#[derive(Args, Debug)]
struct PushArg {
    #[clap(value_parser)]
    path: String,
}

#[derive(Args, Debug)]
struct RemoveArg {
    #[clap(value_parser)]
    pattern: String,
}

fn main() {
    use Command::*;
    let cli = Cli::parse();
    let path = Path::new();
    match &cli.command {
        List => {
            let values = path.get_value_as_vec().unwrap();
            for i in values.into_iter() {
                println!("{:?}", i);
            }
        }
        Push(data) => {
            path.push_value(data.path.clone()).unwrap();
        }
        Remove(data) => {
            path.remove_value(data.pattern.clone()).unwrap();
        }
    }
}
