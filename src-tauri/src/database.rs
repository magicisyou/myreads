use rusqlite::{named_params, Connection};
use std::fs;
use tauri::AppHandle;

use crate::book::{Book, ReadState};

const CURRENT_DB_VERSION: u32 = 1;
const TABLE_NAME: &str = "books";

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created");
    let sqlite_path = app_dir.join("books.sqlite");
    let mut db = Connection::open(sqlite_path)?;
    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| row.get(0))?;
    drop(user_pragma);
    upgrade_database_if_needed(&mut db, existing_user_version)?;
    Ok(db)
}

pub fn upgrade_database_if_needed(
    db: &mut Connection,
    existing_version: u32,
) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;
        let tx = db.transaction()?;
        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;
        let query = format!(
            "
      CREATE TABLE {TABLE_NAME} (
        book TEXT NOT NULL,
        author TEXT NOT NULL,
        read_state TEXT NOT NULL,
        starred BOOL NOT NULL,
        PRIMARY KEY (book, author)
      );"
        );
        tx.execute_batch(&query)?;
        tx.commit()?;
    }
    Ok(())
}

pub fn add_book(db: &Connection, book: &Book) -> Result<Vec<Book>, rusqlite::Error> {
    let starred = match book.starred {
        true => 1,
        false => 0,
    };
    let read_state = match book.read_state {
        ReadState::Read => "Read",
        ReadState::Reading => "Reading",
        ReadState::NotRead => "NotRead",
    };
    let query =
        format!("INSERT INTO {TABLE_NAME} (book, author, read_state, starred) VALUES (@book, @author, @read_state, @starred)");
    let mut statement = db.prepare(&query)?;
    statement.execute(
        named_params! {"@book": book.name, "@author": book.author, "@read_state": read_state, "@starred": starred },
    )?;
    get_books(db)
}

pub fn get_books(db: &Connection) -> Result<Vec<Book>, rusqlite::Error> {
    let query = format!("SELECT * FROM {TABLE_NAME}");
    let mut statement = db.prepare(&query)?;
    let mut rows = statement.query([])?;
    let mut books = Vec::new();
    while let Some(row) = rows.next()? {
        let book: String = row.get("book")?;
        let author: String = row.get("author")?;
        let read_state: String = row.get("read_state")?;
        let read_state = match read_state.as_str() {
            "Read" => ReadState::Read,
            "Reading" => ReadState::Reading,
            _ => ReadState::NotRead,
        };
        let starred = row.get("starred")?;
        let starred = matches!(starred, 1);
        books.push(Book::from(book, author, read_state, starred));
    }
    Ok(books)
}

pub fn delete_book(
    db: &Connection,
    book: &str,
    author: &str,
) -> Result<Vec<Book>, rusqlite::Error> {
    let query = format!("DELETE FROM {TABLE_NAME} WHERE book = @book AND author = @author");
    let mut statement = db.prepare(&query)?;
    statement.execute(named_params! { "@book": book, "@author": author })?;
    get_books(db)
}
/*
fn get_checked_value(
    db: &Connection,
    list_name: &str,
    item_name: &str,
) -> Result<bool, rusqlite::Error> {
    let query = format!(
        "SELECT checked FROM {TABLE_NAME} WHERE item = '{item_name}' AND list = '{list_name}'"
    );
    let mut statement = db.prepare(&query)?;
    let mut rows = statement.query([])?;
    if let Some(row) = rows.next()? {
        let checked = row.get("checked")?;
        return Ok(matches!(checked, 0));
    }
    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn toggle_checked(
    db: &Connection,
    list_name: &str,
    item_name: &str,
) -> Result<Vec<Item>, rusqlite::Error> {
    let checked = get_checked_value(db, list_name, item_name)?;
    let query =
        format!("UPDATE {TABLE_NAME} SET checked = {checked} WHERE item = @item AND list = @list");
    let mut statement = db.prepare(&query)?;
    statement.execute(named_params! { "@item": item_name, "@list": list_name })?;
    get_list(db, list_name)
}

pub fn get_list_names(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let query = format!("SELECT DISTINCT list FROM {TABLE_NAME}");
    let mut statement = db.prepare(&query)?;
    let mut rows = statement.query([])?;
    let mut list = vec![];
    while let Some(row) = rows.next()? {
        list.push(row.get("list")?);
    }
    Ok(list)
}

pub fn delete_list(db: &Connection, list_name: &str) -> Result<Vec<String>, rusqlite::Error> {
    let query = format!("DELETE FROM {TABLE_NAME} WHERE list = @list");
    let mut statement = db.prepare(&query)?;
    statement.execute(named_params! {"@list": list_name})?;
    get_list_names(db)
}
*/
