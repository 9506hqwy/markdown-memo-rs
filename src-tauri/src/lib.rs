pub mod api;
pub mod db;
pub mod error;
pub mod model;

use rusqlite::Connection;
use std::path::Path;

use api::{
    add_memo_tag_fn, create_memo_fn, delete_memo_fn, get_memo_all_fn, get_memo_fn, get_memo_tag_fn,
    get_topics_fn, remove_memo_tag_fn,
};
use std::sync::Mutex;
use tauri::{Builder, Manager, State};
use tauri_plugin_cli::CliExt;

pub struct AppData {
    db: Mutex<Connection>,
}

#[tauri::command]
fn add_memo_tag(state: State<'_, AppData>, topic_id: &str, tag: &str) -> Result<(), ()> {
    add_memo_tag_fn(state.inner(), topic_id, tag).or(Err(()))?;
    Ok(())
}

#[tauri::command]
fn create_memo(
    state: State<'_, AppData>,
    topic_id: &str,
    content: &str,
) -> Result<model::Memo, ()> {
    let memo = create_memo_fn(state.inner(), topic_id, content).or(Err(()))?;
    Ok(memo)
}

#[tauri::command]
fn delete_memo(state: State<'_, AppData>, topic_id: &str, id: &str) -> Result<(), ()> {
    delete_memo_fn(state.inner(), topic_id, id).or(Err(()))?;
    Ok(())
}

#[tauri::command]
fn get_memo(
    state: State<'_, AppData>,
    topic_id: &str,
    id: Option<&str>,
) -> Result<model::Memo, ()> {
    get_memo_fn(state.inner(), topic_id, id).or(Err(()))
}

#[tauri::command]
fn get_memo_all(state: State<'_, AppData>, topic_id: &str) -> Result<Vec<model::Memo>, ()> {
    get_memo_all_fn(state.inner(), topic_id).or(Err(()))
}

#[tauri::command]
fn get_memo_tag(state: State<'_, AppData>, topic_id: &str) -> Result<Vec<String>, ()> {
    get_memo_tag_fn(state.inner(), topic_id).or(Err(()))
}

#[tauri::command]
fn get_topics(state: State<'_, AppData>, keyword: &str) -> Result<Vec<model::Topic>, ()> {
    get_topics_fn(state.inner(), keyword).or(Err(()))
}

#[tauri::command]
fn remove_memo_tag(state: State<'_, AppData>, topic_id: &str, tag: &str) -> Result<(), ()> {
    remove_memo_tag_fn(state.inner(), topic_id, tag).or(Err(()))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            let cli = app.cli().matches()?;

            let file_path = cli
                .args
                .get("path")
                .and_then(|p| p.value.as_str())
                .map(Path::new);

            let in_memory = cli
                .args
                .get("memory")
                .and_then(|p| p.value.as_bool())
                .unwrap_or_default();

            let db = db::setup(file_path, in_memory)?;

            app.manage(AppData { db: Mutex::new(db) });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_memo_tag,
            create_memo,
            delete_memo,
            get_memo,
            get_memo_all,
            get_memo_tag,
            get_topics,
            remove_memo_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
