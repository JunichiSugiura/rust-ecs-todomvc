use dioxus::prelude::*;

pub fn use_once(cx: &ScopeState, f: impl FnOnce()) {
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
    };
}
