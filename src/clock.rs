pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> &mut Clock {
        self.minutes += minutes;
        self.hours += self.minutes / 60;
        self.minutes %= 60;
        self
    }
}

fn main() {
    let mut clock = Clock::new(10, 30);
    println!("Clock: {}:{}", clock.hours, clock.minutes);
    let clock = clock.add_minutes(75);
    println!("Clock: {}:{}", clock.hours, clock.minutes);
}