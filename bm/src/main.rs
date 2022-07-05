#![allow(unused)]

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

use clap::Parser;

#[derive(Parser)]
#[clap(about)]
struct Args {
    /// List all bookmarks (default if no option is selected)
    #[clap(short, long)]
    list: bool,

    /// Delete the bookmark "name" from the bookmarks
    #[clap(short, long, value_name = "name")]
    delete: Option<String>,

    /// Add the current directory as a "name" bookmark
    #[clap(short, long, value_name = "name")]
    add: Option<String>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn list_bookmarks(doc: &yaml_rust::yaml::Yaml)
{
    // Debug support
    println!("List bookmarks called");
    println!("{:?}", doc);

    // Index access for map & array
    //assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    //assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // Chained key/array access is checked and won't panic,
    // return BadValue if they are not exist.
    //assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}

fn delete_bookmark<'a>(name: String) -> &'a str
{
    return "Delete called with parameter";
}

fn add_bookmark<'a>(name: String) -> &'a str
{
    return "Add called";
}

fn main() {
    let args = Args::parse();

    // First we will need to load the YAML. Let's simualte it
    // with a variable for now:
    let bm_yaml =
	"
bm: /home/gthvn1/rust_kindergarten/bm
another: /home/gthvn1/another_dir
";

    let docs = YamlLoader::load_from_str(bm_yaml).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];
    print_type_of(&doc);

    match (args.list, args.delete, args.add) {
	(false, Some(s), None) => println!("<{}>", delete_bookmark(s)),
	(false, None, Some(s)) => println!("<{}>", add_bookmark(s)),
	// By default we list all bookmarks
	_ => list_bookmarks(&doc),
    }



}
