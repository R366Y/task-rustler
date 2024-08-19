use rusqlite::{params, Connection, Result};
use crate::task_manager::Task;

pub fn create_and_return_connection(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn insert_task(conn: &Connection, description: &String) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (description, completed) VALUES (?1, 0)",
        params![description.trim()],
    )?;
    Ok(())
}

pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM tasks")?;
    let task_row_map = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;
    task_row_map.collect()
}

pub fn get_task(conn: &Connection, task_id: i32) -> Result<Option<Task>> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM tasks where id = ?1")?;
    let task_row_map = stmt.query_map(params![task_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;
    Ok(task_row_map.collect::<Result<Vec<Task>>>()?.last().cloned())
}

pub fn set_task_completed(conn: &Connection, task_id: i32) -> Result<usize> {
    conn.execute("UPDATE tasks SET completed = 1 WHERE id = ?1", params![task_id])
}

pub fn delete_task(conn: &Connection, task_id: i32) -> Result<usize> {
    conn.execute("delete from tasks where id = ?1", params![task_id])
}
