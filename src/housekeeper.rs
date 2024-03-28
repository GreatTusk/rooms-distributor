pub struct Housekeeper {
    pub shift: Shift,
    pub name: String,
    pub preferred_floor : char,
}

impl Housekeeper {
    pub fn new(shift: Shift, name: String, preferred_floor: char) -> Housekeeper {
        Housekeeper {
            shift,
            name,
            preferred_floor,
        }
    }
}

pub enum Shift {
    FullTime,
    PartTime,
}

impl PartialEq for Shift {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Shift::FullTime, Shift::FullTime) => true,
            (Shift::PartTime, Shift::PartTime) => true,
            (_, _) => false,
        }
    }
}

