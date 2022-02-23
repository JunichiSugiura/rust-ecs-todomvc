use crate::{state::CORE_TX, styled};
use dioxus::fermi::prelude::*;
use dioxus::prelude::*;
use todo_core::command::core::{CoreCommand, CreateParams};

styled!(
    Wrapper,
    div,
    " display: flex;
    flex-direction: column;
    align-items: center;
    padding-bottom: 1rem;
"
);

styled!(
    Title,
    div,
    "
    font-size: 6.25rem;
    color: rgba(175, 47, 47, 0.15);
"
);

pub fn Header(cx: Scope) -> Element {
    let tx = use_read(&cx, CORE_TX);
    let (name, set_name) = use_state(&cx, || String::new());

    let submit = move || {
        if !name.is_empty() {
            let params = CreateParams::new(name.to_string());
            if let Some(tx) = tx {
                if let Ok(_res) = tx.send(CoreCommand::Create(params)) {
                    set_name("".to_string());
                }
            }
        }
    };

    cx.render(rsx! {
        Wrapper {
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
        }
    })
}
