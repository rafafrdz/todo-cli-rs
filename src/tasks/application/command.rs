use uuid::Uuid;

pub enum CommandTask {
    AddTask { title: String },
    ListTask { filter: Option<FilterTask> },
    MarkTaskDone { id: Uuid },
    DeleteTask { id: Uuid },
}

pub enum FilterTask {
    All,
    Done,
    Todo,
}
