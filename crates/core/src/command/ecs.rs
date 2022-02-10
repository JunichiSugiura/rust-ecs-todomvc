#[derive(Debug, Clone)]
pub enum ECSCommand {
    ListTodo,
    CreateTodo(CreateTodoParams),
}

#[derive(Debug, Clone)]
pub struct CreateTodoParams {
    pub name: String,
}

impl CreateTodoParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
