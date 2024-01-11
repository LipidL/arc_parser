mod modules;
pub mod parser;
mod analyzer;

use clap::{Arg,Command};
use crate::modules::structures::StructureBlock;
use crate::parser::arc_parser;
use crate::analyzer::arc_analyzer::{self, check_atom_consistency, list_energy};
use colored::*;


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
            .arg(Arg::new("count")
                .short('c')
                .long("count")
                .action(clap::ArgAction::Count)
                .help("Sets a count flag"))
            .arg(Arg::new("consistency")
                .long("consistency")
                .action(clap::ArgAction::Count)
                .help("Sets a consistency flag"))
            .arg(Arg::new("energy_list")
                .short('e')
                .long("energy-list")
                .action(clap::ArgAction::Count)
                .help("Sets a count flag"))
            .arg(Arg::new("extract_minimum")
                .long("extract")
                .action(clap::ArgAction::Count)
                .help("extract the minimum"))
            .arg(Arg::new("rearrange_atoms")
                .short('r')
                .long("rearrange")
                .action(clap::ArgAction::SetTrue)
                .help("extract the minimum"))
            .get_matches();
    let default_file = "test.arc".to_string();
    let file: &String = matches.get_one::<String>("file").unwrap_or(&default_file);
    println!("The file passed is: {}", file);

    let minimum_flag = match matches.get_count("minimum"){
        0 => false,
        _ => true,
    };
    let count_flag = match matches.get_count("count") {
        0 => false,
        _ => true,
    };
    let consistency_flag = match matches.get_count("consistency") {
        0 => false,
        _ => true,
    };
    let energy_list_flag = match matches.get_count("energy_list") {
        0 => false,
        _ => true,
    };
    let extract_minimum_flag = match matches.get_count("extract_minimum") {
        0 => false,
        _ => true,
    };
    let rearrange_flag = matches.get_flag("rearrange_atoms");

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
        let minimum_energy = arc_analyzer::find_minimum_energy(&structures);
        match minimum_energy{
            Some(e) => println!("the minimum energy is {}", e),
            None => println!("no minimum energy found!"),
        };
    }
    if count_flag{
        let structure_cout = arc_analyzer::count_strucutre_block(&structures);
        println!("there are {} strucutres", structure_cout);
    }
    if consistency_flag{
        match check_atom_consistency(&structures){
            true => println!("this file's block have {} atoms!","consistent".green()),
            false => println!("this file's block have {} atoms!","non-consistent".red()),
        }
    }
    if energy_list_flag{
        let mut energy_list = list_energy(&structures);
        energy_list.sort_by(|a, b| b.energy.partial_cmp(&a.energy).unwrap());
        for energy_info in energy_list{
            println!("energy: {}, present for {} time(s)", energy_info.energy, energy_info.count);
        }
    }
    if extract_minimum_flag{
        let minimum_block = arc_analyzer::extract_minimum(&structures);
        match minimum_block{
            None =>{
                println!("cannot find the minimum in this file!")
            }
            Some(block) => {
                block.write_to_file(String::from("minimum.arc")).unwrap();
                println!("the minimum strucutre has been written to minimum.arc")
            }
        }
    }
    if rearrange_flag{
        let minimum_block = arc_analyzer::extract_minimum(&structures);
        match minimum_block{
            None => {
                println!("cannot find the minimum in this file!")
            }
            Some(mut block) => {
                arc_analyzer::rearrange_atoms(&mut block, |a, b| a.coordinate.0.partial_cmp(&b.coordinate.0).unwrap());
                block.write_to_file(String::from("rearranged.arc")).unwrap();
                println!("the rearranged minimum structure (by x value) has been generated.\n")
            }
        }
    }
}
