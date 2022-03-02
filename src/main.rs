use clap::Parser;
use poke::Poke;

fn main() {
    if let Err(e) = Poke::parse().run() {
        println!("{e}");
    }
}
