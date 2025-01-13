use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum FolderCommands {
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
pub struct FolderArgs {
    #[command(subcommand)]
    command: Option<FolderCommands>,
}

pub fn handle_folder_commands(args: FolderArgs) {
    match args.command {
        Some(FolderCommands::Add { name }) => {
            println!("Creating folder with name {}", name);
        }
        Some(FolderCommands::Delete { id }) => {
            println!("Deleting folder with id {}", id);
        }
        None => {
            println!("Listing folders");
        }
    }
}
