use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The temperature category for units of temperature measurement
#[derive(Debug, Clone, Copy)]
pub struct TemperatureCategory;

impl Display for TemperatureCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Temperature")
    }
}

impl UnitCategory for TemperatureCategory {
    fn name(&self) -> &'static str {
        "temperature"
    }
    
    fn description(&self) -> &'static str {
        "Units of temperature measurement"
    }
}

// Shared implementation for all temperature units
macro_rules! impl_temperature_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(TemperatureCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(TemperatureCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = TemperatureCategory;
            
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

// Base unit: Kelvin (K)
impl_temperature_unit!(
    Kelvin, 
    "kelvin", 
    "K", 
    |value| value,           // to_base: direct conversion (base unit)
    |value| value            // from_base: direct conversion (base unit)
);

// Celsius (°C)
impl_temperature_unit!(
    Celsius, 
    "celsius", 
    "°C", 
    |value| value + 273.15,  // to_base: °C = K - 273.15, so K = °C + 273.15
    |value| value - 273.15   // from_base: °C = K - 273.15
);

// Fahrenheit (°F)
impl_temperature_unit!(
    Fahrenheit, 
    "fahrenheit", 
    "°F", 
    |value| (value - 32.0) * 5.0/9.0 + 273.15,  // to_base: °F -> K
    |value| (value - 273.15) * 9.0/5.0 + 32.0   // from_base: K -> °F
);

// Rankine (°R)
impl_temperature_unit!(
    Rankine, 
    "rankine", 
    "°R", 
    |value| value * 5.0/9.0,  // to_base: K = °R × 5/9
    |value| value * 9.0/5.0   // from_base: °R = K × 9/5
);

// Réaumur (°Ré)
impl_temperature_unit!(
    Reaumur, 
    "reaumur", 
    "°Ré", 
    |value| value * 5.0/4.0 + 273.15,  // to_base: °Ré to K
    |value| (value - 273.15) * 4.0/5.0  // from_base: K to °Ré
); 