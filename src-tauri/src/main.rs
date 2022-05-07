#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        get_dir, set_dir, get_audio_list, record_audio_list
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_dir() -> Result<String, String> {
    match fs::read_to_string("./dir.txt") {
        Ok(dir_text) => Ok(dir_text),
        Err(_) => Err("not_exist_folder".to_string()),
    }
}

#[tauri::command]
fn set_dir(dir: String) {
    let mut file = File::create("./dir.txt").unwrap();
    file.write_all(dir.as_bytes()).unwrap();
}

#[tauri::command]
fn get_audio_list(path: &Path) -> Result<HashMap<String, f32>, String> {
    let titles: Vec<(String, f32)> = if let Ok(dir) = fs::read_dir(path) {
        dir.filter_map(|entry| {
            let entry = entry.ok()?;
            entry.file_type().ok()?.is_file().then(|| (entry.file_name().to_string_lossy().into_owned(), 0.0))
        })
        .collect()
    } else {
        return Err("cannot find folder.".to_string())
    };

    let mut info: HashMap<String, f32> = titles.into_iter().collect();

    if let Ok(info_text) = fs::read_to_string("./info.txt") {
        let info_fromfile: HashMap<String, f32> = serde_json::from_str(&info_text).unwrap();
        info_fromfile.into_iter().for_each(|(title, time)| {
            match info.get_mut(&title) {
                Some(time_orign) => *time_orign = time,
                None => println!("no {title}"),
            }
        });
    };

    Ok(info)
}

#[tauri::command]
fn record_audio_list(info: HashMap<String, f32>) {
    let info_text = serde_json::to_string(&info).unwrap();
    let mut file = File::create("./info.txt").unwrap();
    file.write_all(info_text.as_bytes()).unwrap();
}