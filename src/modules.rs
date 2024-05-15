pub mod structures {
    //! necessary structures to represent arc block
    pub enum CoordinateChoice{
        X,
        Y,
        Z
    }

    use std::{fs::File, io::{Error, Write}, ops::Sub};
    #[derive(Clone)] 
    pub struct Coordinate(pub f64,pub f64,pub f64);

    /// represents an atom, 
    /// storing its element and coordinate
    #[derive(Clone)] 
    pub struct Atom { 
        pub element: String,
        pub coordinate: Coordinate,
    }

    impl Sub for &Atom{
        type Output = Coordinate;
        fn sub(self, rhs: Self) -> Coordinate {
            Coordinate{
                0: self.coordinate.0 - rhs.coordinate.0,
                1: self.coordinate.1 - rhs.coordinate.1,
                2: self.coordinate.2 - rhs.coordinate.2
            }
        }
    }

    /// parameters of a cell
    #[derive(Clone)] 
    pub struct CrystalInfo{
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub alpha: f64,
        pub beta: f64,
        pub gamma: f64,
    }

    ///a block in an .arc file
    #[derive(Clone)] 
    pub struct StructureBlock{
        /// number: nth block of the file
        pub number: u64,
        pub energy: f64,
        pub symmetry: String,
        pub crystal: CrystalInfo,
        pub atoms: Vec<Atom>,
    }

    impl StructureBlock {
        ///add an atom to a block
        pub fn addatom(&mut self, atom: Atom){
            self.atoms.push(atom);
        }
        ///set the cell parameters of this block
        pub fn set_crystal_info(&mut self, crystal_info: CrystalInfo){
            self.crystal = crystal_info;
        }
        ///write the block to a file
        pub fn write_to_file(self, path:String) -> Result<(), Error>{
            let mut file = File::create(path)?;
            writeln!(file, "!BIOSYM archive 2")?;
            writeln!(file, "PBC=ON")?;
            writeln!(file, "{: >28} Energy {: >10} {: >16.4} {: >18.6} {: >10}", "", 0, 0.0, self.energy, self.symmetry)?;
            writeln!(file, "!DATE")?;
            writeln!(file, "PBC {: >14.8} {: >14.8} {: >14.8} {: >14.8} {: >14.8} {: >14.8}", self.crystal.x, self.crystal.y, self.crystal.z, self.crystal.alpha, self.crystal.beta, self.crystal.gamma)?;
            for (i, atom) in self.atoms.iter().enumerate() {
                writeln!(file, "{: <5} {: >15.9} {: >15.9} {: >15.9} CORE {: >5} {: >1} {: <3} {: <5} {: <6} {: >5}", atom.element, (atom.coordinate.0), (atom.coordinate.1), (atom.coordinate.2), i+1, "", atom.element, atom.element, 0.0, i+1)?;
            }
            writeln!(file, "end")?;
            writeln!(file, "end")?;
            writeln!(file, "\n")?;
            Ok(())
        }
        pub fn expand_crystal(&self, scale:f64) -> StructureBlock{
            let mut new_block:StructureBlock = self.clone();
            new_block.crystal.x *= scale;
            new_block.crystal.y *= scale;
            new_block.crystal.z *= scale;
            new_block
        }
        pub fn scale_crystal(&self, coordination:CoordinateChoice, scale:f64) -> StructureBlock{
            let mut new_block:StructureBlock = self.clone();
            match coordination{
                CoordinateChoice::X => new_block.crystal.x *= scale,
                CoordinateChoice::Y => new_block.crystal.y *= scale,
                CoordinateChoice::Z => new_block.crystal.z *= scale,
            }
            new_block
        }
    }
}
