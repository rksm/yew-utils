//! [Yew](https://yew.rs/) is a great framework for building WebAssembly
//! frontends with Rust. It's not exactly batteries included, however, and does
//! rely quite a bit on macros. This crate is not an official companion crate,
//! it just assembles a bunch of utilities and components I find convenient.
//!
//! ## yew_utils::vdom
//!
//! The `html!{}` macro can be convenient but it lacks code completion and
//! typing start/end tags can get old. `yew_utils::vdom` allows creating yew
//! nodes with simple Rust function calls.
//!
//! ```no_run
//! use yew::prelude::*;
//! use yew_utils::vdom::*;
//! use gloo_utils::window;
//! #[function_component(App)]
//! fn app() -> Html {
//!   div()
//!       .append(h1().text("yew-utils vdom example"))
//!       .append(
//!           form().append_all([
//!               label()
//!                   .attr("style", "font-weight: bold;")
//!                   .text("a button: "),
//!               button().text("click").onclick(|_| {
//!                   window().alert_with_message("hello").unwrap();
//!               }),
//!           ]),
//!       )
//!       .append(comp::<OtherComponent>())
//!       .into()
//! }
//!
//! #[function_component(OtherComponent)]
//! pub fn other_component() -> Html {
//!     text("hello from other component").into()
//! }
//! ```
//!
//! ## yew_utils::components
//!
//! A set of component. I'll likely add more over time. Currently it includes:
//!
//! ### [DropDown](yew_utils::components::DropDown)
//! ```no_run
//! use yew_utils::components::drop_down::{DropDown,DropDownProps};
//! use yew_utils::vdom::*;
//! use yew::prelude::*;
//!
//! # #[function_component(Example)]
//! # fn example() -> Html {
//! comp_with::<DropDown<&'static str>>(DropDownProps {
//!     initial: "item 1",
//!     options: vec!["item 1", "item 2", "item 3"],
//!     selection_changed: Callback::from(move |sel: &'static str| {
//!         gloo_utils::window()
//!             .alert_with_message(&format!("got selection: {sel:?}"))
//!             .unwrap();
//!     }),
//! })
//! # .into()
//! # }
//! ```
//!
//! ### [Table](yew_utils::components::Table)
//!
//! ```no_run
//! use yew_utils::components::table::Table;
//! use yew_utils::vdom::*;
//! use yew::prelude::*;
//!
//! # #[function_component(Example)]
//! # fn example() -> Html {
//! let columns = Children::new(
//!     ["col1", "col2"].map(|ea| text(ea).to_vnode()).to_vec(),
//! );
//!
//! let data = 0..5;
//! let rows = data
//!     .into_iter()
//!     .map(|data| {
//!         tr().key(data.to_string())
//!             .append_all([
//!                 td().text(data.to_string()),
//!                 td().text(format!("{data} (col2)")),
//!             ])
//!             .to_vnode()
//!     })
//!     .collect::<Vec<_>>();
//!
//! let table = Table::render(columns, yew::Children::new(rows));
//! # todo!();
//! # }
//! ```

pub mod components;
pub mod vdom;
