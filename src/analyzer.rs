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
    pub fn count_strucutre_block(blocks: &Vec<StructureBlock>) -> u64{
        blocks.len() as u64
    }
}