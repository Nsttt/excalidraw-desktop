#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = tauri::WindowBuilder::new(
                app,
                "external",
                tauri::WindowUrl::External("https://excalidraw.com".parse().unwrap()),
            )
            .title("Excalidraw Desktop")
            .initialization_script(
                "
                window.onload = () => {
                    const style = document.createElement('style');
                    style.innerHTML = `
                        body {}
                    `;
                    document.head.append(style);
                }
            ",
            )
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
