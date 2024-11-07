use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    // Only there, because it was not possible to create const or static hashmap globally or locally
    let POLYHEDRONS: HashMap<&str, u8> = HashMap::from([
        ("Tetrahedron", 4),
        ("Cube", 6),
        ("Octahedron", 8),
        ("Dodecahedron", 12),
        ("Icosahedron", 20),
    ]);

    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let n: u32 = read_line().trim().parse().unwrap();
    let mut polyhedrons: u32 = 0;

    for _ in 0..n {
        polyhedrons += POLYHEDRONS[read_line().trim()] as u32;
    }

    println!("{}", polyhedrons);
}