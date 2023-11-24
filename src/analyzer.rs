pub mod arc_analyzer{
    //! necessary functions for analyzing StructureBlock
    use crate::modules::structures:: StructureBlock;
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
     return an u64 representing the number of blocks
     */
    pub fn count_strucutre_block(blocks: &Vec<StructureBlock>) -> u64{
        blocks.len() as u64
    }

    /**
    check if all of the blocks have the same atoms
    return `true` if all blocks have the same atoms
    return `false` elsewise
     */
    pub fn check_atom_consistency(blocks: &Vec<StructureBlock>) -> bool{
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
                    return false;
                }
            }
        }
        true
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
        let mut min_diff = f64::MAX;
        let mut min_index:Option<usize> = None;
        for block in blocks{
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
}