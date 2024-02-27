use std::{fmt,cmp::Ordering};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

pub fn validate_clock(hours: i32, minutes: i32) -> (i32, i32) {
    let mut valid_hours = (hours + minutes.div_euclid(60)).rem_euclid(60);
    let valid_minutes = minutes.rem_euclid(60);
    match valid_hours.cmp(&12) {
        Ordering::Greater => valid_hours %= 12,
        Ordering::Equal => valid_hours = 0,
        Ordering::Less => {},
    }
    (valid_hours, valid_minutes)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (valid_hours, valid_minutes) = validate_clock(hours, minutes);
        Clock { 
            hours: valid_hours, 
            minutes: valid_minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_mins = self.minutes + minutes;
        Clock::new(self.hours, total_mins)
    }
}

fn main() {
    let clock = Clock::new(23, 40).add_minutes(20);
    println!("Clock says {0}", clock);
    println!("Clock says {0}", Clock::new(3, 30));
}
