use std::collections::HashMap;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use crate::error::TauriError;

pub type APIResult<T, E = TauriError> = Result<T, E>;

#[derive(Deserialize, Serialize)]
pub struct Task {
    title: String,
    content: String,
    createdAt: Option<DateTime<Utc>>,
    updatedAt: Option<DateTime<Utc>>,
}

pub enum URL {
    WithBaseUrl(&'static str),
    WithoutBaseUrl(String),
}

impl URL {
    pub fn value(self) -> String {
        match self {
            URL::WithBaseUrl(url) => format!("https://api.github.com/{url}"),
            URL::WithoutBaseUrl(url) => url
        }
    }
}