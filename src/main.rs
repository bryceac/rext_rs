use clap::{ App, Arg };
use std::{ ffi::OsStr, fs, path::PathBuf};
use walkdir::{DirEntry, WalkDir };

#[cfg(target_os = "windows")]
use winapi::um::winnt::*;

#[cfg(target_os = "windows")]
use std::os::windows::prelude::*;

fn main() {
    let matches = App::new("rext")
    .version("0.1.0")
    .author("Bryce Campbell <tonyhawk2100@gmail.com>")
    .about("tool that allows extensions to be changed easily")
    .arg(Arg::with_name("directory")
        .short('d')
        .about("directory to use")
        .takes_value(true)
        .default_value(".")
    )
    .arg(
        Arg::with_name("old_extension")
        .about("extension to replace")
        .takes_value(true)
        .required(true)
    )
    .arg(
        Arg::with_name("new_extension")
        .about("the extension to switch to")
        .takes_value(true)
        .required(true)
    )
    .arg(
        Arg::with_name("recursive")
        .short('r')
        .about("specify that recursive operation is desired.")
    )
    .arg(
        Arg::with_name("hidden")
        .short('H')
        .about("include in hidden files.")
    )
    .arg(
        Arg::with_name("verbose")
        .short('v')
        .about("verbose output")
    ).get_matches();

    let directory = if let Some(dir) = matches.value_of("directory") {
        if dir.starts_with("~") {
            // attempt to expand the path
            let input = shellexpand::tilde(dir);

            // convert input to string
            let mut path = String::new();

            // create Path buffer
            path.push_str(&input);

            PathBuf::from(path)
        } else {
            fs::canonicalize(PathBuf::from(dir)).unwrap() 
        }
    } else {
        fs::canonicalize(PathBuf::from(".")).unwrap() 
    };

    let extension = matches.value_of("old_extension").unwrap();
    let new_extension = matches.value_of("new_extension").unwrap();

    let recursive = if matches.is_present("recursive") {
        true
    } else {
        false
    };

    let hidden = if matches.is_present("hidden") {
        true
    } else {
        false
    };

    let verbose = if matches.is_present("verbose") {
        true
    } else {
        false
    };

    rename(directory.to_str().unwrap_or(""), extension, new_extension, recursive, hidden, verbose)
}

fn rename(dir: &str, old_extension: &str, new_extension: &str, recursive: bool, include_hidden: bool, verbose: bool) {
    let walker = if recursive {
        WalkDir::new(dir).into_iter()
    } else {
        WalkDir::new(dir).max_depth(1).into_iter()
    };

    for item in walker.filter_entry(|e| {
        if include_hidden {
            is_hidden(e) || !is_hidden(e)
        } else {
            !is_hidden(e)
        }
    }) {
        if let Ok(entry) = item {
            if !entry.path().is_dir() {
                if let (Some(file_extension), Some(file_name)) = (entry.path().extension().and_then(OsStr::to_str), entry.path().file_stem().and_then(OsStr::to_str)) {
                    if file_extension == old_extension {
                        let new_file_name = format!("{}.{}", file_name, new_extension);
    
                        let new_path = entry.path().with_file_name(new_file_name);
    
                        if verbose {
                            println!("renaming {} to {}", entry.path().display(), new_path.display());
                        }
    
                        fs::rename(entry.path(), new_path).expect("unable to rename file.");
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "unix")]
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
    .to_str()
    .map(|s| s.starts_with("."))
    .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn is_hidden(entry: &DirEntry) -> bool {
    let metadata = fs::metadata(entry.path()).unwrap();
    let attributes = metadata.file_attributes();

    if attributes == FILE_ATTRIBUTE_HIDDEN {
        true
    } else {
        false
    }
}
