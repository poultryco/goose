// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, AboutMetadata, window};

fn build_menu() -> Menu {
  let menu = Menu::new()
    // main submenu
    .add_submenu(Submenu::new(
      "Goose",
      Menu::new()
      .add_item(CustomMenuItem::new("about", "About goose"))
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Services)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Hide)
      .add_native_item(MenuItem::HideOthers)
      .add_native_item(MenuItem::ShowAll)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Quit),
    ))
    // TODO: add file submenu
    .add_submenu(Submenu::new(
      "File",
      Menu::new()
    ))
    // edit submenu
    .add_submenu(Submenu::new(
      "Edit",
      Menu::new()
      .add_native_item(MenuItem::Undo)
      .add_native_item(MenuItem::Redo)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Cut)
      .add_native_item(MenuItem::Copy)
      .add_native_item(MenuItem::Paste)
    ))
    // view submenu
    .add_submenu(Submenu::new(
      "View",
      Menu::new()
      .add_item(CustomMenuItem::new("fontup", "Zoom In").accelerator("CmdOrCtrl+="))
      .add_item(CustomMenuItem::new("fontdown", "Zoom Out").accelerator("CmdOrCtrl+-"))
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::EnterFullScreen)
    ))
    // window submenu
    .add_submenu(Submenu::new(
      "Window",
      Menu::new()
      .add_native_item(MenuItem::Minimize)
      .add_native_item(MenuItem::Zoom)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::CloseWindow)
    ));

    menu
}

fn main() {
  tauri::Builder::default()
    .menu(build_menu())
    .on_menu_event(|event| {
      match event.menu_item_id() {
        "about" => {
         let _ = event.window().emit("about", "ra");
        }
        _ => {}
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

