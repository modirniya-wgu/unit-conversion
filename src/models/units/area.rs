use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The area category for units of area measurement
#[derive(Debug, Clone, Copy)]
pub struct AreaCategory;

impl Display for AreaCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Area")
    }
}

impl UnitCategory for AreaCategory {
    fn name(&self) -> &'static str {
        "area"
    }
    
    fn description(&self) -> &'static str {
        "Units of area measurement"
    }
}

// Shared implementation for all area units
macro_rules! impl_area_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(AreaCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(AreaCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = AreaCategory;
            
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

// Base unit: Square Meter (m²)
impl_area_unit!(
    SquareMeter, 
    "square meter", 
    "m²", 
    |value| value,           // to_base: direct conversion (base unit)
    |value| value            // from_base: direct conversion (base unit)
);

// Square Kilometer (km²)
impl_area_unit!(
    SquareKilometer, 
    "square kilometer", 
    "km²", 
    |value| value * 1_000_000.0,  // to_base: 1 km² = 1,000,000 m²
    |value| value / 1_000_000.0   // from_base: 1 m² = 0.000001 km²
);

// Square Centimeter (cm²)
impl_area_unit!(
    SquareCentimeter, 
    "square centimeter", 
    "cm²", 
    |value| value * 0.0001,  // to_base: 1 cm² = 0.0001 m²
    |value| value / 0.0001   // from_base: 1 m² = 10,000 cm²
);

// Square Millimeter (mm²)
impl_area_unit!(
    SquareMillimeter, 
    "square millimeter", 
    "mm²", 
    |value| value * 0.000001,  // to_base: 1 mm² = 0.000001 m²
    |value| value / 0.000001   // from_base: 1 m² = 1,000,000 mm²
);

// Hectare (ha)
impl_area_unit!(
    Hectare, 
    "hectare", 
    "ha", 
    |value| value * 10_000.0,  // to_base: 1 ha = 10,000 m²
    |value| value / 10_000.0   // from_base: 1 m² = 0.0001 ha
);

// Are (a)
impl_area_unit!(
    Are, 
    "are", 
    "a", 
    |value| value * 100.0,  // to_base: 1 a = 100 m²
    |value| value / 100.0   // from_base: 1 m² = 0.01 a
);

// Square Inch (in²)
impl_area_unit!(
    SquareInch, 
    "square inch", 
    "in²", 
    |value| value * 0.00064516,  // to_base: 1 in² = 0.00064516 m²
    |value| value / 0.00064516   // from_base: 1 m² = 1,550 in²
);

// Square Foot (ft²)
impl_area_unit!(
    SquareFoot, 
    "square foot", 
    "ft²", 
    |value| value * 0.092903,  // to_base: 1 ft² = 0.092903 m²
    |value| value / 0.092903   // from_base: 1 m² = 10.7639 ft²
);

// Square Yard (yd²)
impl_area_unit!(
    SquareYard, 
    "square yard", 
    "yd²", 
    |value| value * 0.836127,  // to_base: 1 yd² = 0.836127 m²
    |value| value / 0.836127   // from_base: 1 m² = 1.19599 yd²
);

// Square Mile (mi²)
impl_area_unit!(
    SquareMile, 
    "square mile", 
    "mi²", 
    |value| value * 2_589_988.0,  // to_base: 1 mi² = 2,589,988 m²
    |value| value / 2_589_988.0   // from_base: 1 m² = 3.861e-7 mi²
);

// Acre (ac)
impl_area_unit!(
    Acre, 
    "acre", 
    "ac", 
    |value| value * 4_046.86,  // to_base: 1 ac = 4,046.86 m²
    |value| value / 4_046.86   // from_base: 1 m² = 0.000247105 ac
); 