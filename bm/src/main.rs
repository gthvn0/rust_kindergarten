#![allow(unused)]

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

use clap::Parser;

#[derive(Parser)]
#[clap(about)]
struct Args {
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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_directory(bkm: &yaml_rust::yaml::Yaml, key: String) -> String
{
    println!("Get directory called with key set to {}", key);
    String::from("/home")
}

fn delete_bookmark(bkm: &yaml_rust::yaml::Yaml, key: String) -> String
{
    println!("Delete called with key set to {}", key);
    String::from(".")
}

fn add_bookmark(bkm: &yaml_rust::yaml::Yaml, key: String) -> String
{
    println!("Add called with key set to {}", key);
    String::from(".")
}

fn list_bookmarks(bkm: &yaml_rust::yaml::Yaml) -> String
{
    // Debug support
    println!("List bookmarks called");
    println!("{:?}", bkm);

    // Index access for map & array
    //assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    //assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // Chained key/array access is checked and won't panic,
    // return BadValue if they are not exist.
    //assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut new_bkm = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut new_bkm);
        emitter.dump(&bkm).unwrap(); // dump the YAML object to a String
    }

    new_bkm
}

fn main() {
    let args = Args::parse();

    // First we will need to load the YAML. Let's simulate it
    // with a variable for now:
    let bm_yaml =
	"
bm: /home/gthvn1/rust_kindergarten/bm
another: /home/gthvn1/another_dir
";

    let docs = YamlLoader::load_from_str(bm_yaml).unwrap();

    // Multi document support:
    //   - bkm is a &yaml_rust::yaml::Yaml
    let bkm = &docs[0];
    print_type_of(&bkm);

    let result = match (args.name, args.list, args.delete, args.add) {
	(Some(key), false, false, false) => get_directory(&bkm, key),
	(Some(key), false, true, false)  => delete_bookmark(&bkm, key),
	(Some(key), false, false, true)  => add_bookmark(&bkm, key),
	// By default we list all bookmarks
	_                                => list_bookmarks(&bkm),
    };

    // It returns the directory to be opened.
    // NOTE: Delete and add returns the current directory.
    println!("{}", result);
}
