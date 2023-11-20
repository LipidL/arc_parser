pub mod structures {
    pub struct StructureHeader{
        pub number: u64,
        pub unknown_variable: f64,
        pub energy: f64,
        pub symmetry: str,
    }
    pub struct Coordinate(pub f64,pub f64,pub f64);

    pub struct Atom {
        pub element: String,
        pub coordinate: Coordinate,
    }

    pub struct StructureBlock{
        pub energy: f64,
        pub atoms: Vec<Atom>,
    }

    impl StructureBlock {
        pub fn addatom(&mut self, atom: Atom){
            self.atoms.push(atom);
        }
    }
}
