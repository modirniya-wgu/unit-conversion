use unit_conversion_api::models::units::time::*;
use unit_conversion_api::models::Measurement;
use unit_conversion_api::models::unit::Unit;

#[test]
fn test_time_conversions() {
    // Create units
    let second = Second::new();
    let millisecond = Millisecond::new();
    let microsecond = Microsecond::new();
    let nanosecond = Nanosecond::new();
    let minute = Minute::new();
    let hour = Hour::new();
    let day = Day::new();
    let week = Week::new();
    let month = Month::new();
    let year = Year::new();
    let decade = Decade::new();
    let century = Century::new();
    
    // Create measurements
    let one_second = Measurement::new(1.0, second);
    let one_minute = Measurement::new(1.0, minute);
    let one_hour = Measurement::new(1.0, hour);
    let one_day = Measurement::new(1.0, day);
    let one_year = Measurement::new(1.0, year);
    
    // Test second to smaller units
    let sec_to_ms = one_second.convert_to(millisecond).unwrap();
    assert!((sec_to_ms.value() - 1000.0).abs() < 0.01);
    
    let sec_to_us = one_second.convert_to(microsecond).unwrap();
    assert!((sec_to_us.value() - 1_000_000.0).abs() < 0.01);
    
    let sec_to_ns = one_second.convert_to(nanosecond).unwrap();
    assert!((sec_to_ns.value() - 1_000_000_000.0).abs() < 0.01);
    
    // Test second to larger units
    let sec_to_min = one_second.convert_to(minute).unwrap();
    assert!((sec_to_min.value() - 1.0/60.0).abs() < 0.0001);
    
    let sec_to_hour = one_second.convert_to(hour).unwrap();
    assert!((sec_to_hour.value() - 1.0/3600.0).abs() < 0.0000001);
    
    // Test minute conversions
    let min_to_sec = one_minute.convert_to(second).unwrap();
    assert!((min_to_sec.value() - 60.0).abs() < 0.01);
    
    let min_to_hour = one_minute.convert_to(hour).unwrap();
    assert!((min_to_hour.value() - 1.0/60.0).abs() < 0.0001);
    
    // Test hour conversions
    let hour_to_sec = one_hour.convert_to(second).unwrap();
    assert!((hour_to_sec.value() - 3600.0).abs() < 0.01);
    
    let hour_to_min = one_hour.convert_to(minute).unwrap();
    assert!((hour_to_min.value() - 60.0).abs() < 0.01);
    
    let hour_to_day = one_hour.convert_to(day).unwrap();
    assert!((hour_to_day.value() - 1.0/24.0).abs() < 0.0001);
    
    // Test day conversions
    let day_to_hour = one_day.convert_to(hour).unwrap();
    assert!((day_to_hour.value() - 24.0).abs() < 0.01);
    
    let day_to_week = one_day.convert_to(week).unwrap();
    assert!((day_to_week.value() - 1.0/7.0).abs() < 0.0001);
    
    // Test year conversions
    let year_to_day = one_year.convert_to(day).unwrap();
    assert!((year_to_day.value() - 365.0).abs() < 0.01);
    
    let year_to_month = one_year.convert_to(month).unwrap();
    assert!((year_to_month.value() - 12.0).abs() < 0.01);
    
    let year_to_decade = one_year.convert_to(decade).unwrap();
    assert!((year_to_decade.value() - 0.1).abs() < 0.001);
    
    let year_to_century = one_year.convert_to(century).unwrap();
    assert!((year_to_century.value() - 0.01).abs() < 0.0001);
}

#[test]
fn test_time_equality() {
    let second = Second::new();
    let millisecond = Millisecond::new();
    let minute = Minute::new();
    let hour = Hour::new();
    let day = Day::new();
    
    // 1 second = 1000 milliseconds
    let one_second = Measurement::new(1.0, second);
    let thousand_ms = Measurement::new(1000.0, millisecond);
    assert_eq!(one_second, thousand_ms);
    
    // 1 minute = 60 seconds
    let one_minute = Measurement::new(1.0, minute);
    let sixty_seconds = Measurement::new(60.0, second);
    assert_eq!(one_minute, sixty_seconds);
    
    // 1 hour = 60 minutes
    let one_hour = Measurement::new(1.0, hour);
    let sixty_minutes = Measurement::new(60.0, minute);
    assert_eq!(one_hour, sixty_minutes);
    
    // 1 day = 24 hours
    let one_day = Measurement::new(1.0, day);
    let twenty_four_hours = Measurement::new(24.0, hour);
    assert_eq!(one_day, twenty_four_hours);
}

#[test]
fn test_time_comparison() {
    let second = Second::new();
    let minute = Minute::new();
    let hour = Hour::new();
    
    let time1 = Measurement::new(30.0, second);      // 30 seconds
    let time2 = Measurement::new(2.0, minute);       // 2 minutes (120 seconds)
    let time3 = Measurement::new(1.0, hour);         // 1 hour (3600 seconds)
    
    // Test less-than and greater-than for same unit
    let time4 = Measurement::new(45.0, second);
    assert!(time1 < time4);
    assert!(time4 > time1);
    
    // Test less-than and greater-than for different units
    assert!(time1 < time2);
    assert!(time2 < time3);
    assert!(time3 > time2);
    assert!(time2 > time1);
    
    // Mixed comparisons
    let half_minute = Measurement::new(0.5, minute); // 30 seconds
    assert_eq!(time1, half_minute);
    
    let quarter_hour = Measurement::new(0.25, hour); // 15 minutes
    let fifteen_minutes = Measurement::new(15.0, minute);
    assert_eq!(quarter_hour, fifteen_minutes);
}

#[test]
fn test_time_arithmetic() {
    let second = Second::new();
    let minute = Minute::new();
    let hour = Hour::new();
    
    // Convert everything to seconds for testing arithmetic
    let time1_sec = Measurement::new(30.0, second);        // 30 seconds
    let time2_sec = Measurement::new(90.0, second);        // 90 seconds
    let time3_sec = Measurement::new(120.0, second);       // 2 minutes converted to seconds
    let time4_sec = Measurement::new(1800.0, second);      // 0.5 hours converted to seconds
    
    // Addition with same units
    let sum = time1_sec.clone() + time2_sec.clone();
    assert!((sum.value() - 120.0).abs() < 0.01);
    assert_eq!(sum.unit().symbol(), "s");
    
    // Addition
    let sum2 = time1_sec.clone() + time3_sec.clone();
    assert!((sum2.value() - 150.0).abs() < 0.01);
    assert_eq!(sum2.unit().symbol(), "s");
    
    // Addition 
    let sum3 = time3_sec.clone() + time4_sec.clone();
    println!("120s + 1800s = {}s", sum3.value());
    assert!((sum3.value() - 1920.0).abs() < 0.01);
    assert_eq!(sum3.unit().symbol(), "s");
    
    // Also test with mixed units
    let time3_min = Measurement::new(2.0, minute);        // 2 minutes
    let time4_hour = Measurement::new(0.5, hour);         // 0.5 hours
    
    // First convert both to seconds (base unit)
    let time3_min_as_sec = time3_min.convert_to(second).unwrap();
    let time4_hour_as_sec = time4_hour.convert_to(second).unwrap();
    
    println!("2 minutes = {}s", time3_min_as_sec.value());
    println!("0.5 hours = {}s", time4_hour_as_sec.value());
    
    // Then add
    let sum4 = time3_min_as_sec + time4_hour_as_sec;
    println!("2 minutes + 0.5 hours = {}s", sum4.value());
    assert!((sum4.value() - 1920.0).abs() < 0.01);
    assert_eq!(sum4.unit().symbol(), "s");
    
    // Subtraction
    let diff = time4_sec.clone() - time3_sec.clone();
    assert!((diff.value() - 1680.0).abs() < 0.1);
    assert_eq!(diff.unit().symbol(), "s");
    
    // Multiplication
    let doubled = time1_sec.clone() * 2.0;
    assert!((doubled.value() - 60.0).abs() < 0.01);
    assert_eq!(doubled.unit().symbol(), "s");
    
    // Division
    let halved = time2_sec.clone() / 2.0;
    assert!((halved.value() - 45.0).abs() < 0.01);
    assert_eq!(halved.unit().symbol(), "s");
} 