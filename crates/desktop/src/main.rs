use components::{app, AppProps};
use dioxus;
use todo_core::{command::core::CoreCommand, command::ui::UICommand, start_ecs};
use tokio::sync::broadcast;

mod components;

fn main() {
    let (ui_tx, _) = broadcast::channel::<UICommand>(8);
    let (core_tx, _) = broadcast::channel::<CoreCommand>(8);

    let ui_tx2 = ui_tx.clone();
    let core_tx2 = core_tx.clone();

    std::thread::spawn(|| {
        start_ecs(ui_tx2, core_tx2);
    });

    dioxus::desktop::launch_with_props(app, AppProps { core_tx, ui_tx }, |c| c);
}
