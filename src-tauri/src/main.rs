// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod book;
mod database;
mod state;

use book::{Book, ReadState};
use state::{AppState, ServiceAccess};

use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn add_book_to_db(
    app_handle: AppHandle,
    book: String,
    author: String,
    read_state: ReadState,
) -> Option<Vec<Book>> {
        match app_handle.db(|db| database::add_book(db, &Book::from(book, author, read_state, false)))
    {
        Ok(books) => return Some(books),
        Err(e) => println!("{e}"),
    }
    None
}

#[tauri::command]
fn get_books(app_handle: AppHandle) -> Option<Vec<Book>> {
    match app_handle.db(database::get_books) {
        Ok(books) => return Some(books),
        Err(e) => println!("{e}"),
    }
    None
}

#[tauri::command]
fn delete_book(app_handle: AppHandle, book: String, author: String) -> Option<Vec<Book>> {
    if let Ok(books) = app_handle.db(|db| database::delete_book(db, &book, &author)) {
        return Some(books);
    }
    None
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<AppState> = handle.state();
            let db =
                database::initialize_database(&handle).expect("Database initialize should succed");
            *app_state.db.lock().unwrap() = Some(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_book_to_db,
            delete_book,
            get_books
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
