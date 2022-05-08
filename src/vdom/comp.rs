use std::rc::Rc;
use yew::virtual_dom as vdom;

/// Better interface for [vdom::VComp]
pub struct Comp<T>
where
    T: yew::Component,
{
    props: T::Properties,
}

impl<T> Comp<T>
where
    T: yew::Component,
{
    pub(crate) fn new(props: T::Properties) -> Self {
        Self { props }
    }

    #[must_use]
    pub fn to_vnode(self) -> vdom::VNode {
        let node_ref = yew::NodeRef::default();
        let comp = vdom::VComp::new::<T>(Rc::new(self.props), node_ref, None);
        vdom::VNode::VComp(comp)
    }
}

impl<T> From<Comp<T>> for vdom::VNode
where
    T: yew::Component,
{
    fn from(builder: Comp<T>) -> Self {
        builder.to_vnode()
    }
}

impl<T> From<Comp<T>> for yew::Children
where
    T: yew::Component,
{
    fn from(comp: Comp<T>) -> Self {
        yew::Children::new([comp.into()].to_vec())
    }
}
