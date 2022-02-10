use components::{app, AppProps};
use crossbeam_channel::unbounded;
use dioxus;
use std::cell::Cell;
use todo_core::{command::ecs::ECSCommand, command::ui::UICommand, start_ecs};

mod components;

fn main() {
    let (ui_tx, ui_rx) = unbounded::<UICommand>();
    let (ecs_tx, ecs_rx) = unbounded::<ECSCommand>();

    std::thread::spawn(|| {
        start_ecs(ui_tx, ecs_rx);
    });

    dioxus::desktop::launch_with_props(
        app,
        AppProps {
            sender: Cell::new(Some(ecs_tx)),
            receiver: Cell::new(Some(ui_rx)),
        },
        |c| c,
    );
}
