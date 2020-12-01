use warp::{self, Filter};
use console::Style;

mod api;
mod handlers;
mod routes;

use self::{
    routes::{
        init_route,
    },
    handlers::{
        init_handler,
    },
};

#[cfg(test)] mod tests;

#[tokio::main]
async fn main() {
    let addr: String = "0.0.0.0:8000".parse().unwrap();

    let end = init!().with(warp::log("Init method!"));

    let style = Style::new().blue();

    println!("\nRust Warp Server ready at {}", style.apply_to(&addr));
    println!("System Check... OK!");

    warp::serve(end).run(([0, 0, 0, 0], 8000)).await;
}