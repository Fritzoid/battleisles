use crate::hex::{Hex, HexNeighbor, Terrain};

#[derive(PartialEq, Clone, Debug)]
pub struct Map {
    pub rows: usize,
    pub collumns: usize,
    pub hexes: Vec<Hex>,
}

#[derive(Debug, PartialEq)]
pub enum MapErrors {
    CannotHaveAZeroDimension,
    IfOneColumnThenOnlyOneRow
}

impl std::fmt::Display for MapErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapErrors::CannotHaveAZeroDimension => write!(f, "Map cannot have a zero dimension"),
            MapErrors::IfOneColumnThenOnlyOneRow => write!(f, "If there is only one column, there can only be one row"),
        }
    }
}

impl Map {
    pub fn try_new(rows: usize, collumns: usize) -> Result<Self, MapErrors> {

        if rows == 0 || collumns == 0 {
            return Err(MapErrors::CannotHaveAZeroDimension);
        }

        if collumns == 1 && rows > 1 {
            return Err(MapErrors::IfOneColumnThenOnlyOneRow);
        }
        
        let total_hexes = rows * collumns - rows / 2;
        let mut hexes: Vec<Hex> = Vec::with_capacity(total_hexes as usize);
        let mut current_idx = 0;
        for row in 0..rows {
            let is_odd_row = row % 2 == 1;
            let cols_in_row = if is_odd_row { collumns - 1 } else { collumns };
            for col in 0..cols_in_row {
                let position = (row as i32, col as i32);
                let mut hex = Hex {
                    terrain: Terrain::DeepWater,
                    position,
                    neighbors: std::array::from_fn(|_| HexNeighbor::None),
                };
                match (row, col) {
                    (0, 0) => {},
                    (0, _) => {
                        hex.neighbors[4] = Some((col - 1) as usize);
                        hexes[current_idx - 1].neighbors[1] = Some(current_idx);
                    },
                    (_,0) => {
                        hex.neighbors[0] = Some((current_idx - ((collumns as usize) - 1)) as usize);
                        hexes[current_idx - ((collumns as usize) - 1)].neighbors[3] = Some(current_idx);
                        if is_odd_row {
                            hex.neighbors[5] = Some((current_idx - (collumns as usize)) as usize);
                            hexes[current_idx - (collumns as usize)].neighbors[2] = Some(current_idx);
                        }
                    },
                    (_, _) => {
                        if col != collumns - 1 {
                            hex.neighbors[0] = Some((current_idx - ((collumns as usize) - 1)) as usize);
                            hexes[(current_idx - ((collumns as usize) - 1)) as usize].neighbors[3] = Some(current_idx);
                        }
                        hex.neighbors[5] = Some(current_idx - (collumns as usize));
                        hexes[(current_idx - (collumns as usize)) as usize].neighbors[2] = Some(current_idx);
                        hex.neighbors[4] = Some((current_idx - 1) as usize);
                        hexes[current_idx - 1].neighbors[1] = Some(current_idx);
                    }
                }
                hexes.push(hex);
                current_idx += 1;
            }
        }
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
    use crate::hex::{Terrain};

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
    fn test_map_creation_positions(#[case] rows: usize, #[case] collumns: usize, #[case] expected_positions: &[(i32, i32)]) {
        let sut = Map::try_new(rows, collumns).unwrap();
        assert_eq!(sut.rows, rows);
        assert_eq!(sut.collumns, collumns);
        assert_eq!(sut.hexes.len(), (rows * collumns - rows / 2) as usize);
        assert!(sut.hexes.iter().all(|hex| hex.terrain == Terrain::DeepWater));
        assert_eq!(sut.hexes.len(), expected_positions.len());
        assert!(sut.hexes.iter().zip(expected_positions.iter()).all(|(hex, pos)| hex.position == *pos));
    }

    #[rstest]
    fn test_map_creation_neighbors() {
        let sut = Map::try_new(3, 3).unwrap();
        assert_eq!(sut.hexes[0].neighbors, [None, Some(1), Some(3), None, None, None]);
        assert_eq!(sut.hexes[1].neighbors, [None, Some(2), Some(4), Some(3), Some(0), None]);
        assert_eq!(sut.hexes[2].neighbors, [None, None, None, Some(4), Some(1), None]);
        assert_eq!(sut.hexes[3].neighbors, [Some(1), Some(4), Some(6), Some(5), None, Some(0)]);
        assert_eq!(sut.hexes[4].neighbors, [Some(2), None, Some(7), Some(6), Some(3), Some(1)]);
        assert_eq!(sut.hexes[5].neighbors, [Some(3), Some(6), None, None, None, None]);
        assert_eq!(sut.hexes[6].neighbors, [Some(4), Some(7), None, None, Some(5), Some(3)]);
        assert_eq!(sut.hexes[7].neighbors, [None, None, None, None, Some(6), Some(4)]);
    }
}