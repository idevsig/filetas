use std::{env, net::SocketAddr};

use axum::{
    Router,
    body::{self, Body},
    extract::Request,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{any, get},
};
use regex::Regex;
use url::Url;

// 静态文件扩展名，用于判断是否为下载链接
static FILE_EXT: &str = ""; // 逗号分隔的扩展名列表

// 加载错误模板并返回响应
fn error_response(info: &str, status: StatusCode) -> impl IntoResponse {
    let html_content =
        std::fs::read_to_string("templates/error.html").unwrap_or_else(|_| info.to_string());
    let response_body = html_content.replace("{{ info }}", info);
    (status, Html(response_body))
}

// 流式传输响应体
async fn stream_response(resp: reqwest::Response) -> impl IntoResponse {
    let status = resp.status();
    let resp_headers = resp.headers().clone();
    let stream = resp.bytes_stream();
    let body = Body::from_stream(stream);
    let mut response = Response::new(body);
    let headers = response.headers_mut();

    // add content-type, content-length
    if let Some(content_type) = resp_headers.get(header::CONTENT_TYPE) {
        headers.insert(header::CONTENT_TYPE, content_type.clone());
    }
    if let Some(content_length) = resp_headers.get(header::CONTENT_LENGTH) {
        headers.insert(header::CONTENT_LENGTH, content_length.clone());
    }

    headers.insert(header::ACCESS_CONTROL_EXPOSE_HEADERS, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());

    // delete some headers
    headers.remove(header::SET_COOKIE);
    headers.remove(header::CONTENT_SECURITY_POLICY);
    headers.remove(header::CONTENT_SECURITY_POLICY_REPORT_ONLY);

    *response.status_mut() = status;
    response.into_response()
}

// 处理 / 请求
async fn handler_index() -> Html<String> {
    let title = env::var("TITLE").unwrap_or_else(|_| "文件加速下载".to_string());
    let html_content = std::fs::read_to_string("templates/index.html")
        .unwrap_or_else(|_| "Error: Could not load index.html".to_string());

    Html(html_content.replace("{{ title }}", &title))
}

// 处理 favicon.ico 请求
async fn handler_favicon() -> impl IntoResponse {
    // svg string
    let svg_data = r##"<?xml version="1.0" encoding="UTF-8"?><svg width="24" height="24" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M11.6777 20.271C7.27476 21.3181 4 25.2766 4 30C4 35.5228 8.47715 40 14 40C14.9474 40 15.864 39.8683 16.7325 39.6221" stroke="#333" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M36.0547 20.271C40.4577 21.3181 43.7324 25.2766 43.7324 30C43.7324 35.5228 39.2553 40 33.7324 40C32.785 40 31.8684 39.8683 30.9999 39.6221" stroke="#333" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M36 20C36 13.3726 30.6274 8 24 8C17.3726 8 12 13.3726 12 20" stroke="#333" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M17.0654 30.119L23.9999 37.0764L31.1318 30" stroke="#333" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M24 20V33.5382" stroke="#333" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/></svg>"##;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/svg+xml".parse().unwrap());

    (StatusCode::OK, headers, svg_data)
}

// 处理 robots.txt 请求
async fn handler_robots() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());

    (StatusCode::OK, headers, "User-agent:*\nDisallow:/")
}

// 处理下载请求
async fn handler_download(req: Request<Body>) -> impl IntoResponse {
    println!("");
    println!("{:?}", req);

    let uri = req.uri().to_string();

    // 去除最前面的所有 / 符号
    let re = Regex::new(r"^/*").unwrap();
    let mut target_url = re.replace_all(&uri, "").to_string();

    target_url = match normalize_url(&target_url) {
        Some(url) => url,
        None => {
            return (StatusCode::BAD_REQUEST, "Invalid URL").into_response();
        }
    };

    println!("{:?}", target_url);

    println!("{:?}", req.method());
    println!("{:?}", req.uri());
    println!("{:?}", req.headers());
    println!("");
    println!("");

    let method = reqwest::Method::from_bytes(req.method().as_str().as_bytes()).unwrap();
    println!("{:?}", method);
    println!("{:?}", target_url);
    let headers = req.headers().clone();
    // let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX).await.unwrap();
    // let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    let body_str = String::new();
    // let headers = HeaderMap::new();

    if let Ok(resp) = send_request(method, &target_url, &headers, Some(body_str)).await {
        stream_response(resp).await.into_response()
    } else {
        error_response(
                "Error: Failed to send request",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response()
    }
}

// 发送请求
async fn send_request(
    method: reqwest::Method,
    url: &str,
    headers: &HeaderMap,
    body: Option<String>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        // .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let mut builder = client.request(method.clone(), url);

    println!("url: {:?}", &url);

    // for (key, value) in headers.iter() {
    //     println!("key: {:?}, value: {:?}", key, value);
    //     println!("");
    //     if key == header::HOST {
    //         let parsed_url = Url::parse(&url) .unwrap();
    //         let new_value = parsed_url.host_str().unwrap().to_string();
    //         let header_value = HeaderValue::from_str(&new_value).unwrap();
    //         builder = builder.header(key, header_value);
    //         continue;
    //     }
    //     builder = builder.header(key, value);
    // }

    if let Some(body) = body.as_ref() {
        builder = builder.body(body.clone());
    }

    let request = builder.build()?;
    println!("a");
    println!("req headers: {:?}", request.headers());
    let response = client.execute(request).await?;
    println!("b");
    let new_headers = response.headers().clone();
    println!("new headers: {:?}", new_headers);

    // 检查是否是重定向响应
    if response.status().is_redirection() {
        // 获取重定向目标 URL
        if let Some(location) = response.headers().get(header::LOCATION) {
            println!("location: {:?}", location);
            if let Ok(location_str) = location.to_str() {
                println!("Redirecting1 to: {}", location_str);
                return Box::pin(send_request(method, &location_str, &new_headers, body)).await;
            }
        }
    }

    println!("c: {}", &url);
    Ok(response)
}

// 规范化 URL（若无协议，添加 https://）
fn normalize_url(uri: &str) -> Option<String> {
    if uri.starts_with("http://") || uri.starts_with("https://") {
        Some(uri.to_string())
    } else if !FILE_EXT.is_empty() && is_download_url(uri) {
        Some(format!("https://{}", uri))
    } else {
        let url = format!("https://{}", uri);
        if let Ok(parsed_url) = Url::parse(&url) {
            Some(parsed_url.to_string())
        } else {
            None
        }
    }
}

// 判断是否为下载链接
fn is_download_url(url: &str) -> bool {
    if let Ok(parsed_url) = Url::parse(url) {
        if let Some(segments) = parsed_url.path_segments() {
            if let Some(last_segment) = segments.last() {
                return last_segment
                    .split('.')
                    .last()
                    .map(|extension| {
                        FILE_EXT
                            .split(',')
                            .any(|ext| ext.eq_ignore_ascii_case(extension))
                    })
                    .unwrap_or(false);
            }
        }
    }
    false
}

#[tokio::main]
async fn main() {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let app = Router::new()
        .route("/", get(handler_index))
        .route("/favicon.ico", get(handler_favicon))
        .route("/robots.txt", get(handler_robots))
        .route("/{*uri}", any(handler_download));

    let addr = SocketAddr::new(host.parse().unwrap(), port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
