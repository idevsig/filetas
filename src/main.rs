use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use reqwest::Client;
use std::{env, net::SocketAddr, sync::Arc};
use tokio::sync::OnceCell;
use url::Url;

// 静态文件扩展名，用于判断是否为下载链接
static FILE_EXT: &str = ""; // 逗号分隔的扩展名

// 共享的 HTTP 客户端
static CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

// 初始化 HTTP 客户端
async fn get_client() -> Arc<Client> {
    CLIENT
        .get_or_init(|| async { Arc::new(Client::new()) })
        .await
        .clone()
}

// 判断是否为下载链接
fn is_download_url(url: &str) -> bool {
    if let Ok(parsed_url) = Url::parse(url) {
        if let Some(segments) = parsed_url.path_segments() {
            if let Some(last_segment) = segments.last() {
                return last_segment
                    .split('.')
                    .last()
                    .map(|extension| FILE_EXT.split(',').any(|ext| ext.eq_ignore_ascii_case(extension)))
                    .unwrap_or(false);
            }
        }
    }
    false
}


// 加载错误模板并返回响应
fn error_response(info: &str, status: StatusCode) -> impl IntoResponse {
    let html_content = std::fs::read_to_string("templates/error.html")
        .unwrap_or_else(|_| info.to_string());
    let response_body = html_content.replace("{{ info }}", info);
    (status, Html(response_body))
}

// 代理请求处理
async fn proxy(Path(uri): Path<String>) -> impl IntoResponse {
    let target_url = match normalize_url(&uri) {
        Some(url) => url,
        None => {
            return error_response("下载链接必须含 \"https://\" 或 \"http://\"", StatusCode::BAD_REQUEST).into_response();
        }
    };

    let client = get_client().await;

    match client.get(&target_url).send().await {
        Ok(resp) => stream_response(resp).await,
        Err(_) => error_response("无法访问目标地址，请检查链接是否正确", StatusCode::BAD_REQUEST).into_response(),
    }
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

// 流式传输响应体
async fn stream_response(resp: reqwest::Response) -> Response {
    let status = resp.status();
    let resp_headers  = resp.headers().clone();
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

    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization".parse().unwrap());

    *response.status_mut() = status;
    response
}



// 处理 favicon.ico 请求
async fn favicon_ico() -> impl IntoResponse {
    serve_static_file("static/favicon.ico", "image/x-icon").await
}

// 处理 robots.txt 请求
async fn robots_txt() -> impl IntoResponse {
    (StatusCode::OK, [(header::CONTENT_TYPE, "text/plain")], "User-agent:*\nDisallow:/")
}

// 提供静态文件async fn serve_static_file(path: &str, content_type: &str) -> Response {
async fn serve_static_file(path: &str, content_type: &str) -> Response {
    match tokio::fs::read(path).await {
        Ok(content) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, content_type.to_string())],
            content,
        )
            .into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            axum::body::Body::from("404 Not Found"),
        )
            .into_response(),
    }
}
    

// 主页处理
async fn index_handler() -> Html<String> {
    let title = env::var("TITLE").unwrap_or_else(|_| "文件加速下载".to_string());
    let html_content = std::fs::read_to_string("templates/index.html")
        .unwrap_or_else(|_| "Error: Could not load index.html".to_string());
    Html(html_content.replace("{{ title }}", &title))
}

#[tokio::main]
async fn main() {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8000);

    let app = Router::new()
        .route("/favicon.ico", get(favicon_ico))
        .route("/robots.txt", get(robots_txt))
        .route("/*uri", get(proxy))
        .route("/", get(index_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
