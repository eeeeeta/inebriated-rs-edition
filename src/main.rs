extern crate inebriated;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut db = inebriated::Markov::new();
    println!("inebriated, Rust edition (αlpha)\nan eta thing <http://theta.eu.org>\n");
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
    db.calc_weights();
    db.generate_sentence();
}
