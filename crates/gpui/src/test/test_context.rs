    Element, Empty, EventEmitter, ForegroundExecutor, Global, InputEvent, IntoElement, Keystroke,
    Modifiers, ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, Platform, Point, Render, Result, Size, Task, TestDispatcher, TestPlatform,
    /// Draw an element to the window using the fiber-backed rendering pipeline.
    ///
    /// This is the preferred method for drawing elements in tests. It uses the retained
    /// node path which supports all element types including those that have migrated
    /// to `RenderNode` (List, UniformList, Div, etc.).
    ///
    /// Returns the computed bounds of the rendered element.
    ) -> Bounds<Pixels>
    where
        E: IntoElement,
    {
        self.update(|window, cx| {
            let mut element = f(window, cx).into_any_element();
            window.draw_element_via_fibers(&mut element, origin, space.into(), cx)
        })
    }

    /// Draw an element to the window using the legacy element pipeline.
    ///
    /// This method uses the traditional `request_layout` -> `prepaint` -> `paint` path.
    /// Use this only for elements that specifically need the legacy return types
    /// (RequestLayoutState, PrepaintState) or haven't migrated to RenderNode yet.
    ///
    /// For most cases, prefer `draw` which uses the fiber-backed pipeline.
    pub fn draw_legacy<E>(
        &mut self,
        origin: Point<Pixels>,
        space: impl Into<Size<AvailableSpace>>,
        f: impl FnOnce(&mut Window, &mut App) -> E,
            {
                let mut prepaint_cx = crate::window::context::PrepaintCx::new(window);
                prepaint_cx
                    .with_absolute_element_offset(origin, |window| element.prepaint(window, cx));
            }
            window.snapshot_hitboxes_into_rendered_frame();