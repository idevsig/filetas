use std::{env, net::SocketAddr};

use axum::{
    Router,
    body::Body,
    extract::Request,
    http::{self, HeaderMap, StatusCode, header},
    response::{Html, IntoResponse},
    routing::{any, get},
};
use regex::Regex;
use url::Url;

// 静态文件扩展名，用于判断是否为下载链接
static FILE_EXT: &str = ""; // 逗号分隔的扩展名列表

// 加载错误模板并返回响应
// fn error_response(info: &str, status: StatusCode, headers: &HeaderMap) -> Response {
//     println!("{:?}", headers);
//     if headers.contains_key(header::COOKIE) {
//         let html_content = std::fs::read_to_string("templates/error.html")
//             .unwrap_or_else(|_| info.to_string());
//         let response_body = html_content.replace("{{ info }}", info);
//         return (status, Html(response_body));
//     }

//     (status, Html(info.to_string()))
// }

// 代理请求处理
async fn proxy(uri: &str, req: Request) -> impl IntoResponse {
    // let (method, uri, headers) = (req.method().clone(), req.uri().clone(), req.headers().clone());

    // let mut req_builder = client.request(method.clone(), uri.to_string());

    // // 转移请求头
    // for (key, value) in headers.iter() {
    //     req_builder = req_builder.header(key, value);
    // }

    // // 转换请求体为数据流
    // let data_stream: BodyDataStream = req.into_body().into_data_stream();

    // // 处理数据流
    // let req_body = reqwest::Body::wrap_stream(data_stream);
    // req_builder = req_builder.body(req_body);

    // // 发送请求并获取响应
    // // let res = req_builder.send().await.map_err(|_| StatusCode::BAD_GATEWAY);

    // let response = match req_builder.send().await {
    //     Ok(resp) => {
    //         // 检查响应的 Location 头以处理重定向
    //         if let Some(location) = resp.headers().get("location") {
    //             let new_headers = resp.headers();
    //             if let Ok(location_str) = location.to_str() {
    //                 if let Ok(new_uri) = location_str.parse::<Uri>() {
    //                     let mut builder = Request::builder();

    //                     for (key, value) in new_headers.iter() {
    //                         builder = builder.header(key, value);
    //                     }

    //                     builder.method(method).
    //                         uri(new_uri).
    //                         body(Body::empty()).
    //                         map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    //                     // 递归调用以处理重定向
    //                     // proxy(new_uri.to_string().as_str(), req).await
    //                 }
    //             }
    //         }

    //         stream_response(resp).await
    //     }
    //     Err(_) => {
    //         // 在这里处理错误，例如返回特定的状态码或记录日志
    //         return Err(StatusCode::BAD_GATEWAY);
    //     }
    // };

    // Ok(response.into_response())

    // match client.request(method, uri, headers).send().await {
    //     Ok(resp) => {
    //         let status = resp.status();
    //         println!("{:?}", status);
    //         let headers = resp.headers().clone();

    //         if (headers.contains_key(header::LOCATION) || headers.contains_key(header::CONTENT_TYPE)) && is_download_url(uri) {
    //             let localtion = headers.get(header::LOCATION).unwrap().to_str().unwrap();
    //             let mut new_headers = headers.clone();
    //             new_headers.remove(header::LOCATION);
    //             return proxy(localtion, req).await;
    //         }
    //         // let body = resp.text().await.unwrap_or_default();
    //         // (status, headers, Html(body))
    //         let response = stream_response(resp).await;
    //         return response
    //         // response.into_response()
    //     },
    //     Err(_) => error_response("无法访问目标地址，请检查链接是否正确", StatusCode::BAD_REQUEST, req.headers()).into_response()
    // }

    // match client.get(uri).send().await {
    //     Ok(resp) => {
    //         let status = resp.status();
    //         let headers = resp.headers().clone();
    //         let body = resp.text().await.unwrap_or_default();
    //         (status, headers, Html(body))
    //         // stream_response(resp).await
    //     },
    //     Err(_) => error_response("无法访问目标地址，请检查链接是否正确", StatusCode::BAD_REQUEST, headers)
    // }
    // error_response( "Invalid URL", StatusCode::BAD_REQUEST, &headers).into_response()
    // return resp.into_response();
}

// // 流式传输响应体
// async fn stream_response(resp: reqwest::Response) -> impl IntoResponse {
//     let status = resp.status();
//     let resp_headers  = resp.headers().clone();
//     let stream = resp.bytes_stream();
//     let body = Body::from_stream(stream);
//     let mut response = Response::new(body);
//     let headers = response.headers_mut();

//     // add content-type, content-length
//     if let Some(content_type) = resp_headers.get(header::CONTENT_TYPE) {
//         headers.insert(header::CONTENT_TYPE, content_type.clone());
//     }
//     if let Some(content_length) = resp_headers.get(header::CONTENT_LENGTH) {
//         headers.insert(header::CONTENT_LENGTH, content_length.clone());
//     }

//     headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
//     headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET".parse().unwrap());
//     headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization".parse().unwrap());

//     *response.status_mut() = status;
//     response.into_response()
// }

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

    // (StatusCode::OK, "OK").into_response()
    handler(&target_url, req).await.into_response()
}

// 发送请求
async fn send_request(
    method: reqwest::Method,
    url: &str,
    headers: &HeaderMap,
    body: Option<String>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let mut builder = client.request(method.clone(), url);

    for (key, value) in headers.iter() {
        println!("{:?}", key);
        println!("{:?}", value);
        println!("");
        // builder = builder.header(key, value);
    }

    if let Some(body) = body.as_ref() {
        builder = builder.body(body.clone());
    }

    let request = builder.build()?;
    println!("a");
    let response = client.execute(request).await?;
    println!("b");

    // 检查是否是重定向响应
    if response.status().is_redirection() {
        // 获取重定向目标 URL
        if let Some(location) = response.headers().get(header::LOCATION) {
            println!("location: {:?}", location);
            if let Ok(location_str) = location.to_str() {
                println!("Redirecting1 to: {}", location_str);
                return Box::pin(send_request(method, &location_str, headers, body)).await;
            }
        }
    }

    println!("c");
    Ok(response)
}

async fn middle(
    method: &str,
    url: &str,
    headers: &HeaderMap,
    body: Option<String>,
) -> Result<reqwest::Response, reqwest::Error> {
    let method = reqwest::Method::from_bytes(method.as_bytes()).unwrap();
    let response = send_request(method, url, headers, body).await?;
    println!("HELLO: {:?}", response.headers());
    Ok(response)
}

// 主路由处理函数
async fn handler(req_url: &str, req: Request<Body>) -> impl IntoResponse {
    let method = req.method().as_str();
    println!("{:?}", method);
    println!("{:?}", req_url);

    let response = middle(method, req_url, req.headers(), None).await;

    println!("");
    println!("WORLD: {:?}", response);

    (StatusCode::OK, "OK")
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
