extern crate inebriated;
extern crate clap;
use std::io;
use std::io::prelude::*;
use clap::{Arg, App, SubCommand, AppSettings};

fn main() {
    let matches = App::new("inebriated, Rust edition")
        .version("0.1.0")
        .author("eta <http://theta.eu.org")
        .about("'everybody's favourite drunkard!'")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(Arg::with_name("dbfile")
             .short("d")
             .long("dbfile")
             .takes_value(true)
             .help("Sets a custom database file (./inerust.mkdb by default)"))
        .subcommand(SubCommand::with_name("insert")
                    .about("Insert sentences into the database"))
        .subcommand(SubCommand::with_name("generate")
                    .about("Interactively generate sentences"))
        .subcommand(SubCommand::with_name("genonce")
                    .about("Generate just one sentence"))
        .get_matches();
    let mut db = inebriated::Markov::new();
    println!("inebriated, Rust edition (βeta)\nan eta thing <http://theta.eu.org>\n");
    let dbfile = matches.value_of("dbfile").unwrap_or("./inerust.mkdb");
    println!("will load and save to {}", dbfile);
    println!("Loading database results in: {:?}", inebriated::io::populate_from_mkdb(&mut db, dbfile));

    match matches.subcommand_name() {
        Some("insert") => {
            println!("input sentences, newline-separated (Ctrl-D to stop)");
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let mut key = Some("".to_string());
                let mut val = None::<String>;
                let mut spaces = 0;
                for hunk in line.unwrap().split(" ") {
                    spaces += 1;
                    let closure = |k: &mut String| {
                        if k != "" {
                            k.push_str(" ");
                        }
                        k.push_str(hunk);
                    };
                    if val.is_some() {
                        val.as_mut().map(closure);
                    }
                    else {
                        key.as_mut().map(closure);
                    }
                    if spaces == 2 {
                        if val.is_none() {
                            db.add_starter(key.as_ref().expect("ssf").to_owned());
                        }
                        else {
                            db.insert(key.take().expect("kf"), val.as_ref().expect("vrf").to_owned());
                            key = Some(val.take().expect("vtf"));
                        }
                        val = Some("".to_string());
                        spaces = 0;
                    }
                }
                if val.is_some() && key.is_some() && val.as_ref().unwrap() != "" {
                    db.insert(key.take().unwrap(), val.take().unwrap());
                }
            };
            println!("Saving...");
            println!("Done. Saving database results in: {:?}", inebriated::io::save_to_mkdb(&db, dbfile));
        },
        Some("generate") => {
            db.calc_weights();
            println!("generating sentences (Enter for more, Ctrl-D to stop)");
            db.generate_sentence();
            let stdin = io::stdin();
            for _ in stdin.lock().lines() {
                db.generate_sentence();
            }
        }
        Some("genonce") => {
            db.calc_weights();
            db.generate_sentence();
        }
        _ => {
            panic!("invalid subcommand provided");
        }
    }

}
