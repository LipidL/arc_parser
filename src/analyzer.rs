pub mod arc_analyzer{
    //! necessary functions for analyzing StructureBlock
    use std::cmp::Ordering;

    use crate::modules::structures::{ StructureBlock, Atom};
    
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
        let mut coordination = vec![0; block.atoms.len()];
        let threshold = 2.8; // table of atom radious haven't be implemented
        for i in 0..block.atoms.len(){
            for j in i+1..block.atoms.len(){
                let distance = distance(&block.atoms[i], &block.atoms[j]);
                if distance <= threshold{
                    coordination[i] += 1;
                    coordination[j] += 1;
                }
            }
        }
        coordination
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
}