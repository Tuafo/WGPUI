    Background, Bounds, DevicePixels, GpuSpecs, MonochromeSprite, Path, Point,
    PolychromeSprite, PrimitiveBatch, Quad, ScaledPixels, Scene, SceneSegmentPool, Shadow, Size,
    Underline, get_gamma_correction_ratios,
use crate::transform::GpuTransform;
    transform_index: u32,
    pad: u32,
    b_quad_transforms: gpu::BufferPiece,
    b_context_transforms: gpu::BufferPiece,
    b_shadow_transforms: gpu::BufferPiece,
    b_context_transforms: gpu::BufferPiece,
    b_context_transforms: gpu::BufferPiece,
    b_context_transforms: gpu::BufferPiece,
    b_underline_transforms: gpu::BufferPiece,
    b_context_transforms: gpu::BufferPiece,
    b_context_transforms: gpu::BufferPiece,
    b_context_transforms: gpu::BufferPiece,
    b_poly_sprite_transforms: gpu::BufferPiece,
    b_context_transforms: gpu::BufferPiece,
    b_context_transforms: gpu::BufferPiece,
    transform_index: u32,
    _pad: u32,
        shader.check_struct_size::<GpuTransform>();
        context_transforms: gpu::BufferPiece,
        gpu_transforms: &[GpuTransform],
                let t = resolve_world_transform(path.transform_index, gpu_transforms);
                let world_bounds = transform_bounds(path.bounds, t);
                let clipped_bounds = world_bounds.intersect(&path.content_mask.bounds);
                if clipped_bounds.is_empty() {
                    continue;
                }
                    bounds: clipped_bounds,
                    transform_index: path.transform_index,
                    _pad: 0,
            if vertices.is_empty() {
                return;
            }
                    b_context_transforms: context_transforms,
    pub fn draw(&mut self, scene: &Scene, segment_pool: &SceneSegmentPool) {
        // Keep scene count helpers exercised for diagnostics parity across renderers.
        let _scene_counts = (
            scene.paths_len(segment_pool),
            scene.shadows_len(segment_pool),
            scene.quads_len(segment_pool),
            scene.underlines_len(segment_pool),
            scene.monochrome_sprites_len(segment_pool),
            scene.subpixel_sprites_len(segment_pool),
            scene.polychrome_sprites_len(segment_pool),
            scene.surfaces_len(segment_pool),
        );
        let gpu_transforms = segment_pool.transforms.to_gpu_transforms();
        let context_transforms = unsafe { self.instance_belt.alloc_typed(&gpu_transforms, &self.gpu) };

        for batch in scene.batches(segment_pool) {
                PrimitiveBatch::Quads(quads, transforms) => {
                    let transform_buf =
                        unsafe { self.instance_belt.alloc_typed(transforms, &self.gpu) };
                            b_quad_transforms: transform_buf,
                            b_context_transforms: context_transforms,
                PrimitiveBatch::Shadows(shadows, transforms) => {
                    let transform_buf =
                        unsafe { self.instance_belt.alloc_typed(transforms, &self.gpu) };
                            b_shadow_transforms: transform_buf,
                            b_context_transforms: context_transforms,
                        context_transforms,
                        &gpu_transforms,
                            .filter_map(|path| {
                                let t = resolve_world_transform(path.transform_index, &gpu_transforms);
                                let world_bounds =
                                    transform_bounds(path.clipped_bounds(), t).intersect(&path.content_mask.bounds);
                                if world_bounds.is_empty() {
                                    None
                                } else {
                                    Some(PathSprite { bounds: world_bounds })
                                }
                        let mut bounds: Option<Bounds<ScaledPixels>> = None;
                        for path in paths {
                            let t = resolve_world_transform(path.transform_index, &gpu_transforms);
                            let world_bounds =
                                transform_bounds(path.clipped_bounds(), t).intersect(&path.content_mask.bounds);
                            if world_bounds.is_empty() {
                                continue;
                            }
                            bounds = Some(match bounds {
                                Some(existing) => existing.union(&world_bounds),
                                None => world_bounds,
                            });
                        bounds.map(|bounds| vec![PathSprite { bounds }]).unwrap_or_default()
                    if sprites.is_empty() {
                        continue;
                    }
                            b_context_transforms: context_transforms,
                PrimitiveBatch::Underlines(underlines, transforms) => {
                    let transform_buf =
                        unsafe { self.instance_belt.alloc_typed(transforms, &self.gpu) };
                            b_underline_transforms: transform_buf,
                            b_context_transforms: context_transforms,
                            b_context_transforms: context_transforms,
                    transforms,
                    let transform_buf =
                        unsafe { self.instance_belt.alloc_typed(transforms, &self.gpu) };
                            b_poly_sprite_transforms: transform_buf,
                            b_context_transforms: context_transforms,
                            b_context_transforms: context_transforms,
                                        transform_index: surface.transform_index,
                                        pad: 0,
                                    b_context_transforms: context_transforms,
    pub fn render_to_image(
        &mut self,
        _scene: &Scene,
        _segment_pool: &SceneSegmentPool,
    ) -> Result<RgbaImage> {
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
