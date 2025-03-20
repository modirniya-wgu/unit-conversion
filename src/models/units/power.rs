use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The power category for units of power measurement
#[derive(Debug, Clone, Copy)]
pub struct PowerCategory;

impl Display for PowerCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Power")
    }
}

impl UnitCategory for PowerCategory {
    fn name(&self) -> &'static str {
        "power"
    }
    
    fn description(&self) -> &'static str {
        "Units of power measurement"
    }
}

// Shared implementation for all power units
macro_rules! impl_power_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(PowerCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(PowerCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = PowerCategory;
            
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

// Base unit: Watt (W)
impl_power_unit!(
    Watt, 
    "watt", 
    "W", 
    |value| value,           // to_base: direct conversion (base unit)
    |value| value            // from_base: direct conversion (base unit)
);

// Kilowatt (kW)
impl_power_unit!(
    Kilowatt, 
    "kilowatt", 
    "kW", 
    |value| value * 1000.0,  // to_base: 1 kW = 1,000 W
    |value| value / 1000.0   // from_base: 1 W = 0.001 kW
);

// Megawatt (MW)
impl_power_unit!(
    Megawatt, 
    "megawatt", 
    "MW", 
    |value| value * 1_000_000.0,  // to_base: 1 MW = 1,000,000 W
    |value| value / 1_000_000.0   // from_base: 1 W = 0.000001 MW
);

// Gigawatt (GW)
impl_power_unit!(
    Gigawatt, 
    "gigawatt", 
    "GW", 
    |value| value * 1_000_000_000.0,  // to_base: 1 GW = 1,000,000,000 W
    |value| value / 1_000_000_000.0   // from_base: 1 W = 1e-9 GW
);

// Horsepower (mechanical) (hp)
impl_power_unit!(
    Horsepower, 
    "horsepower", 
    "hp", 
    |value| value * 745.7,  // to_base: 1 hp = 745.7 W
    |value| value / 745.7   // from_base: 1 W = 0.00134 hp
);

// BTU per hour (BTU/h)
impl_power_unit!(
    BTUPerHour, 
    "btu per hour", 
    "BTU/h", 
    |value| value * 0.29307107,  // to_base: 1 BTU/h = 0.29307107 W
    |value| value / 0.29307107   // from_base: 1 W = 3.412142 BTU/h
);

// Foot-pound per second (ft⋅lb/s)
impl_power_unit!(
    FootPoundPerSecond, 
    "foot pound per second", 
    "ft⋅lb/s", 
    |value| value * 1.35582,  // to_base: 1 ft⋅lb/s = 1.35582 W
    |value| value / 1.35582   // from_base: 1 W = 0.737562 ft⋅lb/s
);

// Calorie per second (cal/s)
impl_power_unit!(
    CaloriePerSecond, 
    "calorie per second", 
    "cal/s", 
    |value| value * 4.184,  // to_base: 1 cal/s = 4.184 W
    |value| value / 4.184   // from_base: 1 W = 0.239 cal/s
);

// Joule per second (J/s) - same as Watt
impl_power_unit!(
    JoulePerSecond, 
    "joule per second", 
    "J/s", 
    |value| value,  // to_base: 1 J/s = 1 W
    |value| value   // from_base: 1 W = 1 J/s
);

// Kilocalorie per hour (kcal/h)
impl_power_unit!(
    KilocaloriePerHour, 
    "kilocalorie per hour", 
    "kcal/h", 
    |value| value * 1.163,  // to_base: 1 kcal/h = 1.163 W
    |value| value / 1.163   // from_base: 1 W = 0.860 kcal/h
); 