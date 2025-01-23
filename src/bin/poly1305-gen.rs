use std::fs;

use TD2::{bernstein::poly1305, convert::extract_rs};

fn main() {
    let mut args = std::env::args();
    assert_eq!(args.len(), 3);
    let (r, s) = extract_rs(
        &args
            .nth(1)
            .expect("First argument should be the hexadecimal key"),
    );
    let m = fs::read(
        args.nth(2)
            .expect("Second argument should be the path to the file to be authenticated"),
    )
    .expect("Could not read file");

    eprintln!("R = {r:032x}");
    eprintln!("S = {s:032x}");
    eprintln!("Tried to open = {}", args.nth(2).unwrap());

    println!("{:032X}", poly1305(&m, r, s));
}
