use std::collections::HashMap;
extern crate rand;
use rand::Rng;
struct Key {
    str: String,
    wt: i32,
    cwt: i32
}
pub struct Markov {
    ssdb: Vec<String>,
    kvdb: HashMap<String, Vec<Key>>
}
impl Key {
    fn new(v: String) -> Self {
        Key {
            str: v,
            wt: 0,
            cwt: 0
        }
    }
}

impl Markov {
    pub fn new() -> Self {
        Markov {
            ssdb: Vec::new(),
            kvdb: HashMap::new()
        }
    }
    pub fn insert(&mut self, k: String, v: String) {
        let new_key = Key::new(v);
        if let Some(vec) = self.kvdb.get_mut(&k) {
            for key in vec.iter_mut() {
                if key.str == k {
                    println!("mdb: incremented cwt of {} -> {}", &k, &key.str);
                    key.cwt = key.cwt + 1;
                    return;
                }
            }
            println!("mdb: added new key to vec for {} -> {}", &k, &new_key.str);
            vec.push(new_key);
            return;
        }
        println!("mdb: added new kv pair {} -> vec[{}]", &k, &new_key.str);
        self.kvdb.insert(k, vec![new_key]);
    }
    pub fn generate_sentence(&self) -> String {
        if self.ssdb.len() < 1 {
            panic!("ssdb must have sentences in it");
        }
        let mut rng = rand::thread_rng();
        let mut ss = rng.choose(&self.ssdb).unwrap();
        let mut str = ss.to_owned();
        while let Some(vec) = self.kvdb.get(ss) {
            ss = &rng.choose(&vec).unwrap().str;
            str.push_str(" ");
            str.push_str(ss);
        }
        println!("generated str: {}", str);
        str
    }
    pub fn add_starter(&mut self, ss: String) {
        println!("mdb: added sentence starter {}", ss);
        self.ssdb.push(ss);
    }
}
