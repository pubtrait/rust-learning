mod config;
mod models;
mod routes;

fn main() {
    println!("Hello, world!");
    config::print_config();

    routes::health_routes::print_health_route();
    routes::user_route::user_route();
    println!("{}", "hello world");
    println!("{}", "good motion");
    routes::health_routes::print_health_route();
    routes::health_routes::print_health_route();
}
