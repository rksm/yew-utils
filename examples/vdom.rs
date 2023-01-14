use yew::prelude::*;
use yew_utils::vdom::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(Component1)]
pub fn component1() -> Html {
    text("hello from component 1").into()
}

#[derive(Properties, PartialEq, Clone)]
pub struct Component2Props {
    prop: String,
}

#[function_component(Component2)]
pub fn component2(props: &Component2Props) -> Html {
    div()
        .append_all([text("hello from component 2"), text(props.prop.clone())])
        .into()
}

#[function_component(App)]
fn app() -> Html {
    div()
        .append(h1().text("yew-utils vdom example"))
        .append(comp::<Component1>())
        .into()
}
