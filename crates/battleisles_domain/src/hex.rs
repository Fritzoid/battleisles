use std::rc::Rc;

#[derive(PartialEq, Clone, Debug, Copy)]
pub(crate) enum Terrain {
    Plains,
    Hills,
    Mountains,
    DeepWater,
    ShallowWater,
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum HexNeighbor {
    None,
    Some(Rc<Hex>),
} 

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Hex {
    pub(crate) terrain: Terrain,
    pub(crate) position: (i32, i32),
    pub(crate) neighbors: [HexNeighbor; 6],
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