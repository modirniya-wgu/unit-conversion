use crate::models::{registry};
use crate::errors::ApiError;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;

// Static string storage to ensure we can use 'static strings with the registry
static CATEGORY_CACHE: Lazy<RwLock<HashMap<String, &'static str>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

static UNIT_CACHE: Lazy<RwLock<HashMap<String, &'static str>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

use std::sync::RwLock;

// Helper to get or cache a 'static string
fn get_static_str(s: &str, cache: &RwLock<HashMap<String, &'static str>>) -> &'static str {
    {
        // Try to read from cache first
        if let Ok(cache_read) = cache.read() {
            if let Some(static_str) = cache_read.get(s) {
                return *static_str;
            }
        }
    }
    
    // If not in cache, create a new static str and store it
    // This leaks memory intentionally to get 'static lifetime
    let static_str = Box::leak(s.to_string().into_boxed_str());
    
    if let Ok(mut cache_write) = cache.write() {
        cache_write.insert(s.to_string(), static_str);
    }
    
    static_str
}

/// Data transfer object for conversion requests
#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionRequest {
    pub value: f64,
    pub from_category: String,
    pub from_unit: String,
    pub to_unit: String,
}

/// Data transfer object for conversion responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResponse {
    pub from_value: f64,
    pub from_unit: String,
    pub to_value: f64,
    pub to_unit: String,
    pub category: String,
}

/// Service for handling unit conversions
pub struct ConversionService;

impl ConversionService {
    /// Convert a value from one unit to another
    pub fn convert(request: ConversionRequest) -> Result<ConversionResponse, ApiError> {
        let registry = registry().read().map_err(|_| {
            ApiError::InternalError("Failed to access unit registry".to_string())
        })?;
        
        // Convert to static strings for registry lookup
        let from_category = get_static_str(&request.from_category, &CATEGORY_CACHE);
        let from_unit = get_static_str(&request.from_unit, &UNIT_CACHE);
        let to_unit = get_static_str(&request.to_unit, &UNIT_CACHE);
        
        // Get the source unit
        let from_unit_impl = registry.get_unit(from_category, from_unit)
            .ok_or_else(|| ApiError::NotFound(format!(
                "Unit '{}' not found in category '{}'", 
                request.from_unit, request.from_category
            )))?;
            
        // Get the target unit
        let to_unit_impl = registry.get_unit(from_category, to_unit)
            .ok_or_else(|| ApiError::NotFound(format!(
                "Unit '{}' not found in category '{}'", 
                request.to_unit, request.from_category
            )))?;
            
        // Ensure units are compatible (same category)
        if from_unit_impl.category_name() != to_unit_impl.category_name() {
            return Err(ApiError::BadRequest(format!(
                "Cannot convert between different categories: '{}' and '{}'",
                from_unit_impl.category_name(), to_unit_impl.category_name()
            )));
        }
        
        // Convert to base unit then to target unit
        let base_value = from_unit_impl.to_base(request.value);
        let target_value = to_unit_impl.from_base(base_value);
        
        Ok(ConversionResponse {
            from_value: request.value,
            from_unit: from_unit_impl.symbol().to_string(),
            to_value: target_value,
            to_unit: to_unit_impl.symbol().to_string(),
            category: from_unit_impl.category_name().to_string(),
        })
    }
    
    /// List all available unit categories
    pub fn list_categories() -> Result<Vec<String>, ApiError> {
        let registry = registry().read().map_err(|_| {
            ApiError::InternalError("Failed to access unit registry".to_string())
        })?;
        
        Ok(registry.get_categories()
            .iter()
            .map(|c| c.name().to_string())
            .collect())
    }
    
    /// List all units for a specific category
    pub fn list_units(category_name: &str) -> Result<Vec<String>, ApiError> {
        let registry = registry().read().map_err(|_| {
            ApiError::InternalError("Failed to access unit registry".to_string())
        })?;
        
        let static_category = get_static_str(category_name, &CATEGORY_CACHE);
        
        // Verify the category exists
        if registry.get_category(static_category).is_none() {
            return Err(ApiError::NotFound(format!("Category '{}' not found", category_name)));
        }
        
        Ok(registry.get_units_for_category(static_category)
            .iter()
            .map(|u| u.symbol().to_string())
            .collect())
    }
    
    /// Compare two measurements
    pub fn compare(
        value1: f64, 
        unit1: &str, 
        value2: f64, 
        unit2: &str, 
        category: &str
    ) -> Result<i8, ApiError> {
        let registry = registry().read().map_err(|_| {
            ApiError::InternalError("Failed to access unit registry".to_string())
        })?;
        
        // Convert to static strings for registry lookup
        let static_category = get_static_str(category, &CATEGORY_CACHE);
        let static_unit1 = get_static_str(unit1, &UNIT_CACHE);
        let static_unit2 = get_static_str(unit2, &UNIT_CACHE);
        
        // Get the first unit
        let unit1_impl = registry.get_unit(static_category, static_unit1)
            .ok_or_else(|| ApiError::NotFound(format!(
                "Unit '{}' not found in category '{}'", unit1, category
            )))?;
            
        // Get the second unit
        let unit2_impl = registry.get_unit(static_category, static_unit2)
            .ok_or_else(|| ApiError::NotFound(format!(
                "Unit '{}' not found in category '{}'", unit2, category
            )))?;
            
        // Convert both to base units for comparison
        let base_value1 = unit1_impl.to_base(value1);
        let base_value2 = unit2_impl.to_base(value2);
        
        // Compare and return the result
        if (base_value1 - base_value2).abs() < f64::EPSILON {
            Ok(0) // Equal
        } else if base_value1 < base_value2 {
            Ok(-1) // Less than
        } else {
            Ok(1) // Greater than
        }
    }
} 