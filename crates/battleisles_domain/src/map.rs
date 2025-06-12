use crate::hex::{Hex, HexNeighbor, Terrain};

#[derive(PartialEq, Clone, Debug)]
struct Map {
    rows: u16,
    collumns: u16,
    hexes: Vec<Hex>,
}

#[derive(Debug, PartialEq)]
enum MapErrors {
    CannotHaveAZeroDimension,
    IfOneColumnThenOnlyOneRow
}

impl Map {
    pub fn try_new(rows: u16, collumns: u16) -> Result<Self, MapErrors> {
        if rows == 0 || collumns == 0 {
            return Err(MapErrors::CannotHaveAZeroDimension);
        }

        if collumns == 1 && rows > 1 {
            return Err(MapErrors::IfOneColumnThenOnlyOneRow);
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

    use rstest::*;
    use super::Map;
    use crate::hex::{Terrain, HexNeighbor};

    #[rstest]
    fn test_invalid_dimension_map_creation() {
        let sut = Map::try_new(0, 10);
        assert!(sut.is_err());
        assert_eq!(sut.err(), Some(super::MapErrors::CannotHaveAZeroDimension));
        let sut = Map::try_new(10, 0);
        assert!(sut.is_err());
        assert_eq!(sut.err(), Some(super::MapErrors::CannotHaveAZeroDimension));
    }

    #[rstest]
    fn test_only_one_column_only_one_row_map_creation() {
        let sut = Map::try_new(2, 1);
        assert!(sut.is_err());
        assert_eq!(sut.err(), Some(super::MapErrors::IfOneColumnThenOnlyOneRow));
    }

    #[rstest]
    #[case(3, 3, &[(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (2, 0), (2, 1), (2, 2)])]
    #[case(3, 4, &[(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2), (2, 3)])]
    #[case(4, 3, &[(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (2, 0), (2, 1), (2, 2), (3, 0), (3, 1)])]
    fn test_map_creation(#[case] rows: u16, #[case] collumns: u16, #[case] expected_positions: &[(i32, i32)]) {
        let sut = Map::try_new(rows, collumns).unwrap();
        assert_eq!(sut.rows, rows);
        assert_eq!(sut.collumns, collumns);
        assert_eq!(sut.hexes.len(), (rows * collumns - rows / 2) as usize);
        assert!(sut.hexes.iter().all(|hex| hex.terrain == Terrain::DeepWater));
        assert!(sut.hexes.iter().all(|hex| hex.neighbors.iter().all(|n| *n == HexNeighbor::None)));
        for (i, pos) in expected_positions.iter().enumerate() {
            assert_eq!(sut.hexes[i].position, *pos);
        }
    }
}