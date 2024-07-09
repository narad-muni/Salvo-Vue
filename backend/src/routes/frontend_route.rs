use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use std::env;

pub fn frontend_route() -> Router{

    if env::var("env").unwrap() == "development" {

        Router::with_path("<**path>").goal(
            Proxy::use_hyper_client(["http://localhost:5173"])
        )
    }else{
        Router::with_path("<**path>").get(
            StaticDir::new([
                "./public",
            ])
            .fallback("index.html")
            .defaults("index.html")
            .auto_list(true)
        )
    }
}