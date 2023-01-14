//! Provides a Rust interface for constructing yew virtual DOM elements.

mod comp;
mod tag;
mod text;

pub use comp::Comp;
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

pub fn input() -> Tag<tag::TagTypeInput> {
    Tag::input()
}

pub fn checkbox() -> Tag<tag::TagTypeInput> {
    input().type_checkbox()
}

pub fn slider(min: i32, max: i32, value: i32) -> Tag<tag::TagTypeInput> {
    input()
        .attr("type", "range")
        .attr("min", min.to_string())
        .attr("max", max.to_string())
        .value(value.to_string())
}

pub fn comp_with<'a, T>(props: T::Properties) -> Comp<'a, T>
where
    T: yew::BaseComponent,
{
    Comp::new(props)
}

pub fn comp<'a, T>() -> Comp<'a, T>
where
    T: yew::BaseComponent,
    <T as yew::BaseComponent>::Properties: Default,
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

known_tag!(a);
known_tag!(abbr);
known_tag!(acronym);
known_tag!(address);
known_tag!(applet);
known_tag!(area);
known_tag!(article);
known_tag!(aside);
known_tag!(audio);
known_tag!(b);
known_tag!(base);
known_tag!(basefont);
known_tag!(bdi);
known_tag!(bdo);
known_tag!(bgsound);
known_tag!(big);
known_tag!(blink);
known_tag!(blockquote);
known_tag!(body);
known_tag!(br);
known_tag!(button);
known_tag!(canvas);
known_tag!(caption);
known_tag!(center);
known_tag!(cite);
known_tag!(code);
known_tag!(col);
known_tag!(colgroup);
known_tag!(content);
known_tag!(data);
known_tag!(datalist);
known_tag!(dd);
known_tag!(del);
known_tag!(details);
known_tag!(dfn);
known_tag!(dialog);
known_tag!(dir);
known_tag!(div);
known_tag!(dl);
known_tag!(dt);
known_tag!(em);
known_tag!(embed);
known_tag!(fieldset);
known_tag!(figcaption);
known_tag!(figure);
known_tag!(font);
known_tag!(footer);
known_tag!(form);
known_tag!(frame);
known_tag!(frameset);
known_tag!(h1);
known_tag!(h2);
known_tag!(h3);
known_tag!(h4);
known_tag!(h5);
known_tag!(h6);
known_tag!(head);
known_tag!(header);
known_tag!(hgroup);
known_tag!(hr);
known_tag!(html);
known_tag!(i);
known_tag!(iframe);
known_tag!(image);
known_tag!(img);
// known_tag!(input);
known_tag!(ins);
known_tag!(kbd);
known_tag!(keygen);
known_tag!(label);
known_tag!(legend);
known_tag!(li);
known_tag!(link);
known_tag!(main);
known_tag!(map);
known_tag!(mark);
known_tag!(marquee);
known_tag!(math);
known_tag!(menu);
known_tag!(menuitem);
known_tag!(meta);
known_tag!(meter);
known_tag!(nav);
known_tag!(nobr);
known_tag!(noembed);
known_tag!(noframes);
known_tag!(noscript);
known_tag!(object);
known_tag!(ol);
known_tag!(optgroup);
known_tag!(option);
known_tag!(output);
known_tag!(p);
known_tag!(param);
known_tag!(picture);
known_tag!(plaintext);
known_tag!(portal);
known_tag!(pre);
known_tag!(progress);
known_tag!(q);
known_tag!(rb);
known_tag!(rp);
known_tag!(rt);
known_tag!(rtc);
known_tag!(ruby);
known_tag!(s);
known_tag!(samp);
known_tag!(script);
known_tag!(section);
known_tag!(select);
known_tag!(shadow);
known_tag!(slot);
known_tag!(small);
known_tag!(source);
known_tag!(spacer);
known_tag!(span);
known_tag!(strike);
known_tag!(strong);
known_tag!(style);
known_tag!(sub);
known_tag!(summary);
known_tag!(sup);
known_tag!(svg);
known_tag!(table);
known_tag!(tbody);
known_tag!(td);
known_tag!(template);
known_tag!(textarea);
known_tag!(tfoot);
known_tag!(th);
known_tag!(thead);
known_tag!(time);
known_tag!(title);
known_tag!(tr);
known_tag!(track);
known_tag!(tt);
known_tag!(u);
known_tag!(ul);
known_tag!(var);
known_tag!(video);
known_tag!(wbr);
known_tag!(xmp);

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// yew-router link

#[cfg(feature = "yew-router")]
mod link;

#[cfg(feature = "yew-router")]
pub use link::Link;

#[cfg(feature = "yew-router")]
pub fn yew_link<R: yew_router::Routable + 'static>(to: R) -> Link<R> {
    Link::new(to)
}
