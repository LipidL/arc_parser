mod modules;
pub mod parser;
mod analyzer;

use clap::{Arg,Command};
use crate::modules::structures::StructureBlock;
use crate::parser::arc_parser;
use crate::analyzer::arc_analyzer;


fn main() {
    println!("Hello, world!");
    // println!("input your file");
    // let mut path = String::new();
    // io::stdin()
    //     .read_line(&mut path)
    //     .expect("an error occurs in std::io::stdin.readline()");
    // println!("your path is {}",path);
    let matches = Command::new("arc_stat")
        .version("0.1.0")
        .author("Lipid<23110220057@m.fudan.edu.cn>")
        .about("parses .arc file")
        .arg(Arg::new("file")
                 .short('f')
                 .long("file")
                 .help("target .arc file"))
             .arg(Arg::new("minimum")
                 .short('m')
                 .long("minimum")
                 .action(clap::ArgAction::Count)
                 .help("Sets a minimum flag"))
             .get_matches();
    let default_file = "test.arc".to_string();
    let file: &String = matches.get_one::<String>("file").unwrap_or(&default_file);
    println!("The file passed is: {}", file);

    let minimum_flag = match matches.get_count("minimum"){
        0 => false,
        _ => true,
    };


    let current_path = std::env::current_dir().unwrap();
    println!("The current directory is {}", current_path.display());
    // let path = String::from("best.arc");
    let structures:Vec<StructureBlock> = match arc_parser::read_file(file.to_string()){
        Ok(blocks) => blocks,
        Err(error) =>{
            panic!("{}", error);
        }
    };
    if minimum_flag{
        let minimum_energy = arc_analyzer::find_minimum_energy(structures);
        match minimum_energy{
            Some(e) => println!("the minimum energy is {}", e),
            None => println!("no minimum energy found!"),
        };
    }
}
