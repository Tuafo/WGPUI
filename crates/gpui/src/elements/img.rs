    AnyElement, AnyImageCache, App, Asset, AssetLogger, AvailableSpace, Bounds, DefiniteLength,
    Display, Element, ElementId, Entity, GlobalElementId, Image, ImageCache, InspectorElementId,
    InteractiveElement, Interactivity, IntoElement, LayoutId, Length, ObjectFit, Pixels,
    RenderImage, Resource, SharedString, SharedUri, Size, Style,
    StyleRefinement, Styled, Task, UpdateResult, VKey, Window, px, taffy::ToTaffy,
use refineable::Refineable;
    hash::{Hash, Hasher},
use collections::FxHasher;
use super::{
    Stateful, StatefulInteractiveElement,
    div::StatefulInner,
};
        let StatefulInner::Element(element) = &mut self.inner;
        &mut element.style

    pub(crate) fn take_interactivity(&mut self) -> Interactivity {
        std::mem::take(&mut self.interactivity)
    }
        let StatefulInner::Element(element) = &self.inner;
        element
        let StatefulInner::Element(element) = &mut self.inner;
        element
    type RequestLayoutState = ();
    type PrepaintState = ();
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("Img uses retained node path")
    }
    fn prepaint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        unreachable!("Img uses retained node path")
    }
    fn paint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _layout_state: &mut Self::RequestLayoutState,
        _hitbox: &mut Self::PrepaintState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        unreachable!("Img uses retained node path")
    }
    fn fiber_key(&self) -> VKey {
        VKey::None
    }
    fn cached_style(&self) -> Option<&StyleRefinement> {
        Some(&self.interactivity.base_style)
    }
    fn create_render_node(&mut self) -> Option<Box<dyn crate::RenderNode>> {
        let loading_factory = self.style.loading.take().map(Arc::from);
        let fallback_factory = self.style.fallback.take().map(Arc::from);
        Some(Box::new(ImgNode::new(
            self.take_interactivity(),
            self.source.clone(),
            self.style.grayscale,
            self.style.object_fit,
            self.image_cache.take(),
            loading_factory,
            fallback_factory,
        )))
    fn update_render_node(
        node: &mut dyn crate::RenderNode,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<UpdateResult> {
        if let Some(img_node) = node.as_any_mut().downcast_mut::<ImgNode>() {
            let interactivity = self.take_interactivity();
            let update_result = img_node.interactivity.diff_styles(&interactivity);

            let source = self.source.clone();
            let grayscale = self.style.grayscale;
            let object_fit = self.style.object_fit;
            let image_cache = self.image_cache.take();
            let loading_factory = self.style.loading.take().map(Arc::from);
            let fallback_factory = self.style.fallback.take().map(Arc::from);

            let image_cache_changed = match (&img_node.image_cache, &image_cache) {
                (Some(a), Some(b)) => !a.identity_eq(b),
                (None, None) => false,
                _ => true,
            };
            let content_changed = !img_node.source.identity_eq(&source)
                || img_node.grayscale != grayscale
                || img_node.object_fit != object_fit
                || image_cache_changed
                || img_node.loading_factory.is_some() != loading_factory.is_some()
                || img_node.fallback_factory.is_some() != fallback_factory.is_some();

            let mut layout_changed = update_result.layout_changed;
            let mut paint_changed = update_result.paint_changed;
            if content_changed {
                layout_changed = true;
                paint_changed = true;
            }
            if layout_changed {
                paint_changed = true;
            }
            img_node.update_from(
                interactivity,
                source,
                grayscale,
                object_fit,
                image_cache,
                loading_factory,
                fallback_factory,
            );
            Some(UpdateResult {
                layout_changed,
                paint_changed,
            })
        } else {
            None
        }
/// Retained render node for Img elements.
///
/// This node owns all image-specific data and state, enabling fully
/// node-handled layout, prepaint, and paint phases.
pub(crate) struct ImgNode {
    /// Interactivity state for this image element.
    pub interactivity: Interactivity,
    /// The image source.
    pub source: ImageSource,
    /// Whether to render in grayscale.
    pub grayscale: bool,
    /// How to fit the image within bounds.
    pub object_fit: ObjectFit,
    /// Optional image cache override.
    pub image_cache: Option<AnyImageCache>,
    /// Factory function to create loading indicator element.
    pub loading_factory: Option<Arc<dyn Fn() -> AnyElement>>,
    /// Factory function to create fallback element on error.
    pub fallback_factory: Option<Arc<dyn Fn() -> AnyElement>>,

    // --- Persistent state (retained across frames) ---
    /// Current animation frame index.
    frame_index: usize,
    /// Timestamp of last frame change for animation.
    last_frame_time: Option<Instant>,
    /// Loading state: (start time, notification task).
    started_loading: Option<(Instant, Task<()>)>,
    pending_paint_style: Option<Style>,

    // --- Layout cache (computed in layout_begin) ---
    /// Cached taffy style computed from interactivity.base_style.
    cached_taffy_style: taffy::style::Style,
    /// Cached image data result for current frame.
    cached_image_data: Option<Result<Arc<RenderImage>, ImageCacheError>>,
    /// Image cache to use (either explicit or inherited).
    effective_image_cache: Option<AnyImageCache>,
}

impl ImgNode {
    /// Create a new ImgNode from descriptor data.
    pub fn new(
        interactivity: Interactivity,
        source: ImageSource,
        grayscale: bool,
        object_fit: ObjectFit,
        image_cache: Option<AnyImageCache>,
        loading_factory: Option<Arc<dyn Fn() -> AnyElement>>,
        fallback_factory: Option<Arc<dyn Fn() -> AnyElement>>,
    ) -> Self {
        Self {
            interactivity,
            source,
            grayscale,
            object_fit,
            image_cache,
            loading_factory,
            fallback_factory,
            frame_index: 0,
            last_frame_time: None,
            started_loading: None,
            pending_paint_style: None,
            cached_taffy_style: taffy::style::Style::default(),
            cached_image_data: None,
            effective_image_cache: None,
        }
    }

    /// Update this node from a descriptor.
    pub fn update_from(
        &mut self,
        interactivity: Interactivity,
        source: ImageSource,
        grayscale: bool,
        object_fit: ObjectFit,
        image_cache: Option<AnyImageCache>,
        loading_factory: Option<Arc<dyn Fn() -> AnyElement>>,
        fallback_factory: Option<Arc<dyn Fn() -> AnyElement>>,
    ) {
        self.interactivity = interactivity;
        self.source = source;
        self.grayscale = grayscale;
        self.object_fit = object_fit;
        self.image_cache = image_cache;
        self.loading_factory = loading_factory;
        self.fallback_factory = fallback_factory;
    }
}

impl crate::RenderNode for ImgNode {
    fn needs_child_bounds(&self) -> bool {
        false
    }

    fn layout_begin(&mut self, ctx: &mut crate::LayoutCtx) -> crate::LayoutFrame {
        let mut frame = crate::LayoutFrame {
            handled: true,
            ..Default::default()
        };

        // Determine effective image cache (explicit or inherited from stack)
        self.effective_image_cache = self
            .image_cache
            .clone()
            .or_else(|| ctx.window.image_cache_stack.last().cloned());

        // Fetch/update image data and handle animation state
        let image_result =
            self.source
                .use_data(self.effective_image_cache.clone(), ctx.window, ctx.cx);

        match &image_result {
            Some(Ok(data)) => {
                // Image loaded successfully - handle animation
                let frame_count = data.frame_count();
                if frame_count > 1 {
                    let current_time = Instant::now();
                    if let Some(last_frame_time) = self.last_frame_time {
                        let elapsed = current_time - last_frame_time;
                        let frame_duration = Duration::from(data.delay(self.frame_index));

                        if elapsed >= frame_duration {
                            self.frame_index = (self.frame_index + 1) % frame_count;
                            self.last_frame_time = Some(current_time - (elapsed - frame_duration));
                        }
                    } else {
                        self.last_frame_time = Some(current_time);
                    }
                    ctx.window.request_animation_frame();
                }
                self.started_loading = None;
            }
            Some(Err(_)) => {
                self.started_loading = None;
            }
            None => {
                if self.started_loading.is_none() {
                    // Start the loading delay timer
                    let current_view = ctx.window.current_view();
                    let task = ctx.window.spawn(ctx.cx, async move |cx| {
                        cx.background_executor().timer(LOADING_DELAY).await;
                        cx.update(move |_, cx| {
                            cx.notify(current_view);
                        })
                        .log_err();
                    });
                    self.started_loading = Some((Instant::now(), task));
                }
            }
        }

        self.cached_image_data = image_result;

        // Compute taffy style from interactivity.base_style
        let mut style = Style::default();
        style.refine(&self.interactivity.base_style);

        if let Some(text_style) = style.text_style() {
            ctx.window.text_style_stack.push(text_style.clone());
            frame.pushed_text_style = true;
        }

        // If image loaded, adjust style for aspect ratio and intrinsic sizing
        if let Some(Ok(ref data)) = self.cached_image_data {
            let image_size = data.render_size(self.frame_index);
            style.aspect_ratio = Some(image_size.width / image_size.height);

            if let Length::Auto = style.size.width {
                style.size.width = match style.size.height {
                    Length::Definite(DefiniteLength::Absolute(abs_length)) => {
                        let height_px = abs_length.to_pixels(ctx.rem_size);
                        Length::Definite(
                            px(image_size.width.0 * height_px.0 / image_size.height.0).into(),
                        )
                    }
                    _ => Length::Definite(image_size.width.into()),
                };
            }

            if let Length::Auto = style.size.height {
                style.size.height = match style.size.width {
                    Length::Definite(DefiniteLength::Absolute(abs_length)) => {
                        let width_px = abs_length.to_pixels(ctx.rem_size);
                        Length::Definite(
                            px(image_size.height.0 * width_px.0 / image_size.width.0).into(),
                        )
                    }
                    _ => Length::Definite(image_size.height.into()),
                };
            }
        }

        self.cached_taffy_style = style.to_taffy(ctx.rem_size, ctx.scale_factor);

        frame
    }

    fn layout_end(&mut self, ctx: &mut crate::LayoutCtx, frame: crate::LayoutFrame) {
        if frame.pushed_text_style {
            ctx.window.text_style_stack.pop();
        }
    }

    fn taffy_style(&self, _rem_size: crate::Pixels, _scale_factor: f32) -> taffy::style::Style {
        // Return the pre-computed style from layout_begin
        self.cached_taffy_style.clone()
    }

    fn compute_intrinsic_size(
        &mut self,
        _ctx: &mut crate::SizingCtx,
    ) -> crate::IntrinsicSizeResult {
        let mut hasher = FxHasher::default();

        match &self.source {
            ImageSource::Resource(resource) => {
                0u8.hash(&mut hasher);
                resource.hash(&mut hasher);
            }
            ImageSource::Render(image) => {
                1u8.hash(&mut hasher);
                (Arc::as_ptr(image) as usize).hash(&mut hasher);
            }
            ImageSource::Image(image) => {
                2u8.hash(&mut hasher);
                (Arc::as_ptr(image) as usize).hash(&mut hasher);
            }
            ImageSource::Custom(loader) => {
                3u8.hash(&mut hasher);
                (Arc::as_ptr(loader) as *const () as usize).hash(&mut hasher);
            }
        }

        self.frame_index.hash(&mut hasher);
        if let Some(Ok(ref data)) = self.cached_image_data {
            let image_size = data.render_size(self.frame_index);
            image_size.width.0.to_bits().hash(&mut hasher);
            image_size.height.0.to_bits().hash(&mut hasher);
        }

        let input = crate::SizingInput::new(hasher.finish(), 0);

        let size = if let Some(Ok(ref data)) = self.cached_image_data {
            let image_size = data.render_size(self.frame_index);
            crate::IntrinsicSize {
                min_content: image_size,
                max_content: image_size,
            }
        } else {
            crate::IntrinsicSize::default()
        };

        crate::IntrinsicSizeResult { size, input }
    }

    fn resolve_size_query(
        &mut self,
        query: crate::SizeQuery,
        cached: &crate::IntrinsicSize,
        _ctx: &mut crate::SizingCtx,
    ) -> Size<Pixels> {
        match query {
            crate::SizeQuery::MinContent => cached.min_content,
            crate::SizeQuery::MaxContent => cached.max_content,
            crate::SizeQuery::ForWidth(width) => Size {
                width,
                height: cached.max_content.height,
            },
            crate::SizeQuery::ForHeight(height) => Size {
                width: cached.max_content.width,
                height,
            },
            crate::SizeQuery::Definite(size) => size,
        }
    }

    fn measure(
        &mut self,
        known: Size<Option<Pixels>>,
        _available: Size<AvailableSpace>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<Size<Pixels>> {
        // If we have image data, use its intrinsic size
        if let Some(Ok(ref data)) = self.cached_image_data {
            let image_size = data.render_size(self.frame_index);
            Some(Size {
                width: known.width.unwrap_or(image_size.width),
                height: known.height.unwrap_or(image_size.height),
            })
        } else {
            // No image data - let Taffy use style-defined size
            None
        }
    }

    fn prepaint_begin(&mut self, ctx: &mut crate::PrepaintCtx) -> crate::PrepaintFrame {
        use crate::window::context::PrepaintCx;

        let mut frame = crate::PrepaintFrame {
            handled: true,
            ..Default::default()
        };

        let prepaint = self.interactivity.prepare_prepaint(
            ctx.fiber_id,
            ctx.inspector_id.as_ref(),
            ctx.bounds,
            ctx.bounds.size,
            ctx.window,
            ctx.cx,
        );

        if prepaint.style.display == Display::None {
            frame.skip_children = true;
            return frame;
        }

        let has_children = !ctx.window.fiber.tree.children_slice(&ctx.fiber_id).is_empty();
        if has_children {
            if let Some(text_style) = prepaint.style.text_style() {
                ctx.window.text_style_stack.push(text_style.clone());
                frame.pushed_text_style = true;
            }

            let child_mask = crate::ContentMask { bounds: ctx.bounds };
            let world_mask = ctx.window.transform_mask_to_world(child_mask);
            let intersected = world_mask.intersect(&PrepaintCx::new(ctx.window).content_mask());
            ctx.window.content_mask_stack.push(intersected);
            frame.pushed_content_mask = true;
        }

        frame
    }

    fn prepaint_end(&mut self, ctx: &mut crate::PrepaintCtx, frame: crate::PrepaintFrame) {
        if frame.pushed_content_mask {
            ctx.window.content_mask_stack.pop();
        }
        if frame.pushed_text_style {
            ctx.window.text_style_stack.pop();
        }
    }

    fn paint_begin(&mut self, ctx: &mut crate::PaintCtx) -> crate::PaintFrame {
        use crate::window::context::PaintCx;

        let mut frame = crate::PaintFrame {
            handled: true,
            ..Default::default()
        };
        self.pending_paint_style = None;

        let hitbox = ctx.window.resolve_hitbox(&ctx.fiber_id);

        let Some(paint) = self.interactivity.prepare_paint(
            ctx.fiber_id,
            ctx.bounds,
            hitbox.as_ref(),
            ctx.window,
            ctx.cx,
        ) else {
            return frame;
        };
        let crate::elements::div::InteractivityPaint { style, tab_group } = paint;

        if style.display == Display::None {
            frame.skip_children = true;
            return frame;
        }

        if let Some(opacity) = style.opacity {
            frame.previous_opacity = Some(ctx.window.element_opacity);
            ctx.window.element_opacity *= opacity;
        }

        if let Some(text_style) = style.text_style() {
            ctx.window.text_style_stack.push(text_style.clone());
            frame.pushed_text_style = true;
        }

        if let Some(tab_group) = tab_group {
            ctx.window.begin_tab_group(tab_group);
            frame.pushed_tab_group = true;
        }

        style.paint_before_children(ctx.bounds, ctx.window, ctx.cx);

        if let Some(Ok(image)) = self.cached_image_data.clone() {
            let image_bounds = self
                .object_fit
                .get_bounds(ctx.bounds, image.size(self.frame_index));
            let corner_radii = style
                .corner_radii
                .to_pixels(ctx.window.rem_size())
                .clamp_radii_for_quad_size(image_bounds.size);
            ctx.window
                .paint_image(
                    image_bounds,
                    corner_radii,
                    image,
                    self.frame_index,
                    self.grayscale,
                )
                .log_err();
        }

        if let Some(hitbox) = hitbox.as_ref() {
            if let Some(drag) = ctx.cx.active_drag.as_ref() {
                if let Some(mouse_cursor) = drag.cursor_style {
                    ctx.window.set_window_cursor_style(mouse_cursor);
                }
            } else if let Some(mouse_cursor) = style.mouse_cursor {
                ctx.window.set_cursor_style(mouse_cursor, hitbox);
            }

            if let Some(group) = self.interactivity.group.clone() {
                crate::GroupHitboxes::push(group, hitbox.id, ctx.cx);
                frame.pushed_group_hitbox = self.interactivity.group.clone();
            }

            if let Some(area) = self.interactivity.window_control {
                ctx.window
                    .insert_window_control_hitbox(area, hitbox.clone());
            }

            #[cfg(any(feature = "inspector", debug_assertions))]
            ctx.window
                .insert_inspector_hitbox(hitbox.id, ctx.inspector_id.as_ref(), ctx.cx);
        }

        self.interactivity
            .paint_keyboard_listeners(ctx.window, ctx.cx);

        let has_children = !ctx.window.fiber.tree.children_slice(&ctx.fiber_id).is_empty();
        if has_children {
            let child_mask = crate::ContentMask { bounds: ctx.bounds };
            let world_mask = ctx.window.transform_mask_to_world(child_mask);
            let intersected = world_mask.intersect(&PaintCx::new(ctx.window).content_mask());
            ctx.window.content_mask_stack.push(intersected);
            frame.pushed_content_mask = true;
        }

        self.pending_paint_style = Some(style);

        frame
    }

    fn paint_end(&mut self, ctx: &mut crate::PaintCtx, frame: crate::PaintFrame) {
        if frame.pushed_content_mask {
            ctx.window.content_mask_stack.pop();
        }

        if let Some(style) = self.pending_paint_style.take() {
            style.paint_after_children(ctx.bounds, ctx.window, ctx.cx);
        }

        if let Some(group) = frame.pushed_group_hitbox.as_ref() {
            crate::GroupHitboxes::pop(group, ctx.cx);
        }
        if let Some(previous_opacity) = frame.previous_opacity {
            ctx.window.element_opacity = previous_opacity;
        }
        if frame.pushed_tab_group {
            ctx.window.end_tab_group();
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

    fn conditional_slots(
        &mut self,
        _fiber_id: GlobalElementId,
    ) -> SmallVec<[crate::ConditionalSlot; 4]> {
        let mut slots = SmallVec::new();

        match &self.cached_image_data {
            Some(Err(_)) => {
                if let Some(factory) = self.fallback_factory.clone() {
                    slots.push(crate::ConditionalSlot::active(VKey::Positional(1), move || {
                        let element = (factory)();
                        crate::div()
                            .absolute()
                            .inset_0()
                            .with_element_child(element)
                            .into_any_element()
                    }));
                }
            }
            None => {
                if let Some((started_loading, _task)) = self.started_loading.as_ref() {
                    if started_loading.elapsed() > LOADING_DELAY {
                        if let Some(factory) = self.loading_factory.clone() {
                            slots.push(crate::ConditionalSlot::active(VKey::Positional(0), move || {
                                let element = (factory)();
                                crate::div()
                                    .absolute()
                                    .inset_0()
                                    .with_element_child(element)
                                    .into_any_element()
                            }));
                        }
                    }
                }
            }
            Some(Ok(_)) => {}
        }

        slots
    }

    fn interactivity(&self) -> Option<&crate::Interactivity> {
        Some(&self.interactivity)
    }
}

    pub(crate) fn identity_eq(&self, other: &ImageSource) -> bool {
        match (self, other) {
            (ImageSource::Resource(a), ImageSource::Resource(b)) => a == b,
            (ImageSource::Render(a), ImageSource::Render(b)) => Arc::ptr_eq(a, b),
            (ImageSource::Image(a), ImageSource::Image(b)) => Arc::ptr_eq(a, b),
            (ImageSource::Custom(a), ImageSource::Custom(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
