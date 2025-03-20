use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
use std::any::Any;
use once_cell::sync::Lazy;
use crate::models::unit::{Unit, UnitCategory};
use crate::errors::ApiError;

/// Trait object for units with type erasure
pub trait AnyUnit: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn name(&self) -> &'static str;
    fn symbol(&self) -> &'static str;
    fn category_name(&self) -> &'static str;
    fn to_base(&self, value: f64) -> f64;
    fn from_base(&self, value: f64) -> f64;
}

impl<T: Unit + 'static> AnyUnit for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn name(&self) -> &'static str {
        self.name()
    }
    
    fn symbol(&self) -> &'static str {
        self.symbol()
    }
    
    fn category_name(&self) -> &'static str {
        self.category().name()
    }
    
    fn to_base(&self, value: f64) -> f64 {
        self.to_base(value)
    }
    
    fn from_base(&self, value: f64) -> f64 {
        self.from_base(value)
    }
}

/// Registry for storing and accessing available unit categories and units
pub struct UnitRegistry {
    // Maps category names to their implementations
    categories: HashMap<&'static str, Box<dyn UnitCategory>>,
    
    // Maps (category_name, unit_symbol) to the unit implementation
    units: HashMap<(&'static str, &'static str), Box<dyn AnyUnit>>,
    
    // Maps category names to the set of unit symbols in that category
    category_units: HashMap<&'static str, HashSet<&'static str>>,
}

impl UnitRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            units: HashMap::new(),
            category_units: HashMap::new(),
        }
    }
    
    /// Register a unit category
    pub fn register_category<C: UnitCategory>(&mut self, category: C) -> Result<(), ApiError> {
        let name = category.name();
        
        if self.categories.contains_key(name) {
            return Err(ApiError::BadRequest(format!("Category '{}' already registered", name)));
        }
        
        self.categories.insert(name, Box::new(category));
        self.category_units.insert(name, HashSet::new());
        
        Ok(())
    }
    
    /// Register a unit
    pub fn register_unit<U: Unit + 'static>(&mut self, unit: U) -> Result<(), ApiError> {
        let category_name = unit.category().name();
        let symbol = unit.symbol();
        
        // Ensure the category exists
        if !self.categories.contains_key(category_name) {
            return Err(ApiError::BadRequest(format!(
                "Cannot register unit '{}': category '{}' not registered", 
                unit.name(), category_name
            )));
        }
        
        // Ensure the unit isn't already registered
        if self.units.contains_key(&(category_name, symbol)) {
            return Err(ApiError::BadRequest(format!(
                "Unit with symbol '{}' already registered in category '{}'",
                symbol, category_name
            )));
        }
        
        // Register the unit
        self.units.insert((category_name, symbol), Box::new(unit));
        
        // Add to the category's units
        if let Some(units) = self.category_units.get_mut(category_name) {
            units.insert(symbol);
        }
        
        Ok(())
    }
    
    /// Get all registered categories
    pub fn get_categories(&self) -> Vec<&dyn UnitCategory> {
        self.categories.values()
            .map(|c| c.as_ref())
            .collect()
    }
    
    /// Get a category by name
    pub fn get_category(&self, name: &str) -> Option<&dyn UnitCategory> {
        self.categories.get(name).map(|c| c.as_ref())
    }
    
    /// Get all units for a specific category
    /// Note: we're using 'static as a workaround for the registry keys which are static,
    /// but this isn't restrictive for how the API is used.
    pub fn get_units_for_category(&self, category_name: &'static str) -> Vec<&dyn AnyUnit> {
        if let Some(symbols) = self.category_units.get(category_name) {
            let mut result = Vec::new();
            for &symbol in symbols {
                if let Some(unit) = self.units.get(&(category_name, symbol)) {
                    result.push(unit.as_ref());
                }
            }
            result
        } else {
            Vec::new()
        }
    }
    
    /// Get a unit by category name and symbol
    /// Note: we're using 'static as a workaround for the registry keys which are static strings,
    /// but we clone the strings in the service implementation to make it user-friendly
    pub fn get_unit(&self, category_name: &'static str, symbol: &'static str) -> Option<&dyn AnyUnit> {
        self.units.get(&(category_name, symbol)).map(|u| u.as_ref())
    }
}

// Global registry instance
static REGISTRY: Lazy<RwLock<UnitRegistry>> = Lazy::new(|| {
    RwLock::new(UnitRegistry::new())
});

/// Get the global unit registry
pub fn registry() -> &'static RwLock<UnitRegistry> {
    &REGISTRY
} 