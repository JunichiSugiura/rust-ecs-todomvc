pub mod core {
    pub use bevy_ecs::entity::Entity;

    #[derive(Debug, Clone)]
    pub enum CoreCommand {
        List,
        Create(CreateParams),
        Update(UpdateParams),
        Delete(Entity),
    }

    #[derive(Debug, Clone)]
    pub struct CreateParams {
        pub name: String,
    }

    impl CreateParams {
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UpdateParams {
        pub entity: Entity,
        pub name: Option<String>,
        pub done: Option<bool>,
    }
}

pub mod ui {
    use bevy_ecs::entity::Entity;
    use chrono::{DateTime, Utc};

    #[derive(Debug, Clone)]
    pub enum UICommand {
        List(UITodoList),
        Create(UITodoList),
        Update(UITodoList),
        Delete(UITodoList),
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
}
