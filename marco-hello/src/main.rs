use ::clap::Parser;
use marco_hello::marco_polo;

#[derive(Parser)]
#[clap(version = "1.0", author = "Saksham")]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = marco_hello::marco_polo(&name);
            println!("{}", result);
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
