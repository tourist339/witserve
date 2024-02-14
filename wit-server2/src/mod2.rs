use crate::mod2::exports::request::proxy::preproxy;

wit_bindgen::generate!({
    world:"proxy",
    path: "wit/proxy.wit",
    exports:{
        world:MyProxy,
        "request:proxy/preproxy":MyProxy
    },
});

struct MyProxy;

impl preproxy::Guest for MyProxy{
    fn pre(n:u32) -> u32 {
        n+1
    }
}

