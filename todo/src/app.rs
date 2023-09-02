

use clap::{
    App,
    Arg
};
use rusqlite::Result;


use crate::database::Database;


pub struct Todo
{
    db: Database,
}


impl Todo
{
    pub fn new(
        database_path: String
    ) -> Result<Self>
    {
        let db = Database::new(
            database_path
        )?;

        Ok(Todo { db })
    }

    pub fn run(&mut self) -> Result<()>
    {
        let matches = App::new(
            "Todo List Manager CLI-Tool (rusticraft)"
        )
            .arg(
                Arg::with_name("add").short("a").takes_value(true)
            )
            .arg(
                Arg::with_name("done").short("x").takes_value(true)
            )
            .arg(
                Arg::with_name("list").short("l")
            )
            .arg(
                Arg::with_name("clear").short("c")
            )
            .arg(
                Arg::with_name("delete").short("d").takes_value(true)
            )
            .get_matches();

        if let Some(description) = matches.value_of("add")
        {
            self.db.insert_task(description)?;
        }
        else if let Some(id) = matches.value_of("done")
        {
            if let Ok(id) = id.parse::<i32>()
            {
                self.db.mark_task_as_done(id)?;
            }
            else
            {

            }  
        }
        else if matches.is_present("list")
        {
            let tasks = self.db.get_all_tasks()?;
            
            for task in tasks.iter()
            {
                println!(
                    "{} '{}': [{}]",
                    task.id,
                    task.description,
                    if task.completed { "X" } else { "" },

                );
            }
        }
        else if matches.is_present("clear")
        {
            self.db.clear_tasks()?;
        }
        else if let Some(id) = matches.value_of("delete")
        {
            if let Ok(id) = id.parse::<i32>()
            {
                self.db.remove_task(id)?;
            }
            else
            {

            }
        }

        Ok(())
    }
}
