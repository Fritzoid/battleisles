use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Type {
    Ocean,
    Water,
    Plain,
    Hill,
    Mountain
}

#[derive(Serialize, Deserialize, Debug)]
struct Hex {
    hex_type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BattleMap {
    size: (u16,u16), 
    hexes: Vec<Hex>
}

impl BattleMap {

    pub fn new() -> BattleMap {
        let mut map = BattleMap {
            size: (3,3),
            hexes: Vec::new(),
        };

        map.hexes.push(Hex {
            hex_type: Type::Ocean,
        });
        map.hexes.push(Hex {
            hex_type: Type::Plain,
        });
        map.hexes.push(Hex {
            hex_type: Type::Plain,
        });
        map.hexes.push(Hex {
            hex_type: Type::Ocean,
        });
        map.hexes.push(Hex {
            hex_type: Type::Plain,
        });
        map.hexes.push(Hex {
            hex_type: Type::Plain,
        });
        map.hexes.push(Hex {
            hex_type: Type::Ocean,
        });
        map.hexes.push(Hex {
            hex_type: Type::Plain,
        });
        map.hexes.push(Hex {
            hex_type: Type::Plain,
        });
        map
    }

    pub fn from_json(input: &str) -> BattleMap {
        let battle_map = serde_json::from_str(input).unwrap();
        battle_map
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn from_json_test() {
        let mapstr =  r#"{
            "size":[3,3],
            "hexes":[
                {"hex_type":"Ocean"},
                {"hex_type":"Plain"},
                {"hex_type":"Water"},
                {"hex_type":"Mountain"}, 
                {"hex_type":"Ocean"}, 
                {"hex_type":"Ocean"}, 
                {"hex_type":"Ocean"}, 
                {"hex_type":"Ocean"}, 
                {"hex_type":"Ocean"}
            ]
        }"#;
        let map = BattleMap::from_json(mapstr);
        assert_eq!(map.size, (3,3));
        assert_eq!(map.hexes.len(), 9);
    }
}