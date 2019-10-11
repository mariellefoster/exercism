use std::fmt;

pub struct Clock {
    hr: i32,
    min: i32,
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hr, self.min)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.min == other.min
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hr, self.min)
    }
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_hours = ((hours % 24) + 24) % 24;
        let new_mins = ((minutes % 60) + 60) % 60;
        let new_clock = Clock { hr : new_hours, min : new_mins };
        new_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let add_hours = minutes / 60;
        let new_min = ((((minutes % 60) + self.min) % 60) + 60) % 60;
        let hours = ((add_hours + self.hr % 24) + 24) % 24;
        Clock::new(hours, new_min)
    }

}
