use unit_conversion_api::models::units::mass::*;
use unit_conversion_api::models::{Measurement};

#[test]
fn test_mass_conversions() {
    // Create units
    let kg = Kilogram::new();
    let g = Gram::new();
    let mg = Milligram::new();
    let t = MetricTon::new();
    let lb = Pound::new();
    let oz = Ounce::new();
    let st = Stone::new();
    
    // Create measurements
    let one_kg = Measurement::new(1.0, kg);
    let thousand_g = Measurement::new(1000.0, g);
    let million_mg = Measurement::new(1_000_000.0, mg);
    let one_lb = Measurement::new(1.0, lb);
    
    // Test kilogram to various units
    let kg_to_g = one_kg.convert_to(g).unwrap();
    assert!((kg_to_g.value() - 1000.0).abs() < 0.0001);
    
    let kg_to_mg = one_kg.convert_to(mg).unwrap();
    assert!((kg_to_mg.value() - 1_000_000.0).abs() < 0.0001);
    
    let kg_to_lb = one_kg.convert_to(lb).unwrap();
    assert!((kg_to_lb.value() - 2.20462).abs() < 0.0001);
    
    let kg_to_oz = one_kg.convert_to(oz).unwrap();
    assert!((kg_to_oz.value() - 35.274).abs() < 0.001);
    
    // Test gram to kilogram
    let g_to_kg = thousand_g.convert_to(kg).unwrap();
    assert!((g_to_kg.value() - 1.0).abs() < f64::EPSILON);
    
    // Test milligram to kilogram
    let mg_to_kg = million_mg.convert_to(kg).unwrap();
    assert!((mg_to_kg.value() - 1.0).abs() < f64::EPSILON);
    
    // Test pound to other imperial units (oz and stone)
    let lb_to_oz = one_lb.convert_to(oz).unwrap();
    assert!((lb_to_oz.value() - 16.0).abs() < 0.0001);
    
    let lb_to_st = Measurement::new(14.0, lb).convert_to(st).unwrap();
    assert!((lb_to_st.value() - 1.0).abs() < 0.0001);
    
    // Test ton to kilogram
    let one_ton = Measurement::new(1.0, t);
    let ton_to_kg = one_ton.convert_to(kg).unwrap();
    assert!((ton_to_kg.value() - 1000.0).abs() < 0.0001);
    
    // Test US ton to metric ton
    let one_us_ton = Measurement::new(1.0, USTon::new());
    let us_ton_to_metric = one_us_ton.convert_to(t).unwrap();
    assert!((us_ton_to_metric.value() - 0.90718474).abs() < 0.0001);
}

#[test]
fn test_mass_equality() {
    let kg = Kilogram::new();
    let g = Gram::new();
    let lb = Pound::new();
    
    // 1 kg = 1000 g
    let one_kg = Measurement::new(1.0, kg);
    let thousand_g = Measurement::new(1000.0, g);
    assert_eq!(one_kg, thousand_g);
    
    // 1 kg ≈ 2.20462 lb
    let one_kg = Measurement::new(1.0, kg);
    let kg_in_lb = Measurement::new(2.20462, lb);
    assert!((one_kg.to_base() - kg_in_lb.to_base()).abs() < 0.0001);
    
    // Non-equal values
    let two_kg = Measurement::new(2.0, kg);
    assert_ne!(one_kg, two_kg);
}

#[test]
fn test_mass_comparison() {
    let kg = Kilogram::new();
    let lb = Pound::new();
    let oz = Ounce::new();
    
    let one_kg = Measurement::new(1.0, kg);
    let one_lb = Measurement::new(1.0, lb); // ~0.45359237 kg
    let thirty_six_oz = Measurement::new(36.0, oz); // ~1.02 kg
    
    assert!(one_kg > one_lb);
    assert!(one_kg < thirty_six_oz);
    assert!(one_lb < thirty_six_oz);
}

#[test]
fn test_mass_arithmetic() {
    let kg = Kilogram::new();
    let g = Gram::new();
    let lb = Pound::new();
    
    let one_kg = Measurement::new(1.0, kg);
    let five_hundred_g = Measurement::new(500.0, g); // 0.5 kg
    let one_lb = Measurement::new(1.0, lb); // ~0.45359237 kg
    
    // Addition: 1kg + 500g = 1.5kg
    let sum = one_kg.clone() + five_hundred_g.clone();
    assert!((sum.value() - 1.5).abs() < 0.0001);
    
    // Addition: 1kg + 1lb = ~1.45359237kg
    let sum2 = one_kg.clone() + one_lb.clone();
    assert!((sum2.value() - 1.45359237).abs() < 0.0001);
    
    // Subtraction: 1kg - 500g = 0.5kg
    let diff = one_kg.clone() - five_hundred_g.clone();
    assert!((diff.value() - 0.5).abs() < 0.0001);
    
    // Multiplication: 1kg * 2 = 2kg
    let doubled = one_kg.clone() * 2.0;
    assert!((doubled.value() - 2.0).abs() < 0.0001);
    
    // Division: 1kg / 2 = 0.5kg
    let halved = one_kg.clone() / 2.0;
    assert!((halved.value() - 0.5).abs() < 0.0001);
} 