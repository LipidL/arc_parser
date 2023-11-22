pub mod structures {
    //! necessary structures to represent arc block
    
    pub struct Coordinate(pub f64,pub f64,pub f64);

    /// represents an atom, 
    /// storing its element and coordinate
    pub struct Atom { 
        pub element: String,
        pub coordinate: Coordinate,
    }

    /// parameters of a cell
    pub struct CrystalInfo{
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub alpha: f64,
        pub beta: f64,
        pub gamma: f64,
    }

    ///a block in an .arc file
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
    }

    impl StructureBlock{
        ///set the cell parameters of this block
        pub fn set_crystal_info(&mut self, crystal_info: CrystalInfo){
            self.crystal = crystal_info;
        }
    }
}
