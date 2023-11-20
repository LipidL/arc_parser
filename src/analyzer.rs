pub mod arc_analyzer{
    use crate::modules::structures:: StructureBlock;
    pub fn find_minimum_energy(blocks: Vec<StructureBlock>) -> Option<f64> {
    println!("Hello, world!");
        blocks.iter().fold(None, |min, b| match min {
            None => Some(b.energy),
            Some(min_energy) => Some(min_energy.min(b.energy)),
        })
    }
}