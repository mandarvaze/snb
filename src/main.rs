use clap::{Args, Parser, Subcommand, ValueEnum};

/// A fast note-taking CLI
#[derive(Debug, Parser)]
#[command(name = "snb", about = "A fast note-taking CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new note
    #[command(arg_required_else_help = true)]
    Add {
        /// Content of the note
        #[arg(required = true)]
        content: String,
        /// Title of the note
        #[arg(short, long)]
        title: Option<String>,
        /// Optional File to save the note. Defaults to <timestamp>.md
        filename: Option<String>,
    },
    /// View a note
    View {
        /// id of the note
        #[arg(required = true)]
        id: u32,
    },
    /// Delete a note
    Delete {
        /// id of the note
        #[arg(required = true)]
        id: u32,
    },
    /// List all notes
    // List,
    /// Edit a note
    Edit {
        /// id of the note
        #[arg(required = true)]
        id: u32,
    },
    Folder(FolderArgs),
}

#[derive(Debug, Subcommand)]
enum FolderCommands {
    /// Create a new folder
    Add {
        /// Name of the folder
        #[arg(required = true)]
        name: String,
    },
    /// Delete a folder
    Delete {
        /// ID of the folder
        #[arg(required = true)]
        id: u32,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct FolderArgs {
    #[command(subcommand)]
    command: Option<FolderCommands>,
}
fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Add {
            title,
            content,
            filename,
        }) => {
            println!("Adding Note with Content {}", content);
            if let Some(filename) = filename {
                println!("Saving to {}", filename);
            }
        }
        Some(Commands::View { id }) => {
            println!("Viewing Note {}", id);
        }
        Some(Commands::Delete { id }) => {
            println!("Deleting Note {}", id);
        }
        // Commands::List => todo!(),
        Some(Commands::Edit { id }) => {
            println!("Editing Note {}", id);
        }
        Some(Commands::Folder(_)) => {
            println!("Folder commands");
        }
        // Use List if no subcommand was provided
        _ => {
            println!("Listing Notes");
        }
    }
}
