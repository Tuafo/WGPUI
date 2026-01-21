use super::virtualized_list::{
    ItemFiberManager, ItemLayout, layout_item_fiber, paint_item_fibers, prepaint_item_fiber,
};
use crate::render_node::{
    CallbackSlot, LayoutCtx, LayoutFrame, PaintCtx, PaintFrame, PrepaintCtx, PrepaintFrame,
    RenderNode, UpdateResult,
};
use crate::taffy::ToTaffy;
    AnyElement, App, AvailableSpace, Bounds, ContentMask, DispatchPhase, Display, Edges, Element,
    EntityId, FocusHandle, GlobalElementId, Hitbox, HitboxBehavior, InspectorElementId,
    IntoElement, Overflow, Pixels, Point, ScrollDelta, ScrollWheelEvent, Size, Style,
    StyleRefinement, Styled, Window, point, px, size,
    item_fibers: ItemFiberManager,
    _max_item_width: Pixels,
/// Retained render node for List elements.
///
/// ListNode owns all List-specific state and implements the scope-based
/// prepaint/paint lifecycle. The render callback is deposited each frame
/// via CallbackSlot.
pub(crate) struct ListNode {
    /// The shared list state (also owned by user code).
    pub state: ListState,
    /// Render callback deposited by the element each frame.
    pub render_item: CallbackSlot<RenderItemFn>,
    /// Styling configuration.
    pub style: StyleRefinement,
    /// Sizing behavior.
    pub sizing_behavior: ListSizingBehavior,
    /// Cached item layouts for paint phase.
    cached_item_layouts: VecDeque<ItemLayout>,
    /// Cached content mask for paint phase.
    cached_content_mask: Option<ContentMask<Pixels>>,
    /// Cached scroll top for paint phase.
    cached_scroll_top: ListOffset,
    /// Cached hitbox for paint phase.
    cached_hitbox: Option<Hitbox>,
}

impl ListNode {
    /// Create a new ListNode.
    pub fn new(
        state: ListState,
        style: StyleRefinement,
        sizing_behavior: ListSizingBehavior,
    ) -> Self {
        Self {
            state,
            render_item: CallbackSlot::new(),
            style,
            sizing_behavior,
            cached_item_layouts: VecDeque::new(),
            cached_content_mask: None,
            cached_scroll_top: ListOffset::default(),
            cached_hitbox: None,
        }
    }
impl RenderNode for ListNode {
    fn taffy_style(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Style {
        let mut style = Style::default();
        style.overflow.y = Overflow::Scroll;
        style.refine(&self.style);
        style.to_taffy(rem_size, scale_factor)
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

    fn layout_begin(&mut self, ctx: &mut LayoutCtx) -> LayoutFrame {
        let mut frame = LayoutFrame {
            handled: true,
            ..Default::default()
        };

        // Push text style refinement for child text measurement
        let mut style = Style::default();
        style.refine(&self.style);
        if let Some(text_style) = style.text_style() {
            ctx.window.text_style_stack.push(text_style.clone());
            frame.pushed_text_style = true;
        }

        frame
    }

    fn layout_end(&mut self, ctx: &mut LayoutCtx, frame: LayoutFrame) {
        if frame.pushed_text_style {
            ctx.window.text_style_stack.pop();
        }
    }

    fn prepaint_begin(&mut self, ctx: &mut PrepaintCtx) -> PrepaintFrame {
        use crate::window::context::PrepaintCx;

        let mut frame = PrepaintFrame {
            handled: true,
            skip_children: true, // We manage items ourselves
            ..Default::default()
        };

        // Clear cached state from previous frame
        self.cached_item_layouts.clear();
        self.cached_content_mask = None;
        self.cached_scroll_top = ListOffset::default();
        self.cached_hitbox = None;

        // Compute style
        let mut style = Style::default();
        style.overflow.y = Overflow::Scroll;
        style.refine(&self.style);

        // Skip if display: none
        if style.display == Display::None {
            return frame;
        }

        // Create hitbox
        let hitbox =
            ctx.window
                .insert_hitbox_with_fiber(ctx.bounds, HitboxBehavior::Normal, ctx.fiber_id);
        frame.hitbox = Some(hitbox.clone());
        self.cached_hitbox = Some(hitbox);

        // Push text style
        if let Some(text_style) = style.text_style() {
            ctx.window.text_style_stack.push(text_style.clone());
            frame.pushed_text_style = true;
        }

        // Push content mask for scrolling
        if let Some(mask) = style.overflow_mask(ctx.bounds, ctx.window.rem_size()) {
            let world_mask = ctx.window.transform_mask_to_world(mask);
            let intersected = world_mask.intersect(&PrepaintCx::new(ctx.window).content_mask());
            ctx.window.content_mask_stack.push(intersected);
            frame.pushed_content_mask = true;
        }

        // Borrow state and perform prepaint
        let state = &mut *self.state.0.borrow_mut();
        state.reset = false;

        // If the width of the list has changed, invalidate all cached item heights
        if state
            .last_layout_bounds
            .is_none_or(|last_bounds| last_bounds.size.width != ctx.bounds.size.width)
        {
            let new_items = SumTree::from_iter(
                state.items.iter().map(|item| ListItem::Unmeasured {
                    focus_handle: item.focus_handle(),
                }),
                (),
            );
            state.items = new_items;
        }

        let padding = style
            .padding
            .to_pixels(ctx.bounds.size.into(), ctx.window.rem_size());

        // Run the prepaint logic with the render callback
        let layout = self.render_item.with_mut(|render_item| {
            match state.prepaint_items(ctx.bounds, padding, true, render_item, ctx.window, ctx.cx) {
                Ok(layout) => layout,
                Err(autoscroll_request) => {
                    state.logical_scroll_top = Some(autoscroll_request);
                    state
                        .prepaint_items(ctx.bounds, padding, false, render_item, ctx.window, ctx.cx)
                        .unwrap()
                }
            }
        });

        if let Some(layout) = layout {
            state.last_layout_bounds = Some(ctx.bounds);
            state.last_padding = Some(padding);
            self.cached_scroll_top = layout.scroll_top;
            self.cached_item_layouts = layout.item_layouts;
            self.cached_content_mask = Some(ContentMask { bounds: ctx.bounds });
        }

        frame
    }

    fn prepaint_end(&mut self, ctx: &mut PrepaintCtx, frame: PrepaintFrame) {
        if frame.pushed_content_mask {
            ctx.window.content_mask_stack.pop();
        }
        if frame.pushed_text_style {
            ctx.window.text_style_stack.pop();
        }
    }

    fn paint_begin(&mut self, ctx: &mut PaintCtx) -> PaintFrame {
        let mut frame = PaintFrame {
            handled: true,
            skip_children: true, // We manage items ourselves
            ..Default::default()
        };

        // Paint items
        if let Some(content_mask) = &self.cached_content_mask {
            let items = self.cached_item_layouts.make_contiguous();
            paint_item_fibers(items, content_mask.clone(), ctx.window, ctx.cx);
        }

        // Register scroll handler using cached hitbox from prepaint
        if let Some(hitbox) = &self.cached_hitbox {
            let list_state = self.state.clone();
            let height = ctx.bounds.size.height;
            let scroll_top = self.cached_scroll_top;
            let current_view = ctx.window.current_view();
            let hitbox_id = hitbox.id;

            let mut accumulated_scroll_delta = ScrollDelta::default();
            ctx.window
                .on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
                    if phase == DispatchPhase::Bubble
                        && window.hitbox_should_handle_scroll(hitbox_id)
                    {
                        accumulated_scroll_delta = accumulated_scroll_delta.coalesce(event.delta);
                        let pixel_delta = accumulated_scroll_delta.pixel_delta(px(20.));
                        list_state.0.borrow_mut().scroll(
                            &scroll_top,
                            height,
                            pixel_delta,
                            current_view,
                            window,
                            cx,
                        );
                    }
                });
        }

        frame
    }

    fn paint_end(&mut self, _ctx: &mut PaintCtx, _frame: PaintFrame) {
        // No stacks to pop
    }
            item_fibers: ItemFiberManager::new(),
            state.item_fibers.clear();

        // Update fiber ID mappings after splice
        state.item_fibers.splice(old_range, spliced_count);
        let items: Vec<ListItem> = self.items.iter().cloned().collect();
        for (ix, item) in items.into_iter().enumerate() {
            let size = match item.size() {
                Some(size) => size,
                None => {
                    let mut element = render_item(ix, window, cx);
                    let fiber_id = self.item_fibers.get_or_create(ix, window);
                    layout_item_fiber(fiber_id, &mut element, available_item_space, window, cx)
                }
            };
                let fiber_id = self.item_fibers.get_or_create(item_index, window);
                let element_size =
                    layout_item_fiber(fiber_id, &mut element, available_item_space, window, cx);
                        fiber_id,
                    let fiber_id = self.item_fibers.get_or_create(item_index, window);
                    let element_size =
                        layout_item_fiber(fiber_id, &mut element, available_item_space, window, cx);
                        fiber_id,
                    let item_index = cursor.start().0;
                    let mut element = render_item(item_index, window, cx);
                    let fiber_id = self.item_fibers.get_or_create(item_index, window);
                    layout_item_fiber(fiber_id, &mut element, available_item_space, window, cx)
            let focused_index = {
                let mut cursor = self
                    .items
                    .filter::<_, Count>((), |summary| summary.has_focus_handles);
                let mut found = None;
                while let Some(item) = cursor.item() {
                    if item.contains_focused(window, cx) {
                        found = Some(cursor.start().0);
                        break;
                    }
                    cursor.next();
                }
                found
            };

            if let Some(item_index) = focused_index {
                let mut element = render_item(item_index, window, cx);
                let fiber_id = self.item_fibers.get_or_create(item_index, window);
                let element_size =
                    layout_item_fiber(fiber_id, &mut element, available_item_space, window, cx);
                item_layouts.push_back(ItemLayout {
                    index: item_index,
                    fiber_id,
                    size: element_size,
                });
            _max_item_width: max_item_width,
                for item in &layout_response.item_layouts {
                    prepaint_item_fiber(
                        item.fiber_id,
                        item_origin,
                        ContentMask { bounds },
                        window,
                        cx,
                    );
                            let old_items = self.items.clone();
                            let mut cursor = old_items.cursor::<Count>(());
                                let item_index = cursor.start().0;
                                let item_size = cursor.item().and_then(|item| item.size());
                                let item_computed_size = match item_size {
                                    Some(size) => size,
                                    None => {
                                        let mut item_element = render_item(item_index, window, cx);
                                        let item_available_size = size(
                                            bounds.size.width.into(),
                                            AvailableSpace::MinContent,
                                        );
                                        let fiber_id =
                                            self.item_fibers.get_or_create(item_index, window);
                                        layout_item_fiber(
                                            fiber_id,
                                            &mut item_element,
                                            item_available_size,
                                            window,
                                            cx,
                                        )
                                    }
                                };
                                height -= item_computed_size.height;
    type PrepaintState = ();
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("List uses retained node path")
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        unreachable!("List uses retained node path")
        _bounds: Bounds<crate::Pixels>,
        _prepaint: &mut Self::PrepaintState,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("List uses retained node path")
    }
    fn create_render_node(&mut self) -> Option<Box<dyn RenderNode>> {
        let mut node = ListNode::new(self.state.clone(), self.style.clone(), self.sizing_behavior);

        // Deposit the render callback into the node's CallbackSlot
        let render_item = std::mem::replace(
            &mut self.render_item,
            Box::new(|_, _, _| crate::Empty.into_any_element()),
        );
        node.render_item.deposit(render_item);

        Some(Box::new(node))
    }

    fn update_render_node(
        &mut self,
        node: &mut dyn RenderNode,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<UpdateResult> {
        let node = node.as_any_mut().downcast_mut::<ListNode>()?;

        // Deposit the render callback into the node's CallbackSlot
        let render_item = std::mem::replace(
            &mut self.render_item,
            Box::new(|_, _, _| crate::Empty.into_any_element()),
        );
        node.render_item.deposit(render_item);

        // Update configuration
        node.style = self.style.clone();
        node.sizing_behavior = self.sizing_behavior;

        // The ListState is shared, so no need to update it

        Some(UpdateResult::UNCHANGED)
        self as gpui, AppContext, Context, IntoElement, ListState, Render, Styled, TestAppContext,
        Window, div, list, point, px, size,
                    div().h(px(10.)).w_full().into_any_element()
                    div().h(px(20.)).w_full().into_any_element()
                    div().h(px(height as f32)).w_full().into_any_element()