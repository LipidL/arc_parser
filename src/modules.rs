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

pub mod periodic_table {
    use std::{collections::HashMap, f64::NAN};

    #[derive(Debug)]
    #[derive(PartialEq)]
pub struct Element {
        pub name: String,
        pub atomic_number: u64,
        pub mass: f64,
        pub valence_radius: f64,
        pub valence_electrons: u32,
        pub ion_radius: HashMap<i32, f64>,
        pub atom_radius: f64, // in angstroms
    }

    pub struct PeriodicTable {
        elements: HashMap<String, Element>,
    }

    impl PeriodicTable {
        // Create a new PeriodicTable
        pub fn new() -> Self {
            let mut elements = HashMap::new();
            // ion_radius for Fe
            let mut iron_ion_radius = HashMap::new();
            iron_ion_radius.insert(2, 0.76); // Fe2+ ion radius
            iron_ion_radius.insert(3, 0.64); // Fe3+ ion radius

            // Add an element to the periodic table
            elements.insert(
                String::from("Fe"),
                Element {
                    name: String::from("Iron"),
                    atomic_number: 26,
                    mass: 55.845,
                    valence_radius: NAN, 
                    valence_electrons: 8, 
                    ion_radius: iron_ion_radius, // Replace with actual value
                    atom_radius: 1.17,
                },
            );

            // Add more elements...

            Self { elements }
        }

        // Get an element by its symbol
        pub fn get(&self, symbol: &str) -> Option<&Element> {
            self.elements.get(symbol)
        }

        // Add more methods as needed...
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::structures::CrystalInfo;

    use super::periodic_table::PeriodicTable;

    #[test]
    fn test_periodic_table() {
        let table = PeriodicTable::new();
        let iron = table.get("Fe").unwrap();
        assert_eq!(iron.name, "Iron");
        assert_eq!(iron.atomic_number, 26);
        assert_eq!(iron.mass, 55.845);
        assert_eq!(iron.valence_electrons, 8);
        assert_eq!(iron.ion_radius.get(&2), Some(&0.76));
        assert_eq!(iron.ion_radius.get(&3), Some(&0.64));
        assert_eq!(iron.atom_radius, 1.17);
    }

    #[test]
    fn test_periodic_table_missing_element() {
        let table = PeriodicTable::new();
        assert_eq!(table.get("H"), None);
    }

    #[test]
    fn test_atom_subtraction() {
        use super::structures::{Atom, Coordinate};
        let atom1 = Atom {
            element: "Fe".to_string(),
            coordinate: Coordinate(5.0, 5.0, 5.0),
        };
        let atom2 = Atom {
            element: "Fe".to_string(),
            coordinate: Coordinate(1.0, 2.0, 3.0),
        };
        let diff = &atom1 - &atom2;
        assert!(diff.0 - 4.0 < 1e-6);
        assert!(diff.1 - 3.0 < 1e-6);
        assert!(diff.2 - 2.0 < 1e-6);
    }

    #[test]
    fn test_addatom() {
        use super::structures::{Atom, Coordinate, StructureBlock};
        let mut block = StructureBlock {
            number: 1,
            energy: 0.0,
            symmetry: "P1".to_string(),
            crystal: CrystalInfo {
                x: 10.0,
                y: 10.0,
                z: 10.0,
                alpha: 90.0,
                beta: 90.0,
                gamma: 90.0,
            },
            atoms: vec![],
        };
        let atom = Atom {
            element: "Fe".to_string(),
            coordinate: Coordinate(1.0, 2.0, 3.0),
        };
        block.addatom(atom);
        assert_eq!(block.atoms.len(), 1);
        assert_eq!(block.atoms[0].element, "Fe");
        assert!(block.atoms[0].coordinate.0 - 1.0 < 1e-6);
        assert!(block.atoms[0].coordinate.1 - 2.0 < 1e-6);
        assert!(block.atoms[0].coordinate.2 - 3.0 < 1e-6);
    }

    #[test]
    fn test_set_crystal_info() {
        use super::structures::{CrystalInfo, StructureBlock};
        let mut block = StructureBlock {
            number: 1,
            energy: 0.0,
            symmetry: "P1".to_string(),
            crystal: CrystalInfo {
                x: 10.0,
                y: 10.0,
                z: 10.0,
                alpha: 90.0,
                beta: 90.0,
                gamma: 90.0,
            },
            atoms: vec![],
        };
        let new_crystal = CrystalInfo {
            x: 20.0,
            y: 20.0,
            z: 20.0,
            alpha: 90.0,
            beta: 90.0,
            gamma: 90.0,
        };
        block.set_crystal_info(new_crystal);
        assert!(block.crystal.x - 20.0 < 1e-6);
        assert!(block.crystal.y - 20.0 < 1e-6);
        assert!(block.crystal.z - 20.0 < 1e-6);
    }

    #[test]
    fn test_expand_crystal() {
        use super::structures::{CrystalInfo, StructureBlock};
        let block = StructureBlock {
            number: 1,
            energy: 0.0,
            symmetry: "P1".to_string(),
            crystal: CrystalInfo {
                x: 10.0,
                y: 10.0,
                z: 10.0,
                alpha: 90.0,
                beta: 90.0,
                gamma: 90.0,
            },
            atoms: vec![],
        };
        let new_block = block.expand_crystal(2.0);
        assert!(new_block.crystal.x - 20.0 < 1e-6);
        assert!(new_block.crystal.y - 20.0 < 1e-6);
        assert!(new_block.crystal.z - 20.0 < 1e-6);
    }

    #[test]
    fn test_scale_crystal() {
        use super::structures::{CoordinateChoice, CrystalInfo, StructureBlock};
        let block = StructureBlock {
            number: 1,
            energy: 0.0,
            symmetry: "P1".to_string(),
            crystal: CrystalInfo {
                x: 10.0,
                y: 10.0,
                z: 10.0,
                alpha: 90.0,
                beta: 90.0,
                gamma: 90.0,
            },
            atoms: vec![],
        };
        let new_block = block.scale_crystal(CoordinateChoice::X, 2.0);
        assert!(new_block.crystal.x - 20.0 < 1e-6);
        assert!(new_block.crystal.y - 10.0 < 1e-6);
        assert!(new_block.crystal.z - 10.0 < 1e-6);
    }
}