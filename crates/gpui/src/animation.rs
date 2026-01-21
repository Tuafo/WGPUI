    AnyElement, App, AvailableSpace, Element, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, Pixels, Size, Window,
    elements::virtualized_list,
    render_node::{
        CallbackSlot, LayoutCtx, LayoutFrame, PaintCtx, PaintFrame, PrepaintCtx, PrepaintFrame,
        RenderNode, UpdateResult,
    },
use taffy::style::{Dimension, Style as TaffyStyle};
/// Type alias for the callback that produces the animated child.
type ProduceChildFn = dyn FnOnce(usize, f32) -> AnyElement;

/// Retained render node for AnimationElement.
///
/// Stores animation timing state and uses fiber-backed rendering for the
/// animated child. The child is computed during measure() and rendered
/// during prepaint/paint.
pub(crate) struct AnimationNode {
    animations: SmallVec<[Animation; 1]>,
    produce_child: CallbackSlot<ProduceChildFn>,
    child_fiber_id: Option<GlobalElementId>,
    done: bool,
}

impl AnimationNode {
    fn new(animations: SmallVec<[Animation; 1]>) -> Self {
        Self {
            start: Instant::now(),
            animation_ix: 0,
            animations,
            produce_child: CallbackSlot::new(),
            child_fiber_id: None,
            done: false,
        }
    }

    fn compute_delta_and_advance(&mut self) -> (usize, f32) {
        let animation_ix = self.animation_ix;
        let mut delta = self.start.elapsed().as_secs_f32()
            / self.animations[animation_ix].duration.as_secs_f32();

        if delta > 1.0 {
            if self.animations[animation_ix].oneshot {
                if animation_ix >= self.animations.len() - 1 {
                    self.done = true;
                } else {
                    self.start = Instant::now();
                    self.animation_ix += 1;
                }
                delta = 1.0;
            } else {
                delta %= 1.0;
            }
        }

        let eased_delta = (self.animations[animation_ix].easing)(delta);
        debug_assert!(
            (0.0..=1.0).contains(&eased_delta),
            "delta should always be between 0 and 1"
        );

        (animation_ix, eased_delta)
    }
}

impl RenderNode for AnimationNode {
    fn taffy_style(&self, _rem_size: Pixels, _scale_factor: f32) -> TaffyStyle {
        TaffyStyle {
            size: taffy::prelude::Size {
                width: Dimension::auto(),
                height: Dimension::auto(),
            },
            ..Default::default()
        }
    }

    fn compute_intrinsic_size(
        &mut self,
        _ctx: &mut crate::SizingCtx,
    ) -> crate::IntrinsicSizeResult {
        crate::IntrinsicSizeResult {
            size: crate::IntrinsicSize::default(),
            input: crate::SizingInput::default(),
        }
    }

    fn uses_intrinsic_sizing_cache(&self) -> bool {
        false
    }

    fn layout_begin(&mut self, _ctx: &mut LayoutCtx) -> LayoutFrame {
        LayoutFrame {
            handled: true,
            ..Default::default()
        }
    }

    fn layout_end(&mut self, _ctx: &mut LayoutCtx, _frame: LayoutFrame) {}

    fn measure(
        &mut self,
        _known: Size<Option<Pixels>>,
        available: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Size<Pixels>> {
        let (animation_ix, delta) = self.compute_delta_and_advance();

        let produce = self.produce_child.take()?;
        let mut child = produce(animation_ix, delta);

        let child_fiber_id = self
            .child_fiber_id
            .get_or_insert_with(|| window.fiber.tree.create_placeholder_fiber());

        let size =
            virtualized_list::layout_item_fiber(*child_fiber_id, &mut child, available, window, cx);

        if !self.done {
            window.request_animation_frame();
        }

        Some(size)
    }

    fn prepaint_begin(&mut self, ctx: &mut PrepaintCtx) -> PrepaintFrame {
        if let Some(child_fiber_id) = self.child_fiber_id {
            virtualized_list::prepaint_item_fiber(
                child_fiber_id,
                ctx.bounds.origin,
                crate::ContentMask { bounds: ctx.bounds },
                ctx.window,
                ctx.cx,
            );
        }
        PrepaintFrame {
            handled: true,
            ..Default::default()
        }
    }

    fn prepaint_end(&mut self, _ctx: &mut PrepaintCtx, _frame: PrepaintFrame) {}

    fn paint_begin(&mut self, ctx: &mut PaintCtx) -> PaintFrame {
        if let Some(child_fiber_id) = self.child_fiber_id {
            ctx.window
                .fibers()
                .paint_fiber_tree_internal(child_fiber_id, ctx.cx, false);
        }
        PaintFrame {
            handled: true,
            ..Default::default()
        }
    }

    fn paint_end(&mut self, _ctx: &mut PaintCtx, _frame: PaintFrame) {}
    type RequestLayoutState = ();
        _global_id: Option<&GlobalElementId>,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("AnimationElement uses retained node path")
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("AnimationElement uses retained node path")
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("AnimationElement uses retained node path")
    }

    fn create_render_node(&mut self) -> Option<Box<dyn RenderNode>> {
        let mut node = AnimationNode::new(std::mem::take(&mut self.animations));

        // Deposit the callback immediately (same logic as update_render_node)
        if let Some(element) = self.element.take() {
            let animator = std::mem::replace(&mut self.animator, Box::new(|e, _, _| e));
            node.produce_child
                .deposit(Box::new(move |animation_ix, delta| {
                    (animator)(element, animation_ix, delta).into_any_element()
                }));
        }

        Some(Box::new(node))
    }

    fn update_render_node(
        &mut self,
        node: &mut dyn RenderNode,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<UpdateResult> {
        let node = node.as_any_mut().downcast_mut::<AnimationNode>()?;

        node.animations = std::mem::take(&mut self.animations);

        let element = self.element.take()?;
        let animator = std::mem::replace(&mut self.animator, Box::new(|e, _, _| e));

        node.produce_child
            .deposit(Box::new(move |animation_ix, delta| {
                (animator)(element, animation_ix, delta).into_any_element()
            }));

        Some(UpdateResult::LAYOUT_CHANGED)