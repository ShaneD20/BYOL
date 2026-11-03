
// https://www.buildyourownlisp.com/contents

use std::io::{self, Write};

struct Vector {
    x: i32, y: i32
}

fn proclaim() -> () {
    println!("\tBYOL (lisp) Version 0.0.1\n\t** Ctrl+c to exit");
}

fn main() {
    // let vector = Vector {  x: 77, y: 23 };
    // println!("\tx: {}, y: {}", vector.x, vector.y);
    let mut blob = String::new();

    proclaim();

    loop {  // REPL
        // note two unused results, following along
        io::stdout().flush(); // guarantee prompt prints before wait for input
        io::stdin().read_line(&mut blob);
        let reply  = blob.trim();
        println!("\tNo, you're a {}", reply);
    }
}
