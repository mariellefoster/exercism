use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    min: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hr = self.min / 60;
        let min = self.min % 60;
        write!(f, "{:0>2}:{:0>2}", hr, min)
    }
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_clock = Clock { min: (hours * 60 + minutes).rem_euclid(24 * 60) };
        new_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_min = (self.min + minutes).rem_euclid(24 * 60);
        Clock::new(0, new_min)
    }

}
