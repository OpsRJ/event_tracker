use warp::{
    filters::BoxedFilter,
    Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("api")
        .and(warp::path("v1"))
        .boxed()
}

/* PARA PASSAR PARAMETRO
pub fn init() -> BoxedFilter<(String, )> {
    warp::get() 
        .and(path_prefix()) 
        .and(warp::path::param::<String>())
        .boxed()
} */

pub fn init() -> BoxedFilter<()> {
    warp::get() 
        .and(path_prefix()) 
        //.and(warp::path::param::<String>())
        .boxed()
}