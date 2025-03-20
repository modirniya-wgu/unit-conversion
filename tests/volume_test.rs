use unit_conversion_api::models::units::volume::*;
use unit_conversion_api::models::Measurement;
use unit_conversion_api::models::unit::Unit;

#[test]
fn test_volume_conversions() {
    // Create units
    let cubic_meter = CubicMeter::new();
    let liter = Liter::new();
    let ml = Milliliter::new();
    let cm3 = CubicCentimeter::new();
    let us_gallon = USGallon::new();
    let uk_gallon = UKGallon::new();
    let us_quart = USQuart::new();
    let us_pint = USPint::new();
    let us_cup = USCup::new();
    let us_fl_oz = USFluidOunce::new();
    let cubic_inch = CubicInch::new();
    let cubic_foot = CubicFoot::new();
    
    // Create measurements
    let one_cubic_meter = Measurement::new(1.0, cubic_meter);
    let thousand_liter = Measurement::new(1000.0, liter);
    let one_liter = Measurement::new(1.0, liter);
    let one_us_gallon = Measurement::new(1.0, us_gallon);
    
    // Test cubic meter to various units
    let m3_to_l = one_cubic_meter.convert_to(liter).unwrap();
    assert!((m3_to_l.value() - 1000.0).abs() < 0.0001);
    
    let m3_to_ml = one_cubic_meter.convert_to(ml).unwrap();
    assert!((m3_to_ml.value() - 1_000_000.0).abs() < 0.0001);
    
    let m3_to_cm3 = one_cubic_meter.convert_to(cm3).unwrap();
    assert!((m3_to_cm3.value() - 1_000_000.0).abs() < 0.0001);
    
    let m3_to_us_gal = one_cubic_meter.convert_to(us_gallon).unwrap();
    assert!((m3_to_us_gal.value() - 264.17).abs() < 0.01);
    
    let m3_to_uk_gal = one_cubic_meter.convert_to(uk_gallon).unwrap();
    assert!((m3_to_uk_gal.value() - 219.97).abs() < 0.01);
    
    let m3_to_ft3 = one_cubic_meter.convert_to(cubic_foot).unwrap();
    assert!((m3_to_ft3.value() - 35.31).abs() < 0.01);
    
    // Test liter to milliliter
    let l_to_ml = one_liter.convert_to(ml).unwrap();
    assert!((l_to_ml.value() - 1000.0).abs() < 0.0001);
    
    // Test liter to cubic centimeter (should be the same as milliliter)
    let l_to_cm3 = one_liter.convert_to(cm3).unwrap();
    assert!((l_to_cm3.value() - 1000.0).abs() < 0.0001);
    
    // Test US gallon to various units
    let us_gal_to_l = one_us_gallon.convert_to(liter).unwrap();
    assert!((us_gal_to_l.value() - 3.785).abs() < 0.001);
    
    let us_gal_to_quart = one_us_gallon.convert_to(us_quart).unwrap();
    assert!((us_gal_to_quart.value() - 4.0).abs() < 0.0001);
    
    let us_gal_to_pint = one_us_gallon.convert_to(us_pint).unwrap();
    assert!((us_gal_to_pint.value() - 8.0).abs() < 0.0001);
    
    let us_gal_to_cup = one_us_gallon.convert_to(us_cup).unwrap();
    assert!((us_gal_to_cup.value() - 16.0).abs() < 0.0001);
    
    let us_gal_to_fl_oz = one_us_gallon.convert_to(us_fl_oz).unwrap();
    assert!((us_gal_to_fl_oz.value() - 128.0).abs() < 0.0001);
    
    // Test cubic feet to cubic inches
    let one_cubic_foot = Measurement::new(1.0, cubic_foot);
    let ft3_to_in3 = one_cubic_foot.convert_to(cubic_inch).unwrap();
    let expected_value = 1728.0;
    let actual_value = ft3_to_in3.value();
    println!("Expected: {} in³, Actual: {} in³", expected_value, actual_value);
    assert!((actual_value - expected_value).abs() < 0.2, 
            "Expected approx {}, got {}", expected_value, actual_value);
}

#[test]
fn test_volume_equality() {
    let cubic_meter = CubicMeter::new();
    let liter = Liter::new();
    let cm3 = CubicCentimeter::new();
    let ml = Milliliter::new();
    
    // 1 m³ = 1000 L
    let one_cubic_meter = Measurement::new(1.0, cubic_meter);
    let thousand_liters = Measurement::new(1000.0, liter);
    assert_eq!(one_cubic_meter, thousand_liters);
    
    // 1 L = 1000 mL
    let one_liter = Measurement::new(1.0, liter);
    let thousand_ml = Measurement::new(1000.0, ml);
    assert_eq!(one_liter, thousand_ml);
    
    // 1 mL = 1 cm³
    let one_ml = Measurement::new(1.0, ml);
    let one_cm3 = Measurement::new(1.0, cm3);
    assert_eq!(one_ml, one_cm3);
    
    // Non-equal values
    let two_cubic_meter = Measurement::new(2.0, cubic_meter);
    assert_ne!(one_cubic_meter, two_cubic_meter);
}

#[test]
fn test_volume_comparison() {
    let cubic_meter = CubicMeter::new();
    let liter = Liter::new();
    let us_gallon = USGallon::new();
    let us_quart = USQuart::new();
    
    let one_cubic_meter = Measurement::new(1.0, cubic_meter);
    let one_liter = Measurement::new(1.0, liter);      // 0.001 m³
    let one_us_gallon = Measurement::new(1.0, us_gallon); // ~0.00379 m³
    let five_us_quarts = Measurement::new(5.0, us_quart); // ~0.00473 m³
    
    assert!(one_cubic_meter > one_liter);
    assert!(one_cubic_meter > one_us_gallon);
    assert!(one_us_gallon < one_cubic_meter);
    assert!(one_us_gallon > one_liter);
    assert!(five_us_quarts > one_us_gallon);
}

#[test]
fn test_volume_arithmetic() {
    let cubic_meter = CubicMeter::new();
    let liter = Liter::new();
    let us_gallon = USGallon::new();
    
    let one_cubic_meter = Measurement::new(1.0, cubic_meter);
    let five_hundred_liters = Measurement::new(500.0, liter); // 0.5 m³
    let one_us_gallon = Measurement::new(1.0, us_gallon);    // ~0.00379 m³
    
    // Addition: 1m³ + 500L = 1.5m³
    let sum = one_cubic_meter.clone() + five_hundred_liters.clone();
    assert!((sum.value() - 1.5).abs() < 0.0001);
    assert_eq!(sum.unit().symbol(), "m³");
    
    // Addition: 1m³ + 1gal ≈ 1.00379m³
    let sum2 = one_cubic_meter.clone() + one_us_gallon.clone();
    assert!((sum2.value() - 1.00379).abs() < 0.0001);
    assert_eq!(sum2.unit().symbol(), "m³");
    
    // Subtraction: 1m³ - 500L = 0.5m³
    let diff = one_cubic_meter.clone() - five_hundred_liters.clone();
    assert!((diff.value() - 0.5).abs() < 0.0001);
    assert_eq!(diff.unit().symbol(), "m³");
    
    // Multiplication: 1m³ * 2 = 2m³
    let doubled = one_cubic_meter.clone() * 2.0;
    assert!((doubled.value() - 2.0).abs() < 0.0001);
    assert_eq!(doubled.unit().symbol(), "m³");
    
    // Division: 1m³ / 2 = 0.5m³
    let halved = one_cubic_meter.clone() / 2.0;
    assert!((halved.value() - 0.5).abs() < 0.0001);
    assert_eq!(halved.unit().symbol(), "m³");
} 