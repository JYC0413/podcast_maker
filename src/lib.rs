use flowsnet_platform_sdk::logger;
use serde_json::Value;
use std::collections::HashMap;
use webhook_flows::{create_endpoint, request_handler, send_response};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler(
    _headers: Vec<(String, String)>,
    _subpath: String,
    qry: HashMap<String, Value>,
    body: Vec<u8>,
) {
    logger::init();

    // 处理代理请求
        if _subpath == "/proxy-api" {
            // 目标 API 的地址
            let target_url = "http://159.138.158.109:8005/some-endpoint";

            // 使用 reqwest 发送请求到目标 API
            let client = Client::new();
            let res = client
                .post(target_url)
                .json(&qry)  // 如果需要传递参数，使用 .json()
                .send()
                .await;

            match res {
                Ok(response) => {
                    // 获取响应体
                    let body = response.bytes().await.unwrap();
                    let status_code = response.status().as_u16();

                    // 处理返回响应
                    send_response(
                        status_code,
                        vec![(String::from("content-type"), String::from("application/json"))],
                        body.to_vec(),
                    );
                }
                Err(e) => {
                    // 错误处理
                    send_response(
                        500,
                        vec![(String::from("content-type"), String::from("text/plain"))],
                        format!("Error proxying request: {}", e).as_bytes().to_vec(),
                    );
                }
            }
            return;
        }

    let (content_type, html) = match _subpath.as_str() {
        "/index.html" | "/index" => ("text/html", include_str!("index.html").as_bytes().to_vec()),
        "/favicon.ico" => ("image/x-icon", include_bytes!("favicon.ico").to_vec()),
        _ => ("text/html", include_str!("index.html").as_bytes().to_vec()),
    };
    send_response(
        200,
        vec![(String::from("content-type"), String::from(content_type))],
        html,
    );
}