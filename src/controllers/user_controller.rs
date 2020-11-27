use crate::entity::user::User;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/users")]
async fn get_users(app_data: web::Data<crate::AppState>) -> impl Responder {
    let data = app_data.service_manager.user.get_users();
    let result = web::block(move || data).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Get Users Error, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/users")]
async fn add_user(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
    let action = app_data.service_manager.user.add_user(&user);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Add Use Error, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(add_user);
}
