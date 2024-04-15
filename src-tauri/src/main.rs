// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod book;
mod database;
mod state;

use book::{Book, ReadState};
use state::{AppState, ServiceAccess};

use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn add_book_to_db(app_handle: AppHandle, book: String, author: String) -> bool {
    let book = book.trim().to_string();
    let author = author.trim().to_string();
    let new_book = Book::from(book, author, ReadState::WishToRead, false);
    if let Ok(()) = app_handle.db(|db| database::add_book(db, &new_book)) {
        return true;
    }
    false
}

#[tauri::command]
fn get_books(app_handle: AppHandle) -> Option<Vec<Book>> {
    if let Ok(books) = app_handle.db(database::get_books) {
        return Some(books);
    }
    None
}

#[tauri::command]
fn delete_book(app_handle: AppHandle, book: String, author: String) -> bool {
    if let Ok(()) = app_handle.db(|db| database::delete_book(db, &book, &author)) {
        return true;
    }
    false
}

#[tauri::command]
fn change_starred(app_handle: AppHandle, book: String, author: String) -> bool {
    if let Ok(()) = app_handle.db(|db| database::toggle_starred(db, &book, &author)) {
        return true;
    }
    false
}

#[tauri::command]
fn change_read_state(app_handle: AppHandle, book: String, author: String) -> bool {
    if let Ok(()) = app_handle.db(|db| database::change_read_state(db, &book, &author)) {
        return true;
    }
    false
}

#[tauri::command]
fn search_books(app_handle: AppHandle, keyword: String) -> Option<Vec<Book>> {
    if let Ok(books) = app_handle.db(|db| database::search(db, &keyword)) {
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
            get_books,
            change_starred,
            change_read_state,
            search_books,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
