use unit_conversion_api::models::init::init;
use unit_conversion_api::models::units::power::{
    Watt, Kilowatt, Megawatt, Gigawatt, Horsepower,
    BTUPerHour, FootPoundPerSecond, CaloriePerSecond,
    JoulePerSecond, KilocaloriePerHour
};
use unit_conversion_api::models::unit::Unit;

#[test]
fn test_power_conversions() {
    // Initialize the registry
    init();
    
    // Get power units
    let w = Watt::new();
    let kw = Kilowatt::new();
    let mw = Megawatt::new();
    let gw = Gigawatt::new();
    let hp = Horsepower::new();
    let btu_h = BTUPerHour::new();
    let ft_lb_s = FootPoundPerSecond::new();
    let cal_s = CaloriePerSecond::new();
    let j_s = JoulePerSecond::new();
    let kcal_h = KilocaloriePerHour::new();
    
    // Test conversions to base unit (W)
    assert_eq!(w.to_base(1.0), 1.0);
    assert_eq!(kw.to_base(1.0), 1000.0);
    assert_eq!(mw.to_base(1.0), 1_000_000.0);
    assert_eq!(gw.to_base(1.0), 1_000_000_000.0);
    assert!((hp.to_base(1.0) - 745.7).abs() < 1e-5);
    assert!((btu_h.to_base(1.0) - 0.29307107).abs() < 1e-8);
    assert!((ft_lb_s.to_base(1.0) - 1.35582).abs() < 1e-5);
    assert!((cal_s.to_base(1.0) - 4.184).abs() < 1e-5);
    assert_eq!(j_s.to_base(1.0), 1.0);
    assert!((kcal_h.to_base(1.0) - 1.163).abs() < 1e-3);
    
    // Test conversions from base unit (W)
    assert_eq!(w.from_base(1.0), 1.0);
    assert_eq!(kw.from_base(1000.0), 1.0);
    assert_eq!(mw.from_base(1_000_000.0), 1.0);
    assert_eq!(gw.from_base(1_000_000_000.0), 1.0);
    assert!((hp.from_base(745.7) - 1.0).abs() < 1e-5);
    assert!((btu_h.from_base(0.29307107) - 1.0).abs() < 1e-5);
    assert!((ft_lb_s.from_base(1.35582) - 1.0).abs() < 1e-5);
    assert!((cal_s.from_base(4.184) - 1.0).abs() < 1e-5);
    assert_eq!(j_s.from_base(1.0), 1.0);
    assert!((kcal_h.from_base(1.163) - 1.0).abs() < 1e-3);
    
    // Test round-trip conversions
    let power_value = 1000.0; // 1000 W = 1 kW
    
    // Convert to kW and back manually
    let to_kw = kw.from_base(w.to_base(power_value));
    let back_to_w = w.from_base(kw.to_base(to_kw));
    assert!((back_to_w - power_value).abs() < 1e-5);
    
    // Convert to hp and back manually
    let to_hp = hp.from_base(w.to_base(power_value));
    let back_to_w = w.from_base(hp.to_base(to_hp));
    assert!((back_to_w - power_value).abs() < 1e-5);
    
    // Convert to BTU/h and back manually
    let to_btu_h = btu_h.from_base(w.to_base(power_value));
    let back_to_w = w.from_base(btu_h.to_base(to_btu_h));
    assert!((back_to_w - power_value).abs() < 1e-5);
}

#[test]
fn test_power_equality() {
    init();
    
    // Test equality of equivalent powers in different units
    let w_val = 1000.0; // 1000 W
    let kw_val = 1.0; // 1 kW = 1000 W
    let hp_val = 1.341; // 1.341 hp ≈ 1000 W
    
    let w = Watt::new();
    let kw = Kilowatt::new();
    let hp = Horsepower::new();
    
    // Convert to base unit and compare
    assert!((w.to_base(w_val) - kw.to_base(kw_val)).abs() < 1e-5);
    assert!((w.to_base(w_val) - hp.to_base(hp_val)).abs() < 1.0); // Less precise due to rounding
}

#[test]
fn test_power_comparison() {
    init();
    
    let w = Watt::new();
    let kw = Kilowatt::new();
    let hp = Horsepower::new();
    
    // 2000 W is greater than 1 kW (1000 W)
    let w_val = 2000.0;
    let kw_val = 1.0;
    
    assert!(w.to_base(w_val) > kw.to_base(kw_val));
    
    // 0.5 kW (500 W) is less than 1 hp (745.7 W)
    let kw_val2 = 0.5;
    let hp_val = 1.0;
    
    assert!(kw.to_base(kw_val2) < hp.to_base(hp_val));
    
    // 1 kW equals 1000 W
    let kw_val3 = 1.0;
    let w_val2 = 1000.0;
    
    assert!((kw.to_base(kw_val3) - w.to_base(w_val2)).abs() < 1e-5);
}

#[test]
fn test_power_arithmetic() {
    init();
    
    let w = Watt::new();
    let kw = Kilowatt::new();
    
    // Addition: 1000 W + 1 kW (1000 W) = 2000 W
    let w_val = 1000.0;
    let kw_val = 1.0;
    
    // Convert both to W before adding
    let sum = w.to_base(w_val) + kw.to_base(kw_val);
    assert!((sum - 2000.0).abs() < 1e-5);
    
    // Subtraction: 5000 W - 2 kW (2000 W) = 3000 W
    let w_val2 = 5000.0;
    let kw_val2 = 2.0;
    let diff = w.to_base(w_val2) - kw.to_base(kw_val2);
    assert!((diff - 3000.0).abs() < 1e-5);
    
    // Multiplication: 1000 W * 2 = 2000 W
    let product = w.to_base(w_val) * 2.0;
    assert!((product - 2000.0).abs() < 1e-5);
    
    // Division: 2000 W / 2 = 1000 W
    let quotient = w.to_base(2000.0) / 2.0;
    assert!((quotient - 1000.0).abs() < 1e-5);
} 