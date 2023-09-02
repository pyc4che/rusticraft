#[derive(Debug)]
pub struct Task
{
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

#[allow(warnings)]
impl Task
{
    pub fn new(
        id: i32,
        description: String,
        completed: bool
    ) -> Task
    {
        Task
        {
            id, 
            description,
            completed
        }
    }

    pub fn mark_as_done(&mut self)
    {
        self.completed = true;
    }
}
