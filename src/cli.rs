use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "0xtools",
    about = "Interactive cybersecurity tool browser for Arch Linux with BlackArch support",
    version,
    after_help = "Run without arguments to start the interactive TUI."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable verbose logging
    #[arg(long, short, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for cybersecurity tools
    Search {
        /// Search query (supports structured syntax like category:osint email)
        query: Vec<String>,
    },

    /// Show detailed information about a tool
    Info {
        /// Package name
        name: String,
    },

    /// List all categories with tool counts
    Categories,

    /// List tools in a specific category
    List {
        /// Category name or slug
        category: String,
    },

    /// Show installed security tools
    Installed,

    /// Show all available tools
    Available,

    /// Manage favorites
    Favorites {
        #[command(subcommand)]
        action: Option<FavoritesAction>,
    },

    /// Refresh the package database cache
    Sync,

    /// Check system health and configuration
    Doctor,

    /// Show version information
    Version,

    /// List available profiles
    Profiles,

    /// Show or install a profile
    Profile {
        /// Profile name
        name: String,

        /// Install the profile packages
        #[arg(long)]
        install: bool,
    },
}

#[derive(Subcommand)]
pub enum FavoritesAction {
    /// List favorite tools
    List,

    /// Add a tool to favorites
    Add {
        /// Package name
        name: String,
    },

    /// Remove a tool from favorites
    Remove {
        /// Package name
        name: String,
    },

    /// Toggle a tool in favorites
    Toggle {
        /// Package name
        name: String,
    },
}
