#![feature(plugin)]
#![allow(raw_pointer_derive)]
#![allow(dead_code)]

use std::io;

use db::*;
use db::node::*;
use db::edge::*;

mod db;

fn main() {
    let mut db: Graph = Graph::new();

    let mut stdin = io::stdin();
    let input: &mut String = &mut String::new();

    loop {
        input.clear();
        stdin.read_line(input);
        db.eval(input.clone());
    }
}
