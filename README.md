inebriated, Rust edition (βeta)
====================================
*everybody's favourite drunkard!*

![logo](http://i.imgur.com/j3Oemta.png)

wat?
---

The inebriated project line is a series of implementations of the same thing: a drunk AI that takes
input sentences and uses them to form stuff that sort-of makes sense and is sometimes funny.
By implementing this concept in different languages, I learn the nuances of each, and explore how ergonomic
the language is for performing certain tasks.

I wanna play with it!
---------------------

The Rust edition comes with batteries included: a sample test database full of BOFH excuses and proverbs
so you can see a bit of what it can do immediately.

Getting started is as simple as with any Cargo-powered project:

    cargo build --release
    cargo run -- help
    
If you want to play with the example database and generate a few sentences, try:

    cargo run -- -d example_proverbs_and_excuses.mkdb generate
    
and hit Enter a few times to generate some sentences.

port status
-----------

I haven't spent that much time on this yet, but I *freakin' love this language*. Thanks to Rust's
safety guarantees, the memory problems that plagued the C edition are no more! I love the "be explicit"
philosophy of the language - for example, there are no "copy constructors" or "move constructors" - it's
clear when something is going to be copied or moved, and why simply by how the code is written.

Things that work
----------------

- Sentence input, tokenisation & storage
- Weighting of keys, plus awesome weighted RNG implementation courtesy of the `rand` crate
- Sentence generation (I love the `while let` statement so much)
- Saving & loading standard inebriated database files (`.mkdb` format)

Things to be done
-----------------

- Most of the gripes outlined in the "Things to be done" of the C++ edition

Links to the other inebriateds:

- [the inebriated project spotlight](http://theta.eu.org/inebriated-family.html)
- [inebriated: the original edition](https://github.com/eeeeeta/inebriated-genesis)
- [inebriated, C edition](https://github.com/eeeeeta/inebriated-c-edition)
- [inebriated, C++ edition](https://github.com/eeeeeta/inebriated-cpp-edition)
