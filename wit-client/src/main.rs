// main.rs
use tokio::{io::BufStream, net::TcpListener};
use tokio::io::AsyncWriteExt;
use tracing::info;
use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};
mod req;

// wasmtime::component::bindgen!(
//     {
//         world:"host",
//         path: "../wit-server/wit/host.wit",
//     }
// );

static DEFAULT_PORT: &str = "8080";

struct Fib;
// impl Host_Imports for Fib {
//     fn run(&mut self, n:u32)->wasmtime::Result<u32>{
//         Ok(n*n)
//     }
// }
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the default tracing subscriber.
    tracing_subscriber::fmt::init();

    // let mut config = Config::new();
    // config.wasm_component_model(true);
    //
    // let engine = Engine::new(&config)?;
    // let mut store = Store::new(
    //     &engine, Fib{}
    // );
    //
    //
    // let mut linker = Linker::new(&engine);
    // Host_::add_to_linker(&mut linker, |state: &mut Fib| state);
    //
    // let component = Component::from_file(&engine, "../wit-server/my-component.wasm")?;
    //
    // let (host,_) = Host_::instantiate(&mut store, &component, &linker)?;
    //
    // let k = host.call_execute(&mut store,10).expect("no shit");
    //
    // let y =host.bar().call_getheader(&mut store).expect("ohmh");
    // print!("WASMTIME RESPONDING {} {} ", k, y);
    //

    let port: u16 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_PORT.to_string())
        .parse()?;

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    info!("listening on: {}", listener.local_addr()?);

    loop {
        let (stream, addr) = listener.accept().await?;
        let mut stream = BufStream::new(stream);

        // do not block the main thread, spawn a new task
        tokio::spawn(async move {
            info!(?addr, "new connection");
            loop {
                let req = match req::parse_request(&mut stream).await {
                    Ok(()) => {
                        println!("done");
                        break;
                    }
                    Err(_) => {
                        println!( "failed to parse request");
                        break;
                    }
                };
            }


        });
    }
}