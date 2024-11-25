use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HexType {
    DeepWater,
    ShallowWater,
    Plains,
    Hills,
    Mountains,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BattleHex {
    pub hex_type: HexType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub size: (u16, u16),
    pub hexes: Vec<BattleHex>,
}

impl Map {
    pub fn from_json(input: &str) -> Map {
        match serde_json::from_str(input) {
            Ok(map) => map,
            Err(e) => panic!("Failed to deserialize battle map: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hexx_test() {
        let rect = hexx::shapes::flat_rectangle([-1, 1, -1, 1]);
        assert_eq!(rect.size_hint().0, 9);
    }

    #[test]
    fn from_json_test() {
        let mapstr = r#"{
            "size":[3,3],
            "hexes":[
                {"hex_type":"DeepWater"},
                {"hex_type":"Plains"},
                {"hex_type":"ShallowWater"},
                {"hex_type":"Mountains"}, 
                {"hex_type":"Hills"}, 
                {"hex_type":"DeepWater"}, 
                {"hex_type":"DeepWater"}, 
                {"hex_type":"DeepWater"}, 
                {"hex_type":"DeepWater"}
            ]
        }"#;
        let map = Map::from_json(mapstr);
        assert_eq!(map.size, (3, 3));
        assert_eq!(map.hexes.len(), 9);
        assert_eq!(map.hexes[0].hex_type, HexType::DeepWater);
        assert_eq!(map.hexes[3].hex_type, HexType::Mountains);
    }
}
