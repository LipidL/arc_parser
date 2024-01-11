pub mod arc_parser{
    //! some necessary functions that parses .arc file
    use crate::modules::structures::{Atom, StructureBlock, Coordinate, CrystalInfo};
    use std::fs::File;
    use std::path::Path;
    use std::io::{self, BufRead};
    use regex::Regex;

    /**
    an encapsulation of regex function parse_atom_data
     */
    pub struct AtomDataParser{
        re: Regex,
    }
    impl AtomDataParser{
        pub fn new() -> Self{
            let atom_data_regex = Regex::new(r"^(?P<s>\w+)\s+(?P<f1>-?\d+\.\d+)\s+(?P<f2>-?\d+\.\d+)\s+(?P<f3>-?\d+\.\d+)\s+CORE\s+.*").unwrap();
            Self { re: atom_data_regex }
        }
        /**
        match the atom information and extract necessary values.
        
        common atom info:
            C        7.210469000   10.148070000    0.813536200 CORE    1 C  C    0.0000    1
        extracts:
        + s: String,  element
        + f1: f64, x value
        + f2: f64, y value
        + f3: f64, z value
        */
        pub fn parse_atom_data(&self, input: &str) -> Option<(String, f64, f64, f64)> {
            if let Some(caps) = self.re.captures(input) {
                let s = caps.name("s").unwrap().as_str().to_string();
                let f1 = caps.name("f1").unwrap().as_str().parse().unwrap();
                let f2 = caps.name("f2").unwrap().as_str().parse().unwrap();
                let f3 = caps.name("f3").unwrap().as_str().parse().unwrap();
                return Some((s, f1, f2, f3));
            }
            None
        }
    }

    /**
    an encapsulation fo regex function parse_block_header
     */
    pub struct BlockHeaderParser{
        re: Regex,
        re2: Regex //in case that some block don't have symmetry line
    }
    impl BlockHeaderParser{
        pub fn new() -> Self{
            let block_header_regex = Regex::new(r"^\s+Energy\s+(\d+)\s+([0-9.]+)\s+(-?[0-9.]+)\s+(.*)$").unwrap();
            let block_header_without_symmetry_regex = Regex::new(r"^\s+Energy\s+(\d+)\s+([0-9.]+)\s+(-?[0-9.]+)").unwrap();
            Self { re: block_header_regex, re2: block_header_without_symmetry_regex }
        }
        /**
        match the block header and extract necessary values.
        
        common header:
            Energy         0          0.0099      -3620.679360        C1

        extracts: 
        + number: u64, serial number of the block, 0 in this case
        + float1: f64, a float that I don't know what it means
        + energy: f64, the energy(eV) of this block, -3620.679360 in this case
        + symmetry: String, symmetry of this block, C1 in this case
        */     
        pub fn parse_block_header(&self, input: &str) -> Option<(u64, f64, f64, String)> {

            if let Some(captures) = self.re.captures(input) {
                let number = captures[1].parse::<u64>().unwrap();
                let float1 = captures[2].parse::<f64>().unwrap();
                let energy = captures[3].parse::<f64>().unwrap();
                let symmetry = captures[4].to_string();
                Some((number, float1, energy, symmetry))
            } else if let Some(captures) = self.re2.captures(input){
                let number = captures[1].parse::<u64>().unwrap();
                let float1 = captures[2].parse::<f64>().unwrap();
                let energy = captures[3].parse::<f64>().unwrap();
                let symmetry = String::from("C1");
                Some((number, float1, energy, symmetry))
            } else{
                None
            }
        } 
    }


    pub struct CellDataParser{
        re: Regex,
    }
    impl CellDataParser{
        fn new() -> Self{
            let cell_info_regex = Regex::new(r"^\w+\s+(?P<x>\d+\.\d+)\s+(?P<y>\d+\.\d+)\s+(?P<z>\d+\.\d+)\s+(?P<alpha>\d+.\d+)\s+(?P<beta>\d+.\d+)\s+(?P<gamma>\d+.\d+)").unwrap();
            Self { re: cell_info_regex }
        }
        /**
        match cell information and extract necessary values.
        
        common cell information header:
        PBC   20.19500000   20.19500000   29.51410000   90.00000000   90.00000000  120.00000000
        extracts:
        + crystal: CrystalInfo, cell information of the block
        */
        fn parse_cell_info(&self, input: &str) -> Option<CrystalInfo>{
            if let Some(caps) = self.re.captures(input) {
                let x = caps.name("x").unwrap().as_str().parse().unwrap();
                let y = caps.name("y").unwrap().as_str().parse().unwrap();
                let z = caps.name("z").unwrap().as_str().parse().unwrap();
                let alpha = caps.name("alpha").unwrap().as_str().parse().unwrap();
                let beta = caps.name("beta").unwrap().as_str().parse().unwrap();
                let gamma = caps.name("gamma").unwrap().as_str().parse().unwrap();
                let crystal = CrystalInfo{
                    x,
                    y,
                    z,
                    alpha,
                    beta,
                    gamma
                };
                return Some(crystal);
            }
            None     
        }
    }

    /**
     * read target .arc file and parse block information.
     * return a vector of blocks found in the .arc file
     */
    pub fn read_file(filepath: String) -> io::Result<Vec<StructureBlock>>{
        //generate a file reader
        let path = Path::new(&filepath);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        //initialize block vector and current block
        let mut blocks:Vec<StructureBlock> = Vec::new();
        let mut current_block: Option<StructureBlock> = None;
        //compile necessary regex
        let block_header_parser = BlockHeaderParser::new();
        let atom_data_parser = AtomDataParser::new();
        let cell_data_parser = CellDataParser::new();
        //read the file line by line
        for line in reader.lines(){
            //handle cases of io error
            let line = line?;
            //check if the line is a block header
            let header_parse_result = block_header_parser.parse_block_header(&line);
            if let Some(header_info) = header_parse_result{
                //if so, should initialize a new block
                current_block = Some(StructureBlock { 
                    number: header_info.0,
                    energy: header_info.2,
                    symmetry: header_info.3,
                    crystal: CrystalInfo{
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        alpha: 0.0,
                        beta: 0.0,
                        gamma: 0.0
                    },
                    atoms: Vec::new()
                });
            }
            else if line == "end"{
                //if current line is an end, should push current block to block vector
                if let Some(block) = current_block.take(){
                    blocks.push(block);
                }
                //initialize current block again
                current_block = None;
            }
            //check if the line is a atom information line
            let atom_parse_result = atom_data_parser.parse_atom_data(&line);
            if let Some(atom_info) = atom_parse_result{
                //if so, should add the atom to the current block
                //initialize new atom
                let new_atom = Atom{
                    element: atom_info.0,
                    coordinate: Coordinate(atom_info.1, atom_info.2, atom_info.3)
                };
                //push the new atom to the current block
                if let Some(block) = current_block.as_mut(){
                    block.atoms.push(new_atom);
                }
            }
            //check if the line is a cell information line
            let crystal_parse_result = cell_data_parser.parse_cell_info(&line);
            //if so, should set the block's crystal info
            if let Some(crystal) = crystal_parse_result{
                if let Some(block) = current_block.as_mut(){
                    block.set_crystal_info(crystal);
                }
            }
        }
        return Ok(blocks);
    }
}