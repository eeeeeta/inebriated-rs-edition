use std::io;
use std::io::{BufReader, BufWriter, Error};
use std::io::prelude::*;
use std::fs::File;
use parser;
use markov;

#[derive(Debug)]
pub struct ParserErrorDesc(usize, parser::ParserError);

pub fn populate_from_mkdb(into: &mut markov::Markov, path: &str) -> Result<(u32, Vec<ParserErrorDesc>), io::Error> {
    let mut lines_parsed = 0;
    let mut errors: Vec<ParserErrorDesc> = Vec::new();
    let f = try!(File::open(path));
    for (no, line) in BufReader::new(f).lines().enumerate() {
        let ln = try!(line);
        match parser::parseln(&ln) {
            Ok((key, vec)) => {
                let k = match key {
                    parser::ParsedKey::SentenceStarter(k) => {
                        into.add_starter(k.to_owned());
                        k
                    },
                    parser::ParsedKey::Key(k) => k
                };
                for v in vec.into_iter() {
                    into.insert(k.to_owned(), String::from(v));
                }
                lines_parsed += 1;
            },
            Err(err) => {
                errors.push(ParserErrorDesc(no, err));
            }
        }
    }
    Ok((lines_parsed, errors))
}

pub fn save_to_mkdb(from: &markov::Markov, path: &str) -> Result<(), io::Error> {
    let mut f = BufWriter::new(try!(File::create(path)));
    for (k, vec) in from.kvdb.iter() {
        assert!(vec.len() > 0);
        let mut s = String::new();
        s.push_str("");
        if from.is_sentence_starter(k) {
            s.push_str("");
        }
        s.push_str(k);
        for k in vec {
            s.push_str("");
            s.push_str(&k.str);
        }
        s.push_str("\n");
        try!(f.write(s.as_bytes()));
    }
    Ok(())
}
