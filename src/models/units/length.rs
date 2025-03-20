use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The length category for units of length measurement
#[derive(Debug, Clone, Copy)]
pub struct LengthCategory;

impl Display for LengthCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Length")
    }
}

impl UnitCategory for LengthCategory {
    fn name(&self) -> &'static str {
        "length"
    }
    
    fn description(&self) -> &'static str {
        "Units of length measurement"
    }
}

// Shared implementation for all length units
macro_rules! impl_length_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(LengthCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(LengthCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = LengthCategory;
            
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

// Base unit: Meter (m)
impl_length_unit!(
    Meter, 
    "meter", 
    "m", 
    |value| value,                 // to_base: 1 meter = 1 meter (base unit)
    |value| value                  // from_base: 1 meter = 1 meter (base unit)
);

// Kilometer (km)
impl_length_unit!(
    Kilometer, 
    "kilometer", 
    "km", 
    |value| value * 1000.0,        // to_base: 1 km = 1000 m
    |value| value / 1000.0         // from_base: 1 m = 0.001 km
);

// Centimeter (cm)
impl_length_unit!(
    Centimeter, 
    "centimeter", 
    "cm", 
    |value| value / 100.0,         // to_base: 1 cm = 0.01 m
    |value| value * 100.0          // from_base: 1 m = 100 cm
);

// Millimeter (mm)
impl_length_unit!(
    Millimeter, 
    "millimeter", 
    "mm", 
    |value| value / 1000.0,        // to_base: 1 mm = 0.001 m
    |value| value * 1000.0         // from_base: 1 m = 1000 mm
);

// Micrometer (μm)
impl_length_unit!(
    Micrometer, 
    "micrometer", 
    "μm", 
    |value| value / 1_000_000.0,   // to_base: 1 μm = 0.000001 m
    |value| value * 1_000_000.0    // from_base: 1 m = 1,000,000 μm
);

// Nanometer (nm)
impl_length_unit!(
    Nanometer, 
    "nanometer", 
    "nm", 
    |value| value / 1_000_000_000.0, // to_base: 1 nm = 0.000000001 m
    |value| value * 1_000_000_000.0  // from_base: 1 m = 1,000,000,000 nm
);

// Inch (in)
impl_length_unit!(
    Inch, 
    "inch", 
    "in", 
    |value| value * 0.0254,        // to_base: 1 in = 0.0254 m
    |value| value / 0.0254         // from_base: 1 m = 39.3701 in
);

// Foot (ft)
impl_length_unit!(
    Foot, 
    "foot", 
    "ft", 
    |value| value * 0.3048,        // to_base: 1 ft = 0.3048 m (exactly)
    |value| value / 0.3048         // from_base: 1 m = ~3.28084 ft
);

// Yard (yd)
impl_length_unit!(
    Yard, 
    "yard", 
    "yd", 
    |value| value * 0.9144,        // to_base: 1 yd = 0.9144 m
    |value| value / 0.9144         // from_base: 1 m = 1.09361 yd
);

// Mile (mi)
impl_length_unit!(
    Mile, 
    "mile", 
    "mi", 
    |value| value * 1609.344,      // to_base: 1 mi = 1609.344 m
    |value| value / 1609.344       // from_base: 1 m = 0.000621371 mi
);

// Nautical Mile (nmi)
impl_length_unit!(
    NauticalMile, 
    "nautical mile", 
    "nmi", 
    |value| value * 1852.0,        // to_base: 1 nmi = 1852 m
    |value| value / 1852.0         // from_base: 1 m = 0.000539957 nmi
); 