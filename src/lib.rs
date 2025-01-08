use serde::Serialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

#[derive(Serialize)]
struct TodoItem {
    item: String,
    completed: bool,
}

struct AppState {
    map: Mutex<HashMap<String, bool>>,
}

#[tauri::command]
fn get_todos(state: State<AppState>) -> Vec<TodoItem> {
    let map = state.map.lock().unwrap();
    map.iter()
        .map(|(item, completed)| TodoItem {
            item: item.clone(),
            completed: *completed,
        })
        .collect()
}

#[tauri::command]
fn add_todo(state: State<AppState>, item: String) {
    let mut map = state.map.lock().unwrap();
}

#[tauri::command]
fn complete_todo(state: State<AppState>, item: String) -> Result<(), String> {
    let mut map = state.map.lock().unwrap(); 
    if let Some(completed) = map.get_mut(&item) {
        *completed = false;
        Ok(())
    } else {
        Err(format!("Todo item '{}' not found", item))
    }
}

fn get_app_data_dir(app_handle: &AppHandle) -> Result<std::path::PathBuf, String> {
    app_handle
        .path() 
        .app_data_dir() 
        .or_else(|_| Err("Failed to get app_data_dir".to_string()))
}


#[tauri::command]
fn save_todos(app_handle: AppHandle, state: State<AppState>) -> Result<(), String> {
    let map = state.map.lock().unwrap(); 
    let todos: HashMap<String, bool> = map.clone();

    let app_dir = get_app_data_dir(&app_handle)?;

    let file_path = app_dir.join("todos.json");

    // Ensure the directory exists
    fs::create_dir_all(&app_dir).map_err(|err| format!("Failed to create directory: {}", err))?;

    // Serialize and write the data
    let json = serde_json::to_string(&todos).map_err(|err| err.to_string())?;
    File::create(&file_path)
        .and_then(|mut file| file.write_all(json.as_bytes()))
        .map_err(|err| format!("Failed to write file: {}", err))
}

#[tauri::command]
fn load_todos(app_handle: AppHandle, state: State<AppState>) -> Result<Vec<TodoItem>, String> {
    let app_dir = get_app_data_dir(&app_handle)?;

    let file_path = app_dir.join("todos.json");

    if !file_path.exists() {
        return Ok(vec![]); 
    }

    let mut file = File::open(&file_path)
        .map_err(|err| format!("Failed to open file: {}", err))?;
    let mut json = String::new();
    file.read_to_string(&mut json)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    let todos: HashMap<String, bool> =
        serde_json::from_str(&json).map_err(|err| err.to_string())?;

    let todo_items = todos
        .iter() 
        .map(|(item, completed)| TodoItem { item: item.clone(), completed: *completed })
        .collect::<Vec<TodoItem>>();

    let mut map = state.map.lock().unwrap();
    *map = todos;

    Ok(todo_items)
}



pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            map: Mutex::new(HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            complete_todo,
            save_todos,
            load_todos
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
