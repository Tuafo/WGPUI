    AbsoluteLength, Action, AnyElement, AnyTooltip, AnyView, App, Bounds, ClickEvent,
    DispatchPhase, Display, Element, ElementId, Entity, FocusHandle, Global,
    GlobalElementId, Hitbox, HitboxBehavior, HitboxId, InspectorElementId, IntoElement, IsZero,
	    KeyContext, KeyDownEvent, KeyUpEvent, KeyboardButton, KeyboardClickEvent, LayoutId,
	    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseMoveEvent,
	    MousePressureEvent, MouseUpEvent, Overflow, ParentElement, Pixels, Point, Render,
	    ScrollWheelEvent, SharedString, Size, Style, StyleRefinement, Styled, Task, TooltipId, Length,
	    UpdateResult, VKey, Visibility, Window, WindowControlArea, point, px, size, taffy::ToTaffy,
	};
pub(crate) const DRAG_THRESHOLD: f64 = 2.;
#[derive(Clone)]
    pub(crate) fn diff_styles(&self, new: &Interactivity) -> UpdateResult {
        fn opt_layout_eq(a: Option<&StyleRefinement>, b: Option<&StyleRefinement>) -> bool {
            match (a, b) {
                (Some(a), Some(b)) => a.layout_eq(b),
                (None, None) => true,
                _ => false,
            }
        }

        fn opt_paint_eq(a: Option<&StyleRefinement>, b: Option<&StyleRefinement>) -> bool {
            match (a, b) {
                (Some(a), Some(b)) => a.paint_eq(b),
                (None, None) => true,
                _ => false,
            }
        }

        let layout_changed = !self.base_style.layout_eq(&new.base_style)
            || !opt_layout_eq(self.hover_style.as_deref(), new.hover_style.as_deref())
            || !opt_layout_eq(self.focus_style.as_deref(), new.focus_style.as_deref())
            || !opt_layout_eq(self.in_focus_style.as_deref(), new.in_focus_style.as_deref())
            || !opt_layout_eq(
                self.focus_visible_style.as_deref(),
                new.focus_visible_style.as_deref(),
            )
            || !opt_layout_eq(self.active_style.as_deref(), new.active_style.as_deref());

        let paint_changed = self.element_id != new.element_id
            || !self.base_style.paint_eq(&new.base_style)
            || !opt_paint_eq(self.hover_style.as_deref(), new.hover_style.as_deref())
            || !opt_paint_eq(self.focus_style.as_deref(), new.focus_style.as_deref())
            || !opt_paint_eq(self.in_focus_style.as_deref(), new.in_focus_style.as_deref())
            || !opt_paint_eq(
                self.focus_visible_style.as_deref(),
                new.focus_visible_style.as_deref(),
            )
            || !opt_paint_eq(self.active_style.as_deref(), new.active_style.as_deref());

        if layout_changed {
            UpdateResult::LAYOUT_CHANGED
        } else if paint_changed {
            UpdateResult::PAINT_ONLY
        } else {
            UpdateResult::UNCHANGED
        }
    }

            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            .push(Rc::new(move |event, phase, hitbox, window, cx| {
            Rc::new(move |action, phase, window, cx| {
            Rc::new(move |action, phase, window, cx| {
            Rc::new(move |_, phase, window, cx| {
            .push(Rc::new(move |event, phase, window, cx| {
            .push(Rc::new(move |event, phase, window, cx| {
            .push(Rc::new(move |event, phase, window, cx| {
            .push(Rc::new(move |event, phase, window, cx| {
            .push(Rc::new(move |event, window, cx| {
            Rc::new(move |dragged_value, window, cx| {
        self.can_drop_predicate = Some(Rc::new(predicate));
            Rc::new(move |value, offset, window, cx| {
        self.hover_listener = Some(Rc::new(listener));
        Stateful::from_element(self)
    Rc<dyn Fn(&MouseDownEvent, DispatchPhase, &Hitbox, &mut Window, &mut App) + 'static>;
    Rc<dyn Fn(&MouseUpEvent, DispatchPhase, &Hitbox, &mut Window, &mut App) + 'static>;
    Rc<dyn Fn(&MousePressureEvent, DispatchPhase, &Hitbox, &mut Window, &mut App) + 'static>;
    Rc<dyn Fn(&MouseMoveEvent, DispatchPhase, &Hitbox, &mut Window, &mut App) + 'static>;
    Rc<dyn Fn(&ScrollWheelEvent, DispatchPhase, &Hitbox, &mut Window, &mut App) + 'static>;
    Rc<dyn Fn(&dyn Any, Point<Pixels>, &mut Window, &mut App) -> AnyView + 'static>;

pub(crate) type DropListener = Rc<dyn Fn(&dyn Any, &mut Window, &mut App) + 'static>;
pub(crate) type CanDropPredicate =
    Rc<dyn Fn(&dyn Any, &mut Window, &mut App) -> bool + 'static>;
pub(crate) type HoverListener = Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>;
#[derive(Clone)]
    pub(crate) build: Rc<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>,
    pub(crate) hoverable: bool,
    Rc<dyn Fn(&KeyDownEvent, DispatchPhase, &mut Window, &mut App) + 'static>;
    Rc<dyn Fn(&KeyUpEvent, DispatchPhase, &mut Window, &mut App) + 'static>;
    Rc<dyn Fn(&ModifiersChangedEvent, &mut Window, &mut App) + 'static>;
    Rc<dyn Fn(&dyn Any, DispatchPhase, &mut Window, &mut App) + 'static>;

/// Frame state for a [`Div`] element during the legacy element pipeline.
/// In the fiber-based architecture, this is created but not used internally.
pub struct DivFrameState {
    /// Layout IDs for child elements.
    pub child_layout_ids: SmallVec<[LayoutId; 2]>,
}
    children: SmallVec<[AnyElement; 2]>,
    /// Create a new empty Div element.
    pub fn new() -> Self {
        Div {
            interactivity: Interactivity::new(),
            children: SmallVec::default(),
            prepaint_listener: None,
            image_cache: None,
        }
    }

    /// Create a Div from a style, used during element materialization.
    pub fn from_style(style: StyleRefinement, element_id: Option<ElementId>) -> Self {
        let mut div = Self::new();
        *div.style() = style;
        div.interactivity.element_id = element_id;
        div
    }

    /// Add a child element. Used during materialization.
    pub fn with_element_child(mut self, child: AnyElement) -> Self {
        self.children.push(child);
        self
    }

    pub(crate) fn take_interactivity(&mut self) -> Interactivity {
        mem::take(&mut self.interactivity)
    }

    pub(crate) fn take_prepaint_listener(
        &mut self,
    ) -> Option<Box<dyn Fn(Vec<Bounds<Pixels>>, &mut Window, &mut App) + 'static>> {
        self.prepaint_listener.take()
    }

    pub(crate) fn take_image_cache(&mut self) -> Option<Box<dyn ImageCacheProvider>> {
        self.image_cache.take()
    }
/// Interactivity state displayed and manipulated in the inspector.
        self.children.extend(elements)
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("Div uses retained node path")
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        unreachable!("Div uses retained node path")
    }
    fn paint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        unreachable!("Div uses retained node path")
    }
    fn fiber_key(&self) -> VKey {
        VKey::None
    }
    fn fiber_children(&self) -> &[AnyElement] {
        &self.children
    }
    fn fiber_children_mut(&mut self) -> &mut [AnyElement] {
        &mut self.children
    }
    fn cached_style(&self) -> Option<&StyleRefinement> {
        Some(&self.interactivity.base_style)
    }
    fn create_render_node(&mut self) -> Option<Box<dyn crate::RenderNode>> {
        Some(Box::new(DivNode::new(
            self.take_interactivity(),
            self.take_prepaint_listener(),
            self.take_image_cache(),
        )))
    fn update_render_node(
        node: &mut dyn crate::RenderNode,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<UpdateResult> {
        if let Some(div_node) = node.as_any_mut().downcast_mut::<DivNode>() {
            let interactivity = self.take_interactivity();
            let update_result = div_node.interactivity.diff_styles(&interactivity);
            div_node.update_from(
                interactivity,
                self.take_prepaint_listener(),
                self.take_image_cache(),
            );
            Some(update_result)
        } else {
            None
        }
    }
    fn requires_fiber_layout(&self) -> bool {
        true
    pub(crate) hover_listener: Option<HoverListener>,
pub(crate) struct InteractivityPrepaint {
    pub(crate) style: Style,
    pub(crate) scroll_offset: Point<Pixels>,
}

pub(crate) struct InteractivityPaint {
    pub(crate) style: Style,
    pub(crate) tab_group: Option<isize>,
}

    fn update_prepaint_state(
        &mut self,
        element_state: Option<&mut InteractiveElementState>,
        window: &mut Window,
    ) {
        let Some(element_state) = element_state else {
            return;
        };

        if let Some(clicked_state) = element_state.clicked_state.as_ref() {
            let clicked_state = clicked_state.borrow();
            self.active = Some(clicked_state.element);
        }
        if self.hover_style.is_some() || self.group_hover_style.is_some() {
            element_state
                .hover_state
                .get_or_insert_with(Default::default);
        }
        if let Some(active_tooltip) = element_state.active_tooltip.as_ref() {
            if self.tooltip_builder.is_some() {
                self.tooltip_id = set_tooltip_on_window(active_tooltip, window);
            } else {
                // If there is no longer a tooltip builder, remove the active tooltip.
                element_state.active_tooltip.take();
            }
        }
    }

    fn with_prepaint_state<R>(
        &mut self,
        global_id: GlobalElementId,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        content_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut App,
        f: impl FnOnce(
            &mut Self,
            Style,
            Option<&mut InteractiveElementState>,
            &mut Window,
            &mut App,
        ) -> R,
    ) -> R {
        self.content_size = content_size;

        #[cfg(any(feature = "inspector", debug_assertions))]
        window.with_inspector_state(
            _inspector_id,
            cx,
            |inspector_state: &mut Option<DivInspectorState>, _window| {
                if let Some(inspector_state) = inspector_state {
                    inspector_state.bounds = _bounds;
                    inspector_state.content_size = content_size;
                }
            },
        );

        if let Some(focus_handle) = self.tracked_focus_handle.as_ref() {
            window.set_focus_handle_for(global_id, focus_handle);
        }

        window.with_element_state::<InteractiveElementState, _>(
            &global_id,
            |element_state, window| {
                // Ensure element_state exists before we use it
                let mut element_state = Some(element_state.unwrap_or_default());

                // Initialize scroll_offset from element_state if needed (for fiber architecture).
                // In the legacy architecture this happens in request_layout, but DivNode uses
                // taffy_style() for layout, so we need to do it here during prepaint.
                if self.scroll_offset.is_none() {
                    if let Some(scroll_handle) = self.tracked_scroll_handle.as_ref() {
                        self.scroll_offset = Some(scroll_handle.0.borrow().offset.clone());
                    } else if self.base_style.overflow.x == Some(Overflow::Scroll)
                        || self.base_style.overflow.y == Some(Overflow::Scroll)
                    {
                        let es = element_state.as_mut().unwrap();
                        self.scroll_offset =
                            Some(es.scroll_offset.get_or_insert_with(Rc::default).clone());
                    }
                }

                let style = self.compute_style_internal(None, element_state.as_mut(), window, cx);
                self.update_prepaint_state(element_state.as_mut(), window);
                let result = f(self, style, element_state.as_mut(), window, cx);
                (result, element_state.unwrap())
            },
        )
    }

    pub(crate) fn prepare_prepaint(
        &mut self,
        global_id: GlobalElementId,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        content_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> InteractivityPrepaint {
        self.with_prepaint_state(
            global_id,
            inspector_id,
            bounds,
            content_size,
            window,
            cx,
            |this, style, _element_state, window, cx| {
                let mut hitbox = None;
                let content_mask = style.overflow_mask(bounds, window.rem_size());

                {
                    let mut prepaint_cx = crate::window::context::PrepaintCx::new(window);
                    prepaint_cx.with_content_mask(content_mask, |window| {
                        if this.should_insert_hitbox(&style, window, cx) {
                            hitbox = Some(window.insert_hitbox_with_fiber(
                                bounds,
                                this.hitbox_behavior,
                                global_id,
                            ));
                        }

                        if hitbox.is_none() {
                            window.clear_fiber_mouse_effects(&global_id);
                        }
                    });
                }

                let scroll_offset = this.clamp_scroll_position(bounds, &style, window, cx);

                InteractivityPrepaint {
                    style,
                    scroll_offset,
                }
            },
        )
    }

    fn build_paint_state(
        &mut self,
        global_id: GlobalElementId,
        _bounds: Bounds<Pixels>,
        hitbox: Option<&Hitbox>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InteractivityPaint> {
        self.hovered = hitbox.map(|hitbox| hitbox.is_hovered(window));

        window.with_element_state::<InteractiveElementState, _>(
            &global_id,
            |element_state, window| {
                let mut element_state = element_state;
                let style = self.compute_style_internal(hitbox, element_state.as_mut(), window, cx);

                #[cfg(any(feature = "test-support", test))]
                if let Some(debug_selector) = &self.debug_selector {
                    window
                        .next_frame
                        .debug_bounds
                        .insert(debug_selector.clone(), _bounds);
                }

                self.paint_hover_group_handler(window, cx);

                if style.visibility == Visibility::Hidden {
                    return (None, element_state.unwrap_or_default());
                }

                if let Some(hitbox) = hitbox {
                    self.paint_mouse_listeners(hitbox, element_state.as_mut(), window, cx);
                    self.paint_scroll_listener(hitbox, &style, window, cx);
                }

                let tab_group = if self.tab_group { self.tab_index } else { None };
                if let Some(focus_handle) = &self.tracked_focus_handle {
                    window.register_tab_stop_handle(focus_handle);
                }

                (
                    Some(InteractivityPaint { style, tab_group }),
                    element_state.unwrap_or_default(),
                )
            },
        )
    }

    pub(crate) fn prepare_paint(
        &mut self,
        global_id: GlobalElementId,
        bounds: Bounds<Pixels>,
        hitbox: Option<&Hitbox>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InteractivityPaint> {
        self.build_paint_state(global_id, bounds, hitbox, window, cx)
    }

        if (style.debug || style.debug_below || cx.has_global::<crate::DebugBelow>())
            let element_id = global_id
                .map(|id| format!("{id:?}"))
                .unwrap_or_else(|| "element".to_string());
                        let was_hovered = hitbox.is_hovered(window);
                        let fiber_id: GlobalElementId = hitbox.id.into();

                        window.on_modifiers_changed({
                            move |e: &crate::ModifiersChangedEvent, window, _cx| {
                            move |_: &MouseMoveEvent, phase, window, _cx| {
                                        window.invalidate_fiber_paint(fiber_id);
            window.text_style_stack.push(crate::TextStyleRefinement {
                color: Some(crate::red()),
                line_height: Some(FONT_SIZE.into()),
                background_color: Some(crate::white()),
                ..Default::default()
            });
            render_debug_text(window);
            window.text_style_stack.pop();
        _element_state: Option<&mut InteractiveElementState>,
        _cx: &mut App,
        let fiber_id: GlobalElementId = hitbox.id.into();
        if let Some(effects) = window.register_fiber_effects(&fiber_id) {
            effects.mouse_down_listeners = self.mouse_down_listeners.clone();
            effects.mouse_up_listeners = self.mouse_up_listeners.clone();
            effects.mouse_pressure_listeners = self.mouse_pressure_listeners.clone();
            effects.mouse_move_listeners = self.mouse_move_listeners.clone();
            effects.scroll_wheel_listeners = self.scroll_wheel_listeners.clone();
            effects.click_listeners = self.click_listeners.clone();
            effects.drag_listener = self.drag_listener.clone();
            effects.drop_listeners = self.drop_listeners.clone();
            effects.can_drop_predicate = self.can_drop_predicate.clone();
            effects.hover_listener = self.hover_listener.clone();
            effects.tooltip = self.tooltip_builder.clone();
        }
        window.update_active_mouse_listeners(&fiber_id);
        if is_focused && !self.click_listeners.is_empty() {
            window.on_key_event({
                let click_listeners = self.click_listeners.clone();
                let hitbox = hitbox.clone();
                move |event: &KeyUpEvent, phase, window, cx| {
                    if phase.bubble() && !window.default_prevented() {
                        let stroke = &event.keystroke;
                        let keyboard_button = if stroke.key.eq("enter") {
                            Some(KeyboardButton::Enter)
                        } else if stroke.key.eq("space") {
                            Some(KeyboardButton::Space)
                        } else {
                            None
                        };
                        if let Some(button) = keyboard_button
                            && !stroke.modifiers.modified()
                            let click_event = ClickEvent::Keyboard(KeyboardClickEvent {
                                button,
                                bounds: hitbox.bounds,
                            for listener in &click_listeners {
                                listener(&click_event, window, cx);
                }
            });
    pub(crate) fn paint_keyboard_listeners(&mut self, window: &mut Window, _cx: &mut App) {
        if let Some(key_context) = self.key_context.clone() {
            window.set_key_context(key_context);
        for listener in self.key_down_listeners.iter().cloned() {
                (listener)(event, phase, window, cx);
            });
        for listener in self.key_up_listeners.iter().cloned() {
                (listener)(event, phase, window, cx);
            });
        for listener in self.modifiers_changed_listeners.iter().cloned() {
            window.on_modifiers_changed(move |event, window, cx| {
                (listener)(event, window, cx);
            });
        for (action_type, listener) in self.action_listeners.iter().cloned() {
            window.on_action(action_type, move |action, phase, window, cx| {
                (listener)(action, phase, window, cx);
    fn paint_hover_group_handler(&mut self, _window: &mut Window, _cx: &mut App) {}

        fn round_to_two_decimals(pixels: Pixels) -> Pixels {
            const ROUNDING_FACTOR: f32 = 100.0;
            (pixels * ROUNDING_FACTOR).round() / ROUNDING_FACTOR
        }

            let content_size = self.content_size;
            let padding = style.padding;
            let fiber_id: GlobalElementId = hitbox.id.into();
                    let rem_size = window.rem_size();
                    let padding = padding.to_pixels(hitbox.bounds.size.into(), rem_size);
                    let padding_size = size(
                        padding.left + padding.right,
                        padding.top + padding.bottom,
                    );
                    let padded_content_size = content_size + padding_size;
                    let scroll_max = (padded_content_size - hitbox.bounds.size)
                        .map(round_to_two_decimals)
                        .max(&Default::default());
                    scroll_offset.x = scroll_offset.x.clamp(-scroll_max.width, px(0.));
                    scroll_offset.y = scroll_offset.y.clamp(-scroll_max.height, px(0.));

                    let applied_delta = *scroll_offset - old_scroll_offset;
                    if !applied_delta.is_zero() {
                        window.invalidate_fiber_scroll(fiber_id, *scroll_offset, cx);
        global_id: GlobalElementId,
        window.with_element_state::<InteractiveElementState, _>(
            &global_id,
            |element_state, window| {
                let mut element_state = element_state;
                let style = self.compute_style_internal(hitbox, element_state.as_mut(), window, cx);
                (style, element_state.unwrap_or_default())
            },
        )
    }

    pub(crate) fn compute_style_with_fiber(
        &self,
        global_id: GlobalElementId,
        hitbox: Option<&Hitbox>,
        window: &mut Window,
        cx: &mut App,
    ) -> Style {
        window.with_element_state::<InteractiveElementState, _>(
            &global_id,
            |element_state, window| {
                let mut element_state = element_state;
                let style = self.compute_style_internal(hitbox, element_state.as_mut(), window, cx);
                (style, element_state.unwrap_or_default())
            },
        )
                        window.hitbox_is_hovered(group_hitbox_id)
                            && window.hitbox_is_hovered(group_hitbox_id)
    pub(crate) fn is_clicked(&self) -> bool {
pub(crate) fn handle_tooltip_mouse_move(
    pub(crate) inner: StatefulInner<E>,
}

pub(crate) enum StatefulInner<E> {
    Element(E),
}

impl<E> Stateful<E> {
    fn from_element(element: E) -> Self {
        Self {
            inner: StatefulInner::Element(element),
        }
    }
}

#[doc(hidden)]
pub enum StatefulLayoutState<E: Element> {
    Element(E::RequestLayoutState),
}

#[doc(hidden)]
pub enum StatefulPrepaintState<E: Element> {
    Element(E::PrepaintState),
        let StatefulInner::Element(element) = &mut self.inner;
        element.style()
        let StatefulInner::Element(element) = &mut self.inner;
        element.interactivity()
    type RequestLayoutState = StatefulLayoutState<E>;
    type PrepaintState = StatefulPrepaintState<E>;
        let StatefulInner::Element(element) = &self.inner;
        element.id()
        let StatefulInner::Element(element) = &self.inner;
        element.source_location()
        let StatefulInner::Element(element) = &mut self.inner;
        let (layout_id, state) = element.request_layout(id, inspector_id, window, cx);
        (layout_id, StatefulLayoutState::Element(state))
    ) -> Self::PrepaintState {
        let StatefulInner::Element(element) = &mut self.inner;
        let StatefulLayoutState::Element(layout_state) = state;
        StatefulPrepaintState::Element(element.prepaint(
            id,
            inspector_id,
            bounds,
            layout_state,
            window,
            cx,
        ))
        let StatefulInner::Element(element) = &mut self.inner;
        let StatefulLayoutState::Element(layout_state) = request_layout;
        let StatefulPrepaintState::Element(prepaint_state) = prepaint;
        element.paint(
            layout_state,
            prepaint_state,

    fn fiber_key(&self) -> VKey {
        let StatefulInner::Element(element) = &self.inner;
        element.fiber_key()
    }

    fn fiber_children(&self) -> &[AnyElement] {
        let StatefulInner::Element(element) = &self.inner;
        element.fiber_children()
    }

    fn fiber_children_mut(&mut self) -> &mut [AnyElement] {
        let StatefulInner::Element(element) = &mut self.inner;
        element.fiber_children_mut()
    }

    fn cached_style(&self) -> Option<&StyleRefinement> {
        let StatefulInner::Element(element) = &self.inner;
        element.cached_style()
    }

    fn as_any_view(&self) -> Option<AnyView> {
        let StatefulInner::Element(element) = &self.inner;
        element.as_any_view()
    }

    fn create_render_node(&mut self) -> Option<Box<dyn crate::RenderNode>> {
        let StatefulInner::Element(element) = &mut self.inner;
        element.create_render_node()
    }

    fn update_render_node(
        &mut self,
        node: &mut dyn crate::RenderNode,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<UpdateResult> {
        let StatefulInner::Element(element) = &mut self.inner;
        element.update_render_node(node, window, cx)
    }

    fn requires_fiber_layout(&self) -> bool {
        let StatefulInner::Element(element) = &self.inner;
        element.requires_fiber_layout()
    }
    E: Element + IntoElement,
        let StatefulInner::Element(element) = &mut self.inner;
        element.extend(elements)

    pub(crate) fn set_last_origin(&self, origin: Point<Pixels>) {
        *self.last_origin.borrow_mut() = origin;
    }
    /// Get the scroll offset Rc for sharing with other components.
    pub(crate) fn offset_rc(&self) -> Rc<RefCell<Point<Pixels>>> {
        self.0.borrow().offset.clone()
    }

    pub(crate) fn set_child_bounds(&self, child_bounds: Vec<Bounds<Pixels>>) {
        self.0.borrow_mut().child_bounds = child_bounds;
    }

    pub(crate) fn scroll_to_active_item(&self) {
/// Retained render node for Div elements.
///
/// DivNode owns all Div-specific state and implements the scope-based
/// prepaint/paint lifecycle. This enables fibers to be element-type agnostic
/// while still correctly handling text styles, content masks, offsets, etc.
pub(crate) struct DivNode {
    /// Interactivity state for this Div element.
    pub interactivity: Interactivity,
    /// Optional listener called when children are prepainted.
    pub prepaint_listener:
        Option<Box<dyn Fn(Vec<Bounds<Pixels>>, &mut Window, &mut App) + 'static>>,
    /// Optional image cache provider.
    pub image_cache: Option<Box<dyn ImageCacheProvider>>,
    /// Cached children bounds for prepaint_listener (stored between prepaint_begin and prepaint_end).
    children_bounds_for_listener: Option<Vec<Bounds<Pixels>>>,
    /// Style computed in `paint_begin`, consumed in `paint_end` for after-children painting (borders).
    pending_paint_style: Option<Style>,
    pushed_unculled_scene: bool,
}

impl DivNode {
    /// Create a new DivNode from descriptor data.
    pub fn new(
        interactivity: Interactivity,
        prepaint_listener: Option<
            Box<dyn Fn(Vec<Bounds<Pixels>>, &mut Window, &mut App) + 'static>,
        >,
        image_cache: Option<Box<dyn ImageCacheProvider>>,
    ) -> Self {
        Self {
            interactivity,
            prepaint_listener,
            image_cache,
            children_bounds_for_listener: None,
            pending_paint_style: None,
            pushed_unculled_scene: false,
        }
    }

    /// Update this node from a descriptor.
    pub fn update_from(
        &mut self,
        interactivity: Interactivity,
        prepaint_listener: Option<
            Box<dyn Fn(Vec<Bounds<Pixels>>, &mut Window, &mut App) + 'static>,
        >,
        image_cache: Option<Box<dyn ImageCacheProvider>>,
    ) {
        self.interactivity = interactivity;
        self.prepaint_listener = prepaint_listener;
        self.image_cache = image_cache;
        self.pending_paint_style = None;
        self.pushed_unculled_scene = false;
    }

    /// Compute content size from child bounds.
    fn compute_content_size(
        &self,
        bounds: Bounds<Pixels>,
        child_bounds: &[Bounds<Pixels>],
    ) -> Size<Pixels> {
        if child_bounds.is_empty() {
            bounds.size
        } else {
            let mut child_min = point(Pixels::MAX, Pixels::MAX);
            let mut child_max = Point::default();
            for cb in child_bounds {
                child_min = child_min.min(&cb.origin);
                child_max = child_max.max(&cb.bottom_right());
            }
            (child_max - child_min).into()
        }
    }
}

impl crate::RenderNode for DivNode {
    fn taffy_style(&self, rem_size: crate::Pixels, scale_factor: f32) -> taffy::style::Style {
        // Compute taffy style from interactivity.base_style
        let mut style = crate::Style::default();
        style.refine(&self.interactivity.base_style);
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

    fn is_layout_boundary(&self) -> bool {
        // A layout boundary is a subtree whose descendants cannot affect its outer size.
        //
        // Today we use a conservative set of rules:
        // - Scroll containers: scrolling content shouldn't force ancestor relayout.
        // - Fully-definite sized containers: fixed-size boxes isolate child size changes.
        let style = &*self.interactivity.base_style;
        let is_scroll_container = style.overflow.x == Some(Overflow::Scroll)
            || style.overflow.y == Some(Overflow::Scroll)
            || self.interactivity.tracked_scroll_handle.is_some();

        let has_definite_width = style
            .size
            .width
            .is_some_and(|width| matches!(width, Length::Definite(_)));
        let has_definite_height = style
            .size
            .height
            .is_some_and(|height| matches!(height, Length::Definite(_)));

        is_scroll_container || (has_definite_width && has_definite_height)
    }

    fn needs_child_bounds(&self) -> bool {
        // Only compute child bounds when this div uses them:
        // - scroll containers need them to compute content size / update ScrollHandle child bounds
        // - prepaint listeners need them
        self.prepaint_listener.is_some()
            || self.interactivity.tracked_scroll_handle.is_some()
            || self.interactivity.base_style.overflow.x == Some(Overflow::Scroll)
            || self.interactivity.base_style.overflow.y == Some(Overflow::Scroll)
    }

    fn layout_begin(&mut self, ctx: &mut crate::LayoutCtx) -> crate::LayoutFrame {
        let mut frame = crate::LayoutFrame {
            handled: true,
            ..Default::default()
        };

        // Push text style refinement for child text measurement
        // We need to compute the style first to get the text_style
        let mut style = crate::Style::default();
        style.refine(&self.interactivity.base_style);
        if let Some(text_style) = style.text_style() {
            ctx.window.text_style_stack.push(text_style.clone());
            frame.pushed_text_style = true;
        }

        // Push image cache for child image loading
        if let Some(image_cache_provider) = &mut self.image_cache {
            let image_cache = image_cache_provider.provide(ctx.window, ctx.cx);
            ctx.window.image_cache_stack.push(image_cache);
            frame.pushed_image_cache = true;
        }

        frame
    }

    fn layout_end(&mut self, ctx: &mut crate::LayoutCtx, frame: crate::LayoutFrame) {
        // Pop in reverse order of what was pushed in layout_begin
        if frame.pushed_image_cache {
            ctx.window.image_cache_stack.pop();
        }
        if frame.pushed_text_style {
            ctx.window.text_style_stack.pop();
        }
    }

    fn prepaint_begin(&mut self, ctx: &mut crate::PrepaintCtx) -> crate::PrepaintFrame {
        use crate::window::context::PrepaintCx;

        let mut frame = crate::PrepaintFrame {
            handled: true,
            ..Default::default()
        };

        // Set scroll_anchor last_origin
        if let Some(handle) = self.interactivity.scroll_anchor.as_ref() {
            handle
                .set_last_origin(ctx.bounds.origin - PrepaintCx::new(ctx.window).element_offset());
        }

        // Compute content_size from child bounds
        let content_size = self.compute_content_size(ctx.bounds, &ctx.child_bounds);

        // Update scroll handle with child bounds
        if let Some(scroll_handle) = self.interactivity.tracked_scroll_handle.as_ref() {
            scroll_handle.set_child_bounds(ctx.child_bounds.clone());
        }

        // Scroll to active item if needed
        if let Some(scroll_handle) = self.interactivity.tracked_scroll_handle.as_ref() {
            scroll_handle.scroll_to_active_item();
        }

        // Call prepare_prepaint to set up hitbox and compute style
        let prepaint = self.interactivity.prepare_prepaint(
            ctx.fiber_id,
            ctx.inspector_id.as_ref(),
            ctx.bounds,
            content_size,
            ctx.window,
            ctx.cx,
        );

        // Skip children if display: none
        if prepaint.style.display == Display::None {
            frame.skip_children = true;
            return frame;
        }

        // Push text style
        if let Some(text_style) = prepaint.style.text_style() {
            ctx.window.text_style_stack.push(text_style.clone());
            frame.pushed_text_style = true;
        }

        // Push content mask
        if let Some(mask) = prepaint
            .style
            .overflow_mask(ctx.bounds, ctx.window.rem_size())
        {
            let world_mask = ctx.window.transform_mask_to_world(mask);
            let intersected = world_mask.intersect(&PrepaintCx::new(ctx.window).content_mask());
            ctx.window.content_mask_stack.push(intersected);
            frame.pushed_content_mask = true;
        }

        // Enter the scroll transform context for children.
        if self.interactivity.scroll_offset.is_some() {
            let parent = ctx.window.transform_stack.current();
            let transform_id =
                ctx.window
                    .ensure_scroll_transform(ctx.fiber_id, parent, prepaint.scroll_offset);
            ctx.window.transform_stack.push_existing_transform(transform_id);
            frame.pushed_transform = true;
        }

        // Store children bounds for prepaint_listener if needed
        if self.prepaint_listener.is_some() {
            self.children_bounds_for_listener = Some(ctx.child_bounds.clone());
        }

        frame
    }

    fn prepaint_end(&mut self, ctx: &mut crate::PrepaintCtx, frame: crate::PrepaintFrame) {
        // Pop pushed context in reverse order
        if frame.pushed_transform {
            ctx.window.transform_stack.pop_transform();
        }
        if frame.pushed_content_mask {
            ctx.window.content_mask_stack.pop();
        }
        if frame.pushed_text_style {
            ctx.window.text_style_stack.pop();
        }

        // Call prepaint_listener with children bounds
        if let Some(listener) = self.prepaint_listener.as_ref() {
            if let Some(children_bounds) = self.children_bounds_for_listener.take() {
                listener(children_bounds, ctx.window, ctx.cx);
            }
        }
    }

    fn paint_begin(&mut self, ctx: &mut crate::PaintCtx) -> crate::PaintFrame {
        use crate::window::context::PaintCx;

        let mut frame = crate::PaintFrame {
            handled: true,
            ..Default::default()
        };
        self.pending_paint_style = None;
        self.pushed_unculled_scene = false;

        // Get hitbox from window
        let hitbox = ctx.window.resolve_hitbox(&ctx.fiber_id);

        // Call prepare_paint
        let Some(paint) = self.interactivity.prepare_paint(
            ctx.fiber_id,
            ctx.bounds,
            hitbox.as_ref(),
            ctx.window,
            ctx.cx,
        ) else {
            return frame;
        };
        let InteractivityPaint { style, tab_group } = paint;
        if style.display == Display::None {
            frame.skip_children = true;
            return frame;
        }

        // Push image cache
        if let Some(image_cache) = self
            .image_cache
            .as_mut()
            .map(|provider| provider.provide(ctx.window, ctx.cx))
        {
            ctx.window.image_cache_stack.push(image_cache);
            frame.pushed_image_cache = true;
        }

        // Apply opacity
        if let Some(opacity) = style.opacity {
            frame.previous_opacity = Some(ctx.window.element_opacity);
            ctx.window.element_opacity *= opacity;
        }

        // Push text style
        if let Some(text_style) = style.text_style() {
            ctx.window.text_style_stack.push(text_style.clone());
            frame.pushed_text_style = true;
        }

        // Push content mask
        if let Some(mask) = style.overflow_mask(ctx.bounds, ctx.window.rem_size()) {
            let world_mask = ctx.window.transform_mask_to_world(mask);
            let intersected = world_mask.intersect(&PaintCx::new(ctx.window).content_mask());
            ctx.window.content_mask_stack.push(intersected);
            frame.pushed_content_mask = true;
        }

        // Push tab group
        if let Some(tab_group) = tab_group {
            ctx.window.begin_tab_group(tab_group);
            frame.pushed_tab_group = true;
        }

        // Paint before children (background, shadows, etc.)
        style.paint_before_children(ctx.bounds, ctx.window, ctx.cx);

        // Debug info
        #[cfg(debug_assertions)]
        if let Some(hitbox) = hitbox.as_ref() {
            self.interactivity.paint_debug_info(
                Some(&ctx.fiber_id),
                hitbox,
                &style,
                ctx.window,
                ctx.cx,
            );
        }

        // Handle cursor style
        if let Some(hitbox) = hitbox.as_ref() {
            if let Some(drag) = ctx.cx.active_drag.as_ref() {
                if let Some(mouse_cursor) = drag.cursor_style {
                    ctx.window.set_window_cursor_style(mouse_cursor);
                }
            } else if let Some(mouse_cursor) = style.mouse_cursor {
                ctx.window.set_cursor_style(mouse_cursor, hitbox);
            }

            // Register group hitbox
            if let Some(group) = self.interactivity.group.clone() {
                crate::GroupHitboxes::push(group, hitbox.id, ctx.cx);
                frame.pushed_group_hitbox = self.interactivity.group.clone();
            }

            // Register window control
            if let Some(area) = self.interactivity.window_control {
                ctx.window
                    .insert_window_control_hitbox(area, hitbox.clone());
            }

            #[cfg(any(feature = "inspector", debug_assertions))]
            ctx.window
                .insert_inspector_hitbox(hitbox.id, ctx.inspector_id.as_ref(), ctx.cx);
        }

        // Register keyboard listeners
        self.interactivity
            .paint_keyboard_listeners(ctx.window, ctx.cx);

        if let Some(scroll_offset) = self.interactivity.scroll_offset.as_ref() {
            ctx.window.push_unculled_scene();
            self.pushed_unculled_scene = true;

            let scroll_offset = *scroll_offset.borrow();
            let parent = ctx.window.transform_stack.current();
            let transform_id =
                ctx.window
                    .ensure_scroll_transform(ctx.fiber_id, parent, scroll_offset);
            ctx.window.transform_stack.push_existing_transform(transform_id);
            frame.pushed_transform = true;
        }

        self.pending_paint_style = Some(style);
        frame
    }

    fn paint_end(&mut self, ctx: &mut crate::PaintCtx, frame: crate::PaintFrame) {
        if let Some(style) = self.pending_paint_style.take() {
            style.paint_after_children(ctx.bounds, ctx.window, ctx.cx);
        }

        // Pop pushed context in reverse order
        if let Some(group) = frame.pushed_group_hitbox.as_ref() {
            crate::GroupHitboxes::pop(group, ctx.cx);
        }
        if frame.pushed_transform {
            ctx.window.transform_stack.pop_transform();
        }
        if self.pushed_unculled_scene {
            ctx.window.pop_unculled_scene();
            self.pushed_unculled_scene = false;
        }
        if let Some(previous_opacity) = frame.previous_opacity {
            ctx.window.element_opacity = previous_opacity;
        }
        if frame.pushed_image_cache {
            ctx.window.image_cache_stack.pop();
        }
        if frame.pushed_tab_group {
            ctx.window.end_tab_group();
        }
        if frame.pushed_content_mask {
            ctx.window.content_mask_stack.pop();
        }
        if frame.pushed_text_style {
            ctx.window.text_style_stack.pop();
        }
    }

    fn needs_after_segment(&self) -> bool {
        self.pending_paint_style.as_ref().is_some_and(|style| {
            style
                .border_color
                .is_some_and(|color| !color.is_transparent())
                && style.border_widths.any(|length| !length.is_zero())
        })
    }

    fn interactivity(&self) -> Option<&Interactivity> {
        Some(&self.interactivity)
    }
}
