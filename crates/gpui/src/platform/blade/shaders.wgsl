fn apply_transform(position: vec2<f32>, transform: TransformationMatrix) -> vec2<f32> {
    // Rust stores rotation_scale row-major; transpose for WGSL multiplication
    return transpose(transform.rotation_scale) * position + transform.translation;
}

struct GpuTransform {
    offset: vec2<f32>,
    scale: f32,
    parent_index: u32,
}

struct ResolvedTransform {
    offset: vec2<f32>,
    scale: f32,
}

var<storage, read> b_context_transforms: array<GpuTransform>;

fn resolve_transform(transform_index: u32) -> ResolvedTransform {
    var out: ResolvedTransform;
    out.offset = vec2<f32>(0.0);
    out.scale = 1.0;

    var current = transform_index;
    for (var i = 0u; i < 16u && current != 0u; i = i + 1u) {
        let t = b_context_transforms[current];
        out.offset = out.offset * t.scale + t.offset;
        out.scale = out.scale * t.scale;
        current = t.parent_index;
    }

    return out;
}

fn apply_context_transform(position: vec2<f32>, transform_index: u32) -> vec2<f32> {
    let t = resolve_transform(transform_index);
    return position * t.scale + t.offset;
}

fn apply_context_transform_inverse(position: vec2<f32>, transform_index: u32) -> vec2<f32> {
    let t = resolve_transform(transform_index);
    return (position - t.offset) / t.scale;
}

fn to_device_position(unit_vertex: vec2<f32>, bounds: Bounds, transform_index: u32) -> vec4<f32> {
    let world_position = apply_context_transform(position, transform_index);
    return to_device_position_impl(world_position);
fn to_device_position_transformed(unit_vertex: vec2<f32>, bounds: Bounds, transform: TransformationMatrix, transform_index: u32) -> vec4<f32> {
    let transformed = apply_transform(position, transform);
    let world_position = apply_context_transform(transformed, transform_index);
    return to_device_position_impl(world_position);
fn distance_from_clip_rect_with_context(unit_vertex: vec2<f32>, bounds: Bounds, clip_bounds: Bounds, transform_index: u32) -> vec4<f32> {
    let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
    let world_position = apply_context_transform(position, transform_index);
    return distance_from_clip_rect_impl(world_position, clip_bounds);
}

fn distance_from_clip_rect_transformed(unit_vertex: vec2<f32>, bounds: Bounds, clip_bounds: Bounds, transform: TransformationMatrix, transform_index: u32) -> vec4<f32> {
    let transformed = apply_transform(position, transform);
    let world_position = apply_context_transform(transformed, transform_index);
    return distance_from_clip_rect_impl(world_position, clip_bounds);
    transform_index: u32,
    pad: u32,
var<storage, read> b_quad_transforms: array<TransformationMatrix>;
    let transform = b_quad_transforms[instance_id];
    out.position = to_device_position_transformed(unit_vertex, quad.bounds, transform, quad.transform_index);
    out.clip_distances = distance_from_clip_rect_transformed(unit_vertex, quad.bounds, quad.content_mask, transform, quad.transform_index);
// Helpers to map device-space to the quad's local coordinate space
fn invert2x2(m: mat2x2<f32>) -> mat2x2<f32> {
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    return (1.0 / det) * mat2x2<f32>(m[1][1], -m[0][1], -m[1][0], m[0][0]);
}

fn to_local_position(world: vec2<f32>, t: TransformationMatrix) -> vec2<f32> {
    let m = transpose(t.rotation_scale);
    let inv = invert2x2(m);
    return inv * (world - t.translation);
}

    let transform = b_quad_transforms[input.quad_id];
    let visual_world = apply_context_transform_inverse(input.position.xy, quad.transform_index);
    let local_position = to_local_position(visual_world, transform);
    let background_color = gradient_color(quad.background, local_position, quad.bounds,
    let point = local_position - quad.bounds.origin;
    transform_index: u32,
    pad: u32,
var<storage, read> b_shadow_transforms: array<TransformationMatrix>;
    let transform = b_shadow_transforms[instance_id];
    out.position = to_device_position_transformed(unit_vertex, shadow.bounds, transform, shadow.transform_index);
    out.clip_distances = distance_from_clip_rect_transformed(unit_vertex, shadow.bounds, shadow.content_mask, transform, shadow.transform_index);
    let transform = b_shadow_transforms[input.shadow_id];
    let visual_world = apply_context_transform_inverse(input.position.xy, shadow.transform_index);
    let local_position = to_local_position(visual_world, transform);
    let center_to_point = local_position - center;
    transform_index: u32,
    let world_position = apply_context_transform(v.xy_position, v.transform_index);
    out.position = to_device_position_impl(world_position);
    out.clip_distances = distance_from_clip_rect_impl(world_position, v.bounds);
    let device_position = to_device_position(unit_vertex, sprite.bounds, 0u);
    transform_index: u32,
var<storage, read> b_underline_transforms: array<TransformationMatrix>;
    let transform = b_underline_transforms[instance_id];
    out.position = to_device_position_transformed(unit_vertex, underline.bounds, transform, underline.transform_index);
    out.clip_distances = distance_from_clip_rect_transformed(unit_vertex, underline.bounds, underline.content_mask, transform, underline.transform_index);
    let transform = b_underline_transforms[input.underline_id];
    let visual_world = apply_context_transform_inverse(input.position.xy, underline.transform_index);
    let local_position = to_local_position(visual_world, transform);
    let st = (local_position - underline.bounds.origin) / underline.bounds.size.y - vec2<f32>(0.0, 0.5);
    transform_index: u32,
    out.position = to_device_position_transformed(unit_vertex, sprite.bounds, sprite.transformation, sprite.transform_index);
    out.clip_distances = distance_from_clip_rect_transformed(unit_vertex, sprite.bounds, sprite.content_mask, sprite.transformation, sprite.transform_index);
    transform_index: u32,
var<storage, read> b_poly_sprite_transforms: array<TransformationMatrix>;
    let transform = b_poly_sprite_transforms[instance_id];
    out.position = to_device_position_transformed(unit_vertex, sprite.bounds, transform, sprite.transform_index);
    out.clip_distances = distance_from_clip_rect_transformed(unit_vertex, sprite.bounds, sprite.content_mask, transform, sprite.transform_index);
    let transform = b_poly_sprite_transforms[input.sprite_id];
    let visual_world = apply_context_transform_inverse(input.position.xy, sprite.transform_index);
    let local_position = to_local_position(visual_world, transform);
    let distance = quad_sdf(local_position, sprite.bounds, sprite.corner_radii);
    transform_index: u32,
    pad: u32,
    out.position = to_device_position(unit_vertex, surface_locals.bounds, surface_locals.transform_index);
    out.clip_distances = distance_from_clip_rect_with_context(unit_vertex, surface_locals.bounds, surface_locals.content_mask, surface_locals.transform_index);
    transform_index: u32,
    out.position = to_device_position_transformed(unit_vertex, sprite.bounds, sprite.transformation, sprite.transform_index);
    out.clip_distances = distance_from_clip_rect_transformed(unit_vertex, sprite.bounds, sprite.content_mask, sprite.transformation, sprite.transform_index);