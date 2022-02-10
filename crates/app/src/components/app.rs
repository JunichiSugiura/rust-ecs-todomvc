use crossbeam_channel::{Receiver, Sender};
use dioxus::prelude::*;
use std::cell::Cell;
use todo_core::command::{
    ecs::{CreateTodoParams, ECSCommand},
    ui::{UICommand, UITodoList},
};

pub struct AppProps {
    pub sender: Cell<Option<Sender<ECSCommand>>>,
    pub receiver: Cell<Option<Receiver<UICommand>>>,
}

pub fn app(cx: Scope<AppProps>) -> Element {
    let _todo_list = use_state(&cx, || UITodoList::default());
    let (name, set_name) = use_state(&cx, || String::new());
    let sender = use_ref(&cx, || cx.props.sender.take().unwrap());
    let disable_submit = name.is_empty();

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
                            if !disable_submit {
                                let params = CreateTodoParams::new(name.to_string());
                                sender.read().send(ECSCommand::CreateTodo(params)).unwrap();
                                set_name("".to_string());
                            }
                        }
                    }
                }
                button {
                    onclick: move |_| {
                        if !name.is_empty() {
                            let params = CreateTodoParams::new(name.to_string());
                            sender.read().send(ECSCommand::CreateTodo(params)).unwrap();
                            set_name("".to_string());
                        }
                    },
                    disabled: "{disable_submit}",
                    "Create"
                }
            }
            button {
                onclick: move |_| {
                    sender.read().send(ECSCommand::ListTodo).unwrap();
                },
                "List"
            }
        }
    })
}
