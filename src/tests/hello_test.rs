use warp::Filter;

use crate::{
    handlers::init_handler,
    routes::init_route,
    init,
};

// $cargo test -- --nocapture if you want to use println! etc.

// or test just one function each time.
// For example, $cargo test hello and it passes.

#[cfg(test)]
mod tests {
    use super::*;

    // 1.
    #[tokio::test]
    async fn hello() {
        let res = warp::test::request() // 2.
            .method("GET")
            .path("/hello/eduardo")
            .reply(&init!()) // 3.
            .await;

        // 4.
        assert_eq!(res.status(), 200, "Should return 200 OK.");
        // 5.
        println!("{:#?}", res.body());
    }
}