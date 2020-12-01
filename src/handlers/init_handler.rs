use warp;

pub async fn init() -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("Hello, Welcome to Event Tracker!\r\nNothing here for you ;)");
    Ok(warp::reply::html(reply))
}