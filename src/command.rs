use crate::api::{make_get_request};
use crate::models::{
    APIResult, URL, Task
};

#[tauri::command]
pub fn get_tasks() -> APIResult<Vec<Task>> {
    let response = make_get_request(URL::WithBaseUrl("tasks"))?;
    let response: Vec<Task> = serde_json::from_str(&response).unwrap();
    Ok(response)
}