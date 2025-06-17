#[derive(PartialEq, Clone, Debug, Copy, Eq, Hash)]
pub enum Terrain {
    Plains,
    Hills,
    Mountains,
    DeepWater,
    ShallowWater,
}

pub(crate) type HexNeighbor = Option<usize>;

#[derive(PartialEq, Clone, Debug)]
pub struct Hex {
    pub terrain: Terrain,
    pub position: (i32, i32),
    pub neighbors: [HexNeighbor; 6],
}

#[cfg(test)]
mod tests {
    use super::{Hex, Terrain, HexNeighbor};

    #[test]
    fn test_hex_creation() {
        let hex = Hex {
            terrain: Terrain::Plains,
            position: (0, 0),
            neighbors: std::array::from_fn(|_| HexNeighbor::None),
        };
        assert_eq!(hex.terrain, Terrain::Plains);
        assert_eq!(hex.position, (0, 0));
        assert!(hex.neighbors.iter().all(|n| *n == HexNeighbor::None));
    }
}