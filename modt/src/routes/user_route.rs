use crate::models::user_model;

pub fn user_route() {
    // crate::models::user_model::print_user_model();
    user_model::print_user_model();

    println!("user_route");

    super::health_routes::print_health_route();
}
