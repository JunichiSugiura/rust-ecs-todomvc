use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum UICommand {
    TodoList(UITodoList),
}

#[derive(Debug, Clone)]
pub struct UITodoItem {}

pub type UITodoList = HashMap<u32, UITodoItem>;
