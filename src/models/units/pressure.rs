use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The pressure category for units of pressure measurement
#[derive(Debug, Clone, Copy)]
pub struct PressureCategory;

impl Display for PressureCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Pressure")
    }
}

impl UnitCategory for PressureCategory {
    fn name(&self) -> &'static str {
        "pressure"
    }
    
    fn description(&self) -> &'static str {
        "Units of pressure measurement"
    }
}

// Shared implementation for all pressure units
macro_rules! impl_pressure_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(PressureCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(PressureCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = PressureCategory;
            
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

// Base unit: Pascal (Pa)
impl_pressure_unit!(
    Pascal, 
    "pascal", 
    "Pa", 
    |value| value,           // to_base: direct conversion (base unit)
    |value| value            // from_base: direct conversion (base unit)
);

// Kilopascal (kPa)
impl_pressure_unit!(
    Kilopascal, 
    "kilopascal", 
    "kPa", 
    |value| value * 1000.0,  // to_base: 1 kPa = 1,000 Pa
    |value| value / 1000.0   // from_base: 1 Pa = 0.001 kPa
);

// Megapascal (MPa)
impl_pressure_unit!(
    Megapascal, 
    "megapascal", 
    "MPa", 
    |value| value * 1_000_000.0,  // to_base: 1 MPa = 1,000,000 Pa
    |value| value / 1_000_000.0   // from_base: 1 Pa = 0.000001 MPa
);

// Bar (bar)
impl_pressure_unit!(
    Bar, 
    "bar", 
    "bar", 
    |value| value * 100_000.0,  // to_base: 1 bar = 100,000 Pa
    |value| value / 100_000.0   // from_base: 1 Pa = 0.00001 bar
);

// Millibar (mbar)
impl_pressure_unit!(
    Millibar, 
    "millibar", 
    "mbar", 
    |value| value * 100.0,  // to_base: 1 mbar = 100 Pa
    |value| value / 100.0   // from_base: 1 Pa = 0.01 mbar
);

// Atmosphere (atm)
impl_pressure_unit!(
    Atmosphere, 
    "atmosphere", 
    "atm", 
    |value| value * 101_325.0,  // to_base: 1 atm = 101,325 Pa
    |value| value / 101_325.0   // from_base: 1 Pa = 0.00000986923 atm
);

// Pounds per square inch (psi)
impl_pressure_unit!(
    PoundsPerSquareInch, 
    "pounds per square inch", 
    "psi", 
    |value| value * 6894.76,  // to_base: 1 psi = 6,894.76 Pa
    |value| value / 6894.76   // from_base: 1 Pa = 0.000145038 psi
);

// Torr (Torr) - almost equivalent to mmHg
impl_pressure_unit!(
    Torr, 
    "torr", 
    "Torr", 
    |value| value * 133.322,  // to_base: 1 Torr = 133.322 Pa
    |value| value / 133.322   // from_base: 1 Pa = 0.0075006 Torr
);

// Millimeters of mercury (mmHg)
impl_pressure_unit!(
    MillimetersOfMercury, 
    "millimeters of mercury", 
    "mmHg", 
    |value| value * 133.322,  // to_base: 1 mmHg = 133.322 Pa
    |value| value / 133.322   // from_base: 1 Pa = 0.0075006 mmHg
);

// Inches of mercury (inHg)
impl_pressure_unit!(
    InchesOfMercury, 
    "inches of mercury", 
    "inHg", 
    |value| value * 3386.39,  // to_base: 1 inHg = 3,386.39 Pa
    |value| value / 3386.39   // from_base: 1 Pa = 0.0002953 inHg
); 