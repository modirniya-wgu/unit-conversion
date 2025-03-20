use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The time category for units of time measurement
#[derive(Debug, Clone, Copy)]
pub struct TimeCategory;

impl Display for TimeCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Time")
    }
}

impl UnitCategory for TimeCategory {
    fn name(&self) -> &'static str {
        "time"
    }
    
    fn description(&self) -> &'static str {
        "Units of time measurement"
    }
}

// Shared implementation for all time units
macro_rules! impl_time_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(TimeCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(TimeCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = TimeCategory;
            
            fn symbol(&self) -> &'static str {
                $symbol
            }
            
            fn name(&self) -> &'static str {
                $name
            }
            
            fn category(&self) -> &Self::Category {
                &self.0
            }
            
            fn to_base(&self, value: f64) -> f64 {
                $to_base(value)
            }
            
            fn from_base(&self, value: f64) -> f64 {
                $from_base(value)
            }
        }
    };
}

// Base unit: Second (s)
impl_time_unit!(
    Second, 
    "second", 
    "s", 
    |value| value,           // to_base: direct conversion (base unit)
    |value| value            // from_base: direct conversion (base unit)
);

// Millisecond (ms)
impl_time_unit!(
    Millisecond, 
    "millisecond", 
    "ms", 
    |value| value * 0.001,  // to_base: 1 ms = 0.001 s
    |value| value / 0.001   // from_base: 1 s = 1000 ms
);

// Microsecond (μs)
impl_time_unit!(
    Microsecond, 
    "microsecond", 
    "μs", 
    |value| value * 0.000001,  // to_base: 1 μs = 0.000001 s
    |value| value / 0.000001   // from_base: 1 s = 1,000,000 μs
);

// Nanosecond (ns)
impl_time_unit!(
    Nanosecond, 
    "nanosecond", 
    "ns", 
    |value| value * 0.000000001,  // to_base: 1 ns = 0.000000001 s
    |value| value / 0.000000001   // from_base: 1 s = 1,000,000,000 ns
);

// Minute (min)
impl_time_unit!(
    Minute, 
    "minute", 
    "min", 
    |value| value * 60.0,  // to_base: 1 min = 60 s
    |value| value / 60.0   // from_base: 1 s = 1/60 min
);

// Hour (h)
impl_time_unit!(
    Hour, 
    "hour", 
    "h", 
    |value| value * 3600.0,  // to_base: 1 h = 3600 s
    |value| value / 3600.0   // from_base: 1 s = 1/3600 h
);

// Day (d)
impl_time_unit!(
    Day, 
    "day", 
    "d", 
    |value| value * 86400.0,  // to_base: 1 d = 86400 s
    |value| value / 86400.0   // from_base: 1 s = 1/86400 d
);

// Week (wk)
impl_time_unit!(
    Week, 
    "week", 
    "wk", 
    |value| value * 604800.0,  // to_base: 1 week = 604800 s
    |value| value / 604800.0   // from_base: 1 s = 1/604800 week
);

// Month (avg, mo)
impl_time_unit!(
    Month, 
    "month", 
    "mo", 
    |value| value * 2628000.0,  // to_base: 1 month ≈ 30.4375 days ≈ 2628000 s
    |value| value / 2628000.0   // from_base: 1 s = 1/2628000 month
);

// Year (yr)
impl_time_unit!(
    Year, 
    "year", 
    "yr", 
    |value| value * 31536000.0,  // to_base: 1 year = 365 days = 31536000 s
    |value| value / 31536000.0   // from_base: 1 s = 1/31536000 year
);

// Decade (dec)
impl_time_unit!(
    Decade, 
    "decade", 
    "dec", 
    |value| value * 315360000.0,  // to_base: 1 decade = 10 years = 315360000 s
    |value| value / 315360000.0   // from_base: 1 s = 1/315360000 decade
);

// Century (c)
impl_time_unit!(
    Century, 
    "century", 
    "c", 
    |value| value * 3153600000.0,  // to_base: 1 century = 100 years = 3153600000 s
    |value| value / 3153600000.0   // from_base: 1 s = 1/3153600000 century
); 