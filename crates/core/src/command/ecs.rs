use bevy_ecs::entity::Entity;

#[derive(Debug, Clone)]
pub enum ECSCommand {
    List,
    Create(CreateParams),
    Update(UpdateParams),
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
