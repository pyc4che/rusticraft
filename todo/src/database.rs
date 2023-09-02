use crate::task::Task;

use rusqlite::{
    Result,
    Connection
};


pub struct Database
{
    conn: Connection,
}


impl Database
{
    pub fn new(
        database_path: String
    ) -> Result<Self>
    {
        let conn = Connection::open(
            &database_path
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks
            (
                id INTEGER PRIMARY KEY,
                description TEXT,
                completed INTEGER
            )",
            []
        )?;

        Ok(Database { conn })
    }

    pub fn insert_task(
        &self,
        description: &str
    ) -> Result<()>
    {
        self.conn.execute(
            "INSERT INTO tasks
            (
                description,
                completed
            )
            VALUES (?, ?)",
            &[description, &0.to_string()]
        )?;

        Ok(())
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>>
    {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM tasks"
        )?;

        let task_iter = stmt.query_map(
            [],
            |row| 
            {
                Ok(
                    Task
                    {
                        id: row.get(0)?,
                        description: row.get(1)?,
                        completed: row.get(2)?,
                    }
                )
            }
        )?;

        let mut tasks = Vec::new();

        for task in task_iter
        {
            tasks.push(task?);
        }

        Ok(tasks)
    }

    pub fn mark_task_as_done(
        &self,
        id: i32
    ) -> Result<()>
    {
        self.conn.execute(
            "UPDATE tasks SET completed = ? WHERE id = ?",
            &[&1.to_string(), &id.to_string()]
        )?;

        Ok(())
    }

    pub fn remove_task(
        &self,
        id: i32
    ) -> Result<()>
    {
        self.conn.execute(
            "DELETE FROM tasks WHERE id = ?",
            &[&id]
        )?;

        Ok(())
    }

    pub fn clear_tasks(&self) -> Result<()>
    {
        self.conn.execute(
            "DELETE FROM tasks",
            []
        )?;

        Ok(())
    }
}
