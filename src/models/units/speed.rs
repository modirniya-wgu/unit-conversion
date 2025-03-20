use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The speed category for units of speed/velocity measurement
#[derive(Debug, Clone, Copy)]
pub struct SpeedCategory;

impl Display for SpeedCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Speed")
    }
}

impl UnitCategory for SpeedCategory {
    fn name(&self) -> &'static str {
        "speed"
    }
    
    fn description(&self) -> &'static str {
        "Units of speed/velocity measurement"
    }
}

// Shared implementation for all speed units
macro_rules! impl_speed_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(SpeedCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(SpeedCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = SpeedCategory;
            
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

// Base unit: Meter per second (m/s)
impl_speed_unit!(
    MeterPerSecond, 
    "meter per second", 
    "m/s", 
    |value| value,           // to_base: direct conversion (base unit)
    |value| value            // from_base: direct conversion (base unit)
);

// Kilometer per hour (km/h)
impl_speed_unit!(
    KilometerPerHour, 
    "kilometer per hour", 
    "km/h", 
    |value| value * 0.277778,  // to_base: 1 km/h = 0.277778 m/s
    |value| value / 0.277778   // from_base: 1 m/s = 3.6 km/h
);

// Miles per hour (mph)
impl_speed_unit!(
    MilePerHour, 
    "mile per hour", 
    "mph", 
    |value| value * 0.44704,  // to_base: 1 mph = 0.44704 m/s
    |value| value / 0.44704   // from_base: 1 m/s = 2.23694 mph
);

// Knot (kn)
impl_speed_unit!(
    Knot, 
    "knot", 
    "kn", 
    |value| value * 0.514444,  // to_base: 1 knot = 0.514444 m/s
    |value| value / 0.514444   // from_base: 1 m/s = 1.94384 knots
);

// Foot per second (ft/s)
impl_speed_unit!(
    FootPerSecond, 
    "foot per second", 
    "ft/s", 
    |value| value * 0.3048,  // to_base: 1 ft/s = 0.3048 m/s
    |value| value / 0.3048   // from_base: 1 m/s = 3.28084 ft/s
);

// Centimeter per second (cm/s)
impl_speed_unit!(
    CentimeterPerSecond, 
    "centimeter per second", 
    "cm/s", 
    |value| value * 0.01,  // to_base: 1 cm/s = 0.01 m/s
    |value| value / 0.01   // from_base: 1 m/s = 100 cm/s
);

// Mach (M) - at sea level, standard conditions
impl_speed_unit!(
    Mach, 
    "mach", 
    "M", 
    |value| value * 343.0,  // to_base: Mach 1 = 343 m/s (approx. at 20°C, sea level)
    |value| value / 343.0   // from_base: 1 m/s = 0.00291545 Mach
); 