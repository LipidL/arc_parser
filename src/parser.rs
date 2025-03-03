pub mod parser{
    //! some necessary functions to parse a structure file
    //! currently support: .arc, .xyz
    use std::fmt::Debug;
    use std::{path::Path, fs::File, fmt};
    use std::io::{self, BufRead, Write};
    use regex::Regex;

    use crate::modules::structures::{Atom, StructureBlock, CrystalInfo, Coordinate};

    // custom error type for parsing
    pub enum ParseError {
        IoError(io::Error),
        ParseError(Vec<String>),
    }
    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ParseError::IoError(e) => write!(f, "IO error: {}", e),
                ParseError::ParseError(s) => write!(f, "Parse error: {:?}", s),
            }
        }
    }
    impl Debug for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ParseError::IoError(e) => write!(f, "IO error: {}", e),
                ParseError::ParseError(s) => write!(f, "Parse error: {:?}", s),
            }
        }
    }
    pub trait StructureIO{
        /// parse atom information line
        fn parse_atom(&self, input: &str) -> Option<Atom>;

        /// parse cell information line
        fn parse_cell(&self, intput: &str) -> Option<CrystalInfo>;

        /// parse block header line
        fn parse_header(&self, input: &str) -> Option<StructureBlock>;
        
        /// check if the input is illegal
        /// 
        /// the default implementation returns false if the input is empty, or none of the parse functions can parse the input
        fn is_illegal(&self, input: &str) -> bool{
            input.is_empty() || (self.parse_atom(input).is_none() && self.parse_header(input).is_none() && self.parse_cell(input).is_none())
        }
        fn parse_structure(&self, input: &Path, ignore_parse_error: bool) -> Result<Option<Vec<StructureBlock>>, ParseError>{
            // open the input file
            let file = File::open(input).map_err(ParseError::IoError)?;
            let reader = io::BufReader::new(file);
            // initialize block vector and current block
            let mut blocks: Vec<StructureBlock> = Vec::new();
            let mut current_block: Option<StructureBlock> = None;
            // initialize error vector
            let mut errors: Vec<String> = Vec::new();
            for line in reader.lines(){
                // handle cases of io error
                let line = line.map_err(ParseError::IoError)?;
                // parse the line as atom information line
                if let Some(atom) = self.parse_atom(&line){
                    // if so, should add the atom to the current block
                    // initialize new atom
                    let new_atom = atom;
                    // push the new atom to the current block
                    if let Some(block) = current_block.as_mut(){
                        block.atoms.push(new_atom);
                    }
                }
                // parse the line as block header line
                else if let Some(initial_block) = self.parse_header(&line){
                    // if the current_block is not None, should push it to the block vector
                    if let Some(block) = current_block.take(){
                        blocks.push(block);
                    }
                    // initialize the current block
                    current_block = Some(initial_block);
                }
                // parse the line as cell information line
                else if let Some(cell) = self.parse_cell(&line){
                    // if the current block is not None, should set the cell information
                    if let Some(block) = current_block.as_mut(){
                        block.set_crystal_info(cell);
                    }
                }
                else if !ignore_parse_error && self.is_illegal(&line) {
                    errors.push(line);
                }
            }
            // if the user doesn't ignore_parse_error, return the possible error
            if !errors.is_empty() && !ignore_parse_error {
                return Err(ParseError::ParseError(errors));
            }
            // push the last block to the block vector
            if let Some(block) = current_block.take(){
                blocks.push(block);
            }
            // if the length of blocks is larger than 0, return the blocks
            if !blocks.is_empty() {
                Ok(Some(blocks))
            } else {
                Ok(None)
            }
        }
        fn write_structure(&self, blocks: &Vec<StructureBlock>, output: &Path) -> io::Result<()> ;
            
    }

    pub struct ArcParser{
        atom_data_regex: Vec<Regex>,
        block_header_regex: Vec<Regex>,
        cell_info_regex: Vec<Regex>,       
    }
    impl ArcParser {
        pub fn new() -> Self {
            Self { atom_data_regex: vec![
                Regex::new(r"^(?P<s>\w+)\s+(?P<f1>-?\d+\.\d+)\s+(?P<f2>-?\d+\.\d+)\s+(?P<f3>-?\d+\.\d+)\s+CORE\s+.*").unwrap()
            ], 
            block_header_regex: vec![
                Regex::new(r"^\s+Energy\s+(\d+)\s+(-?[0-9.]+)\s+(-?[0-9.]+)\s+(.*)$").unwrap(), 
                Regex::new(r"^\s+Energy\s+(\d+)\s+(-?[0-9.]+)\s+(-?[0-9.]+)").unwrap()
            ], 
            cell_info_regex: vec![
                Regex::new(r"^\w+\s+(?P<x>\d+\.\d+)\s+(?P<y>\d+\.\d+)\s+(?P<z>\d+\.\d+)\s+(?P<alpha>\d+.\d+)\s+(?P<beta>\d+.\d+)\s+(?P<gamma>\d+.\d+)").unwrap()
            ]}
        }
    }
    impl StructureIO for ArcParser{
        fn parse_atom(&self, input: &str) -> Option<Atom> {
            for regex in self.atom_data_regex.iter(){
                if let Some(caps) = regex.captures(input) {
                    let s = caps.name("s").unwrap().as_str().to_string();
                    let f1 = caps.name("f1").unwrap().as_str().parse().unwrap();
                    let f2 = caps.name("f2").unwrap().as_str().parse().unwrap();
                    let f3 = caps.name("f3").unwrap().as_str().parse().unwrap();
                    return Some(Atom{
                        element: s,
                        coordinate: Coordinate(f1, f2, f3)
                    });
                }
            }
            None
        }
        fn parse_cell(&self, intput: &str) -> Option<CrystalInfo> {
            for regex in self.cell_info_regex.iter(){
                if let Some(caps) = regex.captures(intput) {
                    let x = caps.name("x").unwrap().as_str().parse().unwrap();
                    let y = caps.name("y").unwrap().as_str().parse().unwrap();
                    let z = caps.name("z").unwrap().as_str().parse().unwrap();
                    let alpha = caps.name("alpha").unwrap().as_str().parse().unwrap();
                    let beta = caps.name("beta").unwrap().as_str().parse().unwrap();
                    let gamma = caps.name("gamma").unwrap().as_str().parse().unwrap();
                    return Some(CrystalInfo{
                        x,
                        y,
                        z,
                        alpha,
                        beta,
                        gamma
                    });
                }
            }
            None
        }
        fn parse_header(&self, input: &str) -> Option<StructureBlock> {
            for regex in self.block_header_regex.iter(){
                if let Some(captures) = regex.captures(input) {
                    let number = captures[1].parse::<u64>().unwrap();
                    let _float1 = captures[2].parse::<f64>().unwrap();
                    let energy = captures[3].parse::<f64>().unwrap();
                    let symmetry = captures.get(4).map(|s| s.as_str()).unwrap_or("C1").to_string();
                    return Some(StructureBlock{
                        number,
                        energy,
                        symmetry,
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
            }
            None
        }
        fn is_illegal(&self, input: &str) -> bool {
            input.is_empty() || (self.parse_atom(input).is_none() && self.parse_header(input).is_none() && self.parse_cell(input).is_none() && !["end", "!DATE"].contains(&input.trim()) && !input.contains("!BIOSYM archive") && !input.contains("PBC="))
        }
        fn write_structure(&self, structures:&Vec<StructureBlock>, path: &Path) -> io::Result<()> {
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
    pub struct XyzParser{
        atom_data_regex: Vec<Regex>,
    }
    impl XyzParser {
        pub fn new() -> Self {
            Self { atom_data_regex: vec![
                Regex::new(r"^(?P<s>\w+)\s+(?P<f1>-?\d+\.\d+)\s+(?P<f2>-?\d+\.\d+)\s+(?P<f3>-?\d+\.\d+)").unwrap()
            ]}
        }
    }

    impl StructureIO for XyzParser{
        fn parse_atom(&self, input: &str) -> Option<Atom> {
            for regex in self.atom_data_regex.iter(){
                if let Some(caps) = regex.captures(input) {
                    let s = caps.name("s").unwrap().as_str().to_string();
                    let f1 = caps.name("f1").unwrap().as_str().parse().unwrap();
                    let f2 = caps.name("f2").unwrap().as_str().parse().unwrap();
                    let f3 = caps.name("f3").unwrap().as_str().parse().unwrap();
                    return Some(Atom{
                        element: s,
                        coordinate: Coordinate(f1, f2, f3)
                    });
                }
            }
            None
        }
        fn parse_cell(&self, _intput: &str) -> Option<CrystalInfo> {
            None // xyz file doesn't have cell information
        }
        fn parse_header(&self, input: &str) -> Option<StructureBlock> {
            if let Ok(number) = input.parse::<u64>() {
                Some(StructureBlock{
                    number,
                    energy: 0.0,
                    symmetry: String::from("C1"),
                    crystal: CrystalInfo{
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        alpha: 0.0,
                        beta: 0.0,
                        gamma: 0.0
                    },
                    atoms: Vec::new()
                })
            } else {
                None
            }
        }
        fn is_illegal(&self, input: &str) -> bool {
            input.is_empty() || (self.parse_atom(input).is_none() && self.parse_header(input).is_none())
        }
        fn write_structure(&self, structures:&Vec<StructureBlock>, path: &Path) -> io::Result<()>{
            let mut file = File::create(path)?;
            for block in structures.iter(){
                writeln!(file, "{}", block.atoms.len())?;
                writeln!(file, "Energy: {}", block.energy)?;
                for atom in block.atoms.iter(){
                    writeln!(file, "{} {} {} {}", atom.element, atom.coordinate.0, atom.coordinate.1, atom.coordinate.2)?;
                }
            }
            Ok(())
        }
    }

    pub fn get_parser(file_type: &str) -> Box<dyn StructureIO> {
        match file_type {
            "arc" => Box::new(ArcParser::new()),
            "xyz" => Box::new(XyzParser::new()),
            _ => panic!("unsupported file type: {}", file_type)
        }
    }
    pub fn read_file(filename: &str, ignore_parse_error: bool) -> Result<Option<Vec<StructureBlock>>, ParseError> {
        let path = Path::new(filename);
        let file_type = path.extension().unwrap().to_str().unwrap();
        let parser = get_parser(file_type);
        parser.parse_structure(path, ignore_parse_error)
    }
    /**
    an encapsulation of regex to parse lasp.out file
     */
    struct LaspOutParser{
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

}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use tempfile::NamedTempFile;
    use std::io::Write;

    use crate::parser::parser::*;
    use crate::modules::structures::*;

    #[test]
    fn test_parse_atom_success() {
        let parser = ArcParser::new();
        let input = "C        7.210469000   10.148070000    0.813536200 CORE    1 C  C    0.0000    1";
        let atom = parser.parse_atom(input).unwrap();
        assert_eq!(atom.element, "C", "Expected 'C'. Got {:?}", atom.element);
        assert_eq!(atom.coordinate.0, 7.210469, "Expected 7.210469. Got {:?}", atom.coordinate.0);
        assert_eq!(atom.coordinate.1, 10.14807, "Expected 10.14807. Got {:?}", atom.coordinate.1);
        assert_eq!(atom.coordinate.2, 0.8135362, "Expected 0.8135362. Got {:?}", atom.coordinate.2);
    }
    #[test]
    fn test_parse_atom_fail() {
        let parser = ArcParser::new();
        let input = "some_random_string";
        let atom = parser.parse_atom(input);
        assert!(atom.is_none(), "Expected None. Got {:?}", atom);
    }

    #[test]
    fn test_parse_header_success() {
        let parser = ArcParser::new();
        let input = "   Energy         0          0.0099      -3620.679360        C1";
        let block = parser.parse_header(input).unwrap();
        assert_eq!(block.number, 0, "Expected 0. Got {:?}", block.number);
        assert_eq!(block.energy, -3620.679360, "Expected -3620.679360. Got {:?}", block.energy);
        assert_eq!(block.symmetry, "C1", "Expected 'C1'. Got {:?}", block.symmetry);
    }
    #[test]
    fn test_parse_header_fail() {
        let parser = ArcParser::new();
        let input = "some_random_string";
        let block = parser.parse_header(input);
        assert!(block.is_none(), "Expected None. Got {:?}", block);
    }

    #[test]
    fn test_parse_cell_success() {
        let parser = ArcParser::new();
        let input = "PBC   20.19500000   20.19500000   29.51410000   90.00000000   90.00000000  120.00000000";
        let cell = parser.parse_cell(input).unwrap();
        assert_eq!(cell.x, 20.195, "Expected 20.195. Got {:?}", cell.x);
        assert_eq!(cell.y, 20.195, "Expected 20.195. Got {:?}", cell.y);
        assert_eq!(cell.z, 29.5141, "Expected 29.5141. Got {:?}", cell.z);
        assert_eq!(cell.alpha, 90.0, "Expected 90.0. Got {:?}", cell.alpha);
        assert_eq!(cell.beta, 90.0, "Expected 90.0. Got {:?}", cell.beta);
        assert_eq!(cell.gamma, 120.0, "Expected 120.0. Got {:?}", cell.gamma);
    }
    #[test]
    fn test_parse_cell_fail() {
        let parser = ArcParser::new();
        let input = "some_random_string";
        let cell = parser.parse_cell(input);
        assert!(cell.is_none(), "Expected None. Got {:?}", cell);
    }

    #[test]
    fn test_parse_structure_success() {
        // prepare a temporary file for testing
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "   Energy         0          0.0099      -3620.679360        C1").unwrap();
        writeln!(file, "PBC   20.19500000   20.19500000   29.51410000   90.00000000   90.00000000  120.00000000").unwrap();
        writeln!(file, "C        7.210469000   10.148070000    0.813536200 CORE    1 C  C    0.0000    1").unwrap();
        writeln!(file, "end").unwrap();
        let parser = ArcParser::new();
        let path = file.path();
        let result = parser.parse_structure(path, false);
        assert!(result.is_ok(), "Expected Ok. Got {:?}", result);
        let blocks = result.unwrap().unwrap();
        assert_eq!(blocks.len(), 1, "Expected 1. Got {:?}", blocks.len());
        let block = &blocks[0];
        assert_eq!(block.number, 0, "Expected 0. Got {:?}", block.number);
        assert_eq!(block.energy, -3620.679360, "Expected -3620.679360. Got {:?}", block.energy);
        assert_eq!(block.symmetry, "C1", "Expected 'C1'. Got {:?}", block.symmetry);
        assert_eq!(block.atoms.len(), 1, "Expected 1. Got {:?}", block.atoms.len());
        let atom = &block.atoms[0];
        assert_eq!(atom.element, "C", "Expected 'C'. Got {:?}", atom.element);
        assert_eq!(atom.coordinate.0, 7.210469, "Expected 7.210469. Got {:?}", atom.coordinate.0);
        assert_eq!(atom.coordinate.1, 10.14807, "Expected 10.14807. Got {:?}", atom.coordinate.1);
        assert_eq!(atom.coordinate.2, 0.8135362, "Expected 0.8135362. Got {:?}", atom.coordinate.2);
    }
    #[test]
    fn test_parse_structure_fail() {
        let parser = ArcParser::new();
        // an invalid file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "some_random_string").unwrap();
        let path = file.path();
        // returns None if setting ignore_parse_error to true
        let result = parser.parse_structure(path, true);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        // returns a ParseError if setting ignore_parse_error to false
        let result = parser.parse_structure(path, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ParseError::ParseError(_)));
        // an IO error
        let path = Path::new("non_existent_file.arc");
        let result = parser.parse_structure(path, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ParseError::IoError(_)));
    }

    #[test]
    fn test_parse_xyz_atom_success() {
        let parser = XyzParser::new();
        let input = "C        7.210469000   10.148070000    0.813536200";
        let atom = parser.parse_atom(input).unwrap();
        assert_eq!(atom.element, "C");
        assert_eq!(atom.coordinate.0, 7.210469);
        assert_eq!(atom.coordinate.1, 10.14807);
        assert_eq!(atom.coordinate.2, 0.8135362);
    }
    #[test]
    fn test_parse_xyz_atom_fail() {
        let parser = XyzParser::new();
        let input = "some_random_string";
        let atom = parser.parse_atom(input);
        assert!(atom.is_none());
    }

    #[test]
    fn test_parse_xyz_header_success() {
        let parser = XyzParser::new();
        let input = "5";
        let block = parser.parse_header(input).unwrap();
        assert_eq!(block.number, 5);
        assert_eq!(block.energy, 0.0);
        assert_eq!(block.symmetry, "C1");
    }
    #[test]
    fn test_parse_xyz_header_fail() {
        let parser = XyzParser::new();
        let input = "some_random_string";
        let block = parser.parse_header(input);
        assert!(block.is_none());
    }

    #[test]
    fn test_parse_xyz_structure_success() {
        // construct a temporary file for testing
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "5").unwrap();
        writeln!(file, "C        7.210469000   10.148070000    0.813536200").unwrap();
        let parser = XyzParser::new();
        let path = file.path();
        let result = parser.parse_structure(path, false);
        assert!(result.is_ok());
        let blocks = result.unwrap().unwrap();
        assert_eq!(blocks.len(), 1);
        let block = &blocks[0];
        assert_eq!(block.number, 5);
        assert_eq!(block.energy, 0.0);
        assert_eq!(block.symmetry, "C1");
        assert_eq!(block.atoms.len(), 1);
        let atom = &block.atoms[0];
        assert_eq!(atom.element, "C");
        assert_eq!(atom.coordinate.0, 7.210469);
        assert_eq!(atom.coordinate.1, 10.14807);
        assert_eq!(atom.coordinate.2, 0.8135362);
    }
    #[test]
    fn test_parse_xyz_structure_fail() {
        let parser = XyzParser::new();
        // an invalid file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "some_random_string").unwrap();
        let path = file.path();
        // returns None if setting ignore_parse_error to true
        let result = parser.parse_structure(path, true);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        // returns a ParseError if setting ignore_parse_error to false
        let result = parser.parse_structure(path, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ParseError::ParseError(_)));
        // an IO error
        let path = Path::new("non_existent_file.xyz");
        let result = parser.parse_structure(path, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ParseError::IoError(_)));
    }
    #[test]
    fn test_write_arc_structure_success() {
        let parser = ArcParser::new();
        let file = NamedTempFile::new().unwrap();
        let blocks = vec![
            StructureBlock{
                number: 0,
                energy: -3620.679360,
                symmetry: "C1".to_string(),
                crystal: CrystalInfo{
                    x: 20.195,
                    y: 20.195,
                    z: 29.5141,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 120.0
                },
                atoms: vec![
                    Atom{
                        element: "C".to_string(),
                        coordinate: Coordinate(7.210469, 10.14807, 0.8135362)
                    }
                ]
            }
        ];
        let path = file.path();
        let result = parser.write_structure(&blocks, path);
        assert!(result.is_ok());
        let read_blocks: Vec<StructureBlock> = parser.parse_structure(path, false).unwrap().unwrap();
        assert_eq!(read_blocks.len(), 1);
        let read_block = &read_blocks[0];
        assert_eq!(read_block.number, 0);
        assert_eq!(read_block.energy, -3620.679360);
        assert_eq!(read_block.symmetry, "C1");
        assert_eq!(read_block.crystal.x, 20.195);
        assert_eq!(read_block.crystal.y, 20.195);
        assert_eq!(read_block.crystal.z, 29.5141);
        assert_eq!(read_block.crystal.alpha, 90.0);
        assert_eq!(read_block.crystal.beta, 90.0);
        assert_eq!(read_block.crystal.gamma, 120.0);
        assert_eq!(read_block.atoms.len(), 1);
        let read_atom = &read_block.atoms[0];
        assert_eq!(read_atom.element, "C");
        assert_eq!(read_atom.coordinate.0, 7.210469);
        assert_eq!(read_atom.coordinate.1, 10.14807);
        assert_eq!(read_atom.coordinate.2, 0.8135362);
    }
}