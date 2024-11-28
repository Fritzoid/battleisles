use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize)]
pub struct HexPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Serialize, Deserialize, PartialEq, Debug)]
pub enum HexType {
    DeepWater,
    ShallowWater,
    Plains,
    Hills,
    Mountains,
}

#[derive(Bundle, Serialize, Deserialize)]
pub struct Hex {
    pub position: HexPosition,
    pub hex_type: HexType,
    #[serde(skip)]
    pub pbr_bundle: PbrBundle,
}

#[derive(Component, Serialize, Deserialize)]
pub struct MapSize {
    pub width: u16,
    pub height: u16,
}

#[derive(Component, Serialize, Deserialize)]
pub struct Hexes {
    pub hexes: Vec<Hex>,
}

#[derive(Bundle, Serialize, Deserialize)]
pub struct Map {
    pub size: MapSize,
    pub hexes: Hexes,
}

impl Map {
    pub fn new(size: (u16, u16), hexes: Vec<Hex>) -> Map {
        Map {
            size: MapSize {
                width: size.0,
                height: size.1,
            },
            hexes: Hexes { hexes },
        }
    }

    pub fn from_json(input: &str) -> Map {
        match serde_json::from_str(input) {
            Ok(map) => map,
            Err(e) => panic!("Failed to deserialize map: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json_test() {

        let mapstr = r#"{
            "size": {"width":3,"height":3},
            "hexes": {
                "hexes":[
                    {"position":{"x":0,"y":0},"hex_type":"DeepWater"},
                    {"position":{"x":0,"y":1},"hex_type":"Plains"},
                    {"position":{"x":0,"y":2},"hex_type":"ShallowWater"},
                    {"position":{"x":1,"y":0},"hex_type":"Mountains"},
                    {"position":{"x":1,"y":1},"hex_type":"Hills"},
                    {"position":{"x":1,"y":2},"hex_type":"DeepWater"},
                    {"position":{"x":2,"y":0},"hex_type":"DeepWater"},
                    {"position":{"x":2,"y":1},"hex_type":"DeepWater"},
                    {"position":{"x":2,"y":2},"hex_type":"DeepWater"}
                ]
            }
        }"#;

        let map = Map::from_json(mapstr);
        assert_eq!(map.size.width, 3);
        assert_eq!(map.size.height, 3);
        assert_eq!(map.hexes.hexes.len(), 9);
        assert_eq!(map.hexes.hexes[0].position.x, 0);
        assert_eq!(map.hexes.hexes[0].position.y, 0);
        assert_eq!(map.hexes.hexes[0].hex_type, HexType::DeepWater);
        assert_eq!(map.hexes.hexes[1].position.x, 0);
        assert_eq!(map.hexes.hexes[1].position.y, 1);
        assert_eq!(map.hexes.hexes[1].hex_type, HexType::Plains);
        assert_eq!(map.hexes.hexes[2].position.x, 0);
        assert_eq!(map.hexes.hexes[2].position.y, 2);
        assert_eq!(map.hexes.hexes[2].hex_type, HexType::ShallowWater);
    }
}
