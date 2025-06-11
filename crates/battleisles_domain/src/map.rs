use crate::hex::{Hex, HexNeighbor, Terrain};

#[derive(PartialEq, Clone, Debug)]
struct Map {
    rows: u16,
    collumns: u16,
    hexes: Vec<Hex>,
}

#[derive(Debug, PartialEq)]
enum MapErrors {
    InvalidDimensions,
}

impl Map {
    pub fn try_new(rows: u16, collumns: u16) -> Result<Self, MapErrors> {
        if rows == 0 || collumns == 0 {
            return Err(MapErrors::InvalidDimensions);
        }
        
        let total_hexes = rows * collumns - rows / 2;
        let mut hexes = Vec::with_capacity(total_hexes as usize);
        let mut hex_count = 0;
        for row in 0..rows {
            let cols_in_row = if row % 2 == 1 && collumns > 0 { collumns - 1 } else { collumns };
            for col in 0..cols_in_row {
                let position = (row as i32, col as i32);
                let hex = Hex {
                    terrain: Terrain::DeepWater,
                    position,
                    neighbors: std::array::from_fn(|_| HexNeighbor::None),
                };
                hexes.push(hex);
                hex_count += 1;
            }
        }
        debug_assert_eq!(hex_count, total_hexes as usize);
        Ok(Map {
            rows,
            collumns,
            hexes,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::Map;
    use crate::hex::{Terrain, HexNeighbor};

    #[test]
    fn test_invalid_map_creation() {
        let invalid_map = Map::try_new(0, 10);
        assert!(invalid_map.is_err());
        assert_eq!(invalid_map.err(), Some(super::MapErrors::InvalidDimensions));
        let invalid_map = Map::try_new(10, 0);
        assert!(invalid_map.is_err());
        assert_eq!(invalid_map.err(), Some(super::MapErrors::InvalidDimensions));
    }

    #[test]
    fn test_map_creation() {
        for rows in 1..=10 {
            for collumns in 1..=10 {
                let sut = Map::try_new(rows, collumns).unwrap();
                assert_eq!(sut.rows, rows);
                assert_eq!(sut.collumns, collumns);
                assert_eq!(sut.hexes.len(), (rows * collumns - rows / 2) as usize);
                assert!(sut.hexes.iter().all(|hex| hex.terrain == Terrain::DeepWater));
                assert!(sut.hexes.iter().all(|hex| hex.neighbors.iter().all(|n| *n == HexNeighbor::None)));
                for (i, hex) in sut.hexes.iter().enumerate() {
                    let expected_position = (
                        (i as u16 / collumns) as i32,
                        (i as u16 % collumns) as i32,
                    );
                    assert_eq!(hex.position, expected_position);
                }
            }
        }
    }
}