use hexx::shapes;
use hexx::Hex;
use hexx::HexLayout;

#[derive(PartialEq, Clone, Debug)]
pub struct Tile {
    position: Hex,
    pub terrain: Terrain,
}

#[derive(Clone, Debug)]
pub struct Map {
    pub hex_size: f32,
    layout: HexLayout,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let hex_size = 1.0;
        let layout = HexLayout::pointy().with_hex_size(hex_size);
        if width == 0 || height == 0 {
            return Map {
                hex_size,
                layout,
                tiles: Vec::new(),
            };
        }

        let q_min = 0_i32;
        let q_max = width as i32 - 1;
        let r_min = 0_i32;
        let r_max = height as i32 - 1;
        let w = width as i32;

        // Generate equal rows with hexx, then drop the rightmost hex on odd rows.
        // Resulting tile count: width*height - height/2 (e.g., 10*10 - 5 = 95).
        let tiles = shapes::pointy_rectangle([q_min, q_max, r_min, r_max])
            .map(Hex::from)
            .filter(|h| {
                let q = h.x; // axial q
                let r = h.y; // axial r
                let is_odd = (r & 1) != 0;
                // axial (q,r) -> odd-r offset column
                let col = q + ((r - (r & 1)) / 2);
                let max_col = if is_odd { w - 2 } else { w - 1 };
                col >= 0 && col <= max_col
            })
            .map(|pos| Tile {
                position: pos,
                terrain: Terrain::DeepWater,
            })
            .collect::<Vec<Tile>>();
    Map { hex_size, layout, tiles }
    }

    pub fn tile_to_world_pos(&self, tile: &Tile) -> (f32, f32) {
    let pos = self.layout.hex_to_world_pos(tile.position);
    (pos.x as f32, pos.y as f32)
    }

    pub fn hex_size(&self) -> f32 {
        self.hex_size
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
    use rstest::rstest;

    #[test]
    fn test_tile_creation() {
        let sut = Tile {
            position: Hex::new(0, 0),
            terrain: Terrain::Plains,
        };
        assert_eq!(sut.position.x, 0);
        assert_eq!(sut.position.y, 0);
        assert_eq!(sut.terrain, Terrain::Plains);
    }

    #[rstest]
    #[case(10, 10, 95)]
    #[case(5, 5, 23)]
    fn test_map_creation(
        #[case] width: u32,
        #[case] height: u32,
        #[case] expected_tile_count: usize,
    ) {
        let sut = Map::new(width, height);
        assert!(sut.tiles.len() == expected_tile_count);
        sut.tiles.iter().for_each(|tile| {
            assert_eq!(tile.terrain, Terrain::DeepWater);
        });
    }
}
