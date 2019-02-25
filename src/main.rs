use sled;
use std::env;
use std::path;
use std::io;
use std::io::Write;

static USAGE: &str = r#"sled-dump - dump a sled database to stdout

Usage:
    sled-dump DATABASE
"#;


fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("expected exactly one arg: database");
        println!("{}", USAGE);
    }
    let mut config = sled::ConfigBuilder::new().path(&args[0]);
    let mut tree = sled::Db::start(config.build()).expect("could not open database");

    let mut iter = tree.iter();
    while let Some(Ok((k, v))) = iter.next() {
        // TODO: make this work for non utf8 stuff: print hex or raw binary?
        println!("{} -> {}", String::from_utf8(k.to_vec()).unwrap(), String::from_utf8(v.to_vec()).unwrap());
    }
}
