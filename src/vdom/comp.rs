use std::rc::Rc;
use yew::virtual_dom as vdom;

/// Better interface for [vdom::VComp]
pub struct Comp<'a, T>
where
    T: yew::BaseComponent,
{
    props: T::Properties,
    key: Option<&'a str>,
}

impl<'a, T> Comp<'a, T>
where
    T: yew::BaseComponent,
{
    pub(crate) fn new(props: T::Properties) -> Self {
        Self { props, key: None }
    }

    #[must_use]
    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn to_vnode(self) -> vdom::VNode {
        let key = self.key.map(|k| k.into());
        let comp = vdom::VComp::new::<T>(Rc::new(self.props), key);
        vdom::VNode::VComp(comp)
    }
}

impl<'a, T> From<Comp<'a, T>> for vdom::VNode
where
    T: yew::BaseComponent,
{
    fn from(builder: Comp<T>) -> Self {
        builder.to_vnode()
    }
}

impl<'a, T> From<Comp<'a, T>> for yew::Children
where
    T: yew::BaseComponent,
{
    fn from(comp: Comp<T>) -> Self {
        yew::Children::new([comp.into()].to_vec())
    }
}
