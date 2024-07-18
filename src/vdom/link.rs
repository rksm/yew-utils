use yew::virtual_dom as vdom;

/// Better interface for Links
pub struct Link<R>
where
    R: yew_router::Routable,
{
    props: yew_router::components::LinkProps<R>,
}

impl<R> Link<R>
where
    R: yew_router::Routable + 'static,
{
    pub fn new(to: R) -> Self {
        let props = yew_router::components::LinkProps::<R> {
            to,
            classes: yew::Classes::new(),
            query: None,
            disabled: false,
            children: vdom::VNode::VList(Default::default()).into(),
            anchor_ref: Default::default(),
            state: Default::default(),
        };

        Self { props }
    }

    #[must_use]
    pub fn append_all(mut self, nodes: impl IntoIterator<Item = impl Into<vdom::VNode>>) -> Self {
        let list = self.props.children.to_vlist_mut();
        list.add_children(nodes.into_iter().map(|ea| ea.into()));
        self
    }

    #[must_use]
    pub fn to_vnode(self) -> vdom::VNode {
        super::comp_with::<yew_router::components::Link<R>>(self.props).to_vnode()
    }
}

impl<R> From<Link<R>> for vdom::VNode
where
    R: yew_router::Routable + 'static,
{
    fn from(builder: Link<R>) -> Self {
        builder.to_vnode()
    }
}
