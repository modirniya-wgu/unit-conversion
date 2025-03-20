use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div};
use crate::errors::ApiError;

/// A trait for unit categories (length, mass, temperature, etc.)
pub trait UnitCategory: Display + Send + Sync + 'static {
    /// Returns the name of the category
    fn name(&self) -> &'static str;
    
    /// Returns the description of the category
    fn description(&self) -> &'static str;
}

/// Represents a specific unit of measurement (meter, kilogram, celsius, etc.)
pub trait Unit: Display + Send + Sync + 'static {
    /// The category this unit belongs to
    type Category: UnitCategory;
    
    /// Returns the symbol for this unit (m, kg, °C, etc.)
    fn symbol(&self) -> &'static str;
    
    /// Returns the name of the unit
    fn name(&self) -> &'static str;
    
    /// Returns the category of this unit
    fn category(&self) -> &Self::Category;
    
    /// Converts a value from the base unit of the category to this unit
    fn from_base(&self, value: f64) -> f64;
    
    /// Converts a value from this unit to the base unit of the category
    fn to_base(&self, value: f64) -> f64;
}

/// A quantity with a specific unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement<U: Unit> {
    value: f64,
    #[serde(skip)]
    unit: U,
}

impl<U: Unit> Measurement<U> {
    /// Creates a new measurement with the given value and unit
    pub fn new(value: f64, unit: U) -> Self {
        Self { value, unit }
    }
    
    /// Returns the value of the measurement
    pub fn value(&self) -> f64 {
        self.value
    }
    
    /// Returns the unit of the measurement
    pub fn unit(&self) -> &U {
        &self.unit
    }
    
    /// Converts this measurement to another unit within the same category
    pub fn convert_to<T: Unit<Category = U::Category>>(&self, target_unit: T) -> Result<Measurement<T>, ApiError> {
        // First convert to base unit, then to target unit
        let base_value = self.unit.to_base(self.value);
        let target_value = target_unit.from_base(base_value);
        
        Ok(Measurement::new(target_value, target_unit))
    }
    
    /// Returns the measurement in the base unit of its category
    pub fn to_base(&self) -> f64 {
        self.unit.to_base(self.value)
    }
}

// Display implementation for Measurement
impl<U: Unit> Display for Measurement<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

// Implement PartialEq for comparing measurements
// Two measurements are equal if their base values are equal
impl<U: Unit, V: Unit<Category = U::Category>> PartialEq<Measurement<V>> for Measurement<U> {
    fn eq(&self, other: &Measurement<V>) -> bool {
        let self_base = self.to_base();
        let other_base = other.to_base();
        
        (self_base - other_base).abs() < f64::EPSILON
    }
}

// Implement PartialOrd for comparing measurements
impl<U: Unit, V: Unit<Category = U::Category>> PartialOrd<Measurement<V>> for Measurement<U> {
    fn partial_cmp(&self, other: &Measurement<V>) -> Option<Ordering> {
        let self_base = self.to_base();
        let other_base = other.to_base();
        
        self_base.partial_cmp(&other_base)
    }
}

// Implement Add for adding measurements of the same category
impl<U: Unit + Clone, V: Unit<Category = U::Category>> Add<Measurement<V>> for Measurement<U> {
    type Output = Measurement<U>;
    
    fn add(self, other: Measurement<V>) -> Self::Output {
        let self_base = self.to_base();
        let other_base = other.to_base();
        let result_base = self_base + other_base;
        
        Measurement::new(self.unit.from_base(result_base), self.unit)
    }
}

// Implement Sub for subtracting measurements of the same category
impl<U: Unit + Clone, V: Unit<Category = U::Category>> Sub<Measurement<V>> for Measurement<U> {
    type Output = Measurement<U>;
    
    fn sub(self, other: Measurement<V>) -> Self::Output {
        let self_base = self.to_base();
        let other_base = other.to_base();
        let result_base = self_base - other_base;
        
        Measurement::new(self.unit.from_base(result_base), self.unit)
    }
}

// Implement Mul for multiplying a measurement by a scalar
impl<U: Unit + Clone> Mul<f64> for Measurement<U> {
    type Output = Measurement<U>;
    
    fn mul(self, scalar: f64) -> Self::Output {
        Measurement::new(self.value * scalar, self.unit)
    }
}

// Implement Div for dividing a measurement by a scalar
impl<U: Unit + Clone> Div<f64> for Measurement<U> {
    type Output = Measurement<U>;
    
    fn div(self, scalar: f64) -> Self::Output {
        Measurement::new(self.value / scalar, self.unit)
    }
} 