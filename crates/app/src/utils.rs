#[macro_export]
macro_rules! styled {
    ($name:ident, $element:ident, $style:expr) => {
        pub fn $name<'a>(cx: Scope, children: Element<'a>) -> Element<'a> {
            #[inline_props]
            cx.render(rsx! {
                $element {
                    style: $style,
                    children
                }
            })
        }
    };
}
