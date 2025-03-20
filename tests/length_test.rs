use unit_conversion_api::models::units::length::*;
use unit_conversion_api::models::{Measurement};

#[test]
fn test_length_conversions() {
    // Create units
    let meter = Meter::new();
    let km = Kilometer::new();
    let cm = Centimeter::new();
    let mm = Millimeter::new();
    let inch = Inch::new();
    let foot = Foot::new();
    let yard = Yard::new();
    let mile = Mile::new();
    
    // Create measurements
    let one_meter = Measurement::new(1.0, meter);
    let one_km = Measurement::new(1.0, km);
    let one_hundred_cm = Measurement::new(100.0, cm);
    let one_thousand_mm = Measurement::new(1000.0, mm);
    let one_foot = Measurement::new(1.0, foot);
    
    // Test meter to various units
    let m_to_km = one_meter.convert_to(km).unwrap();
    assert!((m_to_km.value() - 0.001).abs() < f64::EPSILON);
    
    let m_to_cm = one_meter.convert_to(cm).unwrap();
    assert!((m_to_cm.value() - 100.0).abs() < f64::EPSILON);
    
    let m_to_mm = one_meter.convert_to(mm).unwrap();
    assert!((m_to_mm.value() - 1000.0).abs() < f64::EPSILON);
    
    let m_to_inch = one_meter.convert_to(inch).unwrap();
    assert!((m_to_inch.value() - 39.3701).abs() < 0.0001);
    
    let m_to_foot = one_meter.convert_to(foot).unwrap();
    assert!((m_to_foot.value() - 3.28084).abs() < 0.0001);
    
    let m_to_yard = one_meter.convert_to(yard).unwrap();
    assert!((m_to_yard.value() - 1.09361).abs() < 0.0001);
    
    // Test kilometer to meter
    let km_to_m = one_km.convert_to(meter).unwrap();
    assert!((km_to_m.value() - 1000.0).abs() < f64::EPSILON);
    
    // Test centimeter to meter
    let cm_to_m = one_hundred_cm.convert_to(meter).unwrap();
    assert!((cm_to_m.value() - 1.0).abs() < f64::EPSILON);
    
    // Test millimeter to meter
    let mm_to_m = one_thousand_mm.convert_to(meter).unwrap();
    assert!((mm_to_m.value() - 1.0).abs() < f64::EPSILON);
    
    // Test foot to inch (imperial unit to imperial unit)
    let foot_to_inch = one_foot.convert_to(inch).unwrap();
    assert!((foot_to_inch.value() - 12.0).abs() < 0.0001);
    
    // Test foot to meter to centimeter (multiple conversions)
    let foot_to_m = one_foot.convert_to(meter).unwrap();
    let foot_to_m_to_cm = foot_to_m.convert_to(cm).unwrap();
    assert!((foot_to_m_to_cm.value() - 30.48).abs() < 0.0001);
    
    // Test mile to kilometer
    let one_mile = Measurement::new(1.0, mile);
    let mile_to_km = one_mile.convert_to(km).unwrap();
    assert!((mile_to_km.value() - 1.609344).abs() < 0.0001);
}

#[test]
fn test_length_equality() {
    let meter = Meter::new();
    let cm = Centimeter::new();
    let inch = Inch::new();
    
    // 1 meter = 100 centimeters
    let one_meter = Measurement::new(1.0, meter);
    let hundred_cm = Measurement::new(100.0, cm);
    assert_eq!(one_meter, hundred_cm);
    
    // 1 inch = 2.54 centimeters
    let one_inch = Measurement::new(1.0, inch);
    let inch_in_cm = Measurement::new(2.54, cm);
    assert_eq!(one_inch, inch_in_cm);
    
    // Non-equal values
    let two_meters = Measurement::new(2.0, meter);
    assert_ne!(one_meter, two_meters);
}

#[test]
fn test_length_comparison() {
    let meter = Meter::new();
    let foot = Foot::new();
    let inch = Inch::new();
    
    let one_meter = Measurement::new(1.0, meter);
    let three_feet = Measurement::new(3.0, foot); // ~0.9144 meters
    let forty_inches = Measurement::new(40.0, inch); // ~1.016 meters
    
    assert!(one_meter > three_feet);
    assert!(one_meter < forty_inches);
    assert!(three_feet < forty_inches);
}

#[test]
fn test_length_arithmetic() {
    let meter = Meter::new();
    let cm = Centimeter::new();
    let inch = Inch::new();
    
    let one_meter = Measurement::new(1.0, meter);
    let fifty_cm = Measurement::new(50.0, cm); // 0.5 meters
    let ten_inches = Measurement::new(10.0, inch); // 0.254 meters
    
    // Addition: 1m + 50cm = 1.5m
    let sum = one_meter.clone() + fifty_cm.clone();
    assert!((sum.value() - 1.5).abs() < 0.0001);
    
    // Addition: 1m + 10in = 1.254m
    let sum2 = one_meter.clone() + ten_inches.clone();
    assert!((sum2.value() - 1.254).abs() < 0.0001);
    
    // Subtraction: 1m - 50cm = 0.5m
    let diff = one_meter.clone() - fifty_cm.clone();
    assert!((diff.value() - 0.5).abs() < 0.0001);
    
    // Multiplication: 1m * 2 = 2m
    let doubled = one_meter.clone() * 2.0;
    assert!((doubled.value() - 2.0).abs() < 0.0001);
    
    // Division: 1m / 2 = 0.5m
    let halved = one_meter.clone() / 2.0;
    assert!((halved.value() - 0.5).abs() < 0.0001);
} 