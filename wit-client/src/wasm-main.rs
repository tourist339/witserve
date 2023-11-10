// use wasmtime::{
//     self,
//     component::{Component, Linker},
//     Config, Engine, Store,
// };

// wasmtime::component::bindgen!("host" in "../wit-server/wit/host.wit");


// fn main()->Result<(), Box<dyn std::error::Error>> {
//     let mut config = Config::new();
//     config.wasm_component_model(true);

//     let engine = Engine::new(&config)?;
//     let mut store = Store::new(
//         &engine,
//         0
//     );


//     let mut linker = Linker::new(&engine);

//     let component = Component::from_file(&engine, "../wit-server/my-component.wasm")?;

//     let (host,_) = Host_::instantiate(&mut store, &component, &linker)?;


//     let k = host.call_run(&mut store,10).expect("no shit");
//     print!("result {}",k);
//     Ok(())
// }
