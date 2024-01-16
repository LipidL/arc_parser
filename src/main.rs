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
    //set up command line arguments
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
                .action(clap::ArgAction::SetTrue)
                .help("find the minimum energy in the file"))
            .arg(Arg::new("count")
                .short('c')
                .long("count")
                .action(clap::ArgAction::SetTrue)
                .help("count the number of strucutres in the file"))
            .arg(Arg::new("consistency")
                .long("consistency")
                .action(clap::ArgAction::SetTrue)
                .help("check if all the structures are composed by identical atoms"))
            .arg(Arg::new("energy_list")
                .short('e')
                .long("energy-list")
                .action(clap::ArgAction::SetTrue)
                .help("list all energies in the file, energy difference less than 0.001 are seen as the same"))
            .arg(Arg::new("extract_minimum")
                .long("extract")
                .action(clap::ArgAction::SetTrue)
                .help("extract the minimum structure to minimum.arc"))
            .arg(Arg::new("rearrange_atoms")
                .short('r')
                .long("rearrange")
                .help("rearrange by atom's coordination, write to rearranged.arc"))
            .get_matches();
    // determine the file path: default is test.arc, can be specified by -f myfile.arc
    let default_file = "test.arc".to_string();
    let file: &String = matches.get_one::<String>("file").unwrap_or(&default_file);
    println!("The file passed is: {}", file);
    //set flags from command line arguments
    let minimum_flag = matches.get_flag("minimum");
    let count_flag = matches.get_flag("count");
    let consistency_flag = matches.get_flag("consistency");
    let energy_list_flag = matches.get_flag("energy_list");
    let extract_minimum_flag = matches.get_flag("extract_minimum");
    let rearrange_target = matches.get_one::<String>("rearrange_atoms");
    //read the arc file
    let current_path = std::env::current_dir().unwrap();
    println!("The current directory is {}", current_path.display());
    let structures:Vec<StructureBlock> = match arc_parser::read_file(file.to_string()){
        Ok(blocks) => blocks,
        Err(error) =>{
            panic!("{}", error);
        }
    };
    //extract the minimum energy
    if minimum_flag{
        let minimum_energy = arc_analyzer::find_minimum_energy(&structures);
        match minimum_energy{
            Some(e) => println!("the minimum energy is {}", e),
            None => println!("no minimum energy found!"),
        };
    }
    //count the number of blocks
    if count_flag{
        let structure_cout = arc_analyzer::count_strucutre_block(&structures);
        println!("there are {} strucutres", structure_cout);
    }
    //check the consistency
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

    if let Some(target) = rearrange_target{
        let minimum_block = arc_analyzer::extract_minimum(&structures);
        if let Some(mut real_minimum) = minimum_block{
            let coordination = match target.to_uppercase().as_str(){
                "X" => {
                    arc_analyzer::rearrange_atoms(&mut real_minimum, |a, b| a.coordinate.0.partial_cmp(&b.coordinate.0).unwrap());
                    Some("X")
                },
                "Y" => {
                    arc_analyzer::rearrange_atoms(&mut real_minimum, |a, b| a.coordinate.1.partial_cmp(&b.coordinate.1).unwrap());
                    Some("Y")
                },
                "Z" => {
                    arc_analyzer::rearrange_atoms(&mut real_minimum, |a, b| a.coordinate.2.partial_cmp(&b.coordinate.2).unwrap());
                    Some("Z")
                },
                _ => {
                    println!("Please verify the sorting coordination: X, Y or Z.");
                    None
                }
            };
            real_minimum.write_to_file(String::from("rearranged.arc")).unwrap();
            match coordination{
                Some(coordination) => println!("the rearranged minimum structure (by {} value) has been generated.", coordination),
                None => println!("Please specify the coordination to be sorted!\n rearranged.arc reamains unchanged.")
            }
            
        }

    }

}
