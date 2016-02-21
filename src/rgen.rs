extern crate rand;
use markov;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};
pub trait HasWeight {
    fn getwt(&self) -> u32;
}
pub fn pick_from_vec<T>(vec: &Vec<T>) -> Option<&T> where T: HasWeight {
    match vec.len() {
        0 => None,
        1 => Some(&vec[0]),
        _ => {
            let mut items: Vec<Weighted<usize>> = Vec::new();
            for (i, ref k) in vec.iter().enumerate() {
                items.push(Weighted {
                    weight: k.getwt(),
                    item: i
                });
            }
            let wc = WeightedChoice::new(&mut items);
            let mut rng = rand::thread_rng();
            Some(&vec[wc.ind_sample(&mut rng)])
        }
    }
}
pub fn pick_from_str<'a, 'b>(str: &'a str, kvdb: &'b markov::KvdbType) -> Option<&'b markov::Key> {
    println!("str {}", str);
    if let Some(vec) = kvdb.get(str) {
        println!("str {} vec {:?}", str, vec);
        pick_from_vec(vec)
    }
    else {
        None
    }
}
pub fn weight_key(key: &Vec<markov::Key>, kvdb: &markov::KvdbType) -> u32 {
    let mut weight = key.len();
    let mut vec = key;
    while let Some(k) = pick_from_vec(vec) {
        if let Some(nvec) = kvdb.get(&k.str) {
            if nvec[0].str == vec[0].str {
                break;
            }
            weight += nvec.len();
            vec = nvec;
        }
        else {
            break;
        }
    }
    weight as u32
}
