use clap::{Parser, Subcommand};
mod bookmarks;
mod common;
mod folders;
mod notes;

use bookmarks::{handle_bookmark_commands, BookmarkArgs};
use folders::{handle_folder_commands, FolderArgs};
use notes::{add_note, delete_note, edit_note, list_notes, view_note};

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
    /// Edit a note
    Edit {
        /// id of the note
        #[arg(required = true)]
        id: u32,
    },
    /// Manage folders
    Folder(FolderArgs),
    /// Manage bookmarks
    Bookmark(BookmarkArgs),
}

fn main() {
    let args = Cli::parse();

    // Initialize the application
    common::init::init();

    match args.command {
        Some(Commands::Add {
            title,
            content,
            filename,
        }) => {
            add_note(content, filename, title);
        }
        Some(Commands::View { id }) => {
            view_note(&id);
        }
        Some(Commands::Delete { id }) => {
            delete_note(&id);
        }
        Some(Commands::Edit { id }) => {
            edit_note(&id);
        }
        Some(Commands::Folder(args)) => {
            handle_folder_commands(args);
        }
        Some(Commands::Bookmark(args)) => {
            handle_bookmark_commands(args);
        }
        // Use List if no subcommand was provided
        _ => {
            list_notes();
        }
    }
}
