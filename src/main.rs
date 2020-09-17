// import clap crate, to use clap API.
// use clap::{ App, Arg };
use clap::{ App, load_yaml };

// import stuff needed to interact with paths and filesystem
use std::{ ffi::OsStr, fs, path::PathBuf};
use walkdir::{DirEntry, WalkDir };

// import Windows specific library if necessary.
#[cfg(target_os = "windows")]
use winapi::um::winnt::*;

#[cfg(target_os = "windows")]
use std::os::windows::prelude::*;

// main function that will be run automatically
fn main() {

    let yaml = load_yaml!("cli.yaml");

    let app = App::from(yaml);

    let matches = app.get_matches();

    /* // set app data and arguments
    let matches = App::new("rext")
    .version("0.1.0")
    .author("Bryce Campbell <tonyhawk2100@gmail.com>")
    .about("tool that allows extensions to be changed easily")
    .arg(Arg::with_name("directory")
        .short('d')
        .long("directory")
        .about("directory to go through")
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
        .long("recursive")
        .about("specify that recursive operation is desired")
    )
    .arg(
        Arg::with_name("hidden")
        .short('H')
        .long("include-hidden")
        .about("include hidden files.")
    )
    .arg(
        Arg::with_name("verbose")
        .short('v')
        .long("verbose")
        .about("enable verbose mode")
    ).get_matches(); */

    // attempt to retrieve the specified directory, otherwise grab the working directory.
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

    // attempt to grab the extension to find and what to replace it with
    let extension = matches.value_of("old_extension").unwrap();
    let new_extension = matches.value_of("new_extension").unwrap();

    // determine whether operation should be performed recursively
    let recursive = if matches.is_present("recursive") {
        true
    } else {
        false
    };

    // determine whether hidden files should be included.
    let hidden = if matches.is_present("hidden") {
        true
    } else {
        false
    };

    // specify whether program should run verbose in verbose or not.
    let verbose = if matches.is_present("verbose") {
        true
    } else {
        false
    };

    // pass arguments to function that will rename files
    rename(directory.to_str().unwrap_or(""), extension, new_extension, recursive, hidden, verbose)
} // end function

// function to rename files in a specified directory, matching the given parameters.
fn rename(dir: &str, old_extension: &str, new_extension: &str, recursive: bool, include_hidden: bool, verbose: bool) {
    
    // create iterator based on whether iterator should be recursive or not.
    let walker = if recursive {
        WalkDir::new(dir).into_iter()
    } else {
        WalkDir::new(dir).max_depth(1).into_iter()
    };

    // walk through directory tree
    for item in walker.filter_entry(|e| {

        // filter out hidden files if flag is not present.
        if include_hidden {
            is_hidden(e) || !is_hidden(e)
        } else {
            !is_hidden(e)
        }
    }) {

        // get DirEntry
        if let Ok(entry) = item {
            // make sure entry is not a directory, before commencing operation.
            if !entry.path().is_dir() {
                // attempt to retrieve extension and file stem
                if let (Some(file_extension), Some(file_name)) = (entry.path().extension().and_then(OsStr::to_str), entry.path().file_stem().and_then(OsStr::to_str)) {

                    // check the file extension matches
                    if file_extension == old_extension {

                        // create file name with new extension
                        let new_file_name = format!("{}.{}", file_name, new_extension);
    
                        // create the new path
                        let new_path = entry.path().with_file_name(new_file_name);
    
                        // check if verbose flag is present and let user know what is happening.
                        if verbose {
                            println!("renaming {} to {}", entry.path().display(), new_path.display());
                        }
    
                        // rename the file
                        fs::rename(entry.path(), new_path).expect("Permissions denied.");
                    }
                } // end if let
            } // end if for directory condition
        } // end main if let
    } // end loop
} // end function

// define is_hidden function for only Unix-like operating systems.
#[cfg(target_os = "unix")]
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
    .to_str()
    .map(|s| s.starts_with("."))
    .unwrap_or(false)
}

// define is_hidden function for only Windows operating systems.
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
