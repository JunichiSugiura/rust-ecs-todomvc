#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::cell::Cell;
use todo_core::command::{
    ecs::{CreateParams, ECSCommand, UpdateParams},
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

#[macro_export]
macro_rules! styled {
    ($name:ident, $element:ident, $style:expr) => {
        #[inline_props]
        pub fn $name<'a>(cx: Scope, children: Element<'a>) -> Element<'a> {
            cx.render(rsx! {
                $element {
                    style: $style,
                    children
                }
            })
        }
    }
}

styled!(Title, div, "
    font-size: 6.25rem;
    color: rgba(175, 47, 47, 0.15);
");

styled!(Box, div, "
    background: #f5f5f5;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100vh;
    width: 100%;
");

styled!(Item, div, "
    display: flex;
    align-items: center;
    width: 30rem;
");

styled!(ItemTitle, div, "");

pub fn app(cx: Scope<AppProps>) -> Element {
    let (list, set_list) = use_state(&cx, || UITodoList::default());
    let (name, set_name) = use_state(&cx, || String::new());
    let sender = use_ref(&cx, || cx.props.sender.take().unwrap());

    use_future(&cx, || {
        let receiver = cx.props.receiver.take();
        let set_list = set_list.to_owned(); async move {
            if let Some(receiver) = receiver {
                while let Ok(cmd) = receiver.subscribe().recv().await {
                    println!("🎨 {:?}", cmd);

                    match cmd {
                        UICommand::List(list) |
                        UICommand::Create(list) |
                        UICommand::Update(list) => {
                            set_list(list);
                        },
                    }
                }
            }
        }
    });

    use_once(&cx, || {
        let _res = sender.read().send(ECSCommand::List);
    });

    let submit = move || {
        if !name.is_empty() {
            let params = CreateParams::new(name.to_string());
            if let Ok(_res) = sender.read().send(ECSCommand::Create(params)) {
                set_name("".to_string());
            }
        }
    };

    cx.render(rsx! {
        style { [include_str!("../styles/global.css")] },

        Box {
            Title {
                "todos"
            }
            div {
                input {
                    placeholder: "What needs to be done?",
                    value: "{name}",
                    autofocus: "true",
                    oninput: move |e| set_name(e.value.clone()),
                    onkeydown: move |e| {
                        if e.key == "Enter" {
                            submit();
                        }
                    }
                }
            }

            list.iter().map(|item| {
                rsx!(
                    Item {
                        div {
                            class: "checkbox",
                            onclick: |_| {
                                let params = UpdateParams{
                                    entity: item.entity,
                                    done: Some(!item.done),
                                    name: None,
                                };
                                let _res = sender.read().send(ECSCommand::Update(params));
                            }
                        }
                        ItemTitle {
                            "{item.name}"
                        }
                    }
                )
            })
        }
    })
}
