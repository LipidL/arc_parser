mod modules;
pub mod parser;
mod analyzer;

use std::io;
use crate::modules::structures::StructureBlock;
use crate::parser::arc_parser;
use crate::analyzer::arc_analyzer;


fn main() {
    println!("Hello, world!");
    println!("input your file");
    // let mut path = String::new();
    // io::stdin()
    //     .read_line(&mut path)
    //     .expect("an error occurs in std::io::stdin.readline()");
    // println!("your path is {}",path);
    let path = String::from("/home/ubuntu/learn/rust/tools/arc_stat");
    let structures:Vec<StructureBlock> = match arc_parser::read_file(path){
        Ok(blocks) => blocks,
        Err(error) =>{
            panic!("{}", error);
        }
    };
    let minimum_energy = arc_analyzer::find_minimum_energy(structures);
    match minimum_energy{
        Some(e) => println!("the minimum energy is {}", e),
        None => println!("no minimum energy found!"),
    };
}
