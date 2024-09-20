mod modules;
pub mod parser;
mod analyzer;

use crate::modules::structures::StructureBlock;
use crate::parser::file_parser;
use crate::analyzer::arc_analyzer::{self, check_atom_consistency, list_energy};
use colored::*;
use structopt::StructOpt;
use itertools::Itertools;
use nalgebra::{self as na, Const, Dyn, VecStorage};
use std::sync::Arc;
use ctrlc;
use memory_stats::memory_stats;

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
    Compare(CompareArgs),
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

#[derive(StructOpt)]
struct CompareArgs{
    #[structopt(help = "the file to compare", short="f", long="file1")]
    file: std::path::PathBuf,
    #[structopt(help = "the file to compare", short="F", long="file2")]
    file2: std::path::PathBuf,
    #[structopt(help = "number of threads", short="t", long="threads")]
    threads: Option<usize>,
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

fn compare(args: CompareArgs){
    // setup a canary to dump data when killed
    {
        ctrlc::set_handler(move || {
            eprintln!("{}","SIG received, dumping data".red());
            if let Some(usage) = memory_stats() {
                eprintln!("{}: {}", "Physical memory usage".red(), usage.physical_mem);
                eprintln!("{}: {}", "Virtual memory usage".red(), usage.virtual_mem);
            }
        }).unwrap();
    }


    let blocks1 = match file_parser::read_file(args.file.to_str().unwrap().to_string()){
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }
    };
    let blocks2 = match file_parser::read_file(args.file2.to_str().unwrap().to_string()){
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }
    };
    println!("number of blocks: {}", blocks1.len());
    let ref_block_arc = Arc::new(blocks2[0].clone());
    let blocks1_arc = Arc::new(blocks1);
    let substructure_size = ref_block_arc.atoms.len();
    let num_threads = match args.threads{
        Some(n) => n,
        None => 1,
    };
    let mut handles = Vec::new();
    // compare each block in blocks1 with ref_block
    for i in 0..num_threads{
        let thread_index = i;
        let num_threads = num_threads;
        let ref_block = Arc::clone(&ref_block_arc);
        let blocks1 = Arc::clone(&blocks1_arc);
        let handle = std::thread::spawn(move || {
            for block_index in (0..blocks1.len()).filter(|x| x % num_threads == thread_index){
                // remove all atom that is not Fe
                println!("thread {} checking block {}",thread_index, block_index);
                let block = &blocks1[block_index];
                let mut block = block.clone();
                block.atoms.retain(|atom| atom.element == "Fe");
                // calculate bond matrix of block
                let bond_matrix = arc_analyzer::calc_coordination_matrix(&block);
                // interate over all rows in bond_matrix
                for i in 0..bond_matrix.nrows(){
                    // find all j such that bond_matrix[i][j] != 0
                    let mut neighbors = Vec::new();
                    for j in 0..bond_matrix.ncols(){
                        if bond_matrix[(i,j)] != 0{
                            neighbors.push(j);
                        }
                    }
                    // skip atoms whose neighbors is more than 11
                    if neighbors.len() > 11{
                        continue;
                    }
                    // find all combination of neighbors using Itertools having size of substructure_size
                    let combinations = neighbors.iter().combinations(substructure_size - 1).collect::<Vec<_>>();
                    // iterate over all combinations
                    for combination in combinations.iter(){
                        // create a position matrix consisting of the atoms in combination and atom i
                        let mut position_matrix = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::zeros(substructure_size);
                        for (index, atom_index) in combination.iter().enumerate(){
                            position_matrix[(0, index)] = block.atoms[**atom_index].coordinate.0;
                            position_matrix[(1, index)] = block.atoms[**atom_index].coordinate.1;
                            position_matrix[(2, index)] = block.atoms[**atom_index].coordinate.2;
                        }
                        position_matrix[(0, substructure_size - 1)] = block.atoms[i].coordinate.0;
                        position_matrix[(1, substructure_size - 1)] = block.atoms[i].coordinate.1;
                        position_matrix[(2, substructure_size - 1)] = block.atoms[i].coordinate.2;
                        // create the position matrix of ref_block
                        let mut ref_position_matrix = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::zeros(substructure_size);
                        for index in 0..substructure_size{
                            ref_position_matrix[(0, index)] = ref_block.atoms[index].coordinate.0;
                            ref_position_matrix[(1, index)] = ref_block.atoms[index].coordinate.1;
                            ref_position_matrix[(2, index)] = ref_block.atoms[index].coordinate.2;
                        }
                        // now the two matrices should have same ncols and nrows
                        assert!(position_matrix.nrows() == ref_position_matrix.nrows());
                        assert!(position_matrix.ncols() == ref_position_matrix.ncols());
                        // calculate the rmsd between the two matrices
                        let rmsd = arc_analyzer::calculate_rmsd_by_matrix(&position_matrix, &ref_position_matrix);
                        if rmsd < 0.3 {
                            let mut actual_atoms = combination.clone();
                            actual_atoms.push(&i);
                            println!("thread {} found a substructure at {}: rmsd={}; atoms={:?}",thread_index, block_index, rmsd, actual_atoms);
                        }
                    }
                }
                println!("thread {} completed block {}",thread_index, block_index);
            }
        });
        handles.push(handle); 
    } 
    // wait for all threads to finish
    for handle in handles{
        handle.join().unwrap();
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
        },
        SubProgram::Modify(args) => {
            modify(args);
        },
        SubProgram::Compare(args) => {
            compare(args);
        }
    }
}