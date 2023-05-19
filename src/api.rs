use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT};
use serde::Serialize;

use crate::models::{APIResult, URL};

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Tauri Demo"));
    headers
}

pub fn make_get_request(url: URL) -> APIResult<String> {
    let url = url.value();
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).headers(construct_headers()).send()?;
    let response_body = response.text()?;
    Ok(response_body)
}