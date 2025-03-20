use unit_conversion_api::models::init::init;
use unit_conversion_api::models::units::energy::{
    Joule, Kilojoule, Megajoule, Calorie, Kilocalorie,
    BTU, KilowattHour, WattHour, ElectronVolt, Therm, FootPound
};
use unit_conversion_api::models::unit::Unit;

#[test]
fn test_energy_conversions() {
    // Initialize the registry
    init();
    
    // Get energy units
    let j = Joule::new();
    let kj = Kilojoule::new();
    let mj = Megajoule::new();
    let cal = Calorie::new();
    let kcal = Kilocalorie::new();
    let btu = BTU::new();
    let kwh = KilowattHour::new();
    let wh = WattHour::new();
    let ev = ElectronVolt::new();
    let therm = Therm::new();
    let ft_lb = FootPound::new();
    
    // Test conversions to base unit (J)
    assert_eq!(j.to_base(1.0), 1.0);
    assert_eq!(kj.to_base(1.0), 1000.0);
    assert_eq!(mj.to_base(1.0), 1_000_000.0);
    assert!((cal.to_base(1.0) - 4.184).abs() < 1e-5);
    assert!((kcal.to_base(1.0) - 4184.0).abs() < 1e-5);
    assert!((btu.to_base(1.0) - 1055.06).abs() < 1e-2);
    assert!((kwh.to_base(1.0) - 3_600_000.0).abs() < 1e-5);
    assert!((wh.to_base(1.0) - 3600.0).abs() < 1e-5);
    assert!((ev.to_base(1.0) - 1.602176634e-19).abs() < 1e-25);
    assert!((therm.to_base(1.0) - 105_505_585.257348).abs() < 0.1);
    assert!((ft_lb.to_base(1.0) - 1.35582).abs() < 1e-5);
    
    // Test conversions from base unit (J)
    assert_eq!(j.from_base(1.0), 1.0);
    assert_eq!(kj.from_base(1000.0), 1.0);
    assert_eq!(mj.from_base(1_000_000.0), 1.0);
    assert!((cal.from_base(4.184) - 1.0).abs() < 1e-5);
    assert!((kcal.from_base(4184.0) - 1.0).abs() < 1e-5);
    assert!((btu.from_base(1055.06) - 1.0).abs() < 1e-5);
    assert!((kwh.from_base(3_600_000.0) - 1.0).abs() < 1e-5);
    assert!((wh.from_base(3600.0) - 1.0).abs() < 1e-5);
    assert!((ev.from_base(1.602176634e-19) - 1.0).abs() < 1e-10);
    assert!((therm.from_base(105_505_585.257348) - 1.0).abs() < 1e-5);
    assert!((ft_lb.from_base(1.35582) - 1.0).abs() < 1e-5);
    
    // Test round-trip conversions
    let energy_value = 1000.0; // 1000 J = 1 kJ
    
    // Convert to kJ and back manually
    let to_kj = kj.from_base(j.to_base(energy_value));
    let back_to_j = j.from_base(kj.to_base(to_kj));
    assert!((back_to_j - energy_value).abs() < 1e-5);
    
    // Convert to kcal and back manually
    let to_kcal = kcal.from_base(j.to_base(energy_value));
    let back_to_j = j.from_base(kcal.to_base(to_kcal));
    assert!((back_to_j - energy_value).abs() < 1e-5);
    
    // Convert to kWh and back manually
    let to_kwh = kwh.from_base(j.to_base(energy_value));
    let back_to_j = j.from_base(kwh.to_base(to_kwh));
    assert!((back_to_j - energy_value).abs() < 1e-5);
}

#[test]
fn test_energy_equality() {
    init();
    
    // Test equality of equivalent energies in different units
    let j_val = 4184.0; // 4184 J
    let kcal_val = 1.0; // 1 kcal = 4184 J
    let kwh_val = 0.001161111; // 0.001161111 kWh ≈ 4180 J (rounding)
    
    let j = Joule::new();
    let kcal = Kilocalorie::new();
    let kwh = KilowattHour::new();
    
    // Convert to base unit and compare
    assert!((j.to_base(j_val) - kcal.to_base(kcal_val)).abs() < 1e-5);
    assert!((j.to_base(j_val) - kwh.to_base(kwh_val)).abs() < 5.0); // Less precise due to rounding
}

#[test]
fn test_energy_comparison() {
    init();
    
    let j = Joule::new();
    let kj = Kilojoule::new();
    let kcal = Kilocalorie::new();
    
    // 5000 J is greater than 1 kcal (4184 J)
    let j_val = 5000.0;
    let kcal_val = 1.0;
    
    assert!(j.to_base(j_val) > kcal.to_base(kcal_val));
    
    // 0.5 kJ (500 J) is less than 0.2 kcal (836.8 J)
    let kj_val = 0.5;
    let kcal_val2 = 0.2;
    
    assert!(kj.to_base(kj_val) < kcal.to_base(kcal_val2));
    
    // 1 kJ equals 1000 J
    let kj_val2 = 1.0;
    let j_val2 = 1000.0;
    
    assert!((kj.to_base(kj_val2) - j.to_base(j_val2)).abs() < 1e-5);
}

#[test]
fn test_energy_arithmetic() {
    init();
    
    let j = Joule::new();
    let kj = Kilojoule::new();
    
    // Addition: 1000 J + 1 kJ (1000 J) = 2000 J
    let j_val = 1000.0;
    let kj_val = 1.0;
    
    // Convert both to J before adding
    let sum = j.to_base(j_val) + kj.to_base(kj_val);
    assert!((sum - 2000.0).abs() < 1e-5);
    
    // Subtraction: 5000 J - 2 kJ (2000 J) = 3000 J
    let j_val2 = 5000.0;
    let kj_val2 = 2.0;
    let diff = j.to_base(j_val2) - kj.to_base(kj_val2);
    assert!((diff - 3000.0).abs() < 1e-5);
    
    // Multiplication: 1000 J * 2 = 2000 J
    let product = j.to_base(j_val) * 2.0;
    assert!((product - 2000.0).abs() < 1e-5);
    
    // Division: 2000 J / 2 = 1000 J
    let quotient = j.to_base(2000.0) / 2.0;
    assert!((quotient - 1000.0).abs() < 1e-5);
} 