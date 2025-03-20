use std::fmt::{Display, Formatter, Result as FmtResult};
use unit_conversion_api::models::{Unit, UnitCategory, Measurement};

// Create a simple test unit category
#[derive(Debug, Clone, Copy)]
struct LengthCategory;

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

// Create a test meter unit (base unit)
#[derive(Debug, Clone, Copy)]
struct Meter(LengthCategory);

impl Meter {
    fn new() -> Self {
        Self(LengthCategory)
    }
}

impl Display for Meter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "meter")
    }
}

impl Unit for Meter {
    type Category = LengthCategory;
    
    fn symbol(&self) -> &'static str {
        "m"
    }
    
    fn name(&self) -> &'static str {
        "meter"
    }
    
    fn category(&self) -> &Self::Category {
        &self.0
    }
    
    fn from_base(&self, value: f64) -> f64 {
        value // Meter is the base unit, so no conversion needed
    }
    
    fn to_base(&self, value: f64) -> f64 {
        value // Meter is the base unit, so no conversion needed
    }
}

// Create a test centimeter unit
#[derive(Debug, Clone, Copy)]
struct Centimeter(LengthCategory);

impl Centimeter {
    fn new() -> Self {
        Self(LengthCategory)
    }
}

impl Display for Centimeter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "centimeter")
    }
}

impl Unit for Centimeter {
    type Category = LengthCategory;
    
    fn symbol(&self) -> &'static str {
        "cm"
    }
    
    fn name(&self) -> &'static str {
        "centimeter"
    }
    
    fn category(&self) -> &Self::Category {
        &self.0
    }
    
    fn from_base(&self, value: f64) -> f64 {
        value * 100.0 // 1 meter = 100 centimeters
    }
    
    fn to_base(&self, value: f64) -> f64 {
        value / 100.0 // 100 centimeters = 1 meter
    }
}

#[test]
fn test_measurement_creation() {
    let meter = Meter::new();
    let centimeter = Centimeter::new();
    
    let length_m = Measurement::new(1.0, meter);
    let length_cm = Measurement::new(100.0, centimeter);
    
    assert_eq!(length_m.value(), 1.0);
    assert_eq!(length_cm.value(), 100.0);
}

#[test]
fn test_measurement_conversion() {
    let meter = Meter::new();
    let centimeter = Centimeter::new();
    
    let length_m = Measurement::new(1.0, meter);
    let length_cm = length_m.convert_to(centimeter).unwrap();
    
    assert_eq!(length_cm.value(), 100.0);
    
    let meter2 = Meter::new();
    let length_m2 = length_cm.convert_to(meter2).unwrap();
    
    assert_eq!(length_m2.value(), 1.0);
}

#[test]
fn test_measurement_comparison() {
    let meter = Meter::new();
    let centimeter = Centimeter::new();
    
    let one_meter = Measurement::new(1.0, meter);
    let hundred_cm = Measurement::new(100.0, centimeter);
    let fifty_cm = Measurement::new(50.0, centimeter);
    
    assert_eq!(one_meter, hundred_cm);
    assert!(one_meter > fifty_cm);
    assert!(fifty_cm < one_meter);
}

#[test]
fn test_measurement_arithmetic() {
    let meter = Meter::new();
    let centimeter = Centimeter::new();
    
    let one_meter = Measurement::new(1.0, meter);
    let fifty_cm = Measurement::new(50.0, centimeter);
    
    // Addition
    let sum = one_meter.clone() + fifty_cm.clone();
    assert_eq!(sum.value(), 1.5); // Result should be in meters
    
    // Subtraction
    let diff = one_meter.clone() - fifty_cm.clone();
    assert_eq!(diff.value(), 0.5); // Result should be in meters
    
    // Multiplication by scalar
    let doubled = one_meter.clone() * 2.0;
    assert_eq!(doubled.value(), 2.0);
    
    // Division by scalar
    let halved = one_meter.clone() / 2.0;
    assert_eq!(halved.value(), 0.5);
} 