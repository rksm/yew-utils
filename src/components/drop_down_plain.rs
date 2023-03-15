use std::fmt::Display;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub struct DropDown<T> {
    selected: T,
    node: NodeRef,
}

pub enum Msg {
    SelectionChanged(usize),
}

#[derive(PartialEq, Properties)]
pub struct DropDownProps<T>
where
    T: PartialEq,
{
    pub initial: T,
    pub options: Vec<T>,
    pub selection_changed: Callback<T>,
    pub class_css: Option<String>,
}

impl<T> Component for DropDown<T>
where
    T: Display + Clone + PartialEq + 'static,
{
    type Message = Msg;
    type Properties = DropDownProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected: ctx.props().initial.clone(),
            node: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let node = self.node.clone();
        let css_class = match ctx.props().class_css.clone() {
            Some(class) => class,
            None => "".into(),
        };
        html! {
            <select class={css_class} ref={node.clone()} onchange={ctx.link().callback(move |_| {
                let node2: HtmlSelectElement = node.cast().unwrap();
                let idx = node2.selected_index() as usize;
                Msg::SelectionChanged(idx)
            })}>
            {
                for ctx.props().options.iter().map(|opt| {
                    if opt == &self.selected {
                        html! {
                            <option value={opt.to_string()} selected=true>{opt}</option>
                        }
                    } else {
                        html! {
                            <option value={opt.to_string()}>{opt}</option>
                        }
                    }
                })

            }
            </select>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectionChanged(idx) => {
                if let Some(selected) = ctx.props().options.get(idx) {
                    self.selected = selected.clone();
                    ctx.props().selection_changed.emit(selected.clone());
                }
                true
            }
        }
    }
}
