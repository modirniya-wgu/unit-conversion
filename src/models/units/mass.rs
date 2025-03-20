use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The mass category for units of mass measurement
#[derive(Debug, Clone, Copy)]
pub struct MassCategory;

impl Display for MassCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Mass")
    }
}

impl UnitCategory for MassCategory {
    fn name(&self) -> &'static str {
        "mass"
    }
    
    fn description(&self) -> &'static str {
        "Units of mass measurement"
    }
}

// Shared implementation for all mass units
macro_rules! impl_mass_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(MassCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(MassCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = MassCategory;
            
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

// Base unit: Kilogram (kg)
impl_mass_unit!(
    Kilogram, 
    "kilogram", 
    "kg", 
    |value| value,                 // to_base: 1 kg = 1 kg (base unit)
    |value| value                  // from_base: 1 kg = 1 kg (base unit)
);

// Gram (g)
impl_mass_unit!(
    Gram, 
    "gram", 
    "g", 
    |value| value / 1000.0,        // to_base: 1 g = 0.001 kg
    |value| value * 1000.0         // from_base: 1 kg = 1000 g
);

// Milligram (mg)
impl_mass_unit!(
    Milligram, 
    "milligram", 
    "mg", 
    |value| value / 1_000_000.0,   // to_base: 1 mg = 0.000001 kg
    |value| value * 1_000_000.0    // from_base: 1 kg = 1,000,000 mg
);

// Microgram (μg)
impl_mass_unit!(
    Microgram, 
    "microgram", 
    "μg", 
    |value| value / 1_000_000_000.0, // to_base: 1 μg = 0.000000001 kg
    |value| value * 1_000_000_000.0  // from_base: 1 kg = 1,000,000,000 μg
);

// Metric Ton/Tonne (t)
impl_mass_unit!(
    MetricTon, 
    "metric ton", 
    "t", 
    |value| value * 1000.0,        // to_base: 1 t = 1000 kg
    |value| value / 1000.0         // from_base: 1 kg = 0.001 t
);

// Pound (lb)
impl_mass_unit!(
    Pound, 
    "pound", 
    "lb", 
    |value| value * 0.45359237,    // to_base: 1 lb = 0.45359237 kg (exactly)
    |value| value / 0.45359237     // from_base: 1 kg = ~2.20462 lb
);

// Ounce (oz)
impl_mass_unit!(
    Ounce, 
    "ounce", 
    "oz", 
    |value| value * 0.028349523125, // to_base: 1 oz = 0.028349523125 kg
    |value| value / 0.028349523125  // from_base: 1 kg = ~35.274 oz
);

// Stone (st) - British unit
impl_mass_unit!(
    Stone, 
    "stone", 
    "st", 
    |value| value * 6.35029318,    // to_base: 1 st = 6.35029318 kg (14 lb)
    |value| value / 6.35029318     // from_base: 1 kg = ~0.157473 st
);

// US/Short Ton (US t)
impl_mass_unit!(
    USTon, 
    "US ton", 
    "US t", 
    |value| value * 907.18474,     // to_base: 1 US t = 907.18474 kg (2000 lb)
    |value| value / 907.18474      // from_base: 1 kg = ~0.00110231 US t
);

// Imperial/Long Ton (UK t)
impl_mass_unit!(
    ImperialTon, 
    "imperial ton", 
    "UK t", 
    |value| value * 1016.0469088,  // to_base: 1 UK t = 1016.0469088 kg (2240 lb)
    |value| value / 1016.0469088   // from_base: 1 kg = ~0.000984207 UK t
); 