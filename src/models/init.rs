use std::sync::Once;
use crate::models::registry;
use crate::models::units::length::{
    LengthCategory,
    Meter, Kilometer, Centimeter, Millimeter, 
    Micrometer, Nanometer,
    Inch, Foot, Yard, Mile, NauticalMile
};
use crate::models::units::mass::{
    MassCategory,
    Kilogram, Gram, Milligram, Microgram,
    MetricTon, Pound, Ounce, Stone, USTon, ImperialTon
};
use crate::models::units::volume::{
    VolumeCategory,
    CubicMeter, Liter, Milliliter, CubicCentimeter, CubicDecimeter,
    USGallon, UKGallon, USQuart, USPint, USCup, USFluidOunce,
    CubicInch, CubicFoot, CubicYard
};
use crate::models::units::temperature::{
    TemperatureCategory,
    Kelvin, Celsius, Fahrenheit, Rankine, Reaumur
};
use crate::models::units::area::{
    AreaCategory,
    SquareMeter, SquareKilometer, SquareCentimeter, SquareMillimeter,
    Hectare, Are, SquareInch, SquareFoot, SquareYard, SquareMile, Acre
};
use crate::models::units::time::{
    TimeCategory,
    Second, Millisecond, Microsecond, Nanosecond,
    Minute, Hour, Day, Week, Month, Year, Decade, Century
};
use crate::models::units::speed::{
    SpeedCategory,
    MeterPerSecond, KilometerPerHour, MilePerHour, Knot,
    FootPerSecond, CentimeterPerSecond, Mach
};
use crate::models::units::pressure::{
    PressureCategory,
    Pascal, Kilopascal, Megapascal, Bar, Millibar, Atmosphere,
    PoundsPerSquareInch, Torr, MillimetersOfMercury, InchesOfMercury
};
use crate::models::units::energy::{
    EnergyCategory,
    Joule, Kilojoule, Megajoule, Calorie, Kilocalorie,
    BTU, KilowattHour, WattHour, ElectronVolt, Therm, FootPound
};
use crate::models::units::power::{
    PowerCategory,
    Watt, Kilowatt, Megawatt, Gigawatt, Horsepower,
    BTUPerHour, FootPoundPerSecond, CaloriePerSecond,
    JoulePerSecond, KilocaloriePerHour
};

// Ensure initialization happens only once
static INIT: Once = Once::new();

/// Initialize the unit registry with all available units
pub fn init() {
    INIT.call_once(|| {
        let registry = registry();
        
        // We need to use a write lock to modify the registry
        if let Ok(mut reg) = registry.write() {
            // Register length category
            let length_category = LengthCategory;
            let _ = reg.register_category(length_category);
            
            // Register length units
            let _ = reg.register_unit(Meter::new());
            let _ = reg.register_unit(Kilometer::new());
            let _ = reg.register_unit(Centimeter::new());
            let _ = reg.register_unit(Millimeter::new());
            let _ = reg.register_unit(Micrometer::new());
            let _ = reg.register_unit(Nanometer::new());
            let _ = reg.register_unit(Inch::new());
            let _ = reg.register_unit(Foot::new());
            let _ = reg.register_unit(Yard::new());
            let _ = reg.register_unit(Mile::new());
            let _ = reg.register_unit(NauticalMile::new());
            
            // Register mass category
            let mass_category = MassCategory;
            let _ = reg.register_category(mass_category);
            
            // Register mass units
            let _ = reg.register_unit(Kilogram::new());
            let _ = reg.register_unit(Gram::new());
            let _ = reg.register_unit(Milligram::new());
            let _ = reg.register_unit(Microgram::new());
            let _ = reg.register_unit(MetricTon::new());
            let _ = reg.register_unit(Pound::new());
            let _ = reg.register_unit(Ounce::new());
            let _ = reg.register_unit(Stone::new());
            let _ = reg.register_unit(USTon::new());
            let _ = reg.register_unit(ImperialTon::new());
            
            // Register volume category
            let volume_category = VolumeCategory;
            let _ = reg.register_category(volume_category);
            
            // Register volume units
            let _ = reg.register_unit(CubicMeter::new());
            let _ = reg.register_unit(Liter::new());
            let _ = reg.register_unit(Milliliter::new());
            let _ = reg.register_unit(CubicCentimeter::new());
            let _ = reg.register_unit(CubicDecimeter::new());
            let _ = reg.register_unit(USGallon::new());
            let _ = reg.register_unit(UKGallon::new());
            let _ = reg.register_unit(USQuart::new());
            let _ = reg.register_unit(USPint::new());
            let _ = reg.register_unit(USCup::new());
            let _ = reg.register_unit(USFluidOunce::new());
            let _ = reg.register_unit(CubicInch::new());
            let _ = reg.register_unit(CubicFoot::new());
            let _ = reg.register_unit(CubicYard::new());
            
            // Register temperature category
            let temperature_category = TemperatureCategory;
            let _ = reg.register_category(temperature_category);
            
            // Register temperature units
            let _ = reg.register_unit(Kelvin::new());
            let _ = reg.register_unit(Celsius::new());
            let _ = reg.register_unit(Fahrenheit::new());
            let _ = reg.register_unit(Rankine::new());
            let _ = reg.register_unit(Reaumur::new());
            
            // Register area category
            let area_category = AreaCategory;
            let _ = reg.register_category(area_category);
            
            // Register area units
            let _ = reg.register_unit(SquareMeter::new());
            let _ = reg.register_unit(SquareKilometer::new());
            let _ = reg.register_unit(SquareCentimeter::new());
            let _ = reg.register_unit(SquareMillimeter::new());
            let _ = reg.register_unit(Hectare::new());
            let _ = reg.register_unit(Are::new());
            let _ = reg.register_unit(SquareInch::new());
            let _ = reg.register_unit(SquareFoot::new());
            let _ = reg.register_unit(SquareYard::new());
            let _ = reg.register_unit(SquareMile::new());
            let _ = reg.register_unit(Acre::new());
            
            // Register time category
            let time_category = TimeCategory;
            let _ = reg.register_category(time_category);
            
            // Register time units
            let _ = reg.register_unit(Second::new());
            let _ = reg.register_unit(Millisecond::new());
            let _ = reg.register_unit(Microsecond::new());
            let _ = reg.register_unit(Nanosecond::new());
            let _ = reg.register_unit(Minute::new());
            let _ = reg.register_unit(Hour::new());
            let _ = reg.register_unit(Day::new());
            let _ = reg.register_unit(Week::new());
            let _ = reg.register_unit(Month::new());
            let _ = reg.register_unit(Year::new());
            let _ = reg.register_unit(Decade::new());
            let _ = reg.register_unit(Century::new());
            
            // Register speed category
            let speed_category = SpeedCategory;
            let _ = reg.register_category(speed_category);
            
            // Register speed units
            let _ = reg.register_unit(MeterPerSecond::new());
            let _ = reg.register_unit(KilometerPerHour::new());
            let _ = reg.register_unit(MilePerHour::new());
            let _ = reg.register_unit(Knot::new());
            let _ = reg.register_unit(FootPerSecond::new());
            let _ = reg.register_unit(CentimeterPerSecond::new());
            let _ = reg.register_unit(Mach::new());
            
            // Register pressure category
            let pressure_category = PressureCategory;
            let _ = reg.register_category(pressure_category);
            
            // Register pressure units
            let _ = reg.register_unit(Pascal::new());
            let _ = reg.register_unit(Kilopascal::new());
            let _ = reg.register_unit(Megapascal::new());
            let _ = reg.register_unit(Bar::new());
            let _ = reg.register_unit(Millibar::new());
            let _ = reg.register_unit(Atmosphere::new());
            let _ = reg.register_unit(PoundsPerSquareInch::new());
            let _ = reg.register_unit(Torr::new());
            let _ = reg.register_unit(MillimetersOfMercury::new());
            let _ = reg.register_unit(InchesOfMercury::new());
            
            // Register energy category
            let energy_category = EnergyCategory;
            let _ = reg.register_category(energy_category);
            
            // Register energy units
            let _ = reg.register_unit(Joule::new());
            let _ = reg.register_unit(Kilojoule::new());
            let _ = reg.register_unit(Megajoule::new());
            let _ = reg.register_unit(Calorie::new());
            let _ = reg.register_unit(Kilocalorie::new());
            let _ = reg.register_unit(BTU::new());
            let _ = reg.register_unit(KilowattHour::new());
            let _ = reg.register_unit(WattHour::new());
            let _ = reg.register_unit(ElectronVolt::new());
            let _ = reg.register_unit(Therm::new());
            let _ = reg.register_unit(FootPound::new());
            
            // Register power category
            let power_category = PowerCategory;
            let _ = reg.register_category(power_category);
            
            // Register power units
            let _ = reg.register_unit(Watt::new());
            let _ = reg.register_unit(Kilowatt::new());
            let _ = reg.register_unit(Megawatt::new());
            let _ = reg.register_unit(Gigawatt::new());
            let _ = reg.register_unit(Horsepower::new());
            let _ = reg.register_unit(BTUPerHour::new());
            let _ = reg.register_unit(FootPoundPerSecond::new());
            let _ = reg.register_unit(CaloriePerSecond::new());
            let _ = reg.register_unit(JoulePerSecond::new());
            let _ = reg.register_unit(KilocaloriePerHour::new());
        }
    });
} 