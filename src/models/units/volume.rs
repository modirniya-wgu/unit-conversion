use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::models::unit::{Unit, UnitCategory};

/// The volume category for units of volume measurement
#[derive(Debug, Clone, Copy)]
pub struct VolumeCategory;

impl Display for VolumeCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Volume")
    }
}

impl UnitCategory for VolumeCategory {
    fn name(&self) -> &'static str {
        "volume"
    }
    
    fn description(&self) -> &'static str {
        "Units of volume measurement"
    }
}

// Shared implementation for all volume units
macro_rules! impl_volume_unit {
    ($unit:ident, $name:expr, $symbol:expr, $to_base:expr, $from_base:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $unit(VolumeCategory);
        
        impl $unit {
            pub fn new() -> Self {
                Self(VolumeCategory)
            }
        }
        
        impl Display for $unit {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, $name)
            }
        }
        
        impl Unit for $unit {
            type Category = VolumeCategory;
            
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

// Base unit: Cubic Meter (m³)
impl_volume_unit!(
    CubicMeter, 
    "cubic meter", 
    "m³", 
    |value| value,                 // to_base: 1 m³ = 1 m³ (base unit)
    |value| value                  // from_base: 1 m³ = 1 m³ (base unit)
);

// Liter (L)
impl_volume_unit!(
    Liter, 
    "liter", 
    "L", 
    |value| value / 1000.0,        // to_base: 1 L = 0.001 m³
    |value| value * 1000.0         // from_base: 1 m³ = 1000 L
);

// Milliliter (mL)
impl_volume_unit!(
    Milliliter, 
    "milliliter", 
    "mL", 
    |value| value / 1_000_000.0,   // to_base: 1 mL = 0.000001 m³
    |value| value * 1_000_000.0    // from_base: 1 m³ = 1,000,000 mL
);

// Cubic Centimeter (cm³)
impl_volume_unit!(
    CubicCentimeter, 
    "cubic centimeter", 
    "cm³", 
    |value| value / 1_000_000.0,   // to_base: 1 cm³ = 0.000001 m³
    |value| value * 1_000_000.0    // from_base: 1 m³ = 1,000,000 cm³
);

// Cubic Decimeter (dm³)
impl_volume_unit!(
    CubicDecimeter, 
    "cubic decimeter", 
    "dm³", 
    |value| value / 1000.0,        // to_base: 1 dm³ = 0.001 m³ 
    |value| value * 1000.0         // from_base: 1 m³ = 1000 dm³
);

// US Gallon (gal)
impl_volume_unit!(
    USGallon, 
    "US gallon", 
    "gal", 
    |value| value * 0.00378541,    // to_base: 1 gal = 0.00378541 m³
    |value| value / 0.00378541     // from_base: 1 m³ = ~264.172 gal
);

// UK/Imperial Gallon (UK gal)
impl_volume_unit!(
    UKGallon, 
    "UK gallon", 
    "UK gal", 
    |value| value * 0.00454609,    // to_base: 1 UK gal = 0.00454609 m³
    |value| value / 0.00454609     // from_base: 1 m³ = ~219.969 UK gal
);

// US Quart (qt)
impl_volume_unit!(
    USQuart, 
    "US quart", 
    "qt", 
    |value| value * 0.000946353,   // to_base: 1 qt = 0.000946353 m³
    |value| value / 0.000946353    // from_base: 1 m³ = ~1056.69 qt
);

// US Pint (pt)
impl_volume_unit!(
    USPint, 
    "US pint", 
    "pt", 
    |value| value * 0.000473176,   // to_base: 1 pt = 0.000473176 m³
    |value| value / 0.000473176    // from_base: 1 m³ = ~2113.38 pt
);

// US Cup
impl_volume_unit!(
    USCup, 
    "US cup", 
    "cup", 
    |value| value * 0.000236588,   // to_base: 1 cup = 0.000236588 m³
    |value| value / 0.000236588    // from_base: 1 m³ = ~4226.75 cup
);

// US Fluid Ounce (fl oz)
impl_volume_unit!(
    USFluidOunce, 
    "US fluid ounce", 
    "fl oz", 
    |value| value * 0.0000295735,  // to_base: 1 fl oz = 0.0000295735 m³
    |value| value / 0.0000295735   // from_base: 1 m³ = ~33814 fl oz
);

// Cubic Inch (in³)
impl_volume_unit!(
    CubicInch, 
    "cubic inch", 
    "in³", 
    |value| value * 0.0000163871,  // to_base: 1 in³ = 0.0000163871 m³
    |value| value / 0.0000163871   // from_base: 1 m³ = ~61023.7 in³
);

// Cubic Foot (ft³)
impl_volume_unit!(
    CubicFoot, 
    "cubic foot", 
    "ft³", 
    |value| value * 0.0283168,     // to_base: 1 ft³ = 0.0283168 m³ (1728 in³)
    |value| value / 0.0283168      // from_base: 1 m³ = ~35.3147 ft³
);

// Cubic Yard (yd³)
impl_volume_unit!(
    CubicYard, 
    "cubic yard", 
    "yd³", 
    |value| value * 0.764555,      // to_base: 1 yd³ = 0.764555 m³
    |value| value / 0.764555       // from_base: 1 m³ = ~1.30795 yd³
); 