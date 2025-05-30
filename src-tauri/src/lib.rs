use serde::Deserialize;
use tauri::{utils::mime_type::MimeType, Manager};
use tauri_plugin_fs::FsExt;
use tauri_plugin_http::reqwest;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const LOCALHOST_IP_ADDRESS: &'static str = "192.168.10.53";

#[derive(Debug, Deserialize)]
struct UploadResponse {
    message: String,
    data: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.app_handle();
            let fs = handle.fs();
            let client = reqwest::Client::new();

            let readfile = async |raw: &'static str| {
                let raw_path = handle
                    .path()
                    .resolve(raw, tauri::path::BaseDirectory::Resource)
                    .unwrap();

                let path = raw_path.to_str().unwrap().to_string();

                match fs.read(&raw_path) {
                    Ok(bytes) => {
                        let filename = path.split('/').last().unwrap().to_string();
                        let mime = MimeType::parse(&bytes, &path);

                        println!("path: {}", path);
                        println!("mime: {}", mime);
                        println!("size: {}", bytes.len());

                        let part = reqwest::multipart::Part::bytes(bytes)
                            .file_name(filename.clone())
                            .mime_str(&mime)
                            .unwrap();

                        let form = reqwest::multipart::Form::new().part("file", part);

                        println!("ready upload file {:?}", filename);

                        let res = client
                            .post(format!("http://{}:3500/upload", LOCALHOST_IP_ADDRESS))
                            .multipart(form)
                            .send()
                            .await;

                        match res {
                            Ok(response) => {
                                println!("send file {:?} success!", filename);
                                println!("response = {:?}", response);
                                let res = response.json::<UploadResponse>().await;
                                match res {
                                    Ok(json) => {
                                        println!(
                                            "response message = {}, data = {}",
                                            json.message, json.data
                                        );
                                    }
                                    Err(msg) => {
                                        println!("response text msg = {:?}", msg);
                                    }
                                }
                            }
                            Err(msg) => {
                                println!("send file {:?} msg = {:?}", filename, msg);
                            }
                        }
                    }
                    Err(msg) => {
                        println!("{}", msg);
                    }
                }
            };

            tauri::async_runtime::block_on(readfile("resources/images/icon.ico"));
            tauri::async_runtime::block_on(readfile("resources/images/icon.icns"));
            tauri::async_runtime::block_on(readfile("resources/data/lang.json"));
            tauri::async_runtime::block_on(readfile("resources/html/index.html"));
            tauri::async_runtime::block_on(readfile("resources/images/icon.jpg"));
            tauri::async_runtime::block_on(readfile("resources/images/icon.png"));
            tauri::async_runtime::block_on(readfile("resources/images/icon.webp"));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
