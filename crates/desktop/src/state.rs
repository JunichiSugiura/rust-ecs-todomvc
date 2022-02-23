use dioxus::fermi::prelude::*;
use todo_core::command::core::CoreCommand;
use tokio::sync::broadcast::Sender;

pub static CORE_TX: Atom<Option<Sender<CoreCommand>>> = |_| None;
