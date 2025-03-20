use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::services::{ConversionService, ConversionRequest};

/// Get all available unit categories
pub async fn get_categories() -> impl Responder {
    match ConversionService::list_categories() {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "failed_to_list_categories",
            "message": err.to_string()
        }))
    }
}

/// Get all units for a specific category
pub async fn get_units(path: web::Path<String>) -> impl Responder {
    let category = path.into_inner();
    
    match ConversionService::list_units(&category) {
        Ok(units) => HttpResponse::Ok().json(units),
        Err(err) => match err {
            crate::errors::ApiError::NotFound(_) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "category_not_found",
                    "message": err.to_string()
                }))
            },
            _ => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "failed_to_list_units",
                "message": err.to_string()
            }))
        }
    }
}

/// Convert a value from one unit to another
pub async fn convert(request: web::Json<ConversionRequest>) -> impl Responder {
    match ConversionService::convert(request.into_inner()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => match err {
            crate::errors::ApiError::NotFound(_) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "unit_not_found",
                    "message": err.to_string()
                }))
            },
            crate::errors::ApiError::BadRequest(_) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "invalid_conversion",
                    "message": err.to_string()
                }))
            },
            _ => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "conversion_failed",
                "message": err.to_string()
            }))
        }
    }
}

/// Request DTO for comparing two measurements
#[derive(Debug, Serialize, Deserialize)]
pub struct CompareRequest {
    pub value1: f64,
    pub unit1: String,
    pub value2: f64,
    pub unit2: String,
    pub category: String,
}

/// Response DTO for comparison results
#[derive(Debug, Serialize, Deserialize)]
pub struct CompareResponse {
    pub result: i8,
    pub relation: String,
}

/// Compare two measurements
pub async fn compare(request: web::Json<CompareRequest>) -> impl Responder {
    match ConversionService::compare(
        request.value1,
        &request.unit1,
        request.value2,
        &request.unit2,
        &request.category
    ) {
        Ok(result) => {
            let relation = match result {
                -1 => "less_than",
                0 => "equal",
                1 => "greater_than",
                _ => unreachable!(),
            };
            
            HttpResponse::Ok().json(CompareResponse {
                result,
                relation: relation.to_string(),
            })
        },
        Err(err) => match err {
            crate::errors::ApiError::NotFound(_) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "unit_or_category_not_found",
                    "message": err.to_string()
                }))
            },
            _ => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "comparison_failed",
                "message": err.to_string()
            }))
        }
    }
} 