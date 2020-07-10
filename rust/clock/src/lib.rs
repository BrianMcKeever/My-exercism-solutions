use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        normalize(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        normalize(self.hours, self.minutes + minutes)
    }
}

impl std::string::ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl std::cmp::PartialOrd for Clock {
    fn partial_cmp(&self, other: &Clock) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self.hours < other.hours {
            return Some(Ordering::Less);
        }
        if self.hours > other.hours {
            return Some(Ordering::Greater);
        }
        if self.minutes < other.minutes {
            return Some(Ordering::Less);
        }
        //if self.minutes > other.minutes
        Some(Ordering::Greater)
    }
}

const HOURS_IN_DAY: i32 = 24;
const MINUTES_IN_HOUR: i32 = 60;

fn normalize(mut hours: i32, mut minutes: i32) -> Clock {
    if minutes < 0 {
        let mut hour_loss = minutes / MINUTES_IN_HOUR;
        if minutes % MINUTES_IN_HOUR != 0 {
            //We're adding an extra hour because when we do integer
            //division, we floor off the negative remainder.
            hour_loss -= 1;
        }
        hours += hour_loss;
        minutes -= hour_loss * MINUTES_IN_HOUR;
    } else if minutes >= MINUTES_IN_HOUR {
        hours += minutes / MINUTES_IN_HOUR;
        minutes = minutes % MINUTES_IN_HOUR;
    }
    if hours >= HOURS_IN_DAY {
        hours = hours % HOURS_IN_DAY;
    } else if hours < 0 {
        hours = HOURS_IN_DAY + hours % HOURS_IN_DAY;
        if hours == HOURS_IN_DAY {
            hours = 0;
        }
    }

    Clock { hours, minutes }
}
