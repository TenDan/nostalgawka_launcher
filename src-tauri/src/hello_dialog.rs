use tauri::api::dialog::{MessageDialogBuilder, MessageDialogKind};



#[tauri::command]
pub fn open_dialog() {
    let dialog: MessageDialogBuilder = MessageDialogBuilder::new("New Dialog", "Hello World!");
    dialog.show(|_b| ());
}

#[tauri::command]
pub fn open_message_dialog(message: String) {
    let dialog: MessageDialogBuilder = MessageDialogBuilder::new("New Dialog", message);
    dialog
        .kind(MessageDialogKind::Info)
        .show(|_b| ());
}