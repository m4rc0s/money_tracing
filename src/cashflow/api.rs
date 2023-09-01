use actix_web::{post, web, HttpResponse, Responder};

use crate::cashflow::{servicing, model::Movement};

#[post("/movements/")]
pub async fn register_movement(movement: web::Json<Movement>) -> impl Responder {
    let body = movement.into_inner();
    let movement = Movement { 
        amount: body.amount, 
        description: body.description, 
        effect: body.effect
    };
    let result = servicing::register_movement(movement);
    HttpResponse::Ok().body(result)
}
