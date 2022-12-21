mod colors;
use colors::{rgb_to_256, Color};

use std::env::args;

fn main() {
    let argv = args().collect::<Vec<String>>();
    if argv.len() < 2 {
        println!("Please provide a color");
        std::process::exit(1);
    }
    let hexstr = argv[1].clone();
    let r = i32::from_str_radix(&hexstr[0..=1], 16).expect("Please input a valid R value!");
    let g = i32::from_str_radix(&hexstr[2..=3], 16).expect("Please input a valid G value!");
    let b = i32::from_str_radix(&hexstr[4..], 16).expect("Please input a valid B value!");

    println!("{}", rgb_to_256(Color(r, g, b)));
}
