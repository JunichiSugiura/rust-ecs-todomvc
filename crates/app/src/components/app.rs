use dioxus::prelude::*;
use std::cell::Cell;
use todo_core::command::{
    ecs::{CreateTodoParams, ECSCommand},
    ui::{UICommand, UITodoList},
};
use tokio::sync::broadcast::{Sender};

pub struct AppProps {
    pub sender: Cell<Option<Sender<ECSCommand>>>,
    pub receiver: Cell<Option<Sender<UICommand>>>,
}

fn use_once(cx: &ScopeState, f: impl FnOnce()) {
    let init = cx.use_hook(|_| true);
    if *init {
        f();
        *init = false;
    }
}

pub fn app(cx: Scope<AppProps>) -> Element {
    let (list, set_list)= use_state(&cx, || UITodoList::default());
    let (name, set_name) = use_state(&cx, || String::new());
    let sender = use_ref(&cx, || cx.props.sender.take().unwrap());
    let disable_submit = name.is_empty();

    use_future(&cx, || {
        let receiver = cx.props.receiver.take();
        let set_list = set_list.to_owned();
        async move {
            if let Some(receiver) = receiver {
                while let Ok(cmd) = receiver.subscribe().recv().await {
                    println!("🎨 {:?}", cmd);

                    match cmd {
                        UICommand::ListTodo(list) | UICommand::CreateTodo(list) => {
                            set_list(list);
                        },
                    }
                }
            }
        }
    });

    use_once(&cx, || {
        let _res = sender.read().send(ECSCommand::ListTodo);
    });

    let submit = move || {
        if !name.is_empty() {
            let params = CreateTodoParams::new(name.to_string());
            if let Ok(_res) = sender.read().send(ECSCommand::CreateTodo(params)) {
                set_name("".to_string());
            }
        }
    };

    cx.render(rsx! {
        div {
            div {
                input {
                    placeholder: "What needs to be done?",
                    value: "{name}",
                    autofocus: "true",
                    oninput: move |e| set_name(e.value.clone()),
                    onkeydown: move |evt| {
                        if evt.key == "Enter" {
                            submit();
                        }
                    }
                }
                button {
                    onclick: move |_| {
                        submit();
                    },
                    disabled: "{disable_submit}",
                    "Create"
                }
            }

            list.iter().enumerate().map(|(i, item)| {
                let created_at = item.created_at.format("%Y-%m-%d %H:%M:%S");
                let done = if item.done { "✅" } else { "❌" };
                let index = i + 1;
                rsx!(
                    div {
                        div {
                            "Task #{index}: {item.name}"
                        }
                        div {
                            "Status: {done}"
                        }
                        div {
                            "{created_at}"
                        }
                    }
                )
            })
        }
    })
}
