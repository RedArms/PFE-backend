use actix_web::{web, App};

use crate::routes::{auth, boxe, clients, index, items, tours, users};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    auth::configure_routes(cfg);
    boxe::configure_routes(cfg);
    clients::configure_routes(cfg);
    tours::configure_routes(cfg);
    users::configure_routes(cfg);
    items::configure_routes(cfg);
    index::configure_routes(cfg);
}
