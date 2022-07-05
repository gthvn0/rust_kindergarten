#![allow(unused)]

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

fn main() {
    let args = Args::parse();

    match (args.list, args.delete, args.add) {
	(true, None, None) => println!("List is selected"),
	(false, Some(s), None) => println!("Delete: <{}>", s),
	(false, None, Some(s)) => println!("Add   : <{}>", s),
	_ => println!("By default we list all bookmarks"),
    }
}
