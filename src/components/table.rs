use yew::prelude::*;

use crate::vdom::{comp_with, tag};

/// Table component around html tables.
///
/// Example:
/// ```no_run
/// use yew_utils::components::table::Table;
/// use yew_utils::vdom::*;
/// use yew::prelude::*;
///
/// # #[function_component(Example)]
/// # fn example() -> Html {
/// let columns = Children::new(
///     ["col1", "col2"].map(|ea| text(ea).to_vnode()).to_vec(),
/// );
///
/// let data = 0..5;
/// let rows = data
///     .into_iter()
///     .map(|data| {
///         tr().key(data.to_string())
///             .append_all([
///                 td().text(data.to_string()),
///                 td().text(format!("{data} (col2)")),
///             ])
///             .to_vnode()
///     })
///     .collect::<Vec<_>>();
///
/// let table = Table::render(columns, yew::Children::new(rows));
/// # todo!();
/// # }
/// ```

pub struct Table;

impl Table {
    pub fn render(columns: Children, rows: Children) -> Html {
        comp_with::<Table>(TableProps {
            columns,
            children: rows,
        })
        .to_vnode()
    }
}

#[derive(PartialEq, Properties)]
pub struct TableProps {
    pub columns: Children,
    pub children: Children,
}

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let columns = &ctx.props().columns;
        let children = &ctx.props().children;
        tag("table")
            .class("mui-table")
            .append(
                tag("thead").append(
                    tag("tr").append_all(
                        columns
                            .iter()
                            .enumerate()
                            .map(|(i, node)| tag("th").append(node).key(i.to_string())),
                    ),
                ),
            )
            .append(tag("tbody").append_all(children.iter()))
            .to_vnode()
    }
}
