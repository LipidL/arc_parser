pub mod structures {
    pub struct Coordinate(pub f64,pub f64,pub f64);

    pub struct Atom {
        pub element: String,
        pub coordinate: Coordinate,
    }

    pub struct CrystalInfo{
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub alpha: f64,
        pub beta: f64,
        pub gamma: f64,
    }
    pub struct StructureBlock{
        pub number: u64,
        pub energy: f64,
        pub symmetry: String,
        pub crystal: CrystalInfo,
        pub atoms: Vec<Atom>,
    }

    impl StructureBlock {
        pub fn addatom(&mut self, atom: Atom){
            self.atoms.push(atom);
        }
    }

    impl StructureBlock{
        pub fn set_crystal_info(&mut self, crystal_info: CrystalInfo){
            self.crystal = crystal_info;
        }
    }
}
