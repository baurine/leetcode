use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const ONE_HOUR_MINS: i32 = 60;
const ONE_DAY_MINS: i32 = ONE_HOUR_MINS * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_mins = (hours * ONE_HOUR_MINS + minutes) % ONE_DAY_MINS;
        if total_mins < 0 {
            total_mins += ONE_DAY_MINS;
        }

        Clock {
            hours: total_mins / ONE_HOUR_MINS,
            minutes: total_mins % ONE_HOUR_MINS,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    // pub fn to_string(&self) -> String {
    //     format!("{:02}:{:02}", self.hours, self.minutes)
    // }
}

// impl PartialEq for Clock {
//     fn eq(&self, other: &Self) -> bool {
//         self.to_string() == other.to_string()
//     }
// }

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
