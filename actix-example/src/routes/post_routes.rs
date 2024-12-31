use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares};

pub fn config(config: &mut web::ServiceConfig){
    config
    .service(web::scope("secure/post")
    .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
    .service(handlers::post_handlers::create_post)
    .service(handlers::post_handlers::my_posts)
    )
    .service(web::scope("/post")
    .service(handlers::post_handlers::one_post)
    .service(handlers::post_handlers::all_posts)); 
} 