use std::{path::PathBuf, str::FromStr};

use clap::{command, Parser, Subcommand, ValueEnum};

use crate::lib::organizer::os_str_to_string;

use super::organizer::{self, Organizer};

pub(crate) fn execute(organizer: &mut Organizer) -> (){
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Sort { verbose }) => match verbose {
            Some(v) => {
                organizer::sort_downloads(&organizer, *v);
            }
            None => organizer::sort_downloads(&organizer, false),
        },
        Some(Commands::Assign { path }) => {
            let path_buf: PathBuf = PathBuf::from(&path);
            organizer.sorting_path = PathBuf::from(path_buf);
            organizer.save_file()
        }
        Some(Commands::Display { mode }) => match mode {
            Some(m) => match m {
                DisplayMode::Rules => organizer.display_rules(),
                DisplayMode::Directory => organizer.display_directory(),
                DisplayMode::RootDir => println!("{:?}", organizer.sorting_path),
                DisplayMode::ConfigDir => {
                    println!("{:?}", organizer::get_settings_folder().as_path())
                }
            },
            None => organizer.display_rules(),
        },
        Some(Commands::Add { extension, dest }) => {
            let mut dest_path = PathBuf::new();
            dest_path.push(&organizer.sorting_path);
            dest_path.push(dest);
            let filter: String = String::from(extension);
            organizer.rules.insert(filter, os_str_to_string(dest_path.as_os_str()));
            organizer.save_file();
        }
        Some(Commands::Save) => organizer.save_file(),
        None => println!("Usage: download-org.exe help"),
    }
}


#[derive(Parser)]
#[command(version, about,  long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum DisplayMode {
    Rules,
    Directory,
    RootDir,
    ConfigDir,
}

impl ToString for DisplayMode {
    fn to_string(&self) -> String {
        match &self {
            DisplayMode::Rules => String::from_str("Rules").unwrap(),
            DisplayMode::Directory => String::from_str("Directory").unwrap(),
            DisplayMode::RootDir => String::from_str("Root Directory").unwrap(),
            DisplayMode::ConfigDir => String::from_str("Config Directory").unwrap(),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Sorts the files in the sorting directory.
    Sort {
        /// Whether to output verbose messages.
        #[arg(short = 'v',)]
        verbose: Option<bool>,
    },
    /// Assign a new path as the sorting directory.
    Assign {
        #[arg(short = 'a', help = "Assigns a new path to be sorted.")]
        path: String,
    },
    /// Displays the rules, directory children, sorting, or config directories.
    Display {
        #[arg(
            short,
            long,
            help = "Displays the rules or content of the directory to be sorted."
        )]
        mode: Option<DisplayMode>,
    },
    /// Adds a rule to the current list of rules
    Add {
        #[arg(short, help = "The file extension to be filtered.")]
        extension: String,
        #[arg(short, help = "The destination relative to the sorting directory.")]
        dest: String,
    },
    /// Save the current settings to a file.
    Save,
}