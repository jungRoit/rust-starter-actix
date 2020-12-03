use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Responder};
use bson::doc;
use log::{error, info};
use validator::Validate;

use crate::entity::user::{EmailQuery, NewUser, UsernameQuery};

#[get("/users")]
async fn get_users(app_data: web::Data<crate::AppState>) -> impl Responder {
    info!("Fetching users...");
    let data = app_data.service_manager.user.get_users().await;
    match data {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            error!("Get Users Error, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/users/checkEmail")]
async fn check_email(
    app_data: web::Data<crate::AppState>,
    query: web::Query<EmailQuery>,
) -> impl Responder {
    info!("Validating query param...");
    match query.validate() {
        Ok(_) => (),
        Err(errors) => {
            error!("Validation failed.");
            return HttpResponse::build(StatusCode::BAD_REQUEST).json(errors);
        }
    };

    info!("Checking if email is taken...");
    if app_data
        .service_manager
        .user
        .check_email_taken(&query.email)
        .await
    {
        return HttpResponse::build(StatusCode::BAD_REQUEST).json(doc! {
            "email": [
                {
                    "code": "unique",
                    "message": "A user already exists with this email.",
                    "params": {
                        "value": &query.email
                    }
                }
            ]
        });
    }

    return HttpResponse::Ok().finish();
}

#[get("/users/checkUsername")]
async fn check_username(
    app_data: web::Data<crate::AppState>,
    query: web::Query<UsernameQuery>,
) -> impl Responder {
    info!("Validating query param...");
    match query.validate() {
        Ok(_) => (),
        Err(errors) => {
            error!("Validation failed.");
            return HttpResponse::build(StatusCode::BAD_REQUEST).json(errors);
        }
    };

    info!("Checking if username is taken...");
    if app_data
        .service_manager
        .user
        .check_username_taken(&query.username)
        .await
    {
        return HttpResponse::build(StatusCode::BAD_REQUEST).json(doc! {
            "username": [
                {
                    "code": "unique",
                    "message": "Username already taken.",
                    "params": {
                        "value": &query.username
                    }
                }
            ]
        });
    }

    return HttpResponse::Ok().finish();
}

#[post("/users")]
async fn add_user(
    app_data: web::Data<crate::AppState>,
    user: web::Json<NewUser>,
) -> impl Responder {
    info!("Validating user data...");
    match user.validate() {
        Ok(_) => (),
        Err(errors) => {
            error!("Validation failed.");
            return HttpResponse::build(StatusCode::BAD_REQUEST).json(errors);
        }
    };

    info!("Checking if email is taken...");
    if app_data
        .service_manager
        .user
        .check_email_taken(&user.email)
        .await
    {
        return HttpResponse::build(StatusCode::BAD_REQUEST).json(doc! {
            "email": [
                {
                    "code": "unique",
                    "message": "A user already exists with this email.",
                    "params": {
                        "value": &user.email
                    }
                }
            ]
        });
    }

    info!("Checking if username is taken...");
    if app_data
        .service_manager
        .user
        .check_username_taken(&user.username)
        .await
    {
        return HttpResponse::build(StatusCode::BAD_REQUEST).json(doc! {
            "username": [
                {
                    "code": "unique",
                    "message": "Username already taken.",
                    "params": {
                        "value": &user.username
                    }
                }
            ]
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
    cfg.service(check_username);
}
