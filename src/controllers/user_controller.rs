use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Responder};
use bson::doc;
use log::{error, info};
use serde::Serialize;
use validator::Validate;

use crate::entity::user::{EmailQuery, NewUser};

#[derive(Serialize)]
struct ValidationErrorResponse {
    errors: validator::ValidationErrors,
}

#[get("/users")]
async fn get_users(app_data: web::Data<crate::AppState>) -> impl Responder {
    info!("Fetching users...");
    let data = app_data.service_manager.user.get_users().await;
    return match data {
        Ok(result) => {
            if result.len() == 0 {
                return HttpResponse::build(StatusCode::NOT_FOUND).json(doc! {
                    "error": {
                        "code": 404,
                        "message": "No user found."
                    }
                });
            }
            HttpResponse::build(StatusCode::OK).json(result)
        }
        Err(e) => {
            error!("Get Users Error, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    };
}

#[get("/users/checkEmail")]
async fn check_email(
    app_data: web::Data<crate::AppState>,
    query: web::Query<EmailQuery>,
) -> impl Responder {
    info!("Validating query param...");
    match query.validate() {
        Ok(_) => {
            info!("Validation successful.");
        }
        Err(errors) => {
            error!("Validation failed.");
            return HttpResponse::build(StatusCode::OK)
                .json(ValidationErrorResponse { errors: errors });
        }
    };

    let email: &String = query.email.as_ref().unwrap();
    info!("Checking if email is taken...");
    if app_data
        .service_manager
        .user
        .check_email_taken(&email)
        .await
    {
        error!("Email {:?} is already taken.", email);
        return HttpResponse::build(StatusCode::OK).json(doc! {
            "errors": {
                "email": [
                    {
                        "code": "unique",
                        "message": "A user already exists with this email.",
                        "params": {
                            "value": &email
                        }
                    }
                ]
            }
        });
    }

    return HttpResponse::build(StatusCode::OK).json(doc! {
        "success": {
            "email": {
                "message": "Email is valid.",
                "params": {
                    "value": &email
                }
            }
        }
    });
}

#[post("/users")]
async fn add_user(
    app_data: web::Data<crate::AppState>,
    user: web::Json<NewUser>,
) -> impl Responder {
    info!("Validating user data...");
    match user.validate() {
        Ok(_) => {
            info!("Validation successful.");
        }
        Err(errors) => {
            error!("Validation failed.");
            return HttpResponse::build(StatusCode::OK)
                .json(ValidationErrorResponse { errors: errors });
        }
    };

    let email: &String = user.email.as_ref().unwrap();
    info!("Checking if email is taken...");
    if app_data
        .service_manager
        .user
        .check_email_taken(&email)
        .await
    {
        error!("Email {:?} is already taken.", email);
        return HttpResponse::build(StatusCode::OK).json(doc! {
            "errors": {
                "email": [
                    {
                        "code": "unique",
                        "message": "A user already exists with this email.",
                        "params": {
                            "value": &email
                        }
                    }
                ]
            }
        });
    }

    info!("Creating new user.");
    let result = app_data.service_manager.user.add_user(&user).await;
    match result {
        Ok(result) => {
            info!("Fetching new user.");
            let user_result = app_data
                .service_manager
                .user
                .find_by_id(result.inserted_id.as_object_id().unwrap())
                .await;
            match user_result {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(e) => {
                    error!("Error occurred while creating user: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            error!("Error occurred while creating user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(add_user);
    cfg.service(check_email);
}
