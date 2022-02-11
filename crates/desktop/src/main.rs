use components::{app, AppProps};
use dioxus;
use std::cell::Cell;
use todo_core::{command::ecs::ECSCommand, command::ui::UICommand, start_ecs};
use tokio::sync::broadcast;

mod components;

fn main() {
    let (ui_tx, _ui_rx) = broadcast::channel::<UICommand>(32);
    let (ecs_tx, _ecs_rx) = broadcast::channel::<ECSCommand>(32);

    let ui_tx_clone = ui_tx.clone();
    let ecs_tx_clone = ecs_tx.clone();

    std::thread::spawn(|| {
        start_ecs(ui_tx_clone, ecs_tx_clone);
    });

    dioxus::desktop::launch_with_props(
        app,
        AppProps {
            sender: Cell::new(Some(ecs_tx)),
            receiver: Cell::new(Some(ui_tx)),
        },
        |c| c,
    );
}
