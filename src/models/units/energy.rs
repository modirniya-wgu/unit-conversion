use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The energy category for units of energy measurement
#[derive(Debug, Clone, Copy)]
pub struct EnergyCategory;

impl Display for EnergyCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Energy")
    }
}

impl UnitCategory for EnergyCategory {
    fn name(&self) -> &'static str {
        "energy"
    }
    
    fn description(&self) -> &'static str {
        "Units of energy measurement"
    }
}

// Shared implementation for all energy units
macro_rules! impl_energy_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(EnergyCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(EnergyCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = EnergyCategory;
            
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

// Base unit: Joule (J)
impl_energy_unit!(
    Joule, 
    "joule", 
    "J", 
    |value| value,           // to_base: direct conversion (base unit)
    |value| value            // from_base: direct conversion (base unit)
);

// Kilojoule (kJ)
impl_energy_unit!(
    Kilojoule, 
    "kilojoule", 
    "kJ", 
    |value| value * 1000.0,  // to_base: 1 kJ = 1,000 J
    |value| value / 1000.0   // from_base: 1 J = 0.001 kJ
);

// Megajoule (MJ)
impl_energy_unit!(
    Megajoule, 
    "megajoule", 
    "MJ", 
    |value| value * 1_000_000.0,  // to_base: 1 MJ = 1,000,000 J
    |value| value / 1_000_000.0   // from_base: 1 J = 0.000001 MJ
);

// Calorie (cal)
impl_energy_unit!(
    Calorie, 
    "calorie", 
    "cal", 
    |value| value * 4.184,  // to_base: 1 cal = 4.184 J
    |value| value / 4.184   // from_base: 1 J = 0.239 cal
);

// Kilocalorie (kcal) - food calorie
impl_energy_unit!(
    Kilocalorie, 
    "kilocalorie", 
    "kcal", 
    |value| value * 4184.0,  // to_base: 1 kcal = 4,184 J
    |value| value / 4184.0   // from_base: 1 J = 0.000239 kcal
);

// British Thermal Unit (BTU)
impl_energy_unit!(
    BTU, 
    "british thermal unit", 
    "BTU", 
    |value| value * 1055.06,  // to_base: 1 BTU = 1,055.06 J
    |value| value / 1055.06   // from_base: 1 J = 0.000948 BTU
);

// Kilowatt-hour (kWh)
impl_energy_unit!(
    KilowattHour, 
    "kilowatt hour", 
    "kWh", 
    |value| value * 3_600_000.0,  // to_base: 1 kWh = 3,600,000 J
    |value| value / 3_600_000.0   // from_base: 1 J = 2.778e-7 kWh
);

// Watt-hour (Wh)
impl_energy_unit!(
    WattHour, 
    "watt hour", 
    "Wh", 
    |value| value * 3600.0,  // to_base: 1 Wh = 3,600 J
    |value| value / 3600.0   // from_base: 1 J = 0.000278 Wh
);

// Electron-volt (eV)
impl_energy_unit!(
    ElectronVolt, 
    "electron volt", 
    "eV", 
    |value| value * 1.602176634e-19,  // to_base: 1 eV = 1.602176634e-19 J
    |value| value / 1.602176634e-19   // from_base: 1 J = 6.241509e18 eV
);

// Therm (therm)
impl_energy_unit!(
    Therm, 
    "therm", 
    "therm", 
    |value| value * 105_505_585.257348,  // to_base: 1 therm = 100,000 BTU ≈ 105,505,585 J
    |value| value / 105_505_585.257348   // from_base: 1 J = 9.478e-9 therm
);

// Foot-pound (ft⋅lb)
impl_energy_unit!(
    FootPound, 
    "foot pound", 
    "ft⋅lb", 
    |value| value * 1.35582,  // to_base: 1 ft⋅lb = 1.35582 J
    |value| value / 1.35582   // from_base: 1 J = 0.737562 ft⋅lb
); 