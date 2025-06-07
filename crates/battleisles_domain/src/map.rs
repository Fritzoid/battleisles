#[derive(PartialEq, Clone, Debug, Copy)]
enum HexType {
    Plains,
    Hills,
    Mountains,
    DeepWater,
    ShallowWater,
}

#[derive(PartialEq, Clone, Debug, Copy)]
struct Hex {
    hex_type: HexType,
}

#[derive(PartialEq, Clone, Debug)]
struct Map {
    rows: u16,
    collumns: u16,
    hexes: Vec<Hex>,
}

#[derive(Debug, PartialEq)]
enum MapCreationErrors {
    InvalidDimensions,
}

impl Map {
    pub fn try_new(rows: u16, collumns: u16) -> Result<Self, MapCreationErrors> {
        if rows == 0 || collumns == 0 {
            return Err(MapCreationErrors::InvalidDimensions);
        }
        
        let total_hexes = rows * collumns - rows / 2 ;
        let hexes = vec![Hex {hex_type: HexType::DeepWater}; total_hexes as usize];
        Ok(Map {
            rows,
            collumns,
            hexes,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::{Map, HexType};

    #[test]
    fn test_invalid_map_creation() {
        let invalid_map = Map::try_new(0, 10);
        assert!(invalid_map.is_err());
        assert_eq!(invalid_map.err(), Some(super::MapCreationErrors::InvalidDimensions));
        let invalid_map = Map::try_new(10, 0);
        assert!(invalid_map.is_err());
        assert_eq!(invalid_map.err(), Some(super::MapCreationErrors::InvalidDimensions));
    }

    #[test]
    fn test_map_creation() {
        for rows in 1..=10 {
            for collumns in 1..=10 {
                let sut = Map::try_new(rows, collumns).unwrap();
                assert_eq!(sut.rows, rows);
                assert_eq!(sut.collumns, collumns);
                assert_eq!(sut.hexes.len(), (rows * collumns - rows / 2) as usize);
                assert!(sut.hexes.iter().all(|&hex| hex.hex_type == HexType::DeepWater));
            }
        }
    }
}