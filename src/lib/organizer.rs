use glob::glob;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs::{self},
    path::{Path, PathBuf},
    str::{FromStr},
};

use super::cmd::CleaningMode;

const FILE_NAME: &str = "dorg_config.json";

/// Represents the config for sorting files.
#[derive(Serialize, Deserialize)]
pub(crate) struct Organizer {
    /// A rule is a (File Extension, Relative Path) Key Value pair.
    pub rules: BTreeMap<String, String>,
    /// The destination folder that will be sorted and
    /// combined with a Value from the [`Organizer::rules`].
    pub sorting_path: PathBuf,
}

impl Default for Organizer {
    fn default() -> Self {
        Self {
            rules: Organizer::get_default_rules(),
            sorting_path: get_downloads_folder(),
        }
    }
}

impl Organizer {
    /// Creates a new [`Organizer`].
    pub fn new(rules: BTreeMap<String, String>) -> Self {
        Self {
            rules,
            sorting_path: get_downloads_folder(),
        }
    }

    pub fn save_file(&self) {
        let path = get_settings_folder();
        //  Handle path parsing and errors
        if !Path::exists(&path) {
            let res = fs::create_dir_all(&path);
            if res.is_err() {
                panic!("{:?}", res.unwrap())
            }
        }
        let mut full_path = path;
        full_path.push(FILE_NAME);

        let deserialized = serde_json::to_string_pretty(&self);

        //  Verify successful deserialization.
        match deserialized {
            Ok(obj) => {
                let res = fs::write(&full_path, obj);
                match res {
                    Ok(_) => println!("Successfully saved settings to {:?}", full_path),
                    Err(e) => panic!("{:?}", e),
                }
            }
            //  Deserialization error.
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn from_file(path: &PathBuf) -> Self {
        if !fs::exists(path).unwrap() {
            panic!("File was not found.")
        }
        let read_file_res = fs::read_to_string(path);
        match read_file_res {
            Ok(data) => {
                let org: Organizer = serde_json::from_str(data.as_str()).unwrap();
                org
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn get_default_rules() -> BTreeMap<String, String> {
        let mut rules: BTreeMap<String, String> = BTreeMap::new();
        let mut exe_path = PathBuf::new();
        exe_path.push("Executables");
        let mut meta_path = PathBuf::new();
        meta_path.push("MetaInfo");
        rules.insert("exe".to_string(), os_str_to_string(exe_path.as_os_str()));
        rules.insert(
            "torrent".to_string(),
            os_str_to_string(meta_path.as_os_str()),
        );
        rules
    }

    pub fn display_rules(&self) {
        for r in &self.rules {
            println!("{} : {}", r.0, r.1)
        }
    }

    pub fn display_directory(&self) {
        let items = self.get_downloaded_items();
        let dirs = self.get_downloads_sub_dirs();
        println!("Directories: ");
        for i in dirs {
            println!("{}", i);
        }
        println!("Files: ");
        for i in items {
            println!("{}", i);
        }
    }

    pub(crate) fn get_downloaded_items(&self) -> Vec<String> {
        let sort_dir = PathBuf::from_str(self.sorting_path.to_str().unwrap()).unwrap();
        let pattern = format!("{}/*.*", os_str_to_string(sort_dir.as_os_str()));
        let mut items: Vec<String> = Vec::new();
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => items.push(os_str_to_string(path.as_os_str())),
                Err(e) => println!("{:?}", e),
            }
        }
        items
    }

    fn get_downloads_sub_dirs(&self) -> Vec<String> {
        let down_dir = PathBuf::from(&self.sorting_path);
        let pattern = format!("{}/**/", os_str_to_string(down_dir.as_os_str()));
        let mut dirs: Vec<String> = Vec::new();
        for entry in glob(&pattern).expect("Failed to read glob pattern.") {
            match entry {
                Ok(path) => dirs.push(os_str_to_string(path.as_os_str())),
                Err(e) => println!("{:?}", e),
            }
        }
        dirs
    }

    pub fn run_clean(&self, mode: &CleaningMode, in_str: &String) -> usize {
        let remove_cnt: usize;
        
        match &mode {
            CleaningMode::Extension => {
                let base_path = PathBuf::from(&self.sorting_path);
                let pattern = format!("{}/**/*.{}", os_str_to_string(base_path.as_os_str()), in_str);
                remove_cnt = Self::glob_remove(pattern);
            },
            CleaningMode::Directory => {
                let mut base_path = PathBuf::from(&self.sorting_path);
                base_path.push(in_str);
                let pattern = format!("{}/**/*.*", os_str_to_string(base_path.as_os_str()));
                remove_cnt = Self::glob_remove(pattern);
            },
            CleaningMode::RootDir => {
                let base_path = PathBuf::from(&self.sorting_path);
                let pattern = format!("{}/**/*.*", os_str_to_string(base_path.as_os_str()));
                remove_cnt = Self::glob_remove(pattern);
            },
        }
    
        remove_cnt
    }

    pub(self) fn glob_remove(pattern: String) -> usize {
        let mut remove_cnt: usize = 0;
        for entry in glob(&pattern).expect("") {
            match entry {
                Ok(path) => {
                    remove_cnt += 1;
                    fs::remove_file(path).unwrap();
                },
                Err(e) => println!("Could not read the path. Error\n{:?}", e),
            }
        }
        remove_cnt
    }
}

pub(crate) fn sort_downloads(organizer: &Organizer, verbose: bool) {
    let items: Vec<String> = organizer.get_downloaded_items();
    sort_items(&items, organizer, verbose);
}

fn sort_items(paths: &[String], organizer: &Organizer, verbose: bool) {
    for item in paths {
        let path = PathBuf::from_str(item.as_str()).unwrap();
        let ext = os_str_to_string(path.extension().unwrap());
        let file_name = os_str_to_string(path.file_name().unwrap());
        //  If there is a folder destination for the extension copy the file
        //  to that destination.
        if organizer.rules.contains_key(&ext) {
            if verbose {
                println!("Moving : \"{}\"", os_str_to_string(path.as_os_str()));
            }
            let mut new_path = PathBuf::new();
            //  Prepend the destination path root.
            new_path.push(&organizer.sorting_path);
            //  Append the relative folder path to the new path.
            new_path.push(&organizer.rules[&ext]);
            //  Contains the new full path including file name.
            let mut new_file_path = PathBuf::new();
            new_file_path.push(&new_path);
            new_file_path.push(file_name);

            if verbose {
                println!(
                    "Destinattion: \"{}\"",
                    os_str_to_string(new_path.as_os_str())
                )
            }

            if !fs::exists(&new_path).unwrap() {
                let _ = fs::create_dir(&new_path);
            }
            let _ = fs::rename(path.as_os_str(), new_file_path.as_os_str());
        }
    }
}

pub(crate) fn os_str_to_string(s: &OsStr) -> String {
    return s.to_str().unwrap().to_string();
}

pub(crate) fn get_downloads_folder() -> PathBuf {
    dirs_next::download_dir().unwrap()
}

pub(crate) fn get_settings_folder() -> PathBuf {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("Downloads Organizer");
    path
}

pub(crate) fn get_settings_file() -> PathBuf {
    let mut path = get_settings_folder();
    path.push(FILE_NAME);
    path
}

pub(crate) fn load_organizer() -> Organizer {
    let organizer: Organizer;
    if !Path::exists(&get_settings_file()) {
        organizer = Organizer::default();
    } else {
        organizer = Organizer::from_file(&get_settings_file());
    }
    organizer
}

