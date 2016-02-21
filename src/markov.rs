use std::collections::HashMap;
use rgen;
use rgen::HasWeight;
pub type KvdbType = HashMap<String, Vec<Key>>;
#[derive(Debug)]
pub struct Key {
    pub str: String,
    wt: u32,
    cwt: u32
}
struct SentenceStarter(String, u32);
pub struct Markov {
    ssdb: Vec<SentenceStarter>,
    pub kvdb: KvdbType
}
impl Key {
    fn new(v: String) -> Self {
        Key {
            str: v,
            wt: 1,
            cwt: 0
        }
    }
}

impl HasWeight for SentenceStarter {
    fn getwt(&self) -> u32 {
        self.1
    }
}

impl HasWeight for Key {
    fn getwt(&self) -> u32 {
        self.wt + (self.cwt * 2)
    }
}

impl Markov {
    pub fn new() -> Self {
        Markov {
            ssdb: Vec::new(),
            kvdb: HashMap::new()
        }
    }
    pub fn calc_kv_weights(&mut self) {
        let mut weights: Vec<u32> = Vec::new();
        for (_, ref vec) in self.kvdb.iter() {
            for k in vec.iter() {
                if let Some(nvec) = self.kvdb.get(&k.str) {
                    weights.push(rgen::weight_key(nvec, &self.kvdb));
                }
                else {
                    weights.push(1);
                }
            }
        }
        let mut iter = weights.into_iter();
        for (_, ref mut vec) in self.kvdb.iter_mut() {
            for ref mut k in vec.iter_mut() {
                let wt = iter.next().expect("someone mutated the kvdb whilst we were doing stuff");
                println!("MDB: weight for {} is {}", k.str, wt);
                k.wt = wt;
            }
        }
    }
    pub fn calc_ss_weights(&mut self) {
        for ref mut ss in self.ssdb.iter_mut() {
            if let Some(vec) = self.kvdb.get(&ss.0) {
                let wt = rgen::weight_key(vec, &self.kvdb);
                println!("MDB: weight for ss {} is {}", ss.0, wt);
                ss.1 = wt;
            }
            else {
                println!("bug: sentence starter {} has no key", ss.0);
                ss.1 = 1;
            }
        }
    }
    pub fn calc_weights(&mut self) {
        self.calc_kv_weights();
        self.calc_ss_weights();
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
        let mut ss = &rgen::pick_from_vec(&self.ssdb)
            .expect("failed to choose sentence starter")
            .0;
        let mut str = ss.to_owned();
        while let Some(key) = rgen::pick_from_str(ss, &self.kvdb) {
            ss = &key.str;
            str.push_str(" ");
            str.push_str(&ss);
        }
        println!("generated str: {}", str);
        str
    }
    pub fn add_starter(&mut self, ss: String) {
        println!("mdb: added sentence starter {}", ss);
        self.ssdb.push(SentenceStarter(ss, 1));
    }
}
