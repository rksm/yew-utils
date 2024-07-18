use yew::virtual_dom as vdom;

/// Better interface for [vdom::VText]
pub struct Text {
    text: vdom::VText,
}

impl Text {
    pub(crate) fn new(text: impl Into<vdom::AttrValue>) -> Self {
        Self {
            text: vdom::VText::new(text),
        }
    }

    pub fn to_vnode(self) -> vdom::VNode {
        vdom::VNode::VText(self.text)
    }
}

impl From<Text> for vdom::VNode {
    fn from(text: Text) -> Self {
        text.to_vnode()
    }
}

impl From<Text> for yew::Children {
    fn from(text: Text) -> Self {
        yew::Children::new([text.into()].to_vec())
    }
}
