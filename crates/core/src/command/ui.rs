use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum UICommand {
    ListTodo(UITodoList),
    CreateTodo(UITodoList),
}

#[derive(Debug, Clone)]
pub struct UITodoItem { 
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub done: bool,
}

pub type UITodoList = Vec<UITodoItem>;
