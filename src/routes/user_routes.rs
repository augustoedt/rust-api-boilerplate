use actix_web::web;

use crate::controllers::user_controller::{delete_user, get_users};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/").route(web::get().to(get_users)));
    config.service(web::resource("/post").route(web::post().to(delete_user)));
}
