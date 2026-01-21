    AnyElement, AnyEntity, AnyWeakEntity, App, AvailableSpace, Bounds, Context, Element, ElementId,
    Entity, EntityId, GlobalElementId, InspectorElementId, IntoElement, LayoutId, Pixels, Render,
    RenderNode, Size, StyleRefinement, UpdateResult, VKey, WeakEntity,
use std::{any::TypeId, fmt};
use taffy::style::{Dimension, Style as TaffyStyle};
#[doc(hidden)]
pub struct ViewLayoutState {
    element: AnyElement,
    type RequestLayoutState = ViewLayoutState;
        let global_id = window.register_view_fiber(self.entity_id());

        cx.entities.push_access_scope();
        cx.entities.record_access(self.entity_id());
        let accessed_entities = cx.entities.pop_access_scope();
        window.record_pending_view_accesses(&global_id, accessed_entities);
        (layout_id, ViewLayoutState { element })
        window.with_rendered_view(self.entity_id(), |window| {
            element.element.prepaint(window, cx);
        });
        window.with_rendered_view(self.entity_id(), |window| {
            element.element.paint(window, cx);
        });
    }

    fn fiber_key(&self) -> VKey {
        VKey::View(self.entity_id())
    }

    fn as_any_view(&self) -> Option<AnyView> {
        Some(AnyView::from(self.clone()))
    }

    fn create_render_node(&mut self) -> Option<Box<dyn RenderNode>> {
        Some(Box::new(ViewNode {
            view_id: self.entity_id(),
        }))
    }

    fn update_render_node(
        &mut self,
        node: &mut dyn RenderNode,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<UpdateResult> {
        let view_node = node.as_any_mut().downcast_mut::<ViewNode>()?;
        // View identity is stable (EntityId doesn't change), so nothing to update
        debug_assert_eq!(view_node.view_id, self.entity_id());
        Some(UpdateResult::UNCHANGED)
    }
}

/// Retained render node for view elements.
///
/// Views are boundaries in the component tree. The ViewNode itself doesn't render anything -
/// it just marks the boundary and delegates to the view's rendered element tree (which is
/// expanded during reconciliation via `expand_view_fibers`).
pub(crate) struct ViewNode {
    view_id: EntityId,
}

impl RenderNode for ViewNode {
    fn taffy_style(&self, _rem_size: Pixels, _scale_factor: f32) -> TaffyStyle {
        // Views are layout-transparent - their child element determines the layout
        TaffyStyle {
            size: taffy::prelude::Size {
                width: Dimension::auto(),
                height: Dimension::auto(),
            },
            ..Default::default()
        }
    }

    fn compute_intrinsic_size(&mut self, _ctx: &mut crate::SizingCtx) -> crate::IntrinsicSizeResult {
        crate::IntrinsicSizeResult {
            size: crate::IntrinsicSize::default(),
            input: crate::SizingInput::default(),
        }
    }

    fn measure(
        &mut self,
        _known: Size<Option<Pixels>>,
        _available: Size<AvailableSpace>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<Size<Pixels>> {
        // Views don't have intrinsic size - layout comes from their children
        None
    }

    fn layout_begin(&mut self, ctx: &mut crate::LayoutCtx) -> crate::LayoutFrame {
        // Register this view fiber in view_roots so expand_view_fibers can find it
        // and render the view's content
        ctx.window
            .fiber
            .tree
            .set_view_root(self.view_id, ctx.fiber_id);

        // Push view boundary for scope tracking
        ctx.window.rendered_entity_stack.push(self.view_id);
        crate::LayoutFrame {
            handled: true,
            pushed_view_boundary: true,
            ..Default::default()
        }
    }

    fn layout_end(&mut self, ctx: &mut crate::LayoutCtx, frame: crate::LayoutFrame) {
        if frame.pushed_view_boundary {
            ctx.window.rendered_entity_stack.pop();
        }
    }

    fn prepaint_begin(&mut self, ctx: &mut crate::PrepaintCtx) -> crate::PrepaintFrame {
        // Set the view context for prepaint
        ctx.window.set_view_id(self.view_id);
        crate::PrepaintFrame {
            handled: true,
            ..Default::default()
        }
    }

    fn prepaint_end(&mut self, _ctx: &mut crate::PrepaintCtx, _frame: crate::PrepaintFrame) {
        // Nothing to pop - view context is per-frame
    }

    fn paint_begin(&mut self, _ctx: &mut crate::PaintCtx) -> crate::PaintFrame {
        // Views don't paint anything themselves - their children do
        crate::PaintFrame {
            handled: true,
            ..Default::default()
        }
    }

    fn paint_end(&mut self, _ctx: &mut crate::PaintCtx, _frame: crate::PaintFrame) {
        // Nothing to clean up
    /// Legacy caching hint - now a no-op.
    ///
    /// The fiber architecture handles caching automatically. Views are only
    /// re-rendered when their state changes (via `cx.notify()`). This method
    /// is retained for API compatibility but has no effect.
    #[allow(unused_variables)]
    pub fn cached(self, style: StyleRefinement) -> Self {

    /// Render this view to an AnyElement.
    pub(crate) fn render_element(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        (self.render)(self, window, cx)
    }
    type RequestLayoutState = ViewLayoutState;
    type PrepaintState = ();
        // Register this view's fiber for dirty tracking, but avoid aliasing the descriptor root
        // fiber when drawing the window's root view.
        let is_window_root_view = window
            .root
            .as_ref()
            .is_some_and(|root| root.entity_id() == self.entity_id());
        let global_id =
            (!is_window_root_view).then(|| window.register_view_fiber(self.entity_id()));

            cx.entities.push_access_scope();
            cx.entities.record_access(self.entity_id());
            let mut element = (self.render)(self, window, cx);
            let layout_id = element.request_layout(window, cx);
            let accessed_entities = cx.entities.pop_access_scope();
            if let Some(global_id) = global_id {
                window.record_pending_view_accesses(&global_id, accessed_entities);
            (layout_id, ViewLayoutState { element })
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
    ) {
            element.element.prepaint(window, cx);
        });
        _id: Option<&GlobalElementId>,
        element: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
            element.element.paint(window, cx);

    fn fiber_key(&self) -> VKey {
        VKey::View(self.entity_id())
    }

    fn as_any_view(&self) -> Option<AnyView> {
        Some(self.clone())
    }