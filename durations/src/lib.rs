const SECS_PER_MINUTE: f64 = 60.0;
const SECS_PER_HOURS: f64 = 3600.0;
const SECS_PER_DAYS: f64 = 86400.0;
const SECS_PER_MONTHS: f64 = 2629746.0;
const SECS_PER_YEARS: f64 = 31556952.0;

pub struct TimeUnit {
    value: f64,
    unit: String,
}

impl TimeUnit {
    fn new(value: f64, unit: &str) -> Self {
        Self {
            value,
            unit: unit.to_string(),
        }
    }
}

impl TimeUnit {
    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn unit(&self) -> String {
        self.unit.to_string()
    }

    pub fn round_to(mut self, decimal: i32) -> Self {
        self.value = (self.value * (10 as f64).powi(decimal)).round() / (10 as f64).powi(decimal);
        self
    }

    pub fn as_string(&self) -> String {
        format!("{} {}", self.value, self.unit)
    }
}

pub struct Duration {
    seconds: f64,
}

impl Duration {
    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            seconds,
        }
    }
}

impl Duration {
    pub fn seconds(&self) -> TimeUnit {
        TimeUnit::new(self.seconds, "seconds")
    }

    pub fn minutes(&self) -> TimeUnit {
        TimeUnit::new(self.seconds / SECS_PER_MINUTE, "minutes")
    }

    pub fn hours(&self) -> TimeUnit {
        TimeUnit::new(self.seconds / SECS_PER_HOURS, "hours")
    }

    pub fn days(&self) -> TimeUnit {
        TimeUnit::new(self.seconds / SECS_PER_DAYS, "days")
    }

    pub fn months(&self) -> TimeUnit {
        TimeUnit::new(self.seconds / SECS_PER_MONTHS, "months")
    }

    pub fn years(&self) -> TimeUnit {
        TimeUnit::new(self.seconds / SECS_PER_YEARS, "years")
    }

    //Returns the smallest time unit (superior to 1) of a duration
    pub fn smallest_duration(&self) -> TimeUnit {
        if self.years().value > 1.0 {
            return self.years()
        } else if self.months().value > 1.0 {
            return self.months()
        } else if self.days().value > 1.0 {
            return self.days()
        } else if self.hours().value > 1.0 {
            return self.hours()
        } else if self.minutes().value > 1.0 {
            return self.minutes()
        } else {
            return self.seconds()
        }
    }
}