#![allow(unused)]

use clap::Parser;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::env;
use std::io::prelude::*;
use std::fs::File;

#[derive(Parser)]
#[clap(about)]
struct Args {
    /// File that contains the bookmarks
    #[clap(value_parser)]
    file: String,

    /// Command can be "add", "del" or "list"
    cmd: Option<String>,

    /// Name of the bookmark to use to change directory
    name: Option<String>,

    /// List all bookmarks (default if no option is selected)
    #[clap(short, long)]
    list: bool,

    /// Delete the bookmark "name" from the bookmarks
    #[clap(short, long)]
    delete: bool,

    /// Add the current directory as "name" bookmark
    #[clap(short, long)]
    add: bool,
}

// A bookmark is a tag linked to a path.
type Bookmark = BTreeMap<String, String>;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_directory(bkm: &Bookmark, key: String) -> Option<&String>
{
    return bkm.get(&key).clone();
}

fn delete_bookmark(bkm: &mut Bookmark, key: String) -> Option<&String>
{
    bkm.remove(&key);
    return None;
}

fn add_bookmark(bkm: &mut Bookmark, key: String) -> Option<&String>
{
    let path = env::current_dir().unwrap();

    bkm.insert(key, path.display().to_string());
    return None;
}

fn list_bookmarks(bkm: &Bookmark) -> Option<&String>
{
    for (tag, path) in bkm.iter() {
	println!("{:10}  {} {}",
		 tag.yellow(),
		 "-> PATH: ".magenta().bold(), path.magenta());
    }

    // Should return an empty string to avoid unneeded write of bookmarks
    return None;
}

fn write_yaml_file(filename: &str, bkm: Bookmark)
{
    let mut output = File::create(filename).expect(concat!("Failed to open ", stringify!(filename)));
    let serialized = serde_yaml::to_string(&bkm).unwrap();

    output.write_all(serialized.as_bytes());
}

fn read_yaml_file(filename: &str) -> Bookmark
{
    let mut f = File::open(filename).expect(concat!("Uname to open ", stringify!(filename)));

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(concat!("Uname to read ", stringify!(filename)));

    let deserialized: Result<Bookmark, _> = serde_yaml::from_str(&contents);

    return match deserialized {
	Ok(d) => d,
	Err(_) => Bookmark::new(),
    };
}

fn main()
{
    let args = Args::parse();

    // First we will need to load the YAML. It support multi document
    // but we only use the first one.
    let mut bkm: Bookmark = read_yaml_file(&args.file);

    // Extrace the command if any so we can use it in the main match
    let raw_cmd = match args.cmd {
	Some(c) => c,
	_       => "".to_string(),
    };
    
    let new_dir = match (&*raw_cmd, args.name, args.list, args.delete, args.add) {
	("add", Some(tag), _, _, _) => add_bookmark(&mut bkm, tag),
	("del", Some(tag), _, _, _) => delete_bookmark(&mut bkm, tag),
	("list", _ , _, _, _)       => list_bookmarks(&bkm),
	("", _ , _, _, _)           => list_bookmarks(&bkm),
	// All other strings are the tag and not a command. So check classic flags.
	(tag, _, false, false, false) => get_directory(&bkm, tag.to_string()),
	(tag, _, false, true, false)  => delete_bookmark(&mut bkm, tag.to_string()),
	(tag, _, false, false, true ) => add_bookmark(&mut bkm, tag.to_string()),
	// For all other choices list the bookmarks
	_  => list_bookmarks(&bkm),
    };

    match new_dir {
	Some(d) => println!("{}", d),
	_       => write_yaml_file(&args.file, bkm),
    }
}
