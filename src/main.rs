mod modules;
pub mod parser;
mod analyzer;

use crate::modules::structures::StructureBlock;
use crate::parser::file_parser;
use crate::analyzer::arc_analyzer::{self, check_atom_consistency, list_energy};
use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct MainProgram {
    #[structopt(subcommand)]
    subprogram: SubProgram,
}

#[derive(StructOpt)]
enum SubProgram {
    Parse(ParseArgs),
    Check(CheckArgs),
    Modify(ModifyArgs),
}

#[derive(StructOpt)]
struct ParseArgs{
    #[structopt(help = "The file to parse", short="f", long="file")]
    file: String,
    #[structopt(help = "enable output the minimum energy", short="m", long="minimum")]
    minimum: bool,
    #[structopt(help = "enable count the number of structures", short="c", long="count")]
    count: bool,
    #[structopt(help = "enable check the atom consistency", short="C", long="consistency")]
    consistency: bool,
    #[structopt(help = "enable list the energy of each structure", short="l", long="list")]
    energy_list: bool,
    #[structopt(help = "enable extract the minimum structure", short="e", long="extract")]
    extract: bool,
    #[structopt(help = "calculate the coordination number of the atoms in given structure", long="coord")]
    coordinate: Option<usize>,
    #[structopt(help = "calculate the interplanar spacing of the plains horizontal with the plain specified by given atoms", long="plane")]
    plain: Option<Vec<usize>>,
}

#[derive(StructOpt)]
struct CheckArgs{
    #[structopt(help = "the path to check", short="p", long="path")]
    path: std::path::PathBuf,
}

#[derive(StructOpt)]
struct ModifyArgs{
    #[structopt(help = "the file to modify", short="f", long="file")]
    file: std::path::PathBuf,
    #[structopt(help = "the structhre number to modify", short="n", long="number")]
    number: Option<usize>,
    #[structopt(help = "rearrange atoms by given coordinate", short="r", long="rearrange")]
    rearrange: Option<String>,
    #[structopt(help = "scale the crystal by given factor", short="s", long="scale")]
    scale: Option<Vec<f64>>,
}

fn parse(args: ParseArgs){
    let blocks = match file_parser::read_file(args.file){
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }
    };
    if args.minimum {
        let min_energy = arc_analyzer::find_minimum_energy(&blocks);
        match min_energy {
            Some(energy) => println!("Minimum energy: {}", energy),
            None => println!("No minimum energy found"),
        }
    }
    if args.count {
        let count = arc_analyzer::count_strucutre_block(&blocks);
        println!("Number of structures: {}", count);
    }
    if args.consistency {
        let atom_map = check_atom_consistency(&blocks);
        match atom_map {
            Some(map) => {
                println!("All blocks have the same atoms:");
                for (atom, count) in map.iter() {
                    println!("{}: {}", atom, count);
                }
            },
            None => println!("Not all blocks have the same atoms"),
        }
    }
    if args.energy_list {
        list_energy(&blocks);
    }
    if args.extract {
        let min_block = arc_analyzer::extract_minimum(&blocks);
        match min_block {
            None => println!("No minimum block found"),
            Some(block) => {
                block.write_to_file(String::from("minimum.arc")).unwrap();
                println!("Minimum block written to minimum.arc");
            }
        }
    }
    match args.coordinate{
        Some(n) => {
            let coord = arc_analyzer::calc_coordination(&blocks[n as usize]);
            for i in 0..coord.len(){
                println!("Atom {}: {}", blocks[n as usize].atoms[i].element, coord[i]);
            }
        },
        None => (),
    }
    match args.plain {
        Some(atoms) => {
            let minimum = arc_analyzer::extract_minimum(&blocks).unwrap();
            let spacing = arc_analyzer::calculate_interplanar_spacing(&minimum.atoms, atoms[0], atoms[1], atoms[2]);
            print!("Interplanar spacing: {:?}", spacing);
        }
        None => {
            ()
        }
    }
}

fn check(args: CheckArgs){
    let path = args.path;
    let badstr_path = path.join("badstr");
    let badstr = match file_parser::read_file(badstr_path.to_str().unwrap().to_string()){
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }
    };
    let unconverged_index = file_parser::find_unconverged_strucutres(path).unwrap();
    if badstr.len() >= 3 || unconverged_index.len() >= 3{
        println!("{}","this result might be unreliable!".red());
        println!("structure in Badstr.arc: {}",badstr.len());
        println!("unconverged iterations in lasp.out: {}", unconverged_index.len());
        println!("finding unconverged strucutres");
        let all_strucutres = match file_parser::read_file("all.arc".to_owned()){
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
        file_parser::write_to_file(unconverged_structure, String::from("unconverged.arc")).unwrap();
        println!("the unconverged structures have been written to unconverged.arc")
    }
    else {
        println!("{}","this result might be reliable!".green());
    }    
}

fn modify(args: ModifyArgs){
    let blocks = match file_parser::read_file(args.file.to_str().unwrap().to_string()){
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }
    };
    let mut block = match args.number{
        Some(n) => blocks[n].to_owned(),
        None => {
            arc_analyzer::extract_minimum(&blocks).unwrap().clone()
        }
        
    };
    match args.rearrange{
        Some(coordinate) => {
            let coordination = match coordinate.to_uppercase().as_str(){
                "X" =>{
                    arc_analyzer::rearrange_atoms(&mut block, |a, b| a.coordinate.0.partial_cmp(&b.coordinate.0).unwrap());
                    Some("X")
                },
                "Y" =>{
                    arc_analyzer::rearrange_atoms(&mut block, |a, b| a.coordinate.1.partial_cmp(&b.coordinate.1).unwrap());
                    Some("Y")
                },
                "Z" =>{
                    arc_analyzer::rearrange_atoms(&mut block, |a, b| a.coordinate.2.partial_cmp(&b.coordinate.2).unwrap());
                    Some("Z")
                },
                _ => {
                    println!("Please verify the sorting coordination: X, Y or Z.");
                    None
                }
            };
            block.clone().write_to_file(String::from("rearranged.arc")).unwrap();
            match coordination{
                Some(coord) => println!("the rearranged minimum structure (by {} value) has been generated.", coord),
                None => println!("Please specify the coordination to be sorted!\n rearranged.arc reamains unchanged.")
            }
        },
        None => (),
    }
    match args.scale {
        Some(scale) => {
            let mut new_block = block.clone();
            if scale.len() == 1{
                new_block = new_block.expand_crystal(scale[0]);
            }
            else if scale.len() == 3 {
                new_block = new_block.scale_crystal(modules::structures::CoordinateChoice::X, scale[0]);
                new_block = new_block.scale_crystal(modules::structures::CoordinateChoice::Y, scale[1]);
                new_block = new_block.scale_crystal(modules::structures::CoordinateChoice::Z, scale[2]);
            }
            new_block.write_to_file(String::from("scaled.arc")).unwrap();
            println!("the scaled structure has been generated.");
        },
        None => (),
    }
}

fn main(){
    let main_program = MainProgram::from_args();
    match main_program.subprogram {
        SubProgram::Parse(args) => {
            parse(args);
        },
        SubProgram::Check(args) => {
            check(args);
        }
        SubProgram::Modify(args) => {
            modify(args);
        }
    }
}