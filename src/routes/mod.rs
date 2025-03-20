use actix_web::{web, HttpResponse, Responder};

mod conversion;

// Health check endpoint for Kubernetes/monitoring
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

// Root endpoint
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Unit Conversion API")
}

// Configure routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
       .route("/api/health", web::get().to(health_check))
       // Conversion routes
       .route("/api/categories", web::get().to(conversion::get_categories))
       .route("/api/categories/{category}/units", web::get().to(conversion::get_units))
       .route("/api/convert", web::post().to(conversion::convert))
       .route("/api/compare", web::post().to(conversion::compare));
} 