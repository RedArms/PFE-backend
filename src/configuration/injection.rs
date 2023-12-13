
use actix_web::web;
use crate::AppState;
use crate::service::{
    item_service::ItemService, user_service::UserService, order_service::OrderService, tours_service::ToursService,
    boxe_service::BoxeService, client_service::ClientService};
use crate::repository::{item_repository::ItemRepository, user_repository::UserRepository, order_repository::OrderRepository,
                        tours_repository::ToursRepository, boxe_repository::BoxeRepository, client_repository::ClientRepository};

/// Injects and configures application data, services, and repositories.
pub fn configure_app(app: &mut web::ServiceConfig, app_state: &AppState) {
    let item_repo = ItemRepository::new(web::Data::new(app_state.clone()));
    let item_service = ItemService::new(item_repo.clone());

    let user_repo = UserRepository::new(web::Data::new(app_state.clone()));
    let user_service = UserService::new(user_repo.clone());

    let order_repo = OrderRepository::new(web::Data::new(app_state.clone()));
    let order_service = OrderService::new(order_repo.clone());

    let tours_repo = ToursRepository::new(web::Data::new(app_state.clone()));
    let tours_service = ToursService::new(tours_repo.clone(), order_repo.clone());

    let boxe_repo = BoxeRepository::new(web::Data::new(app_state.clone()));
    let boxe_service = BoxeService::new(boxe_repo.clone());

    let client_repo = ClientRepository::new(web::Data::new(app_state.clone()));
    let client_service = ClientService::new(client_repo.clone(), boxe_repo.clone(), order_repo.clone());

    // Register services and repositories in Actix web's application data
    app.app_data(web::Data::new(item_service.clone()))
        .app_data(web::Data::new(user_service.clone()))
        .app_data(web::Data::new(order_service.clone()))
        .app_data(web::Data::new(tours_service.clone()))
        .app_data(web::Data::new(boxe_service.clone()))
        .app_data(web::Data::new(client_service.clone()));
}

