    ForegroundExecutor, GlobalElementId, GlyphId, GpuSpecs, ImageSource, Keymap, LineLayout,
    Pixels, PlatformInput, Point, Priority, RenderGlyphParams, RenderImage,
    RenderImageParams, RenderSvgParams, Scene, SceneSegmentPool, ShapedGlyph, ShapedRun,
    SharedString, Size, SvgRenderer, SystemWindowTab, Task, TaskTiming,
    fn draw(&self, scene: &Scene, segment_pool: &SceneSegmentPool);
    fn render_to_image(
        &self,
        _scene: &Scene,
        _segment_pool: &SceneSegmentPool,
    ) -> Result<RgbaImage> {
        // Return a deterministic non-zero raster bounds so tests can observe glyph primitives
        // being inserted into the scene when using the test platform.
        //
        // The test platform uses `NoopTextSystem`, which doesn't rasterize real glyphs. However,
        // the retained fiber rendering architecture relies on glyph primitives being stored in
        // scene segments, so it's useful for tests to exercise that code path.
        let side = DevicePixels(10);
        Ok(Bounds {
            origin: Point::default(),
            size: size(side, side),
        })
    fiber_id: GlobalElementId,
    pub fn new(cx: AsyncWindowContext, fiber_id: GlobalElementId) -> Self {
        Self { cx, fiber_id }
                window
                    .with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                        handler.selected_text_range(ignore_disabled_input, window, cx)
                    })
                    .flatten()
            .update(|window, cx| {
                window
                    .with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                        handler.marked_text_range(window, cx)
                    })
                    .flatten()
            })
                window
                    .with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                        handler.text_for_range(range_utf16, adjusted, window, cx)
                    })
                    .flatten()
                window.with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                    handler.replace_text_in_range(replacement_range, text, window, cx);
                });
                window.with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                    handler.replace_and_mark_text_in_range(
                        range_utf16,
                        new_text,
                        new_selected_range,
                        window,
                        cx,
                    );
                });
            .update(|window, cx| {
                window.with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                    handler.unmark_text(window, cx);
                });
            })
            .update(|window, cx| {
                window
                    .with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                        handler.bounds_for_range(range_utf16, window, cx)
                    })
                    .flatten()
            })
        self.cx
            .update(|window, cx| {
                window.with_input_handler_mut(self.fiber_id, cx, |handler, _, _| {
                    handler.apple_press_and_hold_enabled()
                })
            })
            .ok()
            .flatten()
            .unwrap_or(true)
    pub(crate) fn dispatch_input(&mut self, input: &str) {
        self.cx
            .update(|window, cx| {
                window.with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                    handler.replace_text_in_range(None, input, window, cx);
                });
            })
            .ok();
    pub fn selected_bounds(&mut self) -> Option<Bounds<Pixels>> {
        self.cx
            .update(|window, cx| {
                window
                    .with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                        let selection = handler.selected_text_range(true, window, cx)?;
                        let range = if selection.reversed {
                            selection.range.start..selection.range.start
                        } else {
                            selection.range.end..selection.range.end
                        };
                        handler.bounds_for_range(range, window, cx)
                    })
                    .flatten()
            })
            .ok()
            .flatten()
            .update(|window, cx| {
                window
                    .with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                        handler.character_index_for_point(point, window, cx)
                    })
                    .flatten()
            })
    pub(crate) fn accepts_text_input(&mut self) -> bool {
        self.cx
            .update(|window, cx| {
                window.with_input_handler_mut(self.fiber_id, cx, |handler, window, cx| {
                    handler.accepts_text_input(window, cx)
                })
            })
            .ok()
            .flatten()
            .unwrap_or(false)