pub mod file_parser{
    //! some necessary functions that parses .arc file
    use crate::modules::structures::{Atom, StructureBlock, Coordinate, CrystalInfo};
    use std::fs::File;
    use std::path::Path;
    use std::io::{self, BufRead, Error, Write};
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
            let block_header_regex = Regex::new(r"^\s+Energy\s+(\d+)\s+(-?[0-9.]+)\s+(-?[0-9.]+)\s+(.*)$").unwrap();
            let block_header_without_symmetry_regex = Regex::new(r"^\s+Energy\s+(\d+)\s+(-?[0-9.]+)\s+(-?[0-9.]+)").unwrap();
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
                continue;
            }
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
            //check if the line is a cell information line
            let crystal_parse_result = cell_data_parser.parse_cell_info(&line);
            //if so, should set the block's crystal info
            if let Some(crystal) = crystal_parse_result{
                if let Some(block) = current_block.as_mut(){
                    block.set_crystal_info(crystal);
                }
            }
        }
        Ok(blocks)
    }
    /**
    an encapsulation of regex to parse lasp.out file
     */
    pub struct LaspOutParser{
        re:Regex,
    }
    impl LaspOutParser{
        fn new() -> Self{
            let lasp_out_parser = Regex::new(r"^Str symm and Q\s+(?P<num>\d+)").unwrap();
            Self { re: lasp_out_parser }
        }

        /**
        match lasp.out output information
         */
        fn parse(&self, input:&str) -> Option<u64>{
            if let Some(captures) = self.re.captures(input){
                let number = captures.name("num").unwrap().as_str().parse().unwrap();
                return Some(number);
            }
            None
        }
    }

    /**
    check lasp.out file to find all unconverged structures
    returns: `Vec<u64>`, a vector containing the position in all.arc for unconverged strucutres
     */
    pub fn find_unconverged_strucutres(path: std::path::PathBuf) -> io::Result<Vec<u64>>{
        let mut unconverged_strucutres: Vec<u64> = Vec::new();
        let lasp_out_path = path.join("lasp.out");
        let file = File::open(lasp_out_path)?;
        let reader = io::BufReader::new(file);
        let mut previous_line = String::new();
        let lasp_out_parser = LaspOutParser::new();
        for line in reader.lines(){
            let line = line?;
            if line.contains("not converged"){
                let number = lasp_out_parser.parse(&previous_line);
                if let Some(number) = number{
                    unconverged_strucutres.push(number.try_into().unwrap());
                }
                else{
                    eprintln!("unseen situation!");
                    eprintln!("{}", previous_line);
                    eprintln!("{}",line);
                }        
            }
            else {
                previous_line = line.clone();
            }
        }
        Ok(unconverged_strucutres)
    }

    /**
    write some blocks into a file
     */
    pub fn write_to_file(structures:Vec<StructureBlock>, path: String) -> Result<(), Error>
    {
        let mut file = File::create(path)?;
        writeln!(file, "!BIOSYM archive 2")?;
        writeln!(file, "PBC=ON")?;
        for block in structures.iter(){
            writeln!(file, "{: >28} Energy {: >10} {: >16.4} {: >18.6} {: >10}", "", 0, 0.0, block.energy, block.symmetry)?;
            writeln!(file, "!DATE")?;
            writeln!(file, "PBC {: >14.8} {: >14.8} {: >14.8} {: >14.8} {: >14.8} {: >14.8}", block.crystal.x, block.crystal.y, block.crystal.z, block.crystal.alpha, block.crystal.beta, block.crystal.gamma)?;
            for (i, atom) in block.atoms.iter().enumerate() {
                writeln!(file, "{: <5} {: >15.9} {: >15.9} {: >15.9} CORE {: >5} {: >1} {: <3} {: <5} {: <6} {: >5}", atom.element, (atom.coordinate.0), (atom.coordinate.1), (atom.coordinate.2), i+1, "", atom.element, atom.element, 0.0, i+1)?;
            }
            writeln!(file, "end")?;
            writeln!(file, "end")?;
        }
        Ok(())
    }
}