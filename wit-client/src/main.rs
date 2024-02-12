use std::collections::HashMap;
use std::io::Cursor;
use maplit::hashmap;
// main.rs
use tokio::{io::BufStream, net::TcpListener};
use tokio::io::AsyncWriteExt;
use tracing::info;
use serde_derive::{Deserialize, Serialize};

use warp::Filter;
use wasmtime::{self, component::{Component, Linker}, Config, Engine, Store};
use wasmtime::component::__internal::wasmtime_environ::object::U32;
use wasmtime::component::{ComponentNamedList, Val};

wasmtime::component::bindgen!(
    {
        world:"example",
        path: "world.wit",
    }
);


// wasmtime::component::bindgen!(
//     {
//         world:"host",
//         path: "../wit-server/wit/host.wit",
//     }
// );



static DEFAULT_PORT: &str = "8080";

pub struct Fib;
// impl HostImports for Fib {
//     fn run(&mut self, n:u32)->wasmtime::Result<u32>{
//         fn x(j:u32)->u32{
//             if j <=0{
//                return 0;
//             }
//             else if j == 1{
//                 return 1;
//             }
//             return x(j-1)+x(j-2);
//         }
//         Ok(x(n))
//     }
// }

#[derive(Deserialize, Serialize)]
struct WarpResponse{
    ans: u32,
}

fn doit(n:u32)->u32{
    n*10
}
async fn reply() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config).unwrap();
        // let mut store_map = HashMap::from([
        //     ("fib",Fib{})
        // ]);
        let mut store = Store::new(&engine,{});


        let mut linker = Linker::new(&engine);

        let component = Component::from_file(&engine, "../wit-server/my-component.wasm").unwrap();
        let composed = Component::from_file(&engine, "../composed/composed.wasm").unwrap();

        let cargocomp = Component::from_file(&engine, "../proxy/target/wasm32-unknown-unknown/release/proxy.wasm").unwrap();
        let (example,_) = Example::instantiate(&mut store, &composed, &linker).expect("why not");
    //     let mut host_instance = linker.root();
    //     let fib = Component::from_file(&engine, "../wit-server2/fib.wasm").unwrap();
    //
    //     host_instance.func_new(&component, "run", |_,params, results|{
    //         let Val::U32(mut value) = params[0] else { panic!("not possible") };
    //         value*= 2;
    //         results[0] = Val::U32(value);
    //         Ok(())
    //     }).expect("TODO: panic message");
    //     let host = linker.instantiate(&mut store, &component).expect("pacnhod");
    //
    //     let fib_instance = linker.instantiate(&mut store, &fib).expect("pacnhod");
        let preinstantiate = linker.instantiate_pre(&cargocomp).expect("TODO: panic message");

        let hello_instance = preinstantiate.instantiate(&mut store).expect("pacnhod");
        let r = example.interface0.call_run(&mut store, 2).expect("not hey");
        // let exp = hello_instance.exports(&mut store);
    //
    //     let run = host.get_typed_func::<(u32,),(u32,)>(&mut store, "execute").expect("wasnt a func");
    //     let fib_func = fib_instance.get_typed_func::<(u32,),(u32,)>(&mut store, "fib").expect("something wrong");
    // let hello_func = hello_instance.proxy().get_typed_func::<(u32,),(u32,)>(&mut store, "middleware").expect("something wrong");
    //     let he = hello_func.call(&mut store,(10,)).expect("not working helo");
    // // let k = run.call(&mut store,(10,)).expect("aoye");
    //     let y = fib_func.call(&mut store, (1,)).expect("p");
        // let (host,_) = Host_::instantiate(&mut store, &component, &linker).unwrap();


        // let (second,_) = Second::\
    // nstantiate(&mut store, &component, &linker).unwrap();
        // let k = host.call_execute(&mut store,1).expect("no shit");
        //
        // let y =host.bar().call_getheader(&mut store).expect("ohmh");
        // let warp_response = WarpResponse{
        //     ans:k
        // };

        // let z = second.call_count(&mut store).expect("abcd");


        Ok(Box::new(format!("sup {}",r)))
}
#[tokio::main]
async fn main() {
    // Initialize the default tracing subscriber.
    tracing_subscriber::fmt::init();

    let routes = warp::get().and(warp::path("wasm")).and_then(reply);

    warp::serve(routes).run(([127, 0, 0, 1], 3032)).await;

}