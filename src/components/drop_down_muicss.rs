use std::fmt::Display;

use yew::prelude::*;

pub struct DropDown<T> {
    selected: T,
}

pub enum Msg<T> {
    SelectionChanged(T),
}

#[derive(PartialEq, Properties)]
pub struct DropDownProps<T>
where
    T: PartialEq,
{
    pub initial: T,
    pub options: Vec<T>,
    pub selection_changed: Callback<T>,
}

impl<T> Component for DropDown<T>
where
    T: Display + Clone + PartialEq + yew::ToHtml + 'static,
{
    type Message = Msg<T>;
    type Properties = DropDownProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected: ctx.props().initial.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="mui-dropdown">
              <button class="mui-btn mui-btn--primary" data-mui-toggle="dropdown">
                {&self.selected}
                <span class="mui-caret"></span>
              </button>
              <ul class="mui-dropdown__menu">
                    {
                        for ctx.props().options.iter().map(|opt| {
                            let opt2 = opt.clone();
                            let onclick = ctx.link().callback(move |e: MouseEvent| { e.prevent_default(); Msg::SelectionChanged(opt2.clone()) });
                            html! {
                                <li>
                                    <a href="" onclick={onclick}>
                                    {&opt}
                                    </a>
                                </li>
                            }
                        })
                    }
              </ul>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectionChanged(opt) => {
                self.selected = opt.clone();
                ctx.props().selection_changed.emit(opt);
                true
            }
        }
    }
}
