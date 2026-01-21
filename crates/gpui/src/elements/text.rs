    ActiveTooltip, AnyView, App, AvailableSpace, Bounds, DispatchPhase, Element,
    ElementId, GlobalElementId, HighlightStyle, Hitbox, HitboxBehavior, InspectorElementId,
    IntoElement, LayoutId, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point,
    SharedString, Size, TextOverflow, TextRun, TextStyle, TooltipId, TruncateFrom, UpdateResult,
    WhiteSpace, Window, WrappedLine, WrappedLineLayout, register_tooltip_mouse_handlers,
    set_tooltip_on_window,
    hash::{Hash, Hasher},
use collections::FxHasher;
    type RequestLayoutState = ();
        _global_id: Option<&GlobalElementId>,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("&'static str uses retained node path")
        _bounds: Bounds<Pixels>,
        _text_layout: &mut Self::RequestLayoutState,
        unreachable!("&'static str uses retained node path")
        _text_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("&'static str uses retained node path")
    }

    fn into_any(self) -> crate::AnyElement {
        crate::AnyElement::new(self)
    }

    fn create_render_node(&mut self) -> Option<Box<dyn crate::RenderNode>> {
        Some(Box::new(TextNode::new(SharedString::from(*self), None)))
    }

    fn update_render_node(
        &mut self,
        node: &mut dyn crate::RenderNode,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<UpdateResult> {
        if let Some(text_node) = node.as_any_mut().downcast_mut::<TextNode>() {
            let text_changed = text_node.text != *self;
            let runs_changed = text_node.runs.is_some();
            if text_changed || runs_changed {
                text_node.update_from(SharedString::from(*self), None);
                Some(UpdateResult::LAYOUT_CHANGED)
            } else {
                Some(UpdateResult::UNCHANGED)
            }
        } else {
            None
        }
    type RequestLayoutState = ();
        _global_id: Option<&GlobalElementId>,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("SharedString uses retained node path")
        _bounds: Bounds<Pixels>,
        _text_layout: &mut Self::RequestLayoutState,
        unreachable!("SharedString uses retained node path")
        _text_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
        unreachable!("SharedString uses retained node path")
    }

    fn into_any(self) -> crate::AnyElement {
        crate::AnyElement::new(self)
    }

    fn create_render_node(&mut self) -> Option<Box<dyn crate::RenderNode>> {
        Some(Box::new(TextNode::new(self.clone(), None)))
    }

    fn update_render_node(
        &mut self,
        node: &mut dyn crate::RenderNode,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<UpdateResult> {
        if let Some(text_node) = node.as_any_mut().downcast_mut::<TextNode>() {
            let text_changed = text_node.text != *self;
            let runs_changed = text_node.runs.is_some();
            if text_changed || runs_changed {
                text_node.update_from(self.clone(), None);
                Some(UpdateResult::LAYOUT_CHANGED)
            } else {
                Some(UpdateResult::UNCHANGED)
            }
        } else {
            None
        }
        _global_id: Option<&GlobalElementId>,

        let content_hash = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            text.hash(&mut hasher);
            font_size.0.to_bits().hash(&mut hasher);
            line_height.0.to_bits().hash(&mut hasher);
            hasher.finish()
        };

        window.request_measured_layout_cached(Default::default(), content_hash, {
        global_id: Option<&GlobalElementId>,
        self.text
            .request_layout(global_id, inspector_id, window, cx)
        let global_id = *global_id.unwrap();
        window.with_element_state::<InteractiveTextState, _>(
            &global_id,
                let mut interactive_state = interactive_state;
                    .prepaint(Some(&global_id), inspector_id, bounds, state, window, cx);
                let hitbox =
                    window.insert_hitbox_with_fiber(bounds, HitboxBehavior::Normal, global_id);
                (hitbox, interactive_state.unwrap_or_default())
        let global_id = *global_id.unwrap();
            &global_id,
                                window.invalidate_fiber_paint(global_id);
                self.text.paint(
                    Some(&global_id),
                    inspector_id,
                    bounds,
                    &mut (),
                    &mut (),
                    window,
                    cx,
                );

/// Retained render node for text elements.
///
/// This node owns all text-specific data including the text content,
/// optional style runs, and cached layout (shaped lines).
pub(crate) struct TextNode {
    /// The text content.
    text: SharedString,
    /// Optional styled runs.
    runs: Option<Vec<TextRun>>,
    /// Resolved text style captured during layout_begin.
    /// This is the inherited text style from parent elements.
    resolved_text_style: Option<TextStyle>,
    /// Cached intrinsic sizing (min/max content) keyed by `SizingInput`.
    intrinsic: Option<(crate::SizingInput, crate::IntrinsicSize)>,
    /// Cached layout data (populated during measure).
    layout: Option<TextNodeLayout>,
    /// Bounds set during prepaint.
    bounds: Option<Bounds<Pixels>>,
}

/// Cached layout data for TextNode.
struct TextNodeLayout {
    input: crate::SizingInput,
    lines: SmallVec<[WrappedLine; 1]>,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    truncate_width: Option<Pixels>,
    size: Size<Pixels>,
    text_style: TextStyle,
}

impl TextNode {
    /// Create a new TextNode with the given text.
    pub fn new(text: SharedString, runs: Option<Vec<TextRun>>) -> Self {
        Self {
            text,
            runs,
            resolved_text_style: None,
            intrinsic: None,
            layout: None,
            bounds: None,
        }
    }

    /// Update this node from a descriptor.
    pub fn update_from(&mut self, text: SharedString, runs: Option<Vec<TextRun>>) {
        // Clear cached layout since text content may have changed
        self.layout = None;
        self.intrinsic = None;
        self.text = text;
        self.runs = runs;
    }

    fn sizing_input(
        &self,
        text_style: &TextStyle,
        font_size: Pixels,
        line_height: Pixels,
        runs: &[TextRun],
    ) -> crate::SizingInput {
        let mut content_hasher = FxHasher::default();
        self.text.hash(&mut content_hasher);
        runs.len().hash(&mut content_hasher);
        for run in runs {
            run.len.hash(&mut content_hasher);
            run.font.hash(&mut content_hasher);
        }
        let content_hash = content_hasher.finish();

        let mut style_hasher = FxHasher::default();
        text_style.font().hash(&mut style_hasher);
        font_size.0.to_bits().hash(&mut style_hasher);
        line_height.0.to_bits().hash(&mut style_hasher);
        text_style.line_clamp.hash(&mut style_hasher);
        match text_style.white_space {
            WhiteSpace::Normal => 0u8.hash(&mut style_hasher),
            WhiteSpace::Nowrap => 1u8.hash(&mut style_hasher),
        }
        match &text_style.text_overflow {
            None => 0u8.hash(&mut style_hasher),
            Some(TextOverflow::Truncate(affix)) => {
                1u8.hash(&mut style_hasher);
                affix.hash(&mut style_hasher);
            }
            Some(TextOverflow::TruncateStart(affix)) => {
                2u8.hash(&mut style_hasher);
                affix.hash(&mut style_hasher);
            }
        }
        match text_style.text_align {
            crate::TextAlign::Left => 0u8.hash(&mut style_hasher),
            crate::TextAlign::Center => 1u8.hash(&mut style_hasher),
            crate::TextAlign::Right => 2u8.hash(&mut style_hasher),
        }
        let style_hash = style_hasher.finish();

        crate::SizingInput::new(content_hash, style_hash)
    }

    fn shape_and_store_layout(
        &mut self,
        input: crate::SizingInput,
        text_style: TextStyle,
        wrap_width: Option<Pixels>,
        truncate_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let line_height = text_style
            .line_height
            .to_pixels(font_size.into(), window.rem_size());

        let runs = self
            .runs
            .clone()
            .unwrap_or_else(|| vec![text_style.to_run(self.text.len())]);

        let (truncate_width, truncation_affix, truncate_from) =
            if let Some(text_overflow) = text_style.text_overflow.clone()
                && truncate_width.is_some()
            {
                match text_overflow {
                    TextOverflow::Truncate(s) => (truncate_width, s, TruncateFrom::End),
                    TextOverflow::TruncateStart(s) => (truncate_width, s, TruncateFrom::Start),
                }
            } else {
                (None, "".into(), TruncateFrom::End)
            };

        // Only use cached layout if:
        // - input matches (content/style),
        // - wrap_width matches, and
        // - truncation state matches.
        if let Some(layout) = &self.layout
            && layout.input == input
            && layout.wrap_width == wrap_width
            && layout.truncate_width == truncate_width
        {
            return layout.size;
        }

        let mut line_wrapper = cx.text_system().line_wrapper(text_style.font(), font_size);
        let (text, runs) = if let Some(truncate_width) = truncate_width {
            line_wrapper.truncate_line(
                self.text.clone(),
                truncate_width,
                &truncation_affix,
                &runs,
                truncate_from,
            )
        } else {
            (self.text.clone(), Cow::Borrowed(&*runs))
        };

        let Some(lines) = window
            .text_system()
            .shape_text(text, font_size, &runs, wrap_width, text_style.line_clamp)
            .log_err()
        else {
            let size = Size::default();
            self.layout = Some(TextNodeLayout {
                input,
                lines: Default::default(),
                line_height,
                wrap_width,
                truncate_width,
                size,
                text_style,
            });
            return size;
        };

        let mut size: Size<Pixels> = Size::default();
        for line in &lines {
            let line_size = line.size(line_height);
            size.height += line_size.height;
            size.width = size.width.max(line_size.width).ceil();
        }

        self.layout = Some(TextNodeLayout {
            input,
            lines,
            line_height,
            wrap_width,
            truncate_width,
            size,
            text_style,
        });

        size
    }
}

impl crate::RenderNode for TextNode {
    fn needs_child_bounds(&self) -> bool {
        false
    }

    fn taffy_style(&self, _rem_size: crate::Pixels, _scale_factor: f32) -> taffy::style::Style {
        taffy::style::Style::default()
    }

    fn layout_begin(&mut self, ctx: &mut crate::LayoutCtx) -> crate::LayoutFrame {
        // Capture the inherited text style from the window stack.
        // This is set up by parent Divs during their layout_begin.
        self.resolved_text_style = Some(ctx.window.text_style());
        crate::LayoutFrame {
            handled: true,
            ..Default::default()
        }
    }

    fn compute_intrinsic_size(
        &mut self,
        ctx: &mut crate::SizingCtx,
    ) -> crate::IntrinsicSizeResult {
        let text_style = ctx.window.text_style();
        self.resolved_text_style = Some(text_style.clone());

        let font_size = text_style.font_size.to_pixels(ctx.window.rem_size());
        let line_height = text_style
            .line_height
            .to_pixels(font_size.into(), ctx.window.rem_size());

        let runs = self
            .runs
            .clone()
            .unwrap_or_else(|| vec![text_style.to_run(self.text.len())]);

        let input = self.sizing_input(&text_style, font_size, line_height, &runs);
        if let Some((cached_input, cached_size)) = &self.intrinsic
            && *cached_input == input
        {
            return crate::IntrinsicSizeResult {
                size: cached_size.clone(),
                input,
            };
        }

        // Unwrapped (max-content) size; we currently treat min-content as equivalent to preserve
        // existing behavior. Height-for-width is handled in `resolve_size_query`.
        let size = self.shape_and_store_layout(
            input.clone(),
            text_style.clone(),
            None,
            None,
            ctx.window,
            ctx.cx,
        );

        let intrinsic = crate::IntrinsicSize {
            min_content: size,
            max_content: size,
        };
        self.intrinsic = Some((input.clone(), intrinsic.clone()));

        crate::IntrinsicSizeResult {
            size: intrinsic,
            input,
        }
    }

    fn resolve_size_query(
        &mut self,
        query: crate::SizeQuery,
        cached: &crate::IntrinsicSize,
        ctx: &mut crate::SizingCtx,
    ) -> Size<Pixels> {
        let Some(mut text_style) = self.resolved_text_style.clone().or_else(|| {
            // Best-effort fallback: this can happen when layout is invoked without a preceding
            // intrinsic sizing pass (e.g. legacy in-frame layouts).
            Some(ctx.window.text_style())
        }) else {
            return cached.max_content;
        };

        // For queries that provide a definite width, compute wrapped height for that width.
        match query {
            crate::SizeQuery::MinContent => cached.min_content,
            crate::SizeQuery::MaxContent => cached.max_content,
            crate::SizeQuery::ForHeight(height) => Size {
                width: cached.max_content.width,
                height,
            },
            crate::SizeQuery::ForWidth(width) => {
                let wrap_width = if text_style.white_space == WhiteSpace::Normal {
                    Some(width)
                } else {
                    None
                };

                let truncate_width = if text_style.text_overflow.is_some() {
                    match text_style.line_clamp {
                        Some(max_lines) => Some(width * max_lines),
                        None => Some(width),
                    }
                } else {
                    None
                };

                let font_size = text_style.font_size.to_pixels(ctx.window.rem_size());
                let line_height = text_style
                    .line_height
                    .to_pixels(font_size.into(), ctx.window.rem_size());
                let runs = self
                    .runs
                    .clone()
                    .unwrap_or_else(|| vec![text_style.to_run(self.text.len())]);
                let input = self.sizing_input(&text_style, font_size, line_height, &runs);

                self.shape_and_store_layout(
                    input,
                    text_style,
                    wrap_width,
                    truncate_width,
                    ctx.window,
                    ctx.cx,
                )
            }
            crate::SizeQuery::Definite(size) => {
                let wrap_width = if text_style.white_space == WhiteSpace::Normal {
                    Some(size.width)
                } else {
                    None
                };

                let truncate_width = if text_style.text_overflow.is_some() {
                    match text_style.line_clamp {
                        Some(max_lines) => Some(size.width * max_lines),
                        None => Some(size.width),
                    }
                } else {
                    None
                };

                let font_size = text_style.font_size.to_pixels(ctx.window.rem_size());
                let line_height = text_style
                    .line_height
                    .to_pixels(font_size.into(), ctx.window.rem_size());
                let runs = self
                    .runs
                    .clone()
                    .unwrap_or_else(|| vec![text_style.to_run(self.text.len())]);
                let input = self.sizing_input(&text_style, font_size, line_height, &runs);

                let measured = self.shape_and_store_layout(
                    input,
                    text_style,
                    wrap_width,
                    truncate_width,
                    ctx.window,
                    ctx.cx,
                );

                Size {
                    width: measured.width.min(size.width),
                    height: measured.height.min(size.height),
                }
            }
        }
    }

    fn measure(
        &mut self,
        known: Size<Option<Pixels>>,
        available: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Size<Pixels>> {
        let text_style = self
            .resolved_text_style
            .clone()
            .unwrap_or_else(|| window.text_style());

        let wrap_width = if text_style.white_space == WhiteSpace::Normal {
            known.width.or(match available.width {
                AvailableSpace::Definite(x) => Some(x),
                _ => None,
            })
        } else {
            None
        };

        let truncate_width = if text_style.text_overflow.is_some() {
            known.width.or(match available.width {
                AvailableSpace::Definite(x) => match text_style.line_clamp {
                    Some(max_lines) => Some(x * max_lines),
                    None => Some(x),
                },
                _ => None,
            })
        } else {
            None
        };

        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let line_height = text_style
            .line_height
            .to_pixels(font_size.into(), window.rem_size());
        let runs = self
            .runs
            .clone()
            .unwrap_or_else(|| vec![text_style.to_run(self.text.len())]);
        let input = self.sizing_input(&text_style, font_size, line_height, &runs);

        Some(self.shape_and_store_layout(
            input,
            text_style,
            wrap_width,
            truncate_width,
            window,
            cx,
        ))
    }

    fn prepaint_begin(&mut self, ctx: &mut crate::PrepaintCtx) -> crate::PrepaintFrame {
        self.bounds = Some(ctx.bounds);

        crate::PrepaintFrame {
            handled: true,
            skip_children: true,
            ..Default::default()
        }
    }

    fn prepaint_end(&mut self, _ctx: &mut crate::PrepaintCtx, _frame: crate::PrepaintFrame) {
        // Nothing to pop for text
    }

    fn paint_begin(&mut self, ctx: &mut crate::PaintCtx) -> crate::PaintFrame {
        if let Some(ref layout) = self.layout {
            // Use paint-time bounds directly. Text depends on up-to-date bounds for correct positioning
            // (e.g. after window resizes). Prepaint may be replayed, so relying solely on cached
            // prepaint bounds can lead to stale coordinates.
            let bounds = ctx.bounds;
            self.bounds = Some(bounds);

            let mut line_origin = bounds.origin;
            for line in &layout.lines {
                line.paint_background(
                    line_origin,
                    layout.line_height,
                    layout.text_style.text_align,
                    Some(bounds),
                    ctx.window,
                    ctx.cx,
                )
                .log_err();
                line.paint(
                    line_origin,
                    layout.line_height,
                    layout.text_style.text_align,
                    Some(bounds),
                    ctx.window,
                    ctx.cx,
                )
                .log_err();
                line_origin.y += line.size(layout.line_height).height;
            }
        }

        crate::PaintFrame {
            handled: true,
            skip_children: true,
            ..Default::default()
        }
    }

    fn paint_end(&mut self, _ctx: &mut crate::PaintCtx, _frame: crate::PaintFrame) {
        // Nothing to pop for text
    }

    // Uses default downcasting implementations.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{self as gpui, div, px, point, size, Context, Render, RenderNode, TestAppContext};

    struct RootView;

    impl Render for RootView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
        }
    }

    #[gpui::test]
    fn test_text_node_update_render_node_preserves_layout_when_unchanged(cx: &mut TestAppContext) {
        let (_view, cx) = cx.add_window_view(|_, _| RootView);

        cx.update(|window, app| {
            let known = Size {
                width: None,
                height: None,
            };
            let available = Size {
                width: AvailableSpace::Definite(px(500.)),
                height: AvailableSpace::Definite(px(500.)),
            };

            // &'static str updates should preserve cached layout when unchanged.
            let mut node = TextNode::new(SharedString::from("hello"), None);
            node.measure(known, available, window, app);
            assert!(node.layout.is_some());
            let mut element: &'static str = "hello";
            match element.update_render_node(&mut node, window, app) {
                Some(update) => assert!(!update.any_change()),
                None => panic!("expected &'static str to update an existing TextNode"),
            }
            assert!(node.layout.is_some());

            element = "world";
            match element.update_render_node(&mut node, window, app) {
                Some(update) => assert!(update.layout_changed && update.paint_changed),
                None => panic!("expected &'static str to update an existing TextNode"),
            }
            assert!(node.layout.is_none());

            // SharedString updates should preserve cached layout when unchanged.
            let mut node = TextNode::new(SharedString::from("hello"), None);
            node.measure(known, available, window, app);
            assert!(node.layout.is_some());
            let mut element = SharedString::from("hello");
            match element.update_render_node(&mut node, window, app) {
                Some(update) => assert!(!update.any_change()),
                None => panic!("expected SharedString to update an existing TextNode"),
            }
            assert!(node.layout.is_some());

            element = SharedString::from("world");
            match element.update_render_node(&mut node, window, app) {
                Some(update) => assert!(update.layout_changed && update.paint_changed),
                None => panic!("expected SharedString to update an existing TextNode"),
            }
            assert!(node.layout.is_none());
        });
    }

    #[gpui::test]
    fn test_text_node_paint_begin_uses_paint_bounds(cx: &mut TestAppContext) {
        let (_view, cx) = cx.add_window_view(|_, _| RootView);

        cx.update(|window, app| {
            let known = Size {
                width: None,
                height: None,
            };
            let available = Size {
                width: AvailableSpace::Definite(px(500.)),
                height: AvailableSpace::Definite(px(500.)),
            };

            let mut node = TextNode::new(SharedString::from("hello"), None);
            node.measure(known, available, window, app);

            let old_bounds = Bounds::new(point(px(1.), px(2.)), size(px(3.), px(4.)));
            let new_bounds = Bounds::new(point(px(10.), px(20.)), size(px(30.), px(40.)));
            node.bounds = Some(old_bounds);

            window.invalidator.set_phase(crate::window::DrawPhase::Paint);
            window.next_frame.scene.begin_frame();

            let fiber_id = window.fiber.tree.create_placeholder_fiber();
            let mut paint_ctx = crate::PaintCtx {
                fiber_id,
                bounds: new_bounds,
                child_bounds: Vec::new(),
                inspector_id: None,
                window,
                cx: app,
            };
            node.paint_begin(&mut paint_ctx);

            assert_eq!(
                node.bounds,
                Some(new_bounds),
                "TextNode must use paint-time bounds to avoid stale positioning when prepaint is replayed"
            );
        });
    }
}