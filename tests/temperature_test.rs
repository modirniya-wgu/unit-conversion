use unit_conversion_api::models::units::temperature::*;
use unit_conversion_api::models::Measurement;
use unit_conversion_api::models::unit::Unit;

#[test]
fn test_temperature_conversions() {
    // Create units
    let kelvin = Kelvin::new();
    let celsius = Celsius::new();
    let fahrenheit = Fahrenheit::new();
    let rankine = Rankine::new();
    let reaumur = Reaumur::new();
    
    // Create measurements
    let freezing_kelvin = Measurement::new(273.15, kelvin);
    let freezing_celsius = Measurement::new(0.0, celsius);
    let freezing_fahrenheit = Measurement::new(32.0, fahrenheit);
    
    let _boiling_kelvin = Measurement::new(373.15, kelvin);
    let boiling_celsius = Measurement::new(100.0, celsius);
    let boiling_fahrenheit = Measurement::new(212.0, fahrenheit);
    
    // Test Celsius to Kelvin
    let c_to_k = freezing_celsius.convert_to(kelvin).unwrap();
    assert!((c_to_k.value() - 273.15).abs() < 0.0001);
    
    // Test Kelvin to Celsius
    let k_to_c = freezing_kelvin.convert_to(celsius).unwrap();
    assert!((k_to_c.value() - 0.0).abs() < 0.0001);
    
    // Test Fahrenheit to Kelvin
    let f_to_k = freezing_fahrenheit.convert_to(kelvin).unwrap();
    assert!((f_to_k.value() - 273.15).abs() < 0.01);
    
    // Test Fahrenheit to Celsius
    let f_to_c = freezing_fahrenheit.convert_to(celsius).unwrap();
    assert!((f_to_c.value() - 0.0).abs() < 0.01);
    
    // Test Kelvin to Fahrenheit
    let k_to_f = freezing_kelvin.convert_to(fahrenheit).unwrap();
    assert!((k_to_f.value() - 32.0).abs() < 0.01);
    
    // Test Celsius to Fahrenheit
    let c_to_f = freezing_celsius.convert_to(fahrenheit).unwrap();
    assert!((c_to_f.value() - 32.0).abs() < 0.01);
    
    // Test boiling point conversions
    let boiling_c_to_f = boiling_celsius.convert_to(fahrenheit).unwrap();
    assert!((boiling_c_to_f.value() - 212.0).abs() < 0.01);
    
    let boiling_f_to_c = boiling_fahrenheit.convert_to(celsius).unwrap();
    assert!((boiling_f_to_c.value() - 100.0).abs() < 0.01);
    
    // Test Rankine conversions
    let absolute_zero_rankine = Measurement::new(0.0, rankine);
    let absolute_zero_kelvin = absolute_zero_rankine.convert_to(kelvin).unwrap();
    assert!((absolute_zero_kelvin.value() - 0.0).abs() < 0.0001);
    
    let room_temp_kelvin = Measurement::new(298.15, kelvin); // 25°C
    let room_temp_rankine = room_temp_kelvin.convert_to(rankine).unwrap();
    assert!((room_temp_rankine.value() - 536.67).abs() < 0.01);
    
    // Test Réaumur conversions
    let freezing_reaumur = Measurement::new(0.0, reaumur);
    let freezing_reaumur_to_c = freezing_reaumur.convert_to(celsius).unwrap();
    assert!((freezing_reaumur_to_c.value() - 0.0).abs() < 0.0001);
    
    let boiling_reaumur = Measurement::new(80.0, reaumur);
    let boiling_reaumur_to_c = boiling_reaumur.convert_to(celsius).unwrap();
    assert!((boiling_reaumur_to_c.value() - 100.0).abs() < 0.0001);
}

#[test]
fn test_temperature_equality() {
    let kelvin = Kelvin::new();
    let celsius = Celsius::new();
    let fahrenheit = Fahrenheit::new();
    
    // 0°C = 273.15K = 32°F
    let freezing_kelvin = Measurement::new(273.15, kelvin);
    let freezing_celsius = Measurement::new(0.0, celsius);
    let freezing_fahrenheit = Measurement::new(32.0, fahrenheit);
    
    assert_eq!(freezing_kelvin, freezing_celsius);
    assert_eq!(freezing_celsius, freezing_fahrenheit);
    assert_eq!(freezing_kelvin, freezing_fahrenheit);
    
    // 100°C = 373.15K = 212°F (boiling point of water)
    let boiling_kelvin = Measurement::new(373.15, kelvin);
    let boiling_celsius = Measurement::new(100.0, celsius);
    let boiling_fahrenheit = Measurement::new(212.0, fahrenheit);
    
    assert_eq!(boiling_kelvin, boiling_celsius);
    assert_eq!(boiling_celsius, boiling_fahrenheit);
    assert_eq!(boiling_kelvin, boiling_fahrenheit);
    
    // Non-equal values
    let room_temp_celsius = Measurement::new(20.0, celsius);
    assert_ne!(freezing_celsius, room_temp_celsius);
}

#[test]
fn test_temperature_comparison() {
    let kelvin = Kelvin::new();
    let celsius = Celsius::new();
    let fahrenheit = Fahrenheit::new();
    
    let temp1_kelvin = Measurement::new(283.15, kelvin);    // 10°C
    let temp1_celsius = Measurement::new(10.0, celsius);    // 10°C
    let temp1_fahrenheit = Measurement::new(50.0, fahrenheit); // 10°C
    
    let temp2_kelvin = Measurement::new(293.15, kelvin);    // 20°C  
    let temp2_celsius = Measurement::new(20.0, celsius);    // 20°C
    let temp2_fahrenheit = Measurement::new(68.0, fahrenheit); // 20°C
    
    // Test equality comparisons across different units
    assert_eq!(temp1_kelvin, temp1_celsius);
    assert_eq!(temp1_celsius, temp1_fahrenheit);
    
    // Test less-than and greater-than 
    assert!(temp1_kelvin < temp2_kelvin);
    assert!(temp1_celsius < temp2_celsius);
    assert!(temp1_fahrenheit < temp2_fahrenheit);
    
    // Cross-unit comparisons
    assert!(temp1_kelvin < temp2_celsius);
    assert!(temp1_celsius < temp2_fahrenheit);
    assert!(temp1_fahrenheit < temp2_kelvin);
    
    assert!(temp2_kelvin > temp1_celsius);
    assert!(temp2_celsius > temp1_fahrenheit);
    assert!(temp2_fahrenheit > temp1_kelvin);
}

#[test]
fn test_temperature_arithmetic() {
    // Create units
    let kelvin = Kelvin::new();
    let celsius = Celsius::new();

    // For temperature arithmetic, we need to handle both relative temperature 
    // changes and absolute temperature values differently

    // ----- Test with Celsius -----
    // Start with 0°C = 273.15K
    let freezing_celsius = Measurement::new(0.0, celsius);
    
    // 0°C + 10°C = 10°C (283.15K)
    let plus_ten_c = freezing_celsius.clone() + Measurement::new(10.0, celsius);
    println!("0°C + 10°C = {}K (Expected: 283.15K)", plus_ten_c.value());
    assert!((plus_ten_c.value() - 283.15).abs() < 0.01, 
            "Expected 283.15, got {}", plus_ten_c.value());
    
    // ----- Test with Kelvin -----
    // Start with 273.15K (0°C)
    let freezing_kelvin = Measurement::new(273.15, kelvin);
    
    // 273.15K + 10K = 283.15K
    let k_plus_ten_k = freezing_kelvin.clone() + Measurement::new(10.0, kelvin);
    println!("273.15K + 10K = {}K (Expected: 283.15K)", k_plus_ten_k.value());
    assert!((k_plus_ten_k.value() - 283.15).abs() < 0.01, 
            "Expected 283.15, got {}", k_plus_ten_k.value());
            
    // Special case: Adding Kelvin to Celsius
    // For this test we're expecting:
    // 0°C (as 273.15K) + 10K = 283.15K
    // This is because we're adding 10 absolute Kelvin to the absolute 
    // temperature value of 0°C (273.15K)
    // This is an indirect test since our model converts everything to Kelvin for operations
    let c_plus_k = Measurement::new(0.0, celsius).convert_to(kelvin).unwrap().value() + 10.0; 
    println!("0°C + 10K = {}K (Expected: 283.15K)", c_plus_k);
    assert!((c_plus_k - 283.15).abs() < 0.01, 
            "Expected 283.15, got {}", c_plus_k);
            
    // Test multiplication and division using Kelvin (absolute scale)
    let doubled = freezing_kelvin.clone() * 2.0;
    println!("273.15K * 2 = {}K (Expected: 546.3K)", doubled.value());
    assert!((doubled.value() - 546.3).abs() < 0.1, 
            "Expected 546.3, got {}", doubled.value());
            
    let halved = freezing_kelvin.clone() / 2.0;
    println!("273.15K / 2 = {}K (Expected: 136.575K)", halved.value());
    assert!((halved.value() - 136.575).abs() < 0.01, 
            "Expected 136.575, got {}", halved.value());
} 