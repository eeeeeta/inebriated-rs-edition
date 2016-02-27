extern crate rand;
#[macro_use]
extern crate nom;

pub mod markov;
pub mod io;
mod rgen;
mod parser;
pub use markov::Markov;
