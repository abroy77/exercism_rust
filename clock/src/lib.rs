use std::{cmp, fmt};

#[derive(Debug)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        let rollover_hours = minutes / 60;
        hours += rollover_hours;

        minutes = minutes % 60;
        if minutes < 0 {
            hours -= 1;
            minutes = 60 + minutes;
        }

        hours = hours % 24;
        if hours < 0 {
            hours = 24 + hours;
        }

        Clock {
            hours: hours as u8,
            minutes: minutes as u8,
        }
    }

    pub fn add_minutes(&self, mut minutes: i32) -> Self {
        let rollover_hours = (minutes + (self.minutes as i32)) / 60;
        minutes = (minutes + self.minutes as i32) % 60;
        let mut hours = self.hours as i32 + rollover_hours;

        if minutes < 0 {
            hours -= 1;
            minutes = 60 + minutes;
        }

        hours = hours % 24;
        if hours < 0 {
            hours = hours + 24;
        }
        Clock {
            hours: hours as u8,
            minutes: minutes as u8,
        }
    }
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }

    fn ne(&self, other: &Self) -> bool {
        self.hours != other.hours || self.minutes != other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
