use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum HexType {
    Ocean,
    Water,
    Plain,
    Hill,
    Mountain,
}

#[derive(Serialize, Deserialize, Debug)]
struct BattleHex {
    hex_type: HexType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BattleMap {
    size: (u16,u16), 
    hexes: Vec<BattleHex>,
}

impl BattleMap {

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

    use hexx::Hex;

    use super::*;

    #[test]
    fn hexx_test() {
        let rect = hexx::shapes::flat_rectangle([-1, 1, -1, 1]);
        assert_eq!(rect.size_hint().0, 9);

        for (_,h) in rect.enumerate() {
            println!("x = {}, y = {}", h.x, h.y);
        }
        println!("--------");

        let hexa = hexx::shapes::hexagon(Hex::new(0, 0), 3);

        for (_,h) in hexa.enumerate() {
            println!("x = {}, y = {}", h.x, h.y);
        }
    }        

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
        assert_eq!(map.hexes[0].hex_type, HexType::Ocean);
        assert_eq!(map.hexes[3].hex_type, HexType::Mountain);
    }

    #[test]
    fn to_json_test() {
        let map = BattleMap {
            size: (1, 3),
            hexes: vec![
                BattleHex {
                    hex_type: HexType::Ocean
                },
                BattleHex {
                    hex_type: HexType::Plain
                },
                BattleHex {
                    hex_type: HexType::Water
                },
            ]
        };

        let json = r#"{"size":[1,3],"hexes":[{"hex_type":"Ocean"},{"hex_type":"Plain"},{"hex_type":"Water"}]}"#;
        let output = map.to_json();
        assert_eq!(output, json);
    }

}