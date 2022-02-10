use std::collections::HashMap;

#[derive(Debug)]
pub enum UICommand {
    TodoList(UITodoList),
}

#[derive(Debug)]
pub struct UITodoItem {}

pub type UITodoList = HashMap<u32, UITodoItem>;
