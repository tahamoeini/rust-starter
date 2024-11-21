use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let normalized_minutes = (total_minutes % (24 * 60) + (24 * 60)) % (24 * 60);
        Clock {
            hours: normalized_minutes / 60,
            minutes: normalized_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

pub fn main() {
    let clock = Clock::new(10, 0).add_minutes(3);
    println!("{}", clock); // Output: 10:03

    let another_clock = clock.add_minutes(-70);
    println!("{}", another_clock); // Output: 08:53
}
