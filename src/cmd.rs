use std::{path::PathBuf, str::FromStr};

use clap::{command, Parser, Subcommand, ValueEnum};

use super::organizer::{self, os_str_to_string, Organizer};

/// Runs clap with an [`Organizer`] as a parameter that contains the settings.
pub(crate) fn execute(organizer: &mut Organizer) {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Sort { verbose }) => match verbose {
            Some(v) => {
                organizer::run_sort(organizer, *v);
            }
            None => organizer::run_sort(organizer, false),
        },
        Some(Commands::Assign { path }) => {
            let path_buf: PathBuf = PathBuf::from(&path);
            organizer.sorting_path = path_buf;
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
            organizer
                .rules
                .insert(filter, os_str_to_string(dest_path.as_os_str()));
            organizer.save_file();
        }
        Some(Commands::Clean { mode, in_str }) => {
            let removed_cnt: usize = organizer.run_clean(mode, in_str);
            println!("Deleted {} items.", removed_cnt);
        }
        None => println!("Usage: download-org.exe help"),
    }
}

/// Container for the main command line interface.
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
            DisplayMode::Rules => String::from_str("rules").unwrap(),
            DisplayMode::Directory => String::from_str("directory").unwrap(),
            DisplayMode::RootDir => String::from_str("root-dir").unwrap(),
            DisplayMode::ConfigDir => String::from_str("config-dir").unwrap(),
        }
    }
}

/// Reprsents different modes of the cleaning method.
#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CleaningMode {
    /// Represents removing files by extension.
    Extension,
    /// Represents removing files in a sub-directory.
    Directory,
    /// Represents removing all files in the sort directory..
    RootDir,
}

impl ToString for CleaningMode {
    fn to_string(&self) -> String {
        match &self {
            CleaningMode::Extension => String::from_str("extension").unwrap(),
            CleaningMode::Directory => String::from_str("directory").unwrap(),
            CleaningMode::RootDir => String::from_str("root-dir").unwrap(),
        }
    }
}

//  Implement a cleaning command that can remove all items of a type, a directory, or
//  the entire root directory.

/// Commands that can be executed via Cli::parse().
#[derive(Subcommand)]
enum Commands {
    /// Sorts the files in the sorting directory.
    Sort {
        /// Whether to output verbose messages.
        #[arg(short = 'v')]
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
    /// Deletes items in the directory tree.
    Clean {
        #[arg(short, help = "Determines which files are deleted.", 
            default_value_t = CleaningMode::Extension)]
        mode: CleaningMode,
        #[arg(short,
            default_value_t=String::from_str("exe").unwrap())]
        /// Required input for [`CleaningMode::Extension`] and [`CleaningMode::Directory`].
        in_str: String,
    },
}
