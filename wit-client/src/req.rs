#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub data: String
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Method {
    Get,
    Post
}

use std::{collections::HashMap, hash::Hash};
use std::io::Cursor;
use maplit::hashmap;
use serde::de::Unexpected::Str;

use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use serde_json::{Result, Value};
use crate::{Host_, WASM};
use crate::resp::send_resp;
// [...]

impl TryFrom<&str> for Method {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            m => Err(anyhow::anyhow!("unsupported method: {m}")),
        }
    }
}

pub async fn parse_request(stream: &mut (impl AsyncBufReadExt + Unpin + AsyncWriteExt), host: &mut WASM) -> anyhow::Result<Request> {
    let mut line_buffer = String::new();
    //let mut buff = [0;100000];
    //stream.read(&mut buff);
    stream.read_line(&mut line_buffer).await?;
    // let mut buf = vec![];




    let mut parts = line_buffer.split_whitespace();

    let method: Method = parts
        .next()
        .ok_or(anyhow::anyhow!("missing method"))
        .and_then(TryInto::try_into)?;

    let path: String = parts
        .next()
        .ok_or(anyhow::anyhow!("missing path"))
        .map(Into::into)?;

    let mut headers = HashMap::new();

    loop {
        line_buffer.clear();
        let r =stream.read_line(&mut line_buffer).await.unwrap();
        print!("{}",line_buffer);

        if line_buffer.is_empty() || line_buffer == "\n" || line_buffer == "\r\n" {
            break;
        }
            let mut comps = line_buffer.split(":");
            let key = comps.next().ok_or(anyhow::anyhow!("missing header name"))?;
            let value = comps
                .next()
                .ok_or(anyhow::anyhow!("missing header value"))?
                .trim();

            headers.insert(key.to_string(), value.to_string());
    }

    let body_len = headers.get("Content-Length").unwrap_or_else(|| headers.get("content-length").unwrap());
    let mut content;
    let mut c= String::from("");
    
    content = vec![0;body_len.parse::<usize>().unwrap()];
    stream.read_exact(&mut content).await;
    c = String::from_utf8(content).unwrap();
    c = c.trim_matches(char::from(0)).to_string();
    println!("Content Length {:?}",c);
    
    let p =  json::parse(
        c.as_str()
    ).unwrap();
    let n  = p["n"].as_i32().unwrap();
    send_resp(stream,n,host).await?;

    Ok(Request{
        method:method,
        path:path,
        headers:headers,
        data:c
    })
}
