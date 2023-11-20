pub mod arc_parser{
    use crate::modules::structures::{Atom, StructureBlock, Coordinate};
    use std::fs::File;
    use std::path::Path;
    use std::io::{self, BufRead};
    use regex::Regex;

    fn parse_block_header(input: &str) -> Option<(i32, f64, f64, String)> {
        let re = Regex::new(r"^\s+Energy\s+(\d+)\s+([0-9.]+)\s+(-?[0-9.]+)\s+(.*)$").unwrap();
        if let Some(captures) = re.captures(input) {
            let number = captures[1].parse::<i32>().unwrap();
            let float1 = captures[2].parse::<f64>().unwrap();
            let energy = captures[3].parse::<f64>().unwrap();
            let symmetry = captures[4].to_string();
            Some((number, float1, energy, symmetry))
        } else {
            None
        }
    }

    fn parse_atom_data(input: &str) -> Option<(String, f64, f64, f64)> {
        //^(\w+)\s+(-?\d+\.\d+)\s+(-?\d+\.\d+)\s+(-?\d+\.\d+)\s+.*
        let re = Regex::new(r"^(?P<s>\w+)\s+(?P<f1>-?\d+\.\d+)\s+(?P<f2>-?\d+\.\d+)\s+(?P<f3>-?\d+\.\d+)\s+CORE\s+.*").unwrap();
        if let Some(caps) = re.captures(input) {
            let s = caps.name("s").unwrap().as_str().to_string();
            let f1 = caps.name("f1").unwrap().as_str().parse().unwrap();
            let f2 = caps.name("f2").unwrap().as_str().parse().unwrap();
            let f3 = caps.name("f3").unwrap().as_str().parse().unwrap();
            return Some((s, f1, f2, f3));
        }
        None
    }

    pub fn read_file(filepath: String) -> io::Result<Vec<StructureBlock>>{
        let path = Path::new(&filepath);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut blocks:Vec<StructureBlock> = Vec::new();
        let mut current_block: Option<StructureBlock> = None;
        for line in reader.lines(){
            let line = line?;
            let header_parse_result = parse_block_header(&line);
            if let Some(header_info) = header_parse_result{
                current_block = Some(StructureBlock { 
                    energy: header_info.2, 
                    atoms: Vec::new()
                });
            }
            else if line == "end"{
                if let Some(block) = current_block.take(){
                    blocks.push(block);
                }
            }
            let atom_parse_result = parse_atom_data(&line);
            if let Some(atom_info) = atom_parse_result{
                let new_atom = Atom{
                    element: atom_info.0,
                    coordinate: Coordinate(atom_info.1, atom_info.2, atom_info.3)
                };
                if let Some(block) = current_block.as_mut(){
                    block.atoms.push(new_atom);
                }
            }
        }
        return Ok(blocks);
    }
}