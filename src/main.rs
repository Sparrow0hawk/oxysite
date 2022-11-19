mod templates;
use clap::{Parser, Subcommand};
use oxysite::{rebuild_site, Site};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build site
    Build {
        /// Directory containing markdown files
        #[arg(short = 'c', long)]
        content: Option<String>,
        /// Directory to write html files
        #[arg(short = 'p', long)]
        public: Option<String>,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { content, public } => {
            let config = Site {
                content_dir: content.unwrap(),
                build_dir: public.unwrap(),
            };

            let _ = rebuild_site(config);
        }
    }

    // rebuild_site(&str, &str)
    // delete public dir
    // get markdown files
    // for md file
    // read file
    // parse content and pass it into read_body func in templates
    // save templated file into output dir
    Ok(())
}
