#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
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

pub async fn parse_request(stream: &mut (impl AsyncBufReadExt + Unpin + AsyncWriteExt)) -> anyhow::Result<()> {
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

    let body_len = headers.get("Content-Length");
    let mut content;
    let mut c= String::from("");
    if let Some(l) = body_len{
        content = vec![0;l.parse::<usize>().unwrap()];
        stream.read_exact(&mut content).await;
        c = String::from_utf8(content).unwrap();
        println!("Content Length {:?}",c);
    }
    let p =  json::parse(
        c.as_str()
    ).unwrap();
    println!("a {}",p["fs"]);
    let pj = String::from(  r#"
        {
        "resp" : "abc"
        }
        "#);

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


    stream.write_all(&String::from(resp).into_bytes()).await?;

    tokio::io::copy(&mut res, stream).await?;




    Ok(())
}