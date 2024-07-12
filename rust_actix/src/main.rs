use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use log::info;

mod ml;
use ml::OnnxModel;

async fn greet() -> impl Responder {
    HttpResponse::Ok().json(json!({"Hello": "World", "backend": "actix-web"}))
}

async fn inference_handler(session: web::Data<OnnxModel>, query_text: web::Path<String>) -> impl Responder {
    info!("Handling request on thread: {:?}", std::thread::current().id());
    match session.predict_from_string(&query_text) {
        Ok(v) => {
            // If the prediction is successful, return the JSON response
            HttpResponse::Ok().json(json!({"model_out": v}))
        },
        Err(e) => {
            // Log the error and return an appropriate HTTP error response
            log::error!("Prediction failed for query: {}, error: {}", &query_text, e);
            HttpResponse::InternalServerError().json(json!({"error": "Prediction endpoint not working", "details": e.to_string()}))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let model: OnnxModel = OnnxModel::new("../tinybert-imdb", "model.onnx").unwrap();
    let model_data = web::Data::new(model);


    HttpServer::new(move || {
        App::new()
            .app_data(model_data.clone())
            .route("/", web::get().to(greet))
            .route("/{query_text}", web::get().to(inference_handler))
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}