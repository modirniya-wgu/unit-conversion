use unit_conversion_api::models::init::init;
use unit_conversion_api::models::units::pressure::{
    Pascal, Kilopascal, Megapascal, Bar, Millibar, Atmosphere,
    PoundsPerSquareInch, Torr, MillimetersOfMercury, InchesOfMercury
};
use unit_conversion_api::models::unit::Unit;

#[test]
fn test_pressure_conversions() {
    // Initialize the registry
    init();
    
    // Get pressure units
    let pa = Pascal::new();
    let kpa = Kilopascal::new();
    let mpa = Megapascal::new();
    let bar = Bar::new();
    let mbar = Millibar::new();
    let atm = Atmosphere::new();
    let psi = PoundsPerSquareInch::new();
    let torr = Torr::new();
    let mmhg = MillimetersOfMercury::new();
    let inhg = InchesOfMercury::new();
    
    // Test conversions to base unit (Pa)
    assert_eq!(pa.to_base(1.0), 1.0);
    assert_eq!(kpa.to_base(1.0), 1000.0);
    assert_eq!(mpa.to_base(1.0), 1_000_000.0);
    assert_eq!(bar.to_base(1.0), 100_000.0);
    assert_eq!(mbar.to_base(1.0), 100.0);
    assert_eq!(atm.to_base(1.0), 101_325.0);
    assert!((psi.to_base(1.0) - 6894.76).abs() < 0.01);
    assert!((torr.to_base(1.0) - 133.322).abs() < 0.01);
    assert!((mmhg.to_base(1.0) - 133.322).abs() < 0.01);
    assert!((inhg.to_base(1.0) - 3386.39).abs() < 0.01);
    
    // Test conversions from base unit (Pa)
    assert_eq!(pa.from_base(1.0), 1.0);
    assert_eq!(kpa.from_base(1000.0), 1.0);
    assert_eq!(mpa.from_base(1_000_000.0), 1.0);
    assert_eq!(bar.from_base(100_000.0), 1.0);
    assert_eq!(mbar.from_base(100.0), 1.0);
    assert!((atm.from_base(101_325.0) - 1.0).abs() < 1e-10);
    assert!((psi.from_base(6894.76) - 1.0).abs() < 1e-5);
    assert!((torr.from_base(133.322) - 1.0).abs() < 1e-5);
    assert!((mmhg.from_base(133.322) - 1.0).abs() < 1e-5);
    assert!((inhg.from_base(3386.39) - 1.0).abs() < 1e-5);
    
    // Test round-trip conversions
    let pressure_value = 100_000.0; // 100,000 Pa = 1 bar
    
    // Convert to bar and back manually
    let to_bar = bar.from_base(pa.to_base(pressure_value));
    let back_to_pa = pa.from_base(bar.to_base(to_bar));
    assert!((back_to_pa - pressure_value).abs() < 1e-5);
    
    // Convert to psi and back manually
    let to_psi = psi.from_base(pa.to_base(pressure_value));
    let back_to_pa = pa.from_base(psi.to_base(to_psi));
    assert!((back_to_pa - pressure_value).abs() < 1e-5);
    
    // Convert to atm and back manually
    let to_atm = atm.from_base(pa.to_base(pressure_value));
    let back_to_pa = pa.from_base(atm.to_base(to_atm));
    assert!((back_to_pa - pressure_value).abs() < 1e-5);
}

#[test]
fn test_pressure_equality() {
    init();
    
    // Test equality of equivalent pressures in different units
    let pa_val = 100_000.0; // 100,000 Pa
    let bar_val = 1.0; // 1 bar = 100,000 Pa
    let atm_val = 0.986923; // 0.986923 atm ≈ 100,000 Pa
    
    let pa = Pascal::new();
    let bar = Bar::new();
    let atm = Atmosphere::new();
    
    // Convert to base unit and compare
    assert!((pa.to_base(pa_val) - bar.to_base(bar_val)).abs() < 1e-5);
    assert!((pa.to_base(pa_val) - atm.to_base(atm_val)).abs() < 1.0); // Less precise due to rounding
    assert!((bar.to_base(bar_val) - atm.to_base(atm_val)).abs() < 1.0); // Less precise due to rounding
}

#[test]
fn test_pressure_comparison() {
    init();
    
    let pa = Pascal::new();
    let kpa = Kilopascal::new();
    let psi = PoundsPerSquareInch::new();
    
    // 200 kPa is greater than 25 psi (≈ 172.4 kPa)
    let kpa_val = 200.0;
    let psi_val = 25.0;
    
    assert!(kpa.to_base(kpa_val) > psi.to_base(psi_val));
    
    // 1500 Pa is less than 1 kPa (1000 Pa)
    let pa_val = 1500.0;
    let kpa_val2 = 2.0;
    
    assert!(pa.to_base(pa_val) < kpa.to_base(kpa_val2));
    
    // 100,000 Pa equals 1 bar
    let pa_val2 = 100_000.0;
    let bar_val = 1.0;
    let bar = Bar::new();
    
    assert!((pa.to_base(pa_val2) - bar.to_base(bar_val)).abs() < 1e-5);
}

#[test]
fn test_pressure_arithmetic() {
    init();
    
    let pa = Pascal::new();
    let kpa = Kilopascal::new();
    
    // Addition: 1000 Pa + 1 kPa (1000 Pa) = 2000 Pa
    let pa_val = 1000.0;
    let kpa_val = 1.0;
    
    // Convert both to Pa before adding
    let sum = pa.to_base(pa_val) + kpa.to_base(kpa_val);
    assert!((sum - 2000.0).abs() < 1e-5);
    
    // Subtraction: 5000 Pa - 2 kPa (2000 Pa) = 3000 Pa
    let pa_val2 = 5000.0;
    let kpa_val2 = 2.0;
    let diff = pa.to_base(pa_val2) - kpa.to_base(kpa_val2);
    assert!((diff - 3000.0).abs() < 1e-5);
    
    // Multiplication: 1000 Pa * 2 = 2000 Pa
    let product = pa.to_base(pa_val) * 2.0;
    assert!((product - 2000.0).abs() < 1e-5);
    
    // Division: 2000 Pa / 2 = 1000 Pa
    let quotient = pa.to_base(2000.0) / 2.0;
    assert!((quotient - 1000.0).abs() < 1e-5);
} 