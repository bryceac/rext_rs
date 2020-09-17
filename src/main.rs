use clap::{ App, Arg };
use std::{ fs::canonicalize, path::PathBuf};

fn main() {
    let matches = App::new("rext")
    .version("0.1.0")
    .author("Bryce Campbell <tonyhawk2100@gmail.com>")
    .about("tool that allows extensions to be changed easily")
    .arg(Arg::with_name("directory")
        .about("directory to use")
        .takes_value(true)
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

    let directory = if matches.is_present("directory") {
        if let Some(dir) = matches.value_of("directory") {
            if dir.starts_with("~") {
                // attempt to expand the path
                let input = shellexpand::tilde(dir);

                // convert input to string
                let mut path = String::new();

                // create Path buffer
                path.push_str(&input);

                PathBuf::from(path)
            } else if dir.starts_with("..") || dir.starts_with(".") { 
                canonicalize(PathBuf::from(dir)).unwrap();
            } else {
                canonicalize(PathBuf::from(".")).unwrap() 
            }
        } else {
            canonicalize(PathBuf::from(".")).unwrap()
            
        }
    } else {
        canonicalize(PathBuf::from(".")).unwrap()
    };

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

    println!("dir: {}\r\nrecursive: {}\r\nhidden: {}\r\nverbose: {}", directory.display(), recursive, hidden, verbose);
}
