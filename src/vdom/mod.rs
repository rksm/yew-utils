//! Provides a Rust interface for constructing yew virtual DOM elements.

mod comp;
mod link;
mod tag;
mod text;

pub use comp::Comp;
pub use link::Link;
pub use tag::Tag;
pub use text::Text;

use std::borrow::Cow;
use yew::virtual_dom as vdom;

pub fn tag(tag: impl Into<Cow<'static, str>>) -> Tag {
    Tag::new(tag)
}

pub fn text(text: impl Into<vdom::AttrValue>) -> Text {
    Text::new(text)
}

#[cfg(feature = "yew-router")]
pub fn link<R: yew_router::Routable + 'static>(to: R) -> Link<R> {
    Link::new(to)
}

pub fn input() -> Tag<tag::TagTypeInput> {
    Tag::input()
}

pub fn checkbox() -> Tag<tag::TagTypeInput> {
    input().attr("type", "checkbox")
}

pub fn slider(min: i32, max: i32, value: i32) -> Tag<tag::TagTypeInput> {
    input()
        .attr("type", "range")
        .attr("min", min.to_string())
        .attr("max", max.to_string())
        .value(value.to_string())
}

pub fn comp_with<T>(props: T::Properties) -> Comp<T>
where
    T: yew::Component,
{
    Comp::new(props)
}

pub fn comp<T>() -> Comp<T>
where
    T: yew::Component,
    <T as yew::Component>::Properties: Default,
{
    Comp::new(Default::default())
}

macro_rules! known_tag {
    ( $arg:ident ) => {
        pub fn $arg() -> Tag {
            tag(stringify!($arg))
        }
    };
}

known_tag!(div);
known_tag!(span);
known_tag!(header);
known_tag!(footer);
known_tag!(nav);
known_tag!(h1);
known_tag!(h2);
known_tag!(h3);
known_tag!(h4);
known_tag!(h5);
known_tag!(hr);

known_tag!(table);
known_tag!(thead);
known_tag!(tbody);
known_tag!(th);
known_tag!(tr);
known_tag!(td);

known_tag!(ul);
known_tag!(li);

known_tag!(form);
known_tag!(label);
known_tag!(button);
