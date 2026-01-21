use crate::window::context::PaintCx;
    TextAlign, TransformationMatrix, UnderlineStyle, Window, WrapBoundary, WrappedLineLayout,
    black, fill, point, px, size,
    ) -> Result<()> {
        self.paint_with_transform(
            origin,
            line_height,
            align,
            align_width,
            window,
            cx,
            TransformationMatrix::unit(),
        )?;

        Ok(())
    }

    /// Paint the line of text to the window with an explicit visual transform.
    pub fn paint_with_transform(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        align_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
        transform: TransformationMatrix,
            transform,
        )
    }

    /// Paint the background of the line to the window.
    pub fn paint_background(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        align_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<()> {
        self.paint_background_with_transform(
            origin,
            line_height,
            align,
            align_width,
            window,
            cx,
            TransformationMatrix::unit(),
    /// Paint the background of the line to the window with an explicit visual transform.
    pub fn paint_background_with_transform(
        transform: TransformationMatrix,
            transform,
        )
    ) -> Result<()> {
        self.paint_with_transform(
            origin,
            line_height,
            align,
            bounds,
            window,
            cx,
            TransformationMatrix::unit(),
        )
    }

    /// Paint this line of text with an explicit visual transform.
    pub fn paint_with_transform(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        bounds: Option<Bounds<Pixels>>,
        window: &mut Window,
        cx: &mut App,
        transform: TransformationMatrix,
            transform,
    ) -> Result<()> {
        self.paint_background_with_transform(
            origin,
            line_height,
            align,
            bounds,
            window,
            cx,
            TransformationMatrix::unit(),
        )
    }

    /// Paint the background of line of text with an explicit visual transform.
    pub fn paint_background_with_transform(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        bounds: Option<Bounds<Pixels>>,
        window: &mut Window,
        cx: &mut App,
        transform: TransformationMatrix,
            transform,
    transform: TransformationMatrix,
    let local_bounds = Bounds::new(
    // When a visual transform is applied (e.g. from React Native view transforms),
    // compute the transformed axis-aligned bounds so clipping/layering still occurs
    // in window-space coordinates.
    let layer_bounds = if transform.is_unit() {
        local_bounds
    } else {
        // Transform the four corners and take their AABB.
        let corners = [
            local_bounds.origin,
            point(
                local_bounds.origin.x + local_bounds.size.width,
                local_bounds.origin.y,
            ),
            point(
                local_bounds.origin.x,
                local_bounds.origin.y + local_bounds.size.height,
            ),
            point(
                local_bounds.origin.x + local_bounds.size.width,
                local_bounds.origin.y + local_bounds.size.height,
            ),
        ];

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for corner in corners {
            let p = transform.apply(corner);
            let x = f32::from(p.x);
            let y = f32::from(p.y);
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        Bounds {
            origin: point(px(min_x), px(min_y)),
            size: size(px((max_x - min_x).max(0.0)), px((max_y - min_y).max(0.0))),
        }
    };

    window.paint_layer(layer_bounds, |window| {
                        window.paint_underline_with_transform(
                            transform,
                        window.paint_strikethrough_with_transform(
                            transform,
                    window.paint_underline_with_transform(
                        transform,
                    window.paint_strikethrough_with_transform(
                        transform,
                let glyph_intersects_mask = if !window.should_cull_scene_primitives() {
                    true
                } else {
                    let content_mask = PaintCx::new(window).content_mask();
                    if transform.is_unit() {
                        max_glyph_bounds.intersects(&content_mask.bounds)
                    } else {
                        // Transform glyph bounds into window space for correct masking when a
                        // visual transform is applied.
                        let corners = [
                            max_glyph_bounds.origin,
                            point(
                                max_glyph_bounds.origin.x + max_glyph_bounds.size.width,
                                max_glyph_bounds.origin.y,
                            ),
                            point(
                                max_glyph_bounds.origin.x,
                                max_glyph_bounds.origin.y + max_glyph_bounds.size.height,
                            ),
                            point(
                                max_glyph_bounds.origin.x + max_glyph_bounds.size.width,
                                max_glyph_bounds.origin.y + max_glyph_bounds.size.height,
                            ),
                        ];

                        let mut min_x = f32::INFINITY;
                        let mut max_x = f32::NEG_INFINITY;
                        let mut min_y = f32::INFINITY;
                        let mut max_y = f32::NEG_INFINITY;

                        for corner in corners {
                            let p = transform.apply(corner);
                            let x = f32::from(p.x);
                            let y = f32::from(p.y);
                            if x < min_x {
                                min_x = x;
                            }
                            if x > max_x {
                                max_x = x;
                            }
                            if y < min_y {
                                min_y = y;
                            }
                            if y > max_y {
                                max_y = y;
                            }
                        }

                        let world_bounds = Bounds {
                            origin: point(px(min_x), px(min_y)),
                            size: size(px((max_x - min_x).max(0.0)), px((max_y - min_y).max(0.0))),
                        };

                        world_bounds.intersects(&content_mask.bounds)
                    }
                };

                if glyph_intersects_mask {
                        window.paint_emoji_with_transform(
                            transform,
                        window.paint_glyph_with_transform(
                            transform,
            window.paint_underline_with_transform(
                transform,
            window.paint_strikethrough_with_transform(
                transform,
    transform: TransformationMatrix,
    let local_bounds = Bounds::new(
    let layer_bounds = if transform.is_unit() {
        local_bounds
    } else {
        let corners = [
            local_bounds.origin,
            point(
                local_bounds.origin.x + local_bounds.size.width,
                local_bounds.origin.y,
            ),
            point(
                local_bounds.origin.x,
                local_bounds.origin.y + local_bounds.size.height,
            ),
            point(
                local_bounds.origin.x + local_bounds.size.width,
                local_bounds.origin.y + local_bounds.size.height,
            ),
        ];

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for corner in corners {
            let p = transform.apply(corner);
            let x = f32::from(p.x);
            let y = f32::from(p.y);
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        Bounds {
            origin: point(px(min_x), px(min_y)),
            size: size(px((max_x - min_x).max(0.0)), px((max_y - min_y).max(0.0))),
        }
    };

    window.paint_layer(layer_bounds, |window| {
                        window.paint_quad_with_transform(
                            fill(
                                Bounds {
                                    origin: *background_origin,
                                    size: size(glyph_origin.x - background_origin.x, line_height),
                                },
                                *background_color,
                            ),
                            transform,
                        );
                    window.paint_quad_with_transform(
                        fill(
                            Bounds {
                                origin: background_origin,
                                size: size(width, line_height),
                            },
                            background_color,
                        ),
                        transform,
                    );
            window.paint_quad_with_transform(
                fill(
                    Bounds {
                        origin: background_origin,
                        size: size(last_line_end_x - background_origin.x, line_height),
                    },
                    background_color,
                ),
                transform,
            );