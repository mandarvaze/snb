use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum BookmarkCommands {
    /// Create a new bookmark
    Add {
        /// URL of the bookmark
        #[arg(required = true)]
        url: String,
        /// Optional Name of the bookmark
        name: Option<String>,
    },
    /// Delete a bookmark
    Delete {
        /// ID of the bookmark
        #[arg(required = true)]
        id: u32,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct BookmarkArgs {
    #[command(subcommand)]
    command: Option<BookmarkCommands>,
}

pub fn handle_bookmark_commands(args: BookmarkArgs) {
    match args.command {
        Some(BookmarkCommands::Add { url, name }) => {
            println!("Creating bookmark: -> {} Named : {}", url, name.unwrap());
        }
        Some(BookmarkCommands::Delete { id }) => {
            println!("Deleting bookmark with id {}", id);
        }
        None => {
            println!("Listing bookmarks");
        }
    }
}
