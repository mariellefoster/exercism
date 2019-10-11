use std::fmt;

pub struct Clock {
    hr: i32,
    min: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hr, self.min)
    }
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_hours = hours % 24;
        let new_mins = minutes % 60;
        let new_clock = Clock { hr : new_hours, min : new_mins };
        new_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_min = (minutes + self.min) % 60;
        Clock::new(self.hr, new_min)
    }

}
