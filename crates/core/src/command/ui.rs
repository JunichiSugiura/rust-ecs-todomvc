use bevy_ecs::entity::Entity;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum UICommand {
    List(UITodoList),
    Create(UITodoList),
    Update(UITodoList),
}

#[derive(Debug, Clone)]
pub struct UITodoItem { 
    pub entity: Entity,
    pub name: String,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub type UITodoList = Vec<UITodoItem>;
