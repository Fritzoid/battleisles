pub trait Move {
    fn relocate(&mut self, to: (i32, i32));
}

pub trait Damage {
    fn take_damage(&mut self, damage: i32);
}

pub struct Position {
    position: (i32, i32),
}

impl Position {
    pub(crate) fn new(arg: (i32, i32)) -> Self {
        Position {
            position: arg
        }
    }
}

pub struct HitPoints {
    hit_points: i32,
}

impl HitPoints {
    pub(crate) fn new(hit_points: i32) -> Self {
        HitPoints {
            hit_points
        }
    }
}

impl Move for Position {
    fn relocate(&mut self, to: (i32, i32)) {
        self.position = to;
    }
}

impl Damage for HitPoints {
    fn take_damage(&mut self, damage: i32) {
        self.hit_points -= damage;
    }
}

#[derive(Debug)]
pub enum Unit<Position: Move, HitPoints: Damage> {
    Tank(Position, HitPoints),
    SamSite(Position, HitPoints),
}

impl<Position: Move, HitPoints: Damage> Unit<Position, HitPoints> {
    pub fn relocate(&mut self, to: (i32, i32)) {
        match self {
            Unit::Tank(position, _) => position.relocate(to),
            Unit::SamSite(_, _) => {}
        }
    }

    pub fn take(&mut self, damage: i32) {
        match self {
            Unit::Tank(_, hit_points) => hit_points.take_damage(damage),
            Unit::SamSite(_, hit_points) => hit_points.take_damage(damage),
        }
    }
}
