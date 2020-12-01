#[macro_export]
macro_rules! init {
    () => {
        init_route::init()
        .and_then(init_handler::init)
    }
}