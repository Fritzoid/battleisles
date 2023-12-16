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
pub struct BattleMap {
    pub size: (u16,u16), 
    pub hexes: Vec<BattleHex>,
}

impl BattleMap {

    pub fn new(size: (u16,u16)) -> BattleMap {
        let mut hexes = vec![BattleHex { hex_type: HexType::DeepWater }; (size.0 * size.1) as usize];
        hexes[4] = BattleHex { hex_type: HexType::Plains };
        BattleMap { size, hexes }
    }

    pub fn from_json(input: &str) -> BattleMap {
        match serde_json::from_str(input) {
            Ok(battle_map) => battle_map,
            Err(e) => panic!("Failed to deserialize battle map: {}", e)
        }
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(e) => panic!("Failed to serialize battle map: {}", e),
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
        let mapstr =  r#"{
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
        let map = BattleMap::from_json(mapstr);
        assert_eq!(map.size, (3,3));
        assert_eq!(map.hexes.len(), 9);
        assert_eq!(map.hexes[0].hex_type, HexType::DeepWater);
        assert_eq!(map.hexes[3].hex_type, HexType::Mountains);
    }

    #[test]
    fn to_json_test() {
        let map = BattleMap {
            size: (1, 5),
            hexes: vec![
                BattleHex {
                    hex_type: HexType::DeepWater
                },
                BattleHex {
                    hex_type: HexType::ShallowWater
                },
                BattleHex {
                    hex_type: HexType::Plains
                },
                BattleHex {
                    hex_type: HexType::Mountains
                },
                BattleHex {
                    hex_type: HexType::Hills
                },
            ]
        };

        let json = r#"{"size":[1,5],"hexes":[{"hex_type":"DeepWater"},{"hex_type":"ShallowWater"},{"hex_type":"Plains"},{"hex_type":"Mountains"},{"hex_type":"Hills"}]}"#;
        let output = map.to_json();
        assert_eq!(output, json);
    }

}