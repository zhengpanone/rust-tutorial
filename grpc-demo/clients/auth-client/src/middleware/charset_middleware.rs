use axum::extract::Request;
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;

pub async fn charset_utf8_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    if let Some(content_type) = response.headers().get(header::CONTENT_TYPE) {
        let content_type_str = content_type.to_str().unwrap_or("");

        if content_type_str.contains("text/") && !content_type_str.contains(("charset=")) {
            let new_content_type = format!("{}; charset=utf-8", content_type_str);
            response
                .headers_mut()
                .insert(header::CONTENT_TYPE, new_content_type.parse().unwrap());
        }
    }
    response
}
