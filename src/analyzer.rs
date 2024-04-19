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
        + count:u64, the number of blocks in a Vec<StructureBlock>
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
        + blocks:&Vec<StructureBlock>

    returns:
        + Option<StructureBlock>, Some(StructureBlock) if the minimum found; None if no minumum found.
     */
    pub fn extract_minimum(blocks: &Vec<StructureBlock>) -> Option<StructureBlock> {
        blocks.iter().min_by(|a, b| a.energy.partial_cmp(&b.energy).unwrap()).cloned()
    }

    /**
    rearrange the atoms in a Vec<StructureBlock>
     */
    pub fn rearrange_atoms<F>(block: &mut StructureBlock, compare: F)
    where
        F: Fn(&Atom, &Atom) -> Ordering,
    {
        block.atoms.sort_by(compare);
    }

    #[derive(Clone)]
    struct Plane {
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
            d: -(plane.a * coordinate.0 + plane.b * coordinate.1  + plane.c + coordinate.2)
        };
        new_plane
    }

    fn calculate_plain_distance(plane1:&Plane, plane2:&Plane) -> Result<f64, &'static str> {
        // Check if the planes are parallel
        if plane1.a / plane2.a == plane1.b / plane2.b && plane1.b / plane2.b == plane1.c / plane2.c {
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
        for atom in structure{
            let new_plane = calculate_b(&plane, &atom);
            planes.push(new_plane);
        }
        // Sort the planes vector based on the d value
        planes.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());

        let threshold = 0.3; // Change this to your desired threshold

        // Merge the elements of d difference less than the threshold
        let mut i = 0;
        while i < planes.len() - 1 {
            let distnace = match calculate_plain_distance(&planes[i], &planes[i+1]) {
                Ok(distance) => distance,
                Err(e) => return Err(e),
            };
            if distnace < threshold {
                // Merge the two planes here
                // This is a simple example where we average the a, b, c, and d values
                let merged_plane = Plane {
                    a: (planes[i].a + planes[i + 1].a) / 2.0,
                    b: (planes[i].b + planes[i + 1].b) / 2.0,
                    c: (planes[i].c + planes[i + 1].c) / 2.0,
                    d: (planes[i].d + planes[i + 1].d) / 2.0,
                };
        
                // Replace the current plane with the merged one and remove the next plane
                planes[i] = merged_plane;
                planes.remove(i + 1);
            } else {
                i += 1;
            }
        }

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