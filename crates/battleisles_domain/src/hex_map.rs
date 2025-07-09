use hexx::Hex as HexxHex;
use hexx::{HexLayout};
use hexx::shapes;


#[derive(PartialEq, Clone, Debug)]
pub struct Hex {
    position: HexxHex,
    pub terrain: Terrain,
}

impl From<HexxHex> for Hex {
    fn from(hex: HexxHex) -> Self {
        Hex {
            position: hex,
            terrain: Terrain::DeepWater,
        }
    }
}

impl Hex {
    pub fn position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }
}

#[derive(Clone, Debug)]
pub struct HexMap {
    layout: HexLayout,
    pub hexes: Vec<Hex>,
}

impl HexMap {
    pub fn new() -> Self {
        let hex_size = 2.0;
        let layout = HexLayout::pointy().with_hex_size(hex_size);
        let hexes = shapes::pointy_rectangle([-4, 5, -4, 5])
            .map(|pos| Hex::from(hexx::Hex::new(pos.x, pos.y)))
            .collect::<Vec<Hex>>();
        HexMap { layout, hexes }
    }
    pub fn hex_size(&self) -> f32 {
        self.layout.scale.x
    }
    pub fn hex_to_world_pos(&self, hex: &Hex) -> (f32, f32) {
        let v = self.layout.hex_to_world_pos(hex.position);
        (v.x, v.y)
    }
} 

#[derive(PartialEq, Clone, Debug, Copy, Eq, Hash)]
pub enum Terrain {
    Plains,
    Hills,
    Mountains,
    DeepWater,
    ShallowWater,
}

#[cfg(test)]
mod tests {
    use super::*;
    use hexx::shapes;
    use hexx::Vec2;

    #[test]
    fn test_hex_creation() {
        let hex = Hex {
            position: hexx::Hex::new(0, 0),
            terrain: Terrain::Plains,
        };
        assert_eq!(hex.terrain, Terrain::Plains);
    }

    #[test]
    fn test_hex_map_creation() {
        let hex_size = 1.0;
        let layout = HexLayout::flat().with_hex_size(hex_size);
        let hexes = shapes::flat_rectangle([-4, 5, -4, 5])
            .map(|pos| Hex::from(hexx::Hex::new(pos.x, pos.y)))
            .collect::<Vec<Hex>>();
        let hex_map = HexMap {
            layout,
            hexes
        };
        assert_eq!(hex_map.layout.scale, Vec2::splat(hex_size));
        assert!(hex_map.hexes.len() == 100);
    }  
}