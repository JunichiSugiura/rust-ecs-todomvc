use crate::command::{
    ecs::ECSCommand,
    ui::{UICommand, UITodoItem},
};
use bevy_app::prelude::*;
use bevy_ecs::{event::Events, prelude::*};
use chrono::prelude::*;
use tokio::sync::broadcast::Sender;

pub fn start_ecs(ui_tx: Sender<UICommand>, core_tx: Sender<ECSCommand>) {
    App::new()
        .set_runner(runner)
        .insert_resource(ui_tx)
        .insert_resource(core_tx)
        .add_event::<ECSCommand>()
        .add_event::<NotifyCommand>()
        .add_startup_system(setup)
        .add_system(handle_list_todo)
        .add_system(handle_create_todo)
        .add_system(handle_update_todo)
        .add_system(handle_delete_todo)
        .add_system_to_stage(CoreStage::Last, notify_list_todo)
        .add_system_to_stage(CoreStage::Last, notify_create_todo)
        .add_system_to_stage(CoreStage::Last, notify_update_todo)
        .add_system_to_stage(CoreStage::Last, notify_delete_todo)
        .run();
}

// Components

#[derive(Component)]
struct Todo;

#[derive(Component, Clone)]
struct Name(String);

#[derive(Component)]
struct CreatedAt(DateTime<Utc>);

#[derive(Component)]
struct UpdatedAt(Option<DateTime<Utc>>);

#[derive(Component)]
struct Done(bool);

// Runner

fn runner(mut app: App) {
    app.update();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<ECSCommand>(32);

    let runtime = tokio::runtime::Runtime::new().unwrap();

    if let Some(core_tx) = app.world.get_resource_mut::<Sender<ECSCommand>>() {
        let mut rx = core_tx.subscribe();
        runtime.spawn(async move {
            loop {
                while let Ok(cmd) = rx.recv().await {
                    let _ = tx.send(cmd).await;
                }
            }
        });
    }

    while let Some(cmd) = rx.blocking_recv() {
        let mut event = app.world.get_resource_mut::<Events<ECSCommand>>().unwrap();
        event.send(cmd);
        app.update();
    }
}

// Systems

fn setup() {
    println!("ECS: Initializing");
}

fn handle_list_todo(mut events: EventReader<ECSCommand>, mut notify: EventWriter<NotifyCommand>) {
    for event in events.iter() {
        if let ECSCommand::List = event {
            notify.send(NotifyCommand::List);
        }
    }
}

fn handle_create_todo(
    mut events: EventReader<ECSCommand>,
    mut notify: EventWriter<NotifyCommand>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if let ECSCommand::Create(params) = event {
            let entity = commands
                .spawn()
                .insert(Todo)
                .insert(Name(params.name.to_string()))
                .insert(CreatedAt(Utc::now()))
                .insert(UpdatedAt(None))
                .insert(Done(false))
                .id();

            notify.send(NotifyCommand::Create(entity));
        }
    }
}

fn handle_update_todo(
    mut events: EventReader<ECSCommand>,
    mut notify: EventWriter<NotifyCommand>,
    mut query: Query<(Entity, &mut Name, &mut Done, &mut UpdatedAt), With<Todo>>,
) {
    for event in events.iter() {
        if let ECSCommand::Update(params) = event {
            for (entity, mut name, mut done, mut updated_at) in query.iter_mut() {
                if entity == params.entity {
                    if let Some(n) = &params.name {
                        name.0 = n.to_string();
                    }
                    if let Some(d) = params.done {
                        done.0 = d;
                    }
                    updated_at.0 = Some(Utc::now());

                    notify.send(NotifyCommand::Update(entity));
                }
            }
        }
    }
}

fn handle_delete_todo(
    mut events: EventReader<ECSCommand>,
    mut notify: EventWriter<NotifyCommand>,
    query: Query<Entity, With<Todo>>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if let ECSCommand::Delete(target_entity) = event {
            for entity in query.iter() {
                if entity == *target_entity {
                    commands.entity(*target_entity).despawn();

                    notify.send(NotifyCommand::Delete(entity));
                }
            }
        }
    }
}

fn notify_list_todo(
    mut events: EventReader<NotifyCommand>,
    query: Query<(Entity, &Name, &Done, &CreatedAt, &UpdatedAt), With<Todo>>,
    sender: Res<Sender<UICommand>>,
) {
    for event in events.iter() {
        if let NotifyCommand::List = event {
            println!("🧠 List Todo");

            let mut list = vec![];
            for (entity, name, done, created_at, updated_at) in query.iter() {
                list.push(UITodoItem {
                    entity,
                    name: name.0.clone(),
                    done: done.0,
                    created_at: created_at.0,
                    updated_at: updated_at.0,
                });
            }
            let _res = sender.send(UICommand::List(list));
        }
    }
}

// TODO: consider sending just a diff but it requires more work on UI side
fn notify_create_todo(
    mut events: EventReader<NotifyCommand>,
    query: Query<(Entity, &Name, &Done, &CreatedAt, &UpdatedAt), With<Todo>>,
    sender: Res<Sender<UICommand>>,
) {
    for event in events.iter() {
        if let NotifyCommand::Create(target_entity) = event {
            println!("🧠 Create Todo: {:?}", target_entity);

            let mut list = vec![];
            for (entity, name, done, created_at, updated_at) in query.iter() {
                list.push(UITodoItem {
                    entity,
                    name: name.0.clone(),
                    done: done.0,
                    created_at: created_at.0,
                    updated_at: updated_at.0,
                });
            }
            let _res = sender.send(UICommand::Create(list));
        }
    }
}

fn notify_update_todo(
    mut events: EventReader<NotifyCommand>,
    query: Query<(Entity, &Name, &Done, &CreatedAt, &UpdatedAt), With<Todo>>,
    sender: Res<Sender<UICommand>>,
) {
    for event in events.iter() {
        if let NotifyCommand::Update(target_entity) = event {
            println!("🧠 Update Todo: {:?}", target_entity);

            let mut list = vec![];
            for (entity, name, done, created_at, updated_at) in query.iter() {
                list.push(UITodoItem {
                    entity,
                    name: name.0.clone(),
                    done: done.0,
                    created_at: created_at.0,
                    updated_at: updated_at.0,
                });
            }
            let _res = sender.send(UICommand::Update(list));
        }
    }
}

fn notify_delete_todo(
    mut events: EventReader<NotifyCommand>,
    query: Query<(Entity, &Name, &Done, &CreatedAt, &UpdatedAt), With<Todo>>,
    sender: Res<Sender<UICommand>>,
) {
    for event in events.iter() {
        if let NotifyCommand::Delete(target_entity) = event {
            println!("🧠 Delete Todo: {:?}", target_entity);

            let mut list = vec![];
            for (entity, name, done, created_at, updated_at) in query.iter() {
                list.push(UITodoItem {
                    entity,
                    name: name.0.clone(),
                    done: done.0,
                    created_at: created_at.0,
                    updated_at: updated_at.0,
                });
            }
            let _res = sender.send(UICommand::Update(list));
        }
    }
}

// Event

enum NotifyCommand {
    List,
    Create(Entity),
    Update(Entity),
    Delete(Entity),
}
