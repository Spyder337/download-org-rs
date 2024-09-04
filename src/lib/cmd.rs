use std::{path::PathBuf, str::FromStr};

use clap::{command, Parser, Subcommand, ValueEnum};

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
            organizer.update_rules(&PathBuf::from(&path_buf));
            organizer.downloads_path = PathBuf::from(path_buf);
            organizer.save_file()
        }
        Some(Commands::Display { mode }) => match mode {
            Some(m) => match m {
                DisplayMode::Rules => organizer.display_rules(),
                DisplayMode::Directory => organizer.display_directory(),
                DisplayMode::RootDir => println!("{:?}", organizer.downloads_path),
                DisplayMode::ConfigDir => {
                    println!("{:?}", organizer::get_settings_folder().as_path())
                }
            },
            None => organizer.display_rules(),
        },
        Some(Commands::Save) => organizer.save_file(),
        None => (),
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
        #[arg(short = 'v', long)]
        verbose: Option<bool>,
    },
    /// Assign a new path as the sorting directory.s
    Assign {
        #[arg(short = 'a', long, help = "Assigns a new path to be sorted.")]
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
    /// Save the current settings to a file.
    Save,
}