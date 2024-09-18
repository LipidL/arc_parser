pub mod arc_analyzer{
    //! necessary functions for analyzing StructureBlock
    use std::cmp::Ordering;
    use nalgebra::{Const, Dyn, VecStorage};
    use itertools::Itertools;


    use crate::modules::periodic_table::PeriodicTable;
    use crate::modules::structures::{ StructureBlock, Atom};
    extern crate nalgebra as na;
    
    /**
    find the minimum energy of a given vector of StrucutreBlock
    return some(f64) if minimum found
    or None if not found(empty vector or others)
     */
    pub fn find_minimum_energy(blocks: &Vec<StructureBlock>) -> Option<f64> {
        blocks.iter().fold(None, |min, b| match min {
            None => Some(b.energy),
            Some(min_energy) => Some(min_energy.min(b.energy)),
        })
    }

    /**
     count the number of blocks in the file

     returns:
        + count:u64, the number of blocks in a `Vec<StructureBlock>`
     */
    pub fn count_strucutre_block(blocks: &Vec<StructureBlock>) -> u64{
        blocks.len() as u64
    }

    /**
    check if all of the blocks have the same atoms

    returns:
        + `true` if all blocks have the same atoms; 
        + `false` elsewise
     */
    pub fn check_atom_consistency(blocks: &Vec<StructureBlock>) -> Option<std::collections::HashMap<&String, i32>>{
        let mut atom_map = std::collections::HashMap::new();

        for block in blocks {
            let mut local_map = std::collections::HashMap::new();
            for atom in &block.atoms {
                *local_map.entry(&atom.element).or_insert(0) += 1;
            }

            if atom_map.is_empty() {
                atom_map = local_map;
            } else {
                if atom_map != local_map {
                    return None;
                }
            }
        }
        Some(atom_map)
    }
    /**
     strucutre for storing energy and count of a `Vec<StructureBlock>`
     */
    pub struct EnergyInfo{
        pub energy: f64,
        pub count: u64,
    }
    /**
    list different energy in a `Vec<StructureBlock>`.
    threshold setted to be 0.001
     */
    pub fn list_energy(blocks: &Vec<StructureBlock>) -> Vec<EnergyInfo>{
        let mut energy_list: Vec<EnergyInfo> = Vec::new();
        let threshold = 0.001;
        for block in blocks{
            let mut min_diff = f64::MAX;
            let mut min_index:Option<usize> = None;
            let current_energy = block.energy;
            for (i, info) in energy_list.iter_mut().enumerate(){
                let diff = (info.energy - current_energy).abs();
                if diff < threshold && diff < min_diff{
                    min_diff = diff;
                    min_index = Some(i)
                }
            }
            match min_index{
                Some(index) => energy_list[index].count += 1,
                None => energy_list.push(EnergyInfo { energy: current_energy, count: 1 }),
            }
        }
        energy_list
    }

    /**
    return the minimum structure block of the given `Vec<StructureBlock>`

    args:
        + `blocks:&Vec<StructureBlock>`

    returns:
        + `Option<StructureBlock>`, `Some(StructureBlock)` if the minimum found; `None` if no minumum found.
     */
    pub fn extract_minimum(blocks: &Vec<StructureBlock>) -> Option<StructureBlock> {
        blocks.iter().min_by(|a, b| a.energy.partial_cmp(&b.energy).unwrap()).cloned()
    }

    /**
    rearrange the atoms in a `Vec<StructureBlock>`
     */
    pub fn rearrange_atoms<F>(block: &mut StructureBlock, compare: F)
    where
        F: Fn(&Atom, &Atom) -> Ordering,
    {
        block.atoms.sort_by(compare);
    }

    /**
     calculate distance between two atoms

     $distance = \sqrt{(x_1-x_2)^2+(y_1-y_2)^2+(z_1-z_2)^2}$
     */
    fn distance(atom1: &Atom, atom2: &Atom) -> f64
    {
        ((atom1.coordinate.0-atom2.coordinate.0).powi(2) + (atom1.coordinate.1-atom2.coordinate.1).powi(2) + (atom1.coordinate.2-atom2.coordinate.2).powi(2)).sqrt()
    }

    /**
     calculate coordination number of atoms in the 

     # TODO
     + implement support on multiple elements
     + implement changable threshold
     + implement atom radious table for threshold calculation
     */
    pub fn calc_coordination(block:&StructureBlock) -> Vec<u64>
    {
        let periodic_table = PeriodicTable::new();
        let mut coordination = vec![0; block.atoms.len()];
        for i in 0..block.atoms.len(){
            for j in i+1..block.atoms.len(){
                let threshold = periodic_table.get(&block.atoms[i].element).unwrap().atom_radius + periodic_table.get(&block.atoms[j].element).unwrap().atom_radius + 0.3;
                let distance = distance(&block.atoms[i], &block.atoms[j]);
                if distance <= threshold{
                    coordination[i] += 1;
                    coordination[j] += 1;
                }
            }
        }
        coordination
    }

    /**
     calculate the coordination matrix of a `StructureBlock`
     */
    pub fn calc_coordination_matrix(block:&StructureBlock) -> na::Matrix<u64, Dyn, Dyn, VecStorage<u64, Dyn, Dyn>> {
        let mut matrix = na::Matrix::<u64, Dyn, Dyn, VecStorage<u64, Dyn, Dyn>>::zeros(block.atoms.len(), block.atoms.len());
        let periodic_table = PeriodicTable::new();
        for i in 0..block.atoms.len(){
            for j in i+1..block.atoms.len(){
                let threshold = periodic_table.get(&block.atoms[i].element).unwrap().atom_radius + periodic_table.get(&block.atoms[j].element).unwrap().atom_radius + 0.3;
                let distance = distance(&block.atoms[i], &block.atoms[j]);
                if distance <= threshold {
                    matrix[(i, j)] += 1;
                    matrix[(j, i)] += 1;
                }
            }
        }
        matrix 
    }

    #[derive(Clone)]
    #[derive(Debug)]
    struct Plane {
        // The equation of the plane is ax + by + cz + d = 0
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    }

    fn calculate_plane(a1:&Atom, a2:&Atom, a3:&Atom) -> Result<Plane, &'static str>{
        let v1 = a2 - a1;
        let v2 = a3 - a1;

        let a = v1.1 * v2.2 - v1.2 * v2.1;
        let b = v1.2 * v2.0 - v1.0 * v2.2;
        let c = v1.0 * v2.1 - v1.1 * v2.0;
        if a == 0.0 && b == 0.0 && c == 0.0{
            Err("The points are collinear")
        } else {
            let d = -(a * a1.coordinate.0 + b * a1.coordinate.1 + c * a1.coordinate.2);
            Ok(Plane{a, b, c, d})
        }
    }

    fn calculate_b(plane:&Plane, atom:&Atom) -> Plane{
        let coordinate = &atom.coordinate;
        let new_plane = Plane{
            a: plane.a,
            b: plane.b,
            c: plane.c,
            d: -(plane.a * coordinate.0 + plane.b * coordinate.1  + plane.c * coordinate.2)
        };
        new_plane
    }

    fn calculate_distance_from_plane(plane:&Plane, point:&Atom) -> f64{
        let coordinate = &point.coordinate;
        let distance = (plane.a * coordinate.0 + plane.b * coordinate.1 + plane.c * coordinate.2 + plane.d) / f64::sqrt(plane.a.powi(2) + plane.b.powi(2) + plane.c.powi(2));
        distance.abs()
    }

    fn calculate_plain_distance(plane1:&Plane, plane2:&Plane) -> Result<f64, &'static str> {
        // Check if the planes are parallel
        let epsilon = 1e-5;

        if ((plane1.a + epsilon) / (plane2.a + epsilon) - (plane1.b + epsilon) / (plane2.b + epsilon)).abs() <= 1e-5 
            && ((plane1.b + epsilon) / (plane2.b + epsilon) - (plane1.c + epsilon) / (plane2.c + epsilon)).abs() <=1e-5 {
            // Calculate the distance between the planes
            let distance = (plane1.d - plane2.d).abs() / f64::sqrt(plane1.a.powi(2) + plane1.b.powi(2) + plane1.c.powi(2));
            Ok(distance)
        } else {
            Err("The planes are not parallel.")
        }
    }

    pub fn calculate_interplanar_spacing(structure:&Vec<Atom>, a1:usize, a2:usize, a3:usize) -> Result<Vec<f64>, &'static str>{
        // check if the provided atoms are present
        if a1 > structure.len() || a2 > structure.len() || a3 > structure.len() {
            return  Err("Atom number larger than lenth of structure");
        }
        // calculate the base plain
        let plane = calculate_plane(&structure[a1], &structure[a2], &structure[a3]).unwrap();
        // calculate planes that atoms sit on 
        let mut planes:Vec<Plane> = Vec::new();
        planes.push(plane.clone());
        let in_plane_threshold = 0.1;
        for atom in structure{
            let mut distances:Vec<f64> = Vec::new();
            for plane in &planes{
                let distance = calculate_distance_from_plane(plane, &atom);
                distances.push(distance);
            }
            let min_result = distances.iter().min_by(|a, b| a.partial_cmp(b).unwrap());
            if let Some(min) = min_result{
                if *min > in_plane_threshold{
                    let new_plane = calculate_b(&plane, &atom);
                    planes.push(new_plane);
                    continue;
                }
            }
            else{
            let new_plane = calculate_b(&plane, &atom);
            planes.push(new_plane);
            }
        }
        // Sort the planes vector based on the d value
        planes.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());
        println!("{:?}", planes.len());

        // calculate surface distances
        let mut distances = Vec::new();

        for i in 0..planes.len()-1 {
            match calculate_plain_distance(&planes[i], &planes[i+1]) {
                Ok(distance) => distances.push(distance),
                Err(e) => return Err(e),
            }
        }
        Ok(distances)
    }

    pub fn calculate_rmsd_by_matrix(structure1: &na::Matrix<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>, structure2: &na::Matrix<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>) -> f64 {
        let ncols = structure1.ncols();
        // the atom number should be the same
        assert!(ncols == structure2.ncols());
        // construct the mirrored and inversed structure2
        let mirrored_structure2 = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::from_fn(ncols, |i, j| {
            if i == 0 {
                -structure2[(i, j)]
            } else {
                structure2[(i, j)]
            }
        });
        let inversed_structure2 = structure2.clone().map(|x| -x);
        // create an iterator for all possible permutions of columns of stucutre1
        let mut minimum_rmsd = f64::MAX;
        let col_permutations = (0..ncols).permutations(ncols).collect::<Vec<_>>();
        col_permutations.iter().for_each(|perm| {
            let permuted_structure1 = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::from_fn(ncols, |i, j| {
                structure1[(i, perm[j])]
            });
            // calculate the center of mass of two structures
            let center_of_mass1 = permuted_structure1.column_sum() / ncols as f64;
            let center_of_mass2 = structure2.column_sum() / ncols as f64;
            let center_of_mass2_mirrored = mirrored_structure2.column_sum() / ncols as f64;
            let center_of_mass2_inversed = inversed_structure2.column_sum() / ncols as f64;
            // move the structures such that the center of mass is at the origin
            let mut moved_structure1 = permuted_structure1.clone();
            let mut moved_structure2 = structure2.clone();
            let mut moved_structure2_mirrored = mirrored_structure2.clone();
            let mut moved_structure2_inversed = inversed_structure2.clone();
            for i in 0..ncols {
                moved_structure1.column_mut(i).iter_mut().zip(center_of_mass1.iter()).for_each(|(x, y)| *x -= *y);
                moved_structure2.column_mut(i).iter_mut().zip(center_of_mass2.iter()).for_each(|(x, y)| *x -= *y);
                moved_structure2_mirrored.column_mut(i).iter_mut().zip(center_of_mass2_mirrored.iter()).for_each(|(x, y)| *x -= *y);
                moved_structure2_inversed.column_mut(i).iter_mut().zip(center_of_mass2_inversed.iter()).for_each(|(x, y)| *x -= *y);
            }
            // use the kabsch method to calculate the minimum RMSD
            for s2 in [moved_structure2, moved_structure2_mirrored, moved_structure2_inversed] {
                let h = moved_structure1.clone() * (s2.clone().transpose());
                let svd_result = h.svd(true, true);
                let v = svd_result.v_t.unwrap();
                let u = svd_result.u.unwrap();
                // check that the svd result is valid
                // let diagonal_s = na::Matrix::from_diagonal(&svd_result.singular_values);
                // let reconstructed_h = u * diagonal_s * v;
                // assert!((h - reconstructed_h).norm() < 1e-10);
                let d = u * v;
                let det = d.determinant();
                // calculate sign(det)
                let sign = if det > 0.0 { 1.0 } else { -1.0 };
                let s = na::Matrix::from_diagonal(&na::Vector3::new(1.0, 1.0, sign));
                let rotation_matrix = u * s * v;
                let rotated_s2 = rotation_matrix * s2.clone();
                // calculate the RMSD
                let rmsd = (rotated_s2 - moved_structure1.clone()).norm() / f64::sqrt(ncols as f64);
                if minimum_rmsd > rmsd {
                    minimum_rmsd = rmsd;
                }               
            }

        });
        minimum_rmsd
    }
}

#[cfg(test)]
mod tests{
    use crate::analyzer::arc_analyzer;
    use crate::modules::structures::{Atom, Coordinate, StructureBlock};
    use nalgebra::{self as na, Const, Dyn, VecStorage};

    #[test]
    fn test_find_minimum_energy() {
        // Test with a normal vector
        let blocks = vec![
            StructureBlock {
                number: 1,
                energy: 1.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 2,
                energy: 2.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 3,
                energy: 3.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
        ];
        assert!(arc_analyzer::find_minimum_energy(&blocks).unwrap() - 1.0 < 1e-6);
        // Test with an empty vector
        let empty_blocks: Vec<StructureBlock> = vec![];
        assert!(arc_analyzer::find_minimum_energy(&empty_blocks).is_none());
    }

    #[test]
    fn test_count_structure_block() {
        let blocks = vec![
            StructureBlock {
                number: 1,
                energy: 1.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 2,
                energy: 2.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 3,
                energy: 3.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
        ];
        assert_eq!(arc_analyzer::count_strucutre_block(&blocks), 3);
        assert_eq!(arc_analyzer::count_strucutre_block(&vec![]), 0);
    }

    #[test]
    fn test_check_atom_consistency() {
        // Test with a vector of blocks with the same atoms
        let blocks = vec![
            StructureBlock {
                number: 1,
                energy: 1.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 2,
                energy: 2.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 3,
                energy: 3.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
        ];
        assert!(arc_analyzer::check_atom_consistency(&blocks).is_some());
        // Test with a vector of blocks with different atoms
        let blocks = vec![
            StructureBlock {
                number: 1,
                energy: 1.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 2,
                energy: 2.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 3,
                energy: 3.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "O".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
        ];
        assert!(arc_analyzer::check_atom_consistency(&blocks).is_none());

        // Test with a vector of blocks with different number of atoms
        let blocks = vec![
            StructureBlock {
                number: 1,
                energy: 1.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 2,
                energy: 2.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 3,
                energy: 3.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![
                    Atom {
                        element: "Fe".to_string(),
                        coordinate: Coordinate(5.0, 5.0, 5.0),
                    },
                    Atom {
                        element: "Fe".to_string(),
                        coordinate: Coordinate(5.0, 5.0, 5.0),
                    },
                ],
            },
        ];
        assert!(arc_analyzer::check_atom_consistency(&blocks).is_none());

        // Test with an empty vector
        let empty_blocks: Vec<StructureBlock> = vec![];
        assert!(arc_analyzer::check_atom_consistency(&empty_blocks).is_some());
    }

    #[test]
    fn test_list_energy() {
        // Test with a normal vector
        let blocks = vec![
            StructureBlock {
                number: 1,
                energy: 1.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 2,
                energy: 1.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 3,
                energy: 2.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "O".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
        ];
        let energy_list = arc_analyzer::list_energy(&blocks);
        assert_eq!(energy_list.len(), 2);
        assert!(energy_list.iter().any(|info| info.energy - 1.0 < 1e-6 && info.count == 2));
        assert!(energy_list.iter().any(|info| info.energy - 2.0 < 1e-6 && info.count == 1));
        // Test with an empty vector
        let empty_blocks: Vec<StructureBlock> = vec![];
        let energy_list = arc_analyzer::list_energy(&empty_blocks);
        assert!(energy_list.is_empty());
    }

    #[test]
    fn test_extract_minimum() {
        // Test with a normal vector
        let blocks = vec![
            StructureBlock {
                number: 1,
                energy: 1.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 2,
                energy: 2.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
            StructureBlock {
                number: 3,
                energy: 3.0,
                symmetry: "P1".to_string(),
                crystal: crate::modules::structures::CrystalInfo {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
                    alpha: 90.0,
                    beta: 90.0,
                    gamma: 90.0,
                },
                atoms: vec![Atom {
                    element: "Fe".to_string(),
                    coordinate: Coordinate(5.0, 5.0, 5.0),
                }],
            },
        ];
        let min_block = arc_analyzer::extract_minimum(&blocks).unwrap();
        assert_eq!(min_block.energy, 1.0);
        // Test with an empty vector
        let empty_blocks: Vec<StructureBlock> = vec![];
        assert!(arc_analyzer::extract_minimum(&empty_blocks).is_none());
    }

    #[test]
    fn test_calculate_rmsd_by_matrix() {
        // Test with identical structures
        let structure1 = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::from_row_slice(&[
            1.0, 2.0, 3.0, 4.0, 
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
        ]);
        let structure2 = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::from_row_slice(&[
            1.0, 2.0, 3.0, 4.0, 
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
        ]);
        let rmsd = arc_analyzer::calculate_rmsd_by_matrix(&structure1, &structure2);
        assert!(rmsd < 1e-6);
        // Test with identical but mirrored structures
        let structure2 = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::from_row_slice(&[
            5.0, 6.0, 7.0, 8.0,
            1.0, 2.0, 3.0, 4.0, 
            9.0, 10.0, 11.0, 12.0,
        ]);
        let rmsd = arc_analyzer::calculate_rmsd_by_matrix(&structure1, &structure2);
        assert!(rmsd < 1e-6);
        // Test with identical but atom order reversed structures
        let structure2 = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::from_row_slice(&[
            1.0, 2.0, 4.0, 3.0, 
            5.0, 6.0, 8.0, 7.0,
            9.0, 10.0, 12.0, 11.0,
        ]);
        let rmsd = arc_analyzer::calculate_rmsd_by_matrix(&structure1, &structure2);
        assert!(rmsd < 1e-6);
        // Test with identical but inversed structures
        let structure2 = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::from_row_slice(&[
            -1.0, -2.0, -3.0, -4.0, 
            -5.0, -6.0, -7.0, -8.0,
            -9.0, -10.0, -11.0, -12.0,
        ]);
        let rmsd = arc_analyzer::calculate_rmsd_by_matrix(&structure1, &structure2);
        assert!(rmsd < 1e-6);
        // Test with identical but rotated structures
        let rotation = na::Matrix::<f64, Const<3>, Dyn, VecStorage<f64, Const<3>, Dyn>>::from_row_slice(&[
            1.0, 0.0, 0.0,
            0.0, 0.7071, -0.7071,
            0.0, 0.7071, 0.7071
        ]);
        let structure2 = rotation * structure1.clone();
        let rmsd = arc_analyzer::calculate_rmsd_by_matrix(&structure1, &structure2);
        assert!(rmsd < 1e-3);
    }
}