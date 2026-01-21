    AtlasTextureId, Background, Bounds, ContentMask, DevicePixels, MonochromeSprite,
    PaintSurface, Path, Point, PolychromeSprite, PrimitiveBatch, Quad, ScaledPixels, Scene,
    SceneSegmentPool, Shadow, Size, Surface, TransformationMatrix, Underline, point, size,
use crate::transform::GpuTransform;
    pub transform_index: u32,
    pub fn draw(&mut self, scene: &Scene, segment_pool: &SceneSegmentPool) {

            let command_buffer = self.draw_primitives(
                scene,
                segment_pool,
                &mut instance_buffer,
                drawable,
                viewport_size,
            );
    pub fn render_to_image(
        &mut self,
        scene: &Scene,
        segment_pool: &SceneSegmentPool,
    ) -> Result<RgbaImage> {
            let command_buffer = self.draw_primitives(
                scene,
                segment_pool,
                &mut instance_buffer,
                drawable,
                viewport_size,
            );
        segment_pool: &SceneSegmentPool,
        let gpu_transforms = segment_pool.transforms.to_gpu_transforms();
        align_offset(&mut instance_offset);
        let context_transforms_offset = instance_offset;
        let transform_bytes_len = mem::size_of_val(gpu_transforms.as_slice());
        let next_offset = context_transforms_offset + transform_bytes_len;
        if next_offset > instance_buffer.size {
            anyhow::bail!("scene too large: transform table does not fit");
        }
        let buffer_contents = unsafe {
            (instance_buffer.metal_buffer.contents() as *mut u8).add(context_transforms_offset)
        };
        unsafe {
            ptr::copy_nonoverlapping(
                gpu_transforms.as_ptr() as *const u8,
                buffer_contents,
                transform_bytes_len,
            );
        }
        instance_offset = next_offset;

        for batch in scene.batches(segment_pool) {
                PrimitiveBatch::Shadows(shadows, transforms) => self.draw_shadows(
                    transforms,
                    context_transforms_offset,
                PrimitiveBatch::Quads(quads, transforms) => self.draw_quads(
                    transforms,
                    context_transforms_offset,
                        context_transforms_offset,
                        &gpu_transforms,
                            context_transforms_offset,
                            &gpu_transforms,
                PrimitiveBatch::Underlines(underlines, transforms) => self.draw_underlines(
                    transforms,
                    context_transforms_offset,
                    context_transforms_offset,
                    transforms,
                    transforms,
                    context_transforms_offset,
                    context_transforms_offset,
                    "scene too large: {} paths, {} shadows, {} quads, {} underlines, {} mono, {} subpixel, {} poly, {} surfaces",
                    scene.paths_len(segment_pool),
                    scene.shadows_len(segment_pool),
                    scene.quads_len(segment_pool),
                    scene.underlines_len(segment_pool),
                    scene.monochrome_sprites_len(segment_pool),
                    scene.subpixel_sprites_len(segment_pool),
                    scene.polychrome_sprites_len(segment_pool),
                    scene.surfaces_len(segment_pool),
        context_transforms_offset: usize,
        gpu_transforms: &[GpuTransform],
            let t = resolve_world_transform(path.transform_index, gpu_transforms);
            let world_bounds = transform_bounds(path.bounds, t);
            let clipped_bounds = world_bounds.intersect(&path.content_mask.bounds);
            if clipped_bounds.is_empty() {
                continue;
            }
                bounds: clipped_bounds,
                transform_index: path.transform_index,
        if vertices.is_empty() {
            command_encoder.end_encoding();
            return true;
        }
        command_encoder.set_vertex_buffer(
            PathRasterizationInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            PathRasterizationInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
        shadow_transforms: &[TransformationMatrix],
        context_transforms_offset: usize,
        debug_assert_eq!(shadows.len(), shadow_transforms.len());
        let shadows_offset = *instance_offset;
        let mut transforms_offset = shadows_offset + shadow_bytes_len;
        align_offset(&mut transforms_offset);
        let transform_bytes_len = mem::size_of_val(shadow_transforms);
        let next_offset = transforms_offset + transform_bytes_len;
        command_encoder.set_vertex_buffer(
            ShadowInputIndex::Shadows as u64,
            Some(&instance_buffer.metal_buffer),
            shadows_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            ShadowInputIndex::Shadows as u64,
            Some(&instance_buffer.metal_buffer),
            shadows_offset as u64,
        );
        command_encoder.set_vertex_buffer(
            ShadowInputIndex::Transforms as u64,
            Some(&instance_buffer.metal_buffer),
            transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            ShadowInputIndex::Transforms as u64,
            Some(&instance_buffer.metal_buffer),
            transforms_offset as u64,
        );
        command_encoder.set_vertex_buffer(
            ShadowInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            ShadowInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );

        let shadow_contents =
            unsafe { (instance_buffer.metal_buffer.contents() as *mut u8).add(shadows_offset) };
        let transform_contents =
            unsafe { (instance_buffer.metal_buffer.contents() as *mut u8).add(transforms_offset) };

                shadow_contents,
            ptr::copy_nonoverlapping(
                shadow_transforms.as_ptr() as *const u8,
                transform_contents,
                transform_bytes_len,
            );
        quad_transforms: &[TransformationMatrix],
        context_transforms_offset: usize,
        debug_assert_eq!(quads.len(), quad_transforms.len());
        let quads_offset = *instance_offset;
        let mut transforms_offset = quads_offset + quad_bytes_len;
        align_offset(&mut transforms_offset);
        let transform_bytes_len = mem::size_of_val(quad_transforms);
        let next_offset = transforms_offset + transform_bytes_len;
        command_encoder.set_vertex_buffer(
            QuadInputIndex::Quads as u64,
            Some(&instance_buffer.metal_buffer),
            quads_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            QuadInputIndex::Quads as u64,
            Some(&instance_buffer.metal_buffer),
            quads_offset as u64,
        );
        command_encoder.set_vertex_buffer(
            QuadInputIndex::Transforms as u64,
            Some(&instance_buffer.metal_buffer),
            transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            QuadInputIndex::Transforms as u64,
            Some(&instance_buffer.metal_buffer),
            transforms_offset as u64,
        );
        command_encoder.set_vertex_buffer(
            QuadInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            QuadInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );

            let quad_contents =
                (instance_buffer.metal_buffer.contents() as *mut u8).add(quads_offset);
            ptr::copy_nonoverlapping(quads.as_ptr() as *const u8, quad_contents, quad_bytes_len);

            let transform_contents =
                (instance_buffer.metal_buffer.contents() as *mut u8).add(transforms_offset);
            ptr::copy_nonoverlapping(
                quad_transforms.as_ptr() as *const u8,
                transform_contents,
                transform_bytes_len,
            );
        context_transforms_offset: usize,
        gpu_transforms: &[GpuTransform],
        let Some(_first_path) = paths.first() else {
        command_encoder.set_vertex_buffer(
            SpriteInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
        // When copying paths from the intermediate texture to the drawable, each pixel must only
        // be copied once in case of overlapping/transparent paths. Conservatively copy a single
        // spanning rect in world space for the entire batch.
        let mut bounds: Option<Bounds<ScaledPixels>> = None;
        for path in paths {
            let t = resolve_world_transform(path.transform_index, gpu_transforms);
            let world = transform_bounds(path.clipped_bounds(), t).intersect(&path.content_mask.bounds);
            if world.is_empty() {
                continue;
            bounds = Some(match bounds {
                Some(existing) => existing.union(&world),
                None => world,
            });
        let Some(bounds) = bounds else {
            return true;
        };

        let sprites = vec![PathSprite {
            transform_index: 0,
            bounds,
        }];
        underline_transforms: &[TransformationMatrix],
        context_transforms_offset: usize,
        debug_assert_eq!(underlines.len(), underline_transforms.len());
        let underlines_offset = *instance_offset;
        let mut transforms_offset = underlines_offset + underline_bytes_len;
        align_offset(&mut transforms_offset);
        let transform_bytes_len = mem::size_of_val(underline_transforms);
        let next_offset = transforms_offset + transform_bytes_len;
        command_encoder.set_vertex_buffer(
            UnderlineInputIndex::Underlines as u64,
            Some(&instance_buffer.metal_buffer),
            underlines_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            UnderlineInputIndex::Underlines as u64,
            Some(&instance_buffer.metal_buffer),
            underlines_offset as u64,
        );
        command_encoder.set_vertex_buffer(
            UnderlineInputIndex::Transforms as u64,
            Some(&instance_buffer.metal_buffer),
            transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            UnderlineInputIndex::Transforms as u64,
            Some(&instance_buffer.metal_buffer),
            transforms_offset as u64,
        );
        command_encoder.set_vertex_buffer(
            UnderlineInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            UnderlineInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );

            let underline_contents =
                (instance_buffer.metal_buffer.contents() as *mut u8).add(underlines_offset);
                underline_contents,

            let transform_contents =
                (instance_buffer.metal_buffer.contents() as *mut u8).add(transforms_offset);
            ptr::copy_nonoverlapping(
                underline_transforms.as_ptr() as *const u8,
                transform_contents,
                transform_bytes_len,
            );
        context_transforms_offset: usize,
        command_encoder.set_vertex_buffer(
            SpriteInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            SpriteInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
        sprite_transforms: &[TransformationMatrix],
        context_transforms_offset: usize,
        debug_assert_eq!(sprites.len(), sprite_transforms.len());
        let sprites_offset = *instance_offset;
        let mut transforms_offset = sprites_offset + sprite_bytes_len;
        align_offset(&mut transforms_offset);
        let transform_bytes_len = mem::size_of_val(sprite_transforms);
        let next_offset = transforms_offset + transform_bytes_len;
        command_encoder.set_vertex_buffer(
            SpriteInputIndex::Transforms as u64,
            Some(&instance_buffer.metal_buffer),
            transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            SpriteInputIndex::Transforms as u64,
            Some(&instance_buffer.metal_buffer),
            transforms_offset as u64,
        );
        command_encoder.set_vertex_buffer(
            SpriteInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
        command_encoder.set_fragment_buffer(
            SpriteInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );

            let sprite_contents =
                (instance_buffer.metal_buffer.contents() as *mut u8).add(sprites_offset);
                sprite_contents,

            let transform_contents =
                (instance_buffer.metal_buffer.contents() as *mut u8).add(transforms_offset);
            ptr::copy_nonoverlapping(
                sprite_transforms.as_ptr() as *const u8,
                transform_contents,
                transform_bytes_len,
            );
        context_transforms_offset: usize,
        command_encoder.set_vertex_buffer(
            SurfaceInputIndex::ContextTransforms as u64,
            Some(&instance_buffer.metal_buffer),
            context_transforms_offset as u64,
        );
                        transform_index: surface.transform_index,
fn resolve_world_transform(transform_index: u32, transforms: &[GpuTransform]) -> GpuTransform {
    let mut resolved = GpuTransform::identity();
    let mut current = transform_index;
    for _ in 0..16 {
        if current == 0 {
            break;
        }
        let t = transforms
            .get(current as usize)
            .copied()
            .unwrap_or_else(GpuTransform::identity);
        resolved.offset[0] = resolved.offset[0] * t.scale + t.offset[0];
        resolved.offset[1] = resolved.offset[1] * t.scale + t.offset[1];
        resolved.scale *= t.scale;
        current = t.parent_index;
    }
    resolved
}

fn transform_bounds(bounds: Bounds<ScaledPixels>, t: GpuTransform) -> Bounds<ScaledPixels> {
    Bounds {
        origin: Point::new(
            ScaledPixels(bounds.origin.x.0 * t.scale + t.offset[0]),
            ScaledPixels(bounds.origin.y.0 * t.scale + t.offset[1]),
        ),
        size: Size {
            width: ScaledPixels(bounds.size.width.0 * t.scale),
            height: ScaledPixels(bounds.size.height.0 * t.scale),
        },
    }
}

    Transforms = 3,
    ContextTransforms = 4,
    Transforms = 3,
    ContextTransforms = 4,
    Transforms = 3,
    ContextTransforms = 4,
    Transforms = 5,
    ContextTransforms = 6,
    ContextTransforms = 6,
    ContextTransforms = 2,
    pub transform_index: u32,
    pub transform_index: u32,