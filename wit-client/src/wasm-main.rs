use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};

wasmtime::component::bindgen!("host" in "../wit-server/wit/host.wit");


fn main()->Result<(), Box<dyn std::error::Error>> {


    Ok(())
}
