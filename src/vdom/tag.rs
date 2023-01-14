use std::{borrow::Cow, marker::PhantomData, rc::Rc};
use yew::html::IntoEventCallback;
use yew::virtual_dom as vdom;

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

pub struct TagTypeDefault;
pub struct TagTypeInput;

pub trait TagType {}
impl TagType for TagTypeDefault {}
impl TagType for TagTypeInput {}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

pub struct Tag<T: TagType = TagTypeDefault> {
    tag: vdom::VTag,
    listeners: Vec<Option<Rc<dyn vdom::Listener>>>,
    additional_props: PhantomData<T>,
}

impl<T: TagType> From<Tag<T>> for vdom::VNode {
    fn from(node: Tag<T>) -> Self {
        node.to_vnode()
    }
}

impl<T: TagType> From<Tag<T>> for yew::Children {
    fn from(node: Tag<T>) -> Self {
        yew::Children::new([node.into()].to_vec())
    }
}

impl<T> Tag<T>
where
    T: TagType,
{
    pub(crate) fn new(tag: impl Into<Cow<'static, str>>) -> Self {
        Self {
            tag: vdom::VTag::new(tag),
            listeners: Vec::new(),
            additional_props: PhantomData,
        }
    }

    fn with_props(tag: impl Into<Cow<'static, str>>) -> Self {
        Self {
            tag: vdom::VTag::new(tag),
            listeners: Vec::new(),
            additional_props: PhantomData,
        }
    }

    #[must_use]
    pub fn to_vnode(self) -> vdom::VNode {
        vdom::VNode::VTag(Box::new(self.tag))
    }

    #[must_use]
    pub fn class(self, class: impl Into<vdom::AttrValue>) -> Self {
        self.attr("class", class)
    }

    #[must_use]
    pub fn id(self, id: impl Into<vdom::AttrValue>) -> Self {
        self.attr("id", id)
    }

    #[must_use]
    pub fn style(self, style: impl Into<vdom::AttrValue>) -> Self {
        self.attr("style", style)
    }

    #[must_use]
    pub fn attr(mut self, key: &'static str, attr: impl Into<vdom::AttrValue>) -> Self {
        self.tag.add_attribute(key, attr);
        self
    }

    #[must_use]
    pub fn key(mut self, key: impl Into<vdom::Key>) -> Self {
        self.tag.key = Some(key.into());
        self
    }

    #[must_use]
    pub fn append(mut self, node: impl Into<vdom::VNode>) -> Self {
        self.tag.add_child(node.into());
        self
    }

    #[must_use]
    pub fn append_all(mut self, nodes: impl IntoIterator<Item = impl Into<vdom::VNode>>) -> Self {
        self.tag.add_children(nodes.into_iter().map(|n| n.into()));
        self
    }

    #[must_use]
    pub fn text(self, text: impl Into<vdom::AttrValue>) -> Self {
        self.append(vdom::VNode::VText(vdom::VText::new(text)))
    }

    #[must_use]
    pub fn node_ref(mut self, node_ref: yew::NodeRef) -> Self {
        self.tag.node_ref = node_ref;
        self
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// events

macro_rules! add_event_listener {
    ( $arg:ident ) => {
        #[must_use]
        pub fn $arg(mut self, listener: impl IntoEventCallback<::yew::html::$arg::Event>) -> Self {
            self.listeners
                .push(::yew::html::$arg::Wrapper::__macro_new(listener));
            self.update_listeners();
            self
        }
    };
}

impl<T> Tag<T>
where
    T: TagType,
{
    fn update_listeners(&mut self) {
        // argh, yeq implements the listeners list statically
        #[rustfmt::skip]
        self.tag.set_listeners(match self.listeners.as_slice() {
            [a] => Box::new([a.clone()]),
            [a, b] => Box::new([a.clone(), b.clone()]),
            [a, b, c] => Box::new([a.clone(), b.clone(), c.clone()]),
            [a, b, c, d] => Box::new([a.clone(), b.clone(), c.clone(), d.clone()]),
            [a, b, c, d, e] => Box::new([a.clone(), b.clone(), c.clone(), d.clone(), e.clone()]),
            _ => unimplemented!("More listeners than expected... probably need to extend this."),
        });
    }

    add_event_listener!(onabort);
    add_event_listener!(onauxclick);
    add_event_listener!(onblur);
    add_event_listener!(oncancel);
    add_event_listener!(oncanplay);
    add_event_listener!(oncanplaythrough);
    add_event_listener!(onchange);
    add_event_listener!(onclick);
    add_event_listener!(onclose);
    add_event_listener!(oncontextmenu);
    add_event_listener!(oncuechange);
    add_event_listener!(ondblclick);
    add_event_listener!(ondrag);
    add_event_listener!(ondragend);
    add_event_listener!(ondragenter);
    add_event_listener!(ondragexit);
    add_event_listener!(ondragleave);
    add_event_listener!(ondragover);
    add_event_listener!(ondragstart);
    add_event_listener!(ondrop);
    add_event_listener!(ondurationchange);
    add_event_listener!(onemptied);
    add_event_listener!(onended);
    add_event_listener!(onerror);
    add_event_listener!(onfocus);
    add_event_listener!(onfocusin);
    add_event_listener!(onfocusout);
    add_event_listener!(onformdata);
    add_event_listener!(oninput);
    add_event_listener!(oninvalid);
    add_event_listener!(onkeydown);
    add_event_listener!(onkeypress);
    add_event_listener!(onkeyup);
    add_event_listener!(onload);
    add_event_listener!(onloadeddata);
    add_event_listener!(onloadedmetadata);
    add_event_listener!(onloadstart);
    add_event_listener!(onmousedown);
    add_event_listener!(onmouseenter);
    add_event_listener!(onmouseleave);
    add_event_listener!(onmousemove);
    add_event_listener!(onmouseout);
    add_event_listener!(onmouseover);
    add_event_listener!(onmouseup);
    add_event_listener!(onpause);
    add_event_listener!(onplay);
    add_event_listener!(onplaying);
    add_event_listener!(onprogress);
    add_event_listener!(onratechange);
    add_event_listener!(onreset);
    add_event_listener!(onresize);
    add_event_listener!(onscroll);
    add_event_listener!(onsecuritypolicyviolation);
    add_event_listener!(onseeked);
    add_event_listener!(onseeking);
    add_event_listener!(onselect);
    add_event_listener!(onslotchange);
    add_event_listener!(onstalled);
    add_event_listener!(onsubmit);
    add_event_listener!(onsuspend);
    add_event_listener!(ontimeupdate);
    add_event_listener!(ontoggle);
    add_event_listener!(onvolumechange);
    add_event_listener!(onwaiting);
    add_event_listener!(onwheel);
    add_event_listener!(oncopy);
    add_event_listener!(oncut);
    add_event_listener!(onpaste);
    add_event_listener!(onanimationcancel);
    add_event_listener!(onanimationend);
    add_event_listener!(onanimationiteration);
    add_event_listener!(onanimationstart);
    add_event_listener!(ongotpointercapture);
    add_event_listener!(onloadend);
    add_event_listener!(onlostpointercapture);
    add_event_listener!(onpointercancel);
    add_event_listener!(onpointerdown);
    add_event_listener!(onpointerenter);
    add_event_listener!(onpointerleave);
    add_event_listener!(onpointerlockchange);
    add_event_listener!(onpointerlockerror);
    add_event_listener!(onpointermove);
    add_event_listener!(onpointerout);
    add_event_listener!(onpointerover);
    add_event_listener!(onpointerup);
    add_event_listener!(onselectionchange);
    add_event_listener!(onselectstart);
    add_event_listener!(onshow);
    add_event_listener!(ontouchcancel);
    add_event_listener!(ontouchend);
    add_event_listener!(ontouchmove);
    add_event_listener!(ontouchstart);
    add_event_listener!(ontransitioncancel);
    add_event_listener!(ontransitionend);
    add_event_listener!(ontransitionrun);
    add_event_listener!(ontransitionstart);
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

impl Tag<TagTypeInput> {
    pub(crate) fn input() -> Self {
        Tag::with_props("input")
    }

    #[must_use]
    pub fn checked(mut self, checked: bool) -> Self {
        self.tag.set_checked(checked);
        self
    }

    #[must_use]
    pub fn value(mut self, value: impl yew::html::IntoPropValue<Option<vdom::AttrValue>>) -> Self {
        self.tag.set_value(value);
        self
    }

    pub fn type_button(self) -> Self {
        self.attr("type", "button")
    }

    pub fn type_checkbox(self) -> Self {
        self.attr("type", "checkbox")
    }

    pub fn type_color(self) -> Self {
        self.attr("type", "color")
    }

    pub fn type_date(self) -> Self {
        self.attr("type", "date")
    }

    pub fn type_datetime(self) -> Self {
        self.attr("type", "datetime")
    }

    pub fn type_datetime_local(self) -> Self {
        self.attr("type", "datetime-local")
    }

    pub fn type_email(self) -> Self {
        self.attr("type", "email")
    }

    pub fn type_file(self) -> Self {
        self.attr("type", "file")
    }

    pub fn type_hidden(self) -> Self {
        self.attr("type", "hidden")
    }

    pub fn type_image(self) -> Self {
        self.attr("type", "image")
    }

    pub fn type_month(self) -> Self {
        self.attr("type", "month")
    }

    pub fn type_number(self) -> Self {
        self.attr("type", "number")
    }

    pub fn type_password(self) -> Self {
        self.attr("type", "password")
    }

    pub fn type_radio(self) -> Self {
        self.attr("type", "radio")
    }

    pub fn type_range(self) -> Self {
        self.attr("type", "range")
    }

    pub fn type_reset(self) -> Self {
        self.attr("type", "reset")
    }

    pub fn type_search(self) -> Self {
        self.attr("type", "search")
    }

    pub fn type_submit(self) -> Self {
        self.attr("type", "submit")
    }

    pub fn type_tel(self) -> Self {
        self.attr("type", "tel")
    }

    pub fn type_text(self) -> Self {
        self.attr("type", "text")
    }

    pub fn type_time(self) -> Self {
        self.attr("type", "time")
    }

    pub fn type_url(self) -> Self {
        self.attr("type", "url")
    }

    pub fn type_week(self) -> Self {
        self.attr("type", "week")
    }
}
