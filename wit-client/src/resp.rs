use std::io::Cursor;
use maplit::hashmap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use crate::{Host_, WASM, run_fib};

pub async fn send_resp(stream: &mut (impl AsyncBufReadExt + Unpin + AsyncWriteExt), n: i32, host: &mut WASM) -> anyhow::Result<()>{
    println!("done");
    let ans = host.host.call_execute(&mut host.s, n as u32).unwrap();

    
    let pj = format!(  r#"{{'resp' : {}}}"#,ans);


    // let ans = run_fib(n as u32);
    println!("called run {}",ans);

    let data = pj.into_bytes();
    let headers1 = hashmap! {
                                    "Content-Length".to_string() => data.len().to_string(),
                                    "Content-Type".to_string() => String::from("application/json"),
                                };
    let mut res = Box::new(Cursor::new(data));

    let headers =
        headers1
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");
    let resp =    format!("HTTP/1.1 {}\r\n{headers}\r\n\r\n", 201);

    stream.write_all(&String::from(resp).into_bytes()).await.expect("error in writing response");

    tokio::io::copy(&mut res,  stream).await.expect("reso copy failed");
    Ok(())
}