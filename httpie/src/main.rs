use std::collections::HashMap;

use anyhow::{ anyhow, Ok, Result};
use clap::Parser;
use mime::Mime;
use reqwest::{ header, Client, Url, Response };
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

use console::style;

// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助


/// A naive httpie implementation with Rust, can you imagine how easy it is?


#[derive(Parser, Debug)]
#[command(name = "httpie", bin_name = "httpie")]
#[command(version = "1.0", author = "sunlee")]
enum Httpie {
    Get(Get),
    Post(Post),
}

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[arg(value_parser = parse_url)]
    url: String,
}

/// send post request
#[derive(Parser, Debug)]
struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>
}

#[derive(Debug, Clone, PartialEq)]
struct KvPair {
    key: String,
    value: String,
}

fn parse_url(s: &str) -> Result<String> {
    // 检查 URL 是否合法
    let url:Url = s.parse()?;
    Ok(url.into())
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    let mut iter = s.splitn(2, '=');
    let err = || anyhow!(format!("Failed to parse {}", s));
    let key = iter.next().ok_or_else(err)?;
    let value = iter.next().ok_or_else(err)?;
    Ok(KvPair {
        key: key.to_string(),
        value: value.to_string(),
    })
}

fn print_status(resp: &Response) {
    let status = style(format!("{:?} {}", resp.version(), resp.status())).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", style(name.to_string()).green(), value);
    }

    println!();
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn print_body(m: Option<Mime>, body: &str) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if (v) == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if (v) == mime::TEXT_HTML => print_syntect(body, "html"),

        // 默认直接输出
        _ => println!("{}", body),
    }
}

fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.key, &pair.value);
    }
    let resp = client
        .post(&args.url)
        .json(&body)
        .send()
        .await?;
    Ok(print_resp(resp).await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Httpie::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse().unwrap());
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("httpie"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts {
        Httpie::Get(ref args) => get(client, args).await?,
        Httpie:: Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                key: "a".into(),
                value: "1".into()
            }
        );

        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair  {
                key: "b".into(),
                value: "".into()
            }
        );
    }
}