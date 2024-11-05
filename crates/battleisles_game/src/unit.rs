pub struct Unit {
    pub name: String,
    pub health: u16,
}

mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let unit = Unit {
            name: "Tank".to_string(),
            health: 100,
        };
        assert_eq!(unit.name, "Tank");
        assert_eq!(unit.health, 100);
    }
}