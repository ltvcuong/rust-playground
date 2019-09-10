use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        const MINS_ONE_DAY: i32 = 24 * 60;
        let mins = (hours * 60 + minutes) % MINS_ONE_DAY;
        let mins = (mins + MINS_ONE_DAY) % MINS_ONE_DAY;
        Clock {
            hours: (mins / 60) % 24,
            minutes: mins % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0width$}:{:0width$}",
            self.hours,
            self.minutes,
            width = 2
        )
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0width$}:{:0width$}",
            self.hours,
            self.minutes,
            width = 2
        )
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
