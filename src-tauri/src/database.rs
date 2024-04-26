use rusqlite::{named_params, Connection};
use std::fs;
use tauri::AppHandle;

use crate::book::{Book, ReadState};

const CURRENT_DB_VERSION: u32 = 2;
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
        if existing_version == 0 {
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
            tx.execute(&query, [])?;
        } else {
            let query = format!(
                "
              UPDATE {TABLE_NAME} SET read_state = 'Completed' WHERE read_state = 'Read';
              UPDATE {TABLE_NAME} SET read_state = 'WishToRead' WHERE read_state = 'NotRead';
              UPDATE {TABLE_NAME} SET read_state = 'NotCompleted' WHERE read_state = 'PartialRead';
            "
            );
            tx.execute_batch(&query)?;
        }
        tx.commit()?;
    }
    Ok(())
}

pub fn add_book(db: &Connection, book: &Book) -> Result<(), rusqlite::Error> {
    let starred = match book.starred {
        true => 1,
        false => 0,
    };
    let read_state = book.read_state.to_string();
    let query =
        format!("INSERT INTO {TABLE_NAME} (book, author, read_state, starred) VALUES (@book, @author, @read_state, @starred)");
    let mut statement = db.prepare(&query)?;
    statement.execute(
        named_params! {"@book": book.name, "@author": book.author, "@read_state": read_state, "@starred": starred },
    )?;
    Ok(())
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
        let read_state = ReadState::from(&read_state);
        let starred = row.get("starred")?;
        let starred = matches!(starred, 1);
        books.push(Book::from(book, author, read_state, starred));
    }
    books.reverse();
    Ok(books)
}

pub fn delete_book(db: &Connection, book: &str, author: &str) -> Result<(), rusqlite::Error> {
    let query = format!("DELETE FROM {TABLE_NAME} WHERE book = @book AND author = @author");
    let mut statement = db.prepare(&query)?;
    statement.execute(named_params! { "@book": book, "@author": author })?;
    Ok(())
}

fn get_starred_value(db: &Connection, book: &str, author: &str) -> Result<bool, rusqlite::Error> {
    let query = format!("SELECT starred FROM {TABLE_NAME} WHERE book = @book AND author = @author");
    let mut statement = db.prepare(&query)?;
    let mut rows = statement.query(named_params! {"@book": book, "@author": author})?;
    if let Some(row) = rows.next()? {
        let starred = row.get("starred")?;
        return Ok(matches!(starred, 1));
    }
    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn toggle_starred(db: &Connection, book: &str, author: &str) -> Result<(), rusqlite::Error> {
    let starred = !get_starred_value(db, book, author)?;
    let query = format!(
        "UPDATE {TABLE_NAME} SET starred = @starred WHERE book = @book AND author = @author"
    );
    let mut statement = db.prepare(&query)?;
    statement.execute(named_params! { "@starred": starred, "@book": book, "@author": author})?;
    Ok(())
}

fn get_next_read_state(
    db: &Connection,
    book: &str,
    author: &str,
) -> Result<ReadState, rusqlite::Error> {
    let query =
        format!("SELECT read_state FROM {TABLE_NAME} WHERE book = @book AND author = @author");
    let mut statement = db.prepare(&query)?;
    let mut rows = statement.query(named_params! {"@book": book, "@author": author})?;
    if let Some(row) = rows.next()? {
        let read_state: String = row.get("read_state")?;
        let mut read_state = ReadState::from(&read_state);
        read_state.next();
        return Ok(read_state);
    }
    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn change_read_state(db: &Connection, book: &str, author: &str) -> Result<(), rusqlite::Error> {
    let read_state = get_next_read_state(db, book, author)?.to_string();
    let query = format!(
        "UPDATE {TABLE_NAME} SET read_state = @read_state WHERE book = @book AND author = @author"
    );
    let mut statement = db.prepare(&query)?;
    statement
        .execute(named_params! { "@read_state": read_state, "@book": book, "@author": author})?;
    Ok(())
}

pub fn search(db: &Connection, keyword: &str) -> Result<Vec<Book>, rusqlite::Error> {
    let keyword = format!("%{keyword}%");
    let query =
        format!("SELECT * FROM {TABLE_NAME} WHERE book LIKE @keyword OR author LIKE @keyword");
    let mut statement = db.prepare(&query)?;
    let mut rows = statement.query(named_params! {"@keyword": keyword})?;
    let mut books = Vec::new();
    while let Some(row) = rows.next()? {
        let book: String = row.get("book")?;
        let author: String = row.get("author")?;
        let read_state: String = row.get("read_state")?;
        let read_state = ReadState::from(&read_state);
        let starred = row.get("starred")?;
        let starred = matches!(starred, 1);
        books.push(Book::from(book, author, read_state, starred));
    }
    books.reverse();
    Ok(books)
}
