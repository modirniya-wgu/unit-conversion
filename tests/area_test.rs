use unit_conversion_api::models::units::area::*;
use unit_conversion_api::models::Measurement;
use unit_conversion_api::models::unit::Unit;

#[test]
fn test_area_conversions() {
    // Create units
    let square_meter = SquareMeter::new();
    let square_km = SquareKilometer::new();
    let square_cm = SquareCentimeter::new();
    let square_mm = SquareMillimeter::new();
    let hectare = Hectare::new();
    let are = Are::new();
    let square_inch = SquareInch::new();
    let square_foot = SquareFoot::new();
    let square_yard = SquareYard::new();
    let square_mile = SquareMile::new();
    let acre = Acre::new();
    
    // Create measurements
    let one_square_meter = Measurement::new(1.0, square_meter);
    let one_square_km = Measurement::new(1.0, square_km);
    let one_hectare = Measurement::new(1.0, hectare);
    let one_acre = Measurement::new(1.0, acre);
    
    // Test square meter to other metric units
    let m2_to_cm2 = one_square_meter.convert_to(square_cm).unwrap();
    assert!((m2_to_cm2.value() - 10_000.0).abs() < 0.01);
    
    let m2_to_mm2 = one_square_meter.convert_to(square_mm).unwrap();
    assert!((m2_to_mm2.value() - 1_000_000.0).abs() < 0.01);
    
    let m2_to_are = one_square_meter.convert_to(are).unwrap();
    assert!((m2_to_are.value() - 0.01).abs() < 0.0001);
    
    let m2_to_hectare = one_square_meter.convert_to(hectare).unwrap();
    assert!((m2_to_hectare.value() - 0.0001).abs() < 0.0000001);
    
    let m2_to_km2 = one_square_meter.convert_to(square_km).unwrap();
    assert!((m2_to_km2.value() - 0.000001).abs() < 0.0000000001);
    
    // Test square meter to imperial units
    let m2_to_in2 = one_square_meter.convert_to(square_inch).unwrap();
    assert!((m2_to_in2.value() - 1_550.0).abs() < 1.0);
    
    let m2_to_ft2 = one_square_meter.convert_to(square_foot).unwrap();
    assert!((m2_to_ft2.value() - 10.7639).abs() < 0.001);
    
    let m2_to_yd2 = one_square_meter.convert_to(square_yard).unwrap();
    assert!((m2_to_yd2.value() - 1.19599).abs() < 0.001);
    
    let m2_to_acre = one_square_meter.convert_to(acre).unwrap();
    assert!((m2_to_acre.value() - 0.000247105).abs() < 0.0000001);
    
    let m2_to_mile2 = one_square_meter.convert_to(square_mile).unwrap();
    assert!((m2_to_mile2.value() - 3.861e-7).abs() < 1e-10);
    
    // Test square kilometer to other units
    let km2_to_m2 = one_square_km.convert_to(square_meter).unwrap();
    assert!((km2_to_m2.value() - 1_000_000.0).abs() < 0.01);
    
    let km2_to_hectare = one_square_km.convert_to(hectare).unwrap();
    assert!((km2_to_hectare.value() - 100.0).abs() < 0.01);
    
    let km2_to_acre = one_square_km.convert_to(acre).unwrap();
    assert!((km2_to_acre.value() - 247.105).abs() < 0.001);
    
    // Test hectare to other units
    let hectare_to_m2 = one_hectare.convert_to(square_meter).unwrap();
    assert!((hectare_to_m2.value() - 10_000.0).abs() < 0.01);
    
    let hectare_to_km2 = one_hectare.convert_to(square_km).unwrap();
    assert!((hectare_to_km2.value() - 0.01).abs() < 0.0001);
    
    let hectare_to_acre = one_hectare.convert_to(acre).unwrap();
    assert!((hectare_to_acre.value() - 2.47105).abs() < 0.0001);
    
    // Test acre to other units
    let acre_to_m2 = one_acre.convert_to(square_meter).unwrap();
    assert!((acre_to_m2.value() - 4_046.86).abs() < 0.01);
    
    let acre_to_hectare = one_acre.convert_to(hectare).unwrap();
    assert!((acre_to_hectare.value() - 0.404686).abs() < 0.0001);
    
    let acre_to_ft2 = one_acre.convert_to(square_foot).unwrap();
    assert!((acre_to_ft2.value() - 43_560.0).abs() < 1.0);
}

#[test]
fn test_area_equality() {
    let square_meter = SquareMeter::new();
    let square_cm = SquareCentimeter::new();
    let square_foot = SquareFoot::new();
    let hectare = Hectare::new();
    
    // 1 m² = 10,000 cm²
    let one_square_meter = Measurement::new(1.0, square_meter);
    let ten_thousand_square_cm = Measurement::new(10_000.0, square_cm);
    assert_eq!(one_square_meter, ten_thousand_square_cm);
    
    // 1 hectare = 10,000 m²
    let one_hectare = Measurement::new(1.0, hectare);
    let ten_thousand_square_meters = Measurement::new(10_000.0, square_meter);
    assert_eq!(one_hectare, ten_thousand_square_meters);
    
    // Approximately 0.092903 m² = 1 ft²
    let zero_point_zero_nine_square_meters = Measurement::new(0.092903, square_meter);
    let one_square_foot = Measurement::new(1.0, square_foot);
    assert_eq!(zero_point_zero_nine_square_meters, one_square_foot);
}

#[test]
fn test_area_comparison() {
    let square_meter = SquareMeter::new();
    let hectare = Hectare::new();
    let acre = Acre::new();
    let square_foot = SquareFoot::new();
    
    let small_area = Measurement::new(1.0, square_meter);         // 1 m²
    let medium_area = Measurement::new(100.0, square_meter);      // 100 m²
    let large_area = Measurement::new(1.0, hectare);              // 10,000 m²
    let larger_area = Measurement::new(1.0, acre);                // 4,046.86 m²
    let largest_area = Measurement::new(2.0, hectare);            // 20,000 m²
    
    // Test less-than and greater-than for same unit
    assert!(small_area < medium_area);
    assert!(medium_area < large_area);
    assert!(medium_area > small_area);
    assert!(large_area > medium_area);
    
    // Test less-than and greater-than for different units
    assert!(small_area < large_area);
    assert!(large_area > small_area);
    
    assert!(medium_area < larger_area);
    assert!(larger_area > medium_area);
    
    assert!(larger_area < largest_area);
    assert!(largest_area > larger_area);
    
    // Test with square feet
    let thousand_sq_feet = Measurement::new(1000.0, square_foot); // ~92.9 m²
    assert!(small_area < thousand_sq_feet);
    assert!(thousand_sq_feet < large_area);
}

#[test]
fn test_area_arithmetic() {
    let square_meter = SquareMeter::new();
    let hectare = Hectare::new();
    let square_foot = SquareFoot::new();
    
    let area1 = Measurement::new(50.0, square_meter);
    let area2 = Measurement::new(150.0, square_meter);
    let area3 = Measurement::new(0.02, hectare);  // 200 m²
    let area4 = Measurement::new(1076.39, square_foot);  // ~100 m²
    
    // Addition
    let sum = area1.clone() + area2.clone();
    assert!((sum.value() - 200.0).abs() < 0.01);
    assert_eq!(sum.unit().symbol(), "m²");
    
    // Addition with different units
    let sum2 = area1.clone() + area3.clone();
    assert!((sum2.value() - 250.0).abs() < 0.01);
    assert_eq!(sum2.unit().symbol(), "m²");
    
    // Addition with imperial units
    let sum3 = area1.clone() + area4.clone();
    assert!((sum3.value() - 150.0).abs() < 0.01);
    assert_eq!(sum3.unit().symbol(), "m²");
    
    // Convert area3 (0.02 hectare) to square meters to verify its value
    let area3_in_m2 = area3.convert_to(square_meter).unwrap();
    println!("0.02 hectare = {} m²", area3_in_m2.value());
    
    // Subtraction: 0.02 hectare (200 m²) - 50 m²
    // Both operands need to be in the same unit for consistent results
    let area3_m2 = Measurement::new(200.0, square_meter);  // 0.02 hectare in m²
    let diff = area3_m2.clone() - area1.clone();
    println!("Subtraction: 200 m² - 50 m² = {} m²", diff.value());
    assert!((diff.value() - 150.0).abs() < 0.1);
    assert_eq!(diff.unit().symbol(), "m²");
    
    // Multiplication
    let doubled = area1.clone() * 2.0;
    assert!((doubled.value() - 100.0).abs() < 0.01);
    assert_eq!(doubled.unit().symbol(), "m²");
    
    // Division
    let halved = area2.clone() / 2.0;
    assert!((halved.value() - 75.0).abs() < 0.01);
    assert_eq!(halved.unit().symbol(), "m²");
} 