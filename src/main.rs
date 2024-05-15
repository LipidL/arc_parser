mod modules;
pub mod parser;
mod analyzer;

use clap::{value_parser, Arg, ArgAction, Command};
use crate::modules::structures::StructureBlock;
use crate::parser::arc_parser;
use crate::analyzer::arc_analyzer::{self, check_atom_consistency, list_energy};
use colored::*;


fn main() {
    println!("Hello, this is arc_parser!");
    //set up command line arguments
    let matches = Command::new("arc_stat")
        .version("0.2.0")
        .author("Lipid<23110220057@m.fudan.edu.cn>")
        .about("parses .arc file")
            .arg(Arg::new("file")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .short('f')
                .long("file")
                .help("target .arc file"))
            .arg(Arg::new("check")
                .long("check")
                .action(clap::ArgAction::SetTrue)
                .help("set this to check if the computation result is valid. If the result is invalid, unconverged structures will be written to unconverged.arc"))
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
                .short('C')
                .action(clap::ArgAction::SetTrue)
                .help("check if all the structures are composed by identical atoms"))
            .arg(Arg::new("energy_list")
                .short('l')
                .long("list")
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
            .arg(Arg::new("scale_crystal")
                .long("scale")
                .short('s')
                .action(ArgAction::Append)
                .value_parser(value_parser!(f64))
                .required(false)
                .help("scale the minimum structure's crystal by given scale"))
            .arg(Arg::new("coordination_number")
                .long("coordination")
                .action(clap::ArgAction::SetTrue)
                .help("calculates coordination number of the atoms in the first strucutre"))       
            .arg(Arg::new("calculate_surface")
                .long("surface")
                .action(ArgAction::Append)
                .value_parser(value_parser!(usize))
                .required(false)
                .help("calculate the surface distance of given 3 atoms"))
            .get_matches();
    // determine the file path: default is test.arc, can be specified by -f myfile.arc
    let default_file = "test.arc".to_string();
    let file = matches.get_one("file").unwrap_or(&default_file);
    let check_flag = matches.get_flag("check");
    println!("The file passed is: {}", file);
    //set flags from command line arguments
    let minimum_flag = matches.get_flag("minimum");
    let count_flag = matches.get_flag("count");
    let consistency_flag = matches.get_flag("consistency");
    let energy_list_flag = matches.get_flag("energy_list");
    let extract_minimum_flag = matches.get_flag("extract_minimum");
    let rearrange_target = matches.get_one::<String>("rearrange_atoms");
    let scale: Option<Vec<f64>> = matches.get_many("scale_crystal")
                                    .map(|v| v.copied().collect());
    let surface: Option<Vec<usize>> = matches.get_many("calculate_surface")
                                    .map(|v| v.copied().collect());
    let coordination_flag = matches.get_flag("coordination_number");
    let current_path = std::env::current_dir().unwrap();
    println!("The current directory is {}", current_path.display());
    //check if the result is reliable
    if check_flag{
        let structures = match arc_parser::read_file("Badstr.arc".to_owned()) {
            Ok(blocks) => blocks,
            Err(error) =>{
                panic!("{}", error);
            }
        };
        let unconverged_index = arc_parser::find_unconverged_strucutres().unwrap();
        if structures.len() >= 3 || unconverged_index.len() >= 3{
            println!("{}","this result might be unreliable!".red());
            println!("structure in Badstr.arc: {}",structures.len());
            println!("unconverged iterations in lasp.out: {}", unconverged_index.len());
            println!("finding unconverged strucutres");
            let all_strucutres = match arc_parser::read_file("all.arc".to_owned()){
                Ok(blocks) => blocks,
                Err(error) => {
                    panic!("{}", error);
                }
            };
            let mut unconverged_structure: Vec<StructureBlock> = Vec::new();
            for i in unconverged_index{
                for structure in &all_strucutres{
                    if structure.number == i {
                        unconverged_structure.push(structure.clone());
                    }
                }
            }
            arc_parser::write_to_file(unconverged_structure, String::from("unconverged.arc")).unwrap();
            println!("the unconverged structures have been written to unconverged.arc")
        }
        else {
            println!("{}","this result might be reliable!".green());
        }
        return;
    }
    //read the arc file
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
            Some(hashmap) => {
                println!("this file's block have {} atoms!","consistent".green());
                println!("these structures contain following atoms:");
                for (atom, count) in hashmap{
                    println!("{}, {}",atom, count);
                } 
            },
            None => println!("this file's block have {} atoms!","non-consistent".red()),
        }
    }
    //list different energy
    if energy_list_flag{
        let mut energy_list = list_energy(&structures);
        energy_list.sort_by(|a, b| b.energy.partial_cmp(&a.energy).unwrap());
        for energy_info in energy_list{
            println!("energy: {}, present for {} time(s)", energy_info.energy, energy_info.count);
        }
    }
    //extract minimum energy
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
    //rearrange atoms of minimum structure by given target
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
    //expand the minimum structure by given scale
    if let Some(scale) = scale{
        let minimum_block = arc_analyzer::extract_minimum(&structures);
        if let Some(minimum_block) = minimum_block{
            let mut new_block = minimum_block.clone();
            if scale.len() == 1{
                new_block = new_block.expand_crystal(scale[0]);
            }
            else if scale.len() == 3{
                new_block = new_block.scale_crystal(modules::structures::CoordinateChoice::X, scale[0]);
                new_block = new_block.scale_crystal(modules::structures::CoordinateChoice::Y, scale[1]);
                new_block = new_block.scale_crystal(modules::structures::CoordinateChoice::Z, scale[2]);
            }
            new_block.write_to_file(String::from("scaled.arc")).unwrap();
            println!("minimum structure has been scaled to scaled.arc.")
        }
    }
    // calculate coordination number
    if coordination_flag{
        let minimum_block = arc_analyzer::extract_minimum(&structures).unwrap();
        let coord_vec = arc_analyzer::calc_coordination(&minimum_block);
        for i in 0..coord_vec.len(){
            println!("{}, {}",minimum_block.atoms[i].element, coord_vec[i]);
        }
    }

    if let Some(atoms) = surface{
        let minimum = arc_analyzer::extract_minimum(&structures).unwrap();
        let spacing = arc_analyzer::calculate_interplanar_spacing(&minimum.atoms, atoms[0], atoms[1], atoms[2]).unwrap();
        println!("distances are {:?}",spacing);
    }

}
