use std::fmt::Debug;

use strum_macros::EnumCount;

pub struct Room {
    pub id: u16,
    pub room_type: RoomType,
}

impl Room {
    pub fn new(id: u16, room_type: RoomType) -> Room {
        Room {
            id,
            room_type,
        }
    }
}

impl Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, EnumCount)]
pub enum RoomType {
    Single,
    Double,
    Triple,
    Quad,
}

impl RoomType {
    pub fn get_work_units(&self) -> u8 {
        match self {
            RoomType::Single => 1,
            RoomType::Double => 2,
            RoomType::Triple => 3,
            RoomType::Quad => 4,
        }
    }

    pub fn get_value(index: usize) -> Option<RoomType> {
        match index {
            0 => Some(RoomType::Single),
            1 => Some(RoomType::Double),
            2 => Some(RoomType::Triple),
            3 => Some(RoomType::Quad),
            _ => None,
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            RoomType::Single => 0,
            RoomType::Double => 1,
            RoomType::Triple => 2,
            RoomType::Quad => 3,
        }
    }
}

impl PartialEq for RoomType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RoomType::Single, RoomType::Single) => true,
            (RoomType::Double, RoomType::Double) => true,
            (RoomType::Triple, RoomType::Triple) => true,
            (RoomType::Quad, RoomType::Quad) => true,
            (_, _) => false,
        }
    }
}