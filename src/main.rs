use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
mod bookmarks;
mod common;
mod folders;
mod notes;

use crate::common::log::debug_log;
use bookmarks::{handle_bookmark_commands, BookmarkArgs};
use folders::{handle_folder_commands, FolderArgs};
use notes::{add_note, delete_note, edit_note, list_notes, view_note};

/// A fast note-taking CLI
#[derive(Debug, Parser)]
#[command(name = "snb", about = "A fast note-taking CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
    verbose: Verbosity,
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
        id: usize,
    },
    /// Edit a note
    Edit {
        /// id of the note
        #[arg(required = true)]
        id: usize,
    },
    /// Manage folders
    Folder(FolderArgs),
    /// Manage bookmarks
    Bookmark(BookmarkArgs),
}

fn main() {
    let args = Cli::parse();

    let verbosity = args.verbose.filter();
    debug_log(&verbosity, &format!("Verbosity set to: {}", verbosity));
    // Initialize the application
    if let Err(e) = common::init::init(verbosity) {
        eprintln!("Failed to initialize: {}", e);
        std::process::exit(1);
    }

    // TODO: all commands to return Result
    // then
    // let result = match args.command
    match args.command {
        Some(Commands::Add {
            title,
            content,
            filename,
        }) => {
            let _ = add_note(content, filename, title, verbosity);
        }
        Some(Commands::View { id }) => {
            view_note(&id);
        }
        Some(Commands::Delete { id }) => {
            let _ = delete_note(&id, verbosity);
        }
        Some(Commands::Edit { id }) => {
            let _ = edit_note(&id, verbosity);
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

    /* TODO: when all commands return Result,
       uncomment this block

      if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
     */
}
