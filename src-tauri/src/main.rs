#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod hello_dialog;

use tauri::{LogicalSize, Manager, Size};

fn main() {
  tauri::Builder::default()
      .setup(|app| {
          let main_window = app.get_window("main").unwrap();
          let logical_size = LogicalSize { width: 800.0, height: 500.0 };
          main_window
              .set_size(Size::Logical(logical_size))
              .expect("failed to initialize default size of the window");
          main_window
              .set_min_size(
                  Some(Size::Logical(logical_size)))
              .expect("failed to initialize minimum size of the window");
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![
          hello_dialog::open_dialog,
          hello_dialog::open_message_dialog
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
