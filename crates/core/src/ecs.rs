use crate::command::{ecs::ECSCommand, ui::{UICommand, UITodoItem}};
use bevy_app::prelude::*;
use bevy_ecs::{event::Events, prelude::*};
use chrono::prelude::*;
use tokio::sync::broadcast::{Sender};

pub fn start_ecs(sender: Sender<UICommand>, receiver: Sender<ECSCommand>) {
    App::new()
        .set_runner(runner)
        .insert_resource(sender)
        .insert_resource(receiver)
        .add_event::<ECSCommand>()
        .add_event::<NotifyCommand>()
        .add_startup_system(setup)
        .add_system(handle_list_todo)
        .add_system(handle_create_todo)
        .add_system_to_stage(CoreStage::Last, notify_list_todo)
        .add_system_to_stage(CoreStage::Last, notify_create_todo)
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
struct Done(bool);

// Runner

fn runner(mut app: App) {
    app.update();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<ECSCommand>(32);

    let runtime = tokio::runtime::Runtime::new().unwrap();

    if let Some(sender) = app.world.get_resource_mut::<Sender<ECSCommand>>() {
            let mut rx = sender.subscribe();
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
        if let ECSCommand::ListTodo = event {
            notify.send(NotifyCommand::ListTodo);
        }
    }
}

fn handle_create_todo(
    mut events: EventReader<ECSCommand>,
    mut notify: EventWriter<NotifyCommand>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if let ECSCommand::CreateTodo(params) = event {
            let id = commands
                .spawn()
                .insert(Todo)
                .insert(Name(params.name.to_string()))
                .insert(CreatedAt(Utc::now()))
                .insert(Done(false))
                .id();

            notify.send(NotifyCommand::CreateTodo(id));
        }
    }
}

fn notify_list_todo(
    mut events: EventReader<NotifyCommand>,
    query: Query<(&Name, &CreatedAt, &Done), With<Todo>>,
    sender: Res<Sender<UICommand>>
) {
    for event in events.iter() {
        if let NotifyCommand::ListTodo = event {
            println!("🧠 List Todo");

            let mut list = vec![];
            for (name, created_at, done) in  query.iter() {
                list.push(UITodoItem {
                    name: name.0.clone(),
                    created_at: created_at.0,
                    done: done.0,
                });
            }
            let _res = sender.send(UICommand::ListTodo(list));
        }
    }
}

// TODO: consider sending just a diff but it requires more work on UI side
fn notify_create_todo(
    mut events: EventReader<NotifyCommand>,
    query: Query<(&Name, &CreatedAt, &Done), With<Todo>>,
    sender: Res<Sender<UICommand>>
) {
    for event in events.iter() {
        if let NotifyCommand::CreateTodo(_target_entity) = event {
            println!("🧠 Create Todo");

            let mut list = vec![];
            for (name, created_at, done) in  query.iter() {
                list.push(UITodoItem {
                    name: name.0.clone(),
                    created_at: created_at.0,
                    done: done.0,
                });
            }
            let _res = sender.send(UICommand::CreateTodo(list));
        }
    }
}

// Event

enum NotifyCommand {
    ListTodo,
    CreateTodo(Entity),
}
