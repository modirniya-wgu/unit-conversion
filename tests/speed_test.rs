use unit_conversion_api::models::registry;
use unit_conversion_api::models::init::init;
use unit_conversion_api::models::units::speed::{
    MeterPerSecond, KilometerPerHour, MilePerHour, 
    Knot, FootPerSecond, CentimeterPerSecond, Mach
};
use unit_conversion_api::models::unit::Unit;

#[test]
fn test_speed_conversions() {
    // Initialize the registry
    init();
    
    // Get speed units
    let mps = MeterPerSecond::new();
    let kmh = KilometerPerHour::new();
    let mph = MilePerHour::new();
    let kn = Knot::new();
    let fps = FootPerSecond::new();
    let cms = CentimeterPerSecond::new();
    let mach = Mach::new();
    
    // Test conversions to base unit (m/s)
    assert_eq!(mps.to_base(1.0), 1.0);
    assert!((kmh.to_base(3.6) - 1.0).abs() < 1e-5);
    assert!((mph.to_base(2.23694) - 1.0).abs() < 1e-5);
    assert!((kn.to_base(1.94384) - 1.0).abs() < 1e-5);
    assert!((fps.to_base(3.28084) - 1.0).abs() < 1e-5);
    assert!((cms.to_base(100.0) - 1.0).abs() < 1e-5);
    assert!((mach.to_base(0.00291545) - 1.0).abs() < 1e-3);
    
    // Test conversions from base unit (m/s)
    assert_eq!(mps.from_base(1.0), 1.0);
    assert!((kmh.from_base(1.0) - 3.6).abs() < 1e-5);
    assert!((mph.from_base(1.0) - 2.23694).abs() < 1e-5);
    assert!((kn.from_base(1.0) - 1.94384).abs() < 1e-5);
    assert!((fps.from_base(1.0) - 3.28084).abs() < 1e-5);
    assert!((cms.from_base(1.0) - 100.0).abs() < 1e-5);
    assert!((mach.from_base(1.0) - 0.00291545).abs() < 1e-5);
    
    // Test round-trip conversions
    let speed_value = 100.0; // 100 m/s
    
    // Convert to km/h and back manually
    let to_kmh = kmh.from_base(mps.to_base(speed_value));
    let back_to_mps = mps.from_base(kmh.to_base(to_kmh));
    assert!((back_to_mps - speed_value).abs() < 1e-5);
    
    // Convert to mph and back manually
    let to_mph = mph.from_base(mps.to_base(speed_value));
    let back_to_mps = mps.from_base(mph.to_base(to_mph));
    assert!((back_to_mps - speed_value).abs() < 1e-5);
    
    // Convert to knots and back manually
    let to_knots = kn.from_base(mps.to_base(speed_value));
    let back_to_mps = mps.from_base(kn.to_base(to_knots));
    assert!((back_to_mps - speed_value).abs() < 1e-5);
}

#[test]
fn test_speed_equality() {
    init();
    
    // Test equality of equivalent speeds in different units
    let mps_val = 1.0; // 1 m/s
    let kmh_val = 3.6; // 3.6 km/h = 1 m/s
    let mph_val = 2.23694; // 2.23694 mph = 1 m/s
    
    let mps = MeterPerSecond::new();
    let kmh = KilometerPerHour::new();
    let mph = MilePerHour::new();
    
    // Convert to base unit and compare
    assert!((mps.to_base(mps_val) - kmh.to_base(kmh_val)).abs() < 1e-5);
    assert!((mps.to_base(mps_val) - mph.to_base(mph_val)).abs() < 1e-5);
    assert!((kmh.to_base(kmh_val) - mph.to_base(mph_val)).abs() < 1e-5);
}

#[test]
fn test_speed_comparison() {
    init();
    
    let mps = MeterPerSecond::new();
    let kmh = KilometerPerHour::new();
    let mph = MilePerHour::new();
    
    // 10 m/s is greater than 30 km/h (8.33 m/s)
    let mps_val = 10.0;
    let kmh_val = 30.0;
    
    assert!(mps.to_base(mps_val) > kmh.to_base(kmh_val));
    
    // 20 mph (8.94 m/s) is less than 10 m/s
    let mph_val = 20.0;
    
    assert!(mph.to_base(mph_val) < mps.to_base(mps_val));
    
    // 36 km/h (10 m/s) equals 22.3694 mph (10 m/s)
    let equal_kmh = 36.0;
    let equal_mph = 22.3694;
    
    assert!((kmh.to_base(equal_kmh) - mph.to_base(equal_mph)).abs() < 1e-4);
}

#[test]
fn test_speed_arithmetic() {
    init();
    
    let mps = MeterPerSecond::new();
    let kmh = KilometerPerHour::new();
    
    // Addition: 10 m/s + 36 km/h (10 m/s) = 20 m/s
    let mps_val = 10.0;
    let kmh_val = 36.0;
    
    // Convert both to m/s before adding
    let sum = mps.to_base(mps_val) + kmh.to_base(kmh_val);
    assert!((sum - 20.0).abs() < 1e-5);
    
    // Subtraction: 20 m/s - 36 km/h (10 m/s) = 10 m/s
    let mps_val2 = 20.0;
    let diff = mps.to_base(mps_val2) - kmh.to_base(kmh_val);
    assert!((diff - 10.0).abs() < 1e-5);
    
    // Multiplication: 10 m/s * 2 = 20 m/s
    let product = mps.to_base(mps_val) * 2.0;
    assert!((product - 20.0).abs() < 1e-5);
    
    // Division: 20 m/s / 2 = 10 m/s
    let quotient = mps.to_base(mps_val2) / 2.0;
    assert!((quotient - 10.0).abs() < 1e-5);
} 