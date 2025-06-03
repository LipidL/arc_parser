pub mod structures {
    //! necessary structures to represent arc block
    pub enum CoordinateChoice{
        X,
        Y,
        Z
    }

    use std::{fmt::Debug, io::Error, ops::Sub, path::Path};

    use crate::parser::parser;
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
    impl Debug for Atom {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} at ({}, {}, {})", self.element, self.coordinate.0, self.coordinate.1, self.coordinate.2)
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
    impl Debug for CrystalInfo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Cell parameters: a={}, b={}, c={}, alpha={}, beta={}, gamma={}", self.x, self.y, self.z, self.alpha, self.beta, self.gamma)
        }
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
        pub fn write(&self, path:&Path) -> Result<(), Error>{
            // figure out the output format
            let file_type = path.extension().unwrap().to_str().unwrap();
            let writer = parser::get_parser(file_type);
            writer.write_structure(&vec![self.clone()], path)?;
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
    impl Debug for StructureBlock {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Block {} with {} atoms", self.number, self.atoms.len())
        }
    }
}

pub mod periodic_table {
    use std::{collections::HashMap};

    #[derive(Debug)]
    #[derive(PartialEq)]
pub struct Element {
        pub name: String,
        pub atomic_number: u64,
        pub mass: f64,
        pub valence_radius: f64,
        pub valence_electrons: u32,
        pub atom_radius: f64, // in angstroms
    }

    pub struct PeriodicTable {
        elements: HashMap<String, Element>,
    }

    impl PeriodicTable {
        // Create a new PeriodicTable
        pub fn new() -> Self {
            let mut elements = HashMap::new();
            // Note: The Element struct needs an ion_radius field to pass tests
            elements.insert("H".to_string(), Element {
                name: "Hydrogen".to_string(),
                atomic_number: 1,
                mass: 1.008,
                valence_radius: 0.32,
                valence_electrons: 1,
                atom_radius: 0.25,
            });

            elements.insert("He".to_string(), Element {
                name: "Helium".to_string(),
                atomic_number: 2,
                mass: 4.003,
                valence_radius: 0.93,
                valence_electrons: 2,
                atom_radius: 0.31,
            });

            elements.insert("Li".to_string(), Element {
                name: "Lithium".to_string(),
                atomic_number: 3,
                mass: 6.941,
                valence_radius: 1.23,
                valence_electrons: 1,
                atom_radius: 1.67,
            });

            elements.insert("Be".to_string(), Element {
                name: "Beryllium".to_string(),
                atomic_number: 4,
                mass: 9.0122,
                valence_radius: 0.90,
                valence_electrons: 2,
                atom_radius: 1.12,
            });

            elements.insert("B".to_string(), Element {
                name: "Boron".to_string(),
                atomic_number: 5,
                mass: 10.811,
                valence_radius: 0.82,
                valence_electrons: 3,
                atom_radius: 0.87,
            });

            elements.insert("C".to_string(), Element {
                name: "Carbon".to_string(),
                atomic_number: 6,
                mass: 12.011,
                valence_radius: 0.67,
                valence_electrons: 4,
                atom_radius: 0.77,
            });

            elements.insert("N".to_string(), Element {
                name: "Nitrogen".to_string(),
                atomic_number: 7,
                mass: 14.007,
                valence_radius: 0.56,
                valence_electrons: 5,
                atom_radius: 0.75,
            });

            elements.insert("O".to_string(), Element {
                name: "Oxygen".to_string(),
                atomic_number: 8,
                mass: 16.00,
                valence_radius: 0.60,
                valence_electrons: 6,
                atom_radius: 0.73,
            });

            elements.insert("F".to_string(), Element {
                name: "Fluorine".to_string(),
                atomic_number: 9,
                mass: 18.998,
                valence_radius: 0.50,
                valence_electrons: 7,
                atom_radius: 0.71,
            });

            elements.insert("Ne".to_string(), Element {
                name: "Neon".to_string(),
                atomic_number: 10,
                mass: 20.180,
                valence_radius: 1.12,
                valence_electrons: 8,
                atom_radius: 0.69,
            });

            elements.insert("Na".to_string(), Element {
                name: "Sodium".to_string(),
                atomic_number: 11,
                mass: 22.990,
                valence_radius: 1.54,
                valence_electrons: 1,
                atom_radius: 1.90,
            });

            elements.insert("Mg".to_string(), Element {
                name: "Magnesium".to_string(),
                atomic_number: 12,
                mass: 24.305,
                valence_radius: 1.36,
                valence_electrons: 2,
                atom_radius: 1.45,
            });

            elements.insert("Al".to_string(), Element {
                name: "Aluminum".to_string(),
                atomic_number: 13,
                mass: 26.982,
                valence_radius: 1.18,
                valence_electrons: 3,
                atom_radius: 1.18,
            });

            elements.insert("Si".to_string(), Element {
                name: "Silicon".to_string(),
                atomic_number: 14,
                mass: 28.086,
                valence_radius: 1.11,
                valence_electrons: 4,
                atom_radius: 1.11,
            });

            elements.insert("P".to_string(), Element {
                name: "Phosphorus".to_string(),
                atomic_number: 15,
                mass: 30.974,
                valence_radius: 1.06,
                valence_electrons: 5,
                atom_radius: 1.06,
            });

            elements.insert("S".to_string(), Element {
                name: "Sulfur".to_string(),
                atomic_number: 16,
                mass: 32.066,
                valence_radius: 1.02,
                valence_electrons: 6,
                atom_radius: 1.02,
            });

            elements.insert("Cl".to_string(), Element {
                name: "Chlorine".to_string(),
                atomic_number: 17,
                mass: 35.453,
                valence_radius: 0.99,
                valence_electrons: 7,
                atom_radius: 0.99,
            });

            elements.insert("Ar".to_string(), Element {
                name: "Argon".to_string(),
                atomic_number: 18,
                mass: 39.948,
                valence_radius: 1.54,
                valence_electrons: 8,
                atom_radius: 0.97,
            });

            elements.insert("K".to_string(), Element {
                name: "Potassium".to_string(),
                atomic_number: 19,
                mass: 39.098,
                valence_radius: 2.03,
                valence_electrons: 1,
                atom_radius: 2.43,
            });

            elements.insert("Ca".to_string(), Element {
                name: "Calcium".to_string(),
                atomic_number: 20,
                mass: 40.078,
                valence_radius: 1.74,
                valence_electrons: 2,
                atom_radius: 1.94,
            });

            elements.insert("Sc".to_string(), Element {
                name: "Scandium".to_string(),
                atomic_number: 21,
                mass: 44.956,
                valence_radius: 1.44,
                valence_electrons: 3,
                atom_radius: 1.84,
            });

            elements.insert("Ti".to_string(), Element {
                name: "Titanium".to_string(),
                atomic_number: 22,
                mass: 47.867,
                valence_radius: 1.32,
                valence_electrons: 4,
                atom_radius: 1.76,
            });

            elements.insert("V".to_string(), Element {
                name: "Vanadium".to_string(),
                atomic_number: 23,
                mass: 50.942,
                valence_radius: 1.22,
                valence_electrons: 5,
                atom_radius: 1.71,
            });

            elements.insert("Cr".to_string(), Element {
                name: "Chromium".to_string(),
                atomic_number: 24,
                mass: 51.996,
                valence_radius: 1.18,
                valence_electrons: 6,
                atom_radius: 1.66,
            });

            elements.insert("Mn".to_string(), Element {
                name: "Manganese".to_string(),
                atomic_number: 25,
                mass: 54.938,
                valence_radius: 1.17,
                valence_electrons: 7,
                atom_radius: 1.61,
            });

            elements.insert("Fe".to_string(), Element {
                name: "Iron".to_string(),
                atomic_number: 26,
                mass: 55.845,
                valence_radius: 1.32,
                valence_electrons: 8,
                atom_radius: 1.17,
            });

            elements.insert("Co".to_string(), Element {
                name: "Cobalt".to_string(),
                atomic_number: 27,
                mass: 58.933,
                valence_radius: 1.26,
                valence_electrons: 9,
                atom_radius: 1.16,
            });

            elements.insert("Ni".to_string(), Element {
                name: "Nickel".to_string(),
                atomic_number: 28,
                mass: 58.693,
                valence_radius: 1.24,
                valence_electrons: 10,
                atom_radius: 1.15,
            });

            elements.insert("Cu".to_string(), Element {
                name: "Copper".to_string(),
                atomic_number: 29,
                mass: 63.546,
                valence_radius: 1.32,
                valence_electrons: 11,
                atom_radius: 1.17,
            });

            elements.insert("Zn".to_string(), Element {
                name: "Zinc".to_string(),
                atomic_number: 30,
                mass: 65.38,
                valence_radius: 1.22,
                valence_electrons: 12,
                atom_radius: 1.25,
            });

            elements.insert("Ga".to_string(), Element {
                name: "Gallium".to_string(),
                atomic_number: 31,
                mass: 69.723,
                valence_radius: 1.22,
                valence_electrons: 3,
                atom_radius: 1.26,
            });

            elements.insert("Ge".to_string(), Element {
                name: "Germanium".to_string(),
                atomic_number: 32,
                mass: 72.63,
                valence_radius: 1.20,
                valence_electrons: 4,
                atom_radius: 1.22,
            });

            elements.insert("As".to_string(), Element {
                name: "Arsenic".to_string(),
                atomic_number: 33,
                mass: 74.922,
                valence_radius: 1.19,
                valence_electrons: 5,
                atom_radius: 1.19,
            });

            elements.insert("Se".to_string(), Element {
                name: "Selenium".to_string(),
                atomic_number: 34,
                mass: 78.971,
                valence_radius: 1.20,
                valence_electrons: 6,
                atom_radius: 1.16,
            });

            elements.insert("Br".to_string(), Element {
                name: "Bromine".to_string(),
                atomic_number: 35,
                mass: 79.904,
                valence_radius: 1.20,
                valence_electrons: 7,
                atom_radius: 1.14,
            });

            elements.insert("Kr".to_string(), Element {
                name: "Krypton".to_string(),
                atomic_number: 36,
                mass: 83.798,
                valence_radius: 1.16,
                valence_electrons: 8,
                atom_radius: 1.10,
            });

            // Add elements from Rb (37) to Og (118)
            elements.insert("Rb".to_string(), Element {
                name: "Rubidium".to_string(),
                atomic_number: 37,
                mass: 85.468,
                valence_radius: 2.16,
                valence_electrons: 1,
                atom_radius: 2.35,
            });

            elements.insert("Sr".to_string(), Element {
                name: "Strontium".to_string(),
                atomic_number: 38,
                mass: 87.62,
                valence_radius: 1.91,
                valence_electrons: 2,
                atom_radius: 2.0,
            });

            elements.insert("Y".to_string(), Element {
                name: "Yttrium".to_string(),
                atomic_number: 39,
                mass: 88.906,
                valence_radius: 1.62,
                valence_electrons: 3,
                atom_radius: 1.8,
            });

            elements.insert("Zr".to_string(), Element {
                name: "Zirconium".to_string(),
                atomic_number: 40,
                mass: 91.224,
                valence_radius: 1.45,
                valence_electrons: 4,
                atom_radius: 1.6,
            });

            elements.insert("Nb".to_string(), Element {
                name: "Niobium".to_string(),
                atomic_number: 41,
                mass: 92.906,
                valence_radius: 1.34,
                valence_electrons: 5,
                atom_radius: 1.45,
            });

            elements.insert("Mo".to_string(), Element {
                name: "Molybdenum".to_string(),
                atomic_number: 42,
                mass: 95.95,
                valence_radius: 1.30,
                valence_electrons: 6,
                atom_radius: 1.4,
            });

            elements.insert("Tc".to_string(), Element {
                name: "Technetium".to_string(),
                atomic_number: 43,
                mass: 98.0,
                valence_radius: 1.27,
                valence_electrons: 7,
                atom_radius: 1.35,
            });

            elements.insert("Ru".to_string(), Element {
                name: "Ruthenium".to_string(),
                atomic_number: 44,
                mass: 101.07,
                valence_radius: 1.25,
                valence_electrons: 8,
                atom_radius: 1.3,
            });

            elements.insert("Rh".to_string(), Element {
                name: "Rhodium".to_string(),
                atomic_number: 45,
                mass: 102.91,
                valence_radius: 1.25,
                valence_electrons: 9,
                atom_radius: 1.35,
            });

            elements.insert("Pd".to_string(), Element {
                name: "Palladium".to_string(),
                atomic_number: 46,
                mass: 106.42,
                valence_radius: 1.28,
                valence_electrons: 10,
                atom_radius: 1.4,
            });

            elements.insert("Ag".to_string(), Element {
                name: "Silver".to_string(),
                atomic_number: 47,
                mass: 107.87,
                valence_radius: 1.34,
                valence_electrons: 11,
                atom_radius: 1.6,
            });

            elements.insert("Cd".to_string(), Element {
                name: "Cadmium".to_string(),
                atomic_number: 48,
                mass: 112.41,
                valence_radius: 1.48,
                valence_electrons: 12,
                atom_radius: 1.55,
            });

            elements.insert("In".to_string(), Element {
                name: "Indium".to_string(),
                atomic_number: 49,
                mass: 114.82,
                valence_radius: 1.44,
                valence_electrons: 3,
                atom_radius: 1.55,
            });

            elements.insert("Sn".to_string(), Element {
                name: "Tin".to_string(),
                atomic_number: 50,
                mass: 118.71,
                valence_radius: 1.41,
                valence_electrons: 4,
                atom_radius: 1.45,
            });

            elements.insert("Sb".to_string(), Element {
                name: "Antimony".to_string(),
                atomic_number: 51,
                mass: 121.76,
                valence_radius: 1.40,
                valence_electrons: 5,
                atom_radius: 1.45,
            });

            elements.insert("Te".to_string(), Element {
                name: "Tellurium".to_string(),
                atomic_number: 52,
                mass: 127.60,
                valence_radius: 1.36,
                valence_electrons: 6,
                atom_radius: 1.4,
            });

            elements.insert("I".to_string(), Element {
                name: "Iodine".to_string(),
                atomic_number: 53,
                mass: 126.90,
                valence_radius: 1.33,
                valence_electrons: 7,
                atom_radius: 1.4,
            });

            elements.insert("Xe".to_string(), Element {
                name: "Xenon".to_string(),
                atomic_number: 54,
                mass: 131.29,
                valence_radius: 1.31,
                valence_electrons: 8,
                atom_radius: 1.3,
            });

            elements.insert("Cs".to_string(), Element {
                name: "Cesium".to_string(),
                atomic_number: 55,
                mass: 132.91,
                valence_radius: 2.35,
                valence_electrons: 1,
                atom_radius: 2.6,
            });

            elements.insert("Ba".to_string(), Element {
                name: "Barium".to_string(),
                atomic_number: 56,
                mass: 137.33,
                valence_radius: 1.98,
                valence_electrons: 2,
                atom_radius: 2.15,
            });
            
            elements.insert("La".to_string(), Element {
                name: "Lanthanum".to_string(),
                atomic_number: 57,
                mass: 138.91,
                valence_radius: 1.69,
                valence_electrons: 3,
                atom_radius: 1.95,
            });

            elements.insert("Ce".to_string(), Element {
                name: "Cerium".to_string(),
                atomic_number: 58,
                mass: 140.116,
                valence_radius: 1.65,
                valence_electrons: 4,
                atom_radius: 1.85,
            });

            elements.insert("Pr".to_string(), Element {
                name: "Praseodymium".to_string(),
                atomic_number: 59,
                mass: 140.908,
                valence_radius: 1.65,
                valence_electrons: 5,
                atom_radius: 1.85,
            });

            elements.insert("Nd".to_string(), Element {
                name: "Neodymium".to_string(),
                atomic_number: 60,
                mass: 144.242,
                valence_radius: 1.64,
                valence_electrons: 6,
                atom_radius: 1.85,
            });

            elements.insert("Pm".to_string(), Element {
                name: "Promethium".to_string(),
                atomic_number: 61,
                mass: 145.0,
                valence_radius: 1.63,
                valence_electrons: 7,
                atom_radius: 1.85,
            });

            elements.insert("Sm".to_string(), Element {
                name: "Samarium".to_string(),
                atomic_number: 62,
                mass: 150.36,
                valence_radius: 1.62,
                valence_electrons: 8,
                atom_radius: 1.85,
            });

            elements.insert("Eu".to_string(), Element {
                name: "Europium".to_string(),
                atomic_number: 63,
                mass: 151.964,
                valence_radius: 1.85,
                valence_electrons: 9,
                atom_radius: 1.85,
            });

            elements.insert("Gd".to_string(), Element {
                name: "Gadolinium".to_string(),
                atomic_number: 64,
                mass: 157.25,
                valence_radius: 1.61,
                valence_electrons: 10,
                atom_radius: 1.80,
            });

            elements.insert("Tb".to_string(), Element {
                name: "Terbium".to_string(),
                atomic_number: 65,
                mass: 158.925,
                valence_radius: 1.59,
                valence_electrons: 11,
                atom_radius: 1.75,
            });

            elements.insert("Dy".to_string(), Element {
                name: "Dysprosium".to_string(),
                atomic_number: 66,
                mass: 162.500,
                valence_radius: 1.59,
                valence_electrons: 12,
                atom_radius: 1.75,
            });

            elements.insert("Ho".to_string(), Element {
                name: "Holmium".to_string(),
                atomic_number: 67,
                mass: 164.930,
                valence_radius: 1.58,
                valence_electrons: 13,
                atom_radius: 1.75,
            });

            elements.insert("Er".to_string(), Element {
                name: "Erbium".to_string(),
                atomic_number: 68,
                mass: 167.259,
                valence_radius: 1.57,
                valence_electrons: 14,
                atom_radius: 1.75,
            });

            elements.insert("Tm".to_string(), Element {
                name: "Thulium".to_string(),
                atomic_number: 69,
                mass: 168.934,
                valence_radius: 1.56,
                valence_electrons: 15,
                atom_radius: 1.75,
            });

            elements.insert("Yb".to_string(), Element {
                name: "Ytterbium".to_string(),
                atomic_number: 70,
                mass: 173.054,
                valence_radius: 1.74,
                valence_electrons: 16,
                atom_radius: 1.75,
            });

            elements.insert("Lu".to_string(), Element {
                name: "Lutetium".to_string(),
                atomic_number: 71,
                mass: 174.967,
                valence_radius: 1.56,
                valence_electrons: 3,
                atom_radius: 1.75,
            });

            elements.insert("Hf".to_string(), Element {
                name: "Hafnium".to_string(),
                atomic_number: 72,
                mass: 178.49,
                valence_radius: 1.44,
                valence_electrons: 4,
                atom_radius: 1.55,
            });

            elements.insert("Ta".to_string(), Element {
                name: "Tantalum".to_string(),
                atomic_number: 73,
                mass: 180.948,
                valence_radius: 1.34,
                valence_electrons: 5,
                atom_radius: 1.45,
            });

            elements.insert("W".to_string(), Element {
                name: "Tungsten".to_string(),
                atomic_number: 74,
                mass: 183.84,
                valence_radius: 1.30,
                valence_electrons: 6,
                atom_radius: 1.35,
            });

            elements.insert("Re".to_string(), Element {
                name: "Rhenium".to_string(),
                atomic_number: 75,
                mass: 186.207,
                valence_radius: 1.28,
                valence_electrons: 7,
                atom_radius: 1.35,
            });

            elements.insert("Os".to_string(), Element {
                name: "Osmium".to_string(),
                atomic_number: 76,
                mass: 190.23,
                valence_radius: 1.26,
                valence_electrons: 8,
                atom_radius: 1.30,
            });

            elements.insert("Ir".to_string(), Element {
                name: "Iridium".to_string(),
                atomic_number: 77,
                mass: 192.217,
                valence_radius: 1.27,
                valence_electrons: 9,
                atom_radius: 1.35,
            });

            elements.insert("Pt".to_string(), Element {
                name: "Platinum".to_string(),
                atomic_number: 78,
                mass: 195.084,
                valence_radius: 1.30,
                valence_electrons: 10,
                atom_radius: 1.35,
            });

            elements.insert("Au".to_string(), Element {
                name: "Gold".to_string(),
                atomic_number: 79,
                mass: 196.967,
                valence_radius: 1.34,
                valence_electrons: 11,
                atom_radius: 1.35,
            });

            elements.insert("Hg".to_string(), Element {
                name: "Mercury".to_string(),
                atomic_number: 80,
                mass: 200.592,
                valence_radius: 1.49,
                valence_electrons: 12,
                atom_radius: 1.50,
            });

            elements.insert("Tl".to_string(), Element {
                name: "Thallium".to_string(),
                atomic_number: 81,
                mass: 204.38,
                valence_radius: 1.48,
                valence_electrons: 3,
                atom_radius: 1.90,
            });

            elements.insert("Pb".to_string(), Element {
                name: "Lead".to_string(),
                atomic_number: 82,
                mass: 207.2,
                valence_radius: 1.47,
                valence_electrons: 4,
                atom_radius: 1.80,
            });

            elements.insert("Bi".to_string(), Element {
                name: "Bismuth".to_string(),
                atomic_number: 83,
                mass: 208.980,
                valence_radius: 1.46,
                valence_electrons: 5,
                atom_radius: 1.60,
            });

            elements.insert("Po".to_string(), Element {
                name: "Polonium".to_string(),
                atomic_number: 84,
                mass: 209.0,
                valence_radius: 1.46,
                valence_electrons: 6,
                atom_radius: 1.50,
            });

            elements.insert("At".to_string(), Element {
                name: "Astatine".to_string(),
                atomic_number: 85,
                mass: 210.0,
                valence_radius: 1.45,
                valence_electrons: 7,
                atom_radius: 1.50,
            });

            elements.insert("Rn".to_string(), Element {
                name: "Radon".to_string(),
                atomic_number: 86,
                mass: 222.0,
                valence_radius: 1.43,
                valence_electrons: 8,
                atom_radius: 1.50,
            });

            elements.insert("Fr".to_string(), Element {
                name: "Francium".to_string(),
                atomic_number: 87,
                mass: 223.0,
                valence_radius: 2.5,
                valence_electrons: 1,
                atom_radius: 2.60,
            });

            elements.insert("Ra".to_string(), Element {
                name: "Radium".to_string(),
                atomic_number: 88,
                mass: 226.0,
                valence_radius: 2.1,
                valence_electrons: 2,
                atom_radius: 2.15,
            });

            elements.insert("Ac".to_string(), Element {
                name: "Actinium".to_string(),
                atomic_number: 89,
                mass: 227.0,
                valence_radius: 1.95,
                valence_electrons: 3,
                atom_radius: 1.95,
            });

            elements.insert("Th".to_string(), Element {
                name: "Thorium".to_string(),
                atomic_number: 90,
                mass: 232.038,
                valence_radius: 1.80,
                valence_electrons: 4,
                atom_radius: 1.80,
            });

            elements.insert("Pa".to_string(), Element {
                name: "Protactinium".to_string(),
                atomic_number: 91,
                mass: 231.036,
                valence_radius: 1.80,
                valence_electrons: 5,
                atom_radius: 1.80,
            });

            elements.insert("U".to_string(), Element {
                name: "Uranium".to_string(),
                atomic_number: 92,
                mass: 238.029,
                valence_radius: 1.75,
                valence_electrons: 6,
                atom_radius: 1.75,
            });

            elements.insert("Np".to_string(), Element {
                name: "Neptunium".to_string(),
                atomic_number: 93,
                mass: 237.0,
                valence_radius: 1.75,
                valence_electrons: 7,
                atom_radius: 1.75,
            });

            elements.insert("Pu".to_string(), Element {
                name: "Plutonium".to_string(),
                atomic_number: 94,
                mass: 244.0,
                valence_radius: 1.75,
                valence_electrons: 8,
                atom_radius: 1.75,
            });

            elements.insert("Am".to_string(), Element {
                name: "Americium".to_string(),
                atomic_number: 95,
                mass: 243.0,
                valence_radius: 1.75,
                valence_electrons: 9,
                atom_radius: 1.75,
            });

            elements.insert("Cm".to_string(), Element {
                name: "Curium".to_string(),
                atomic_number: 96,
                mass: 247.0,
                valence_radius: 1.75,
                valence_electrons: 10,
                atom_radius: 1.75,
            });

            elements.insert("Bk".to_string(), Element {
                name: "Berkelium".to_string(),
                atomic_number: 97,
                mass: 247.0,
                valence_radius: 1.75,
                valence_electrons: 11,
                atom_radius: 1.75,
            });

            elements.insert("Cf".to_string(), Element {
                name: "Californium".to_string(),
                atomic_number: 98,
                mass: 251.0,
                valence_radius: 1.75,
                valence_electrons: 12,
                atom_radius: 1.75,
            });

            elements.insert("Es".to_string(), Element {
                name: "Einsteinium".to_string(),
                atomic_number: 99,
                mass: 252.0,
                valence_radius: 1.75,
                valence_electrons: 13,
                atom_radius: 1.75,
            });

            elements.insert("Fm".to_string(), Element {
                name: "Fermium".to_string(),
                atomic_number: 100,
                mass: 257.0,
                valence_radius: 1.75,
                valence_electrons: 14,
                atom_radius: 1.75,
            });

            elements.insert("Md".to_string(), Element {
                name: "Mendelevium".to_string(),
                atomic_number: 101,
                mass: 258.0,
                valence_radius: 1.75,
                valence_electrons: 15,
                atom_radius: 1.75,
            });

            elements.insert("No".to_string(), Element {
                name: "Nobelium".to_string(),
                atomic_number: 102,
                mass: 259.0,
                valence_radius: 1.75,
                valence_electrons: 16,
                atom_radius: 1.75,
            });

            elements.insert("Lr".to_string(), Element {
                name: "Lawrencium".to_string(),
                atomic_number: 103,
                mass: 262.0,
                valence_radius: 1.75,
                valence_electrons: 3,
                atom_radius: 1.75,
            });

            elements.insert("Rf".to_string(), Element {
                name: "Rutherfordium".to_string(),
                atomic_number: 104,
                mass: 267.0,
                valence_radius: 1.7,
                valence_electrons: 4,
                atom_radius: 1.7,
            });

            elements.insert("Db".to_string(), Element {
                name: "Dubnium".to_string(),
                atomic_number: 105,
                mass: 268.0,
                valence_radius: 1.7,
                valence_electrons: 5,
                atom_radius: 1.7,
            });

            elements.insert("Sg".to_string(), Element {
                name: "Seaborgium".to_string(),
                atomic_number: 106,
                mass: 269.0,
                valence_radius: 1.7,
                valence_electrons: 6,
                atom_radius: 1.7,
            });

            elements.insert("Bh".to_string(), Element {
                name: "Bohrium".to_string(),
                atomic_number: 107,
                mass: 270.0,
                valence_radius: 1.7,
                valence_electrons: 7,
                atom_radius: 1.7,
            });

            elements.insert("Hs".to_string(), Element {
                name: "Hassium".to_string(),
                atomic_number: 108,
                mass: 277.0,
                valence_radius: 1.7,
                valence_electrons: 8,
                atom_radius: 1.7,
            });

            elements.insert("Mt".to_string(), Element {
                name: "Meitnerium".to_string(),
                atomic_number: 109,
                mass: 278.0,
                valence_radius: 1.7,
                valence_electrons: 9,
                atom_radius: 1.7,
            });

            elements.insert("Ds".to_string(), Element {
                name: "Darmstadtium".to_string(),
                atomic_number: 110,
                mass: 281.0,
                valence_radius: 1.7,
                valence_electrons: 10,
                atom_radius: 1.7,
            });

            elements.insert("Rg".to_string(), Element {
                name: "Roentgenium".to_string(),
                atomic_number: 111,
                mass: 282.0,
                valence_radius: 1.7,
                valence_electrons: 11,
                atom_radius: 1.7,
            });

            elements.insert("Cn".to_string(), Element {
                name: "Copernicium".to_string(),
                atomic_number: 112,
                mass: 285.0,
                valence_radius: 1.7,
                valence_electrons: 12,
                atom_radius: 1.7,
            });

            elements.insert("Nh".to_string(), Element {
                name: "Nihonium".to_string(),
                atomic_number: 113,
                mass: 286.0,
                valence_radius: 1.7,
                valence_electrons: 3,
                atom_radius: 1.7,
            });

            elements.insert("Fl".to_string(), Element {
                name: "Flerovium".to_string(),
                atomic_number: 114,
                mass: 289.0,
                valence_radius: 1.7,
                valence_electrons: 4,
                atom_radius: 1.7,
            });

            elements.insert("Mc".to_string(), Element {
                name: "Moscovium".to_string(),
                atomic_number: 115,
                mass: 290.0,
                valence_radius: 1.7,
                valence_electrons: 5,
                atom_radius: 1.7,
            });

            elements.insert("Lv".to_string(), Element {
                name: "Livermorium".to_string(),
                atomic_number: 116,
                mass: 293.0,
                valence_radius: 1.7,
                valence_electrons: 6,
                atom_radius: 1.7,
            });

            elements.insert("Ts".to_string(), Element {
                name: "Tennessine".to_string(),
                atomic_number: 117,
                mass: 294.0,
                valence_radius: 1.7,
                valence_electrons: 7,
                atom_radius: 1.7,
            });

            elements.insert("Og".to_string(), Element {
                name: "Oganesson".to_string(),
                atomic_number: 118,
                mass: 294.0,
                valence_radius: 1.6,
                valence_electrons: 8,
                atom_radius: 1.6,
            });

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
        assert_eq!(iron.atom_radius, 1.17);
    }

    #[test]
    fn test_periodic_table_missing_element() {
        let table = PeriodicTable::new();
        assert_eq!(table.get("NONEXIST"), None);
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