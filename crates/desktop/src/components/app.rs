use crate::{components::*, state::CORE_TX, styled, util::use_once};
use dioxus::fermi::prelude::*;
use dioxus::prelude::*;
use todo_core::command::{
    core::{CoreCommand, UpdateParams},
    ui::{UICommand, UITodoList},
};
use tokio::sync::broadcast::Sender;

pub struct AppProps {
    pub core_tx: Sender<CoreCommand>,
    pub ui_tx: Sender<UICommand>,
}

styled!(
    Container,
    div,
    "
    background: #f5f5f5;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100vh;
    width: 100%;
"
);

styled!(
    List, div, "
"
);

styled!(
    Item,
    div,
    "
    display: flex;
    align-items: center;
    width: 20rem;
    padding: 0.5rem;
"
);

styled!(ItemTitle, div, "");

pub fn App(cx: Scope<AppProps>) -> Element {
    let (list, set_list) = use_state(&cx, || UITodoList::default());
    let core_tx = cx.props.core_tx.clone();
    let tx = use_read(&cx, CORE_TX);
    let set_core_tx = use_set(&cx, CORE_TX);

    use_future(&cx, || {
        let mut rx = cx.props.ui_tx.subscribe();
        let set_list = set_list.to_owned();
        async move {
            while let Ok(cmd) = rx.recv().await {
                println!("🎨 {:?}", cmd);

                match cmd {
                    UICommand::List(list)
                    | UICommand::Create(list)
                    | UICommand::Update(list)
                    | UICommand::Delete(list) => {
                        set_list(list);
                    }
                }
            }
        }
    });

    use_once(&cx, || {
        let _ = core_tx.send(CoreCommand::List);
        set_core_tx(Some(core_tx));
    });

    match tx {
        Some(tx) => cx.render(rsx! {
            style { [include_str!("../styles/global.css")] },
            Header {}

            Container {
                List {
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
                                        let _res = tx.send(CoreCommand::Update(params));
                                    },
                                    item.done.then(|| rsx!{
                                        "✅"
                                    })
                                }
                                ItemTitle {
                                    "{item.name}"
                                }
                            }
                        )
                    })
                }
            }
        }),
        None => None,
    }
}
