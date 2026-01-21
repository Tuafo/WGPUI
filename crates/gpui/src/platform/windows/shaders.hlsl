struct SceneTransform {
    float2 offset;
    float scale;
    uint parent_index;
};

struct ResolvedTransform {
    float2 offset;
    float scale;
};

ResolvedTransform resolve_transform(uint transform_index, StructuredBuffer<SceneTransform> transforms) {
    ResolvedTransform result;
    result.offset = float2(0.0, 0.0);
    result.scale = 1.0;
    uint current = transform_index;
    for (uint i = 0; i < 16 && current != 0; i++) {
        SceneTransform t = transforms[current];
        result.offset = result.offset * t.scale + t.offset;
        result.scale = result.scale * t.scale;
        current = t.parent_index;
    }
    return result;
}

float2 apply_context_transform(float2 position, uint transform_index, StructuredBuffer<SceneTransform> transforms) {
    ResolvedTransform t = resolve_transform(transform_index, transforms);
    return position * t.scale + t.offset;
}

float2 apply_context_transform_inverse(float2 position, uint transform_index, StructuredBuffer<SceneTransform> transforms) {
    ResolvedTransform t = resolve_transform(transform_index, transforms);
    return (position - t.offset) / t.scale;
}

float4 to_device_position_with_context(float2 unit_vertex, Bounds bounds, uint transform_index, StructuredBuffer<SceneTransform> transforms) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    position = apply_context_transform(position, transform_index, transforms);
    return to_device_position_impl(position);
}

float4 to_device_position_transformed(float2 unit_vertex, Bounds bounds, TransformationMatrix transformation, uint transform_index, StructuredBuffer<SceneTransform> transforms) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    float2 transformed = mul(position, transformation.rotation_scale) + transformation.translation;
    transformed = apply_context_transform(transformed, transform_index, transforms);
    return to_device_position_impl(transformed);
}

float4 distance_from_clip_rect_with_context(float2 unit_vertex, Bounds bounds, Bounds clip_bounds, uint transform_index, StructuredBuffer<SceneTransform> transforms) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    position = apply_context_transform(position, transform_index, transforms);
    return distance_from_clip_rect_impl(position, clip_bounds);
}

float4 distance_from_clip_rect_transformed_with_context(float2 unit_vertex, Bounds bounds, Bounds clip_bounds, TransformationMatrix transformation, uint transform_index, StructuredBuffer<SceneTransform> transforms) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    float2 transformed = mul(position, transformation.rotation_scale) + transformation.translation;
    transformed = apply_context_transform(transformed, transform_index, transforms);
    return distance_from_clip_rect_impl(transformed, clip_bounds);
}

float2x2 invert2x2(float2x2 m) {
    float det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    return float2x2(
        m[1][1] / det, -m[0][1] / det,
        -m[1][0] / det, m[0][0] / det
    );
}

float2 to_local_position(float2 world, TransformationMatrix transformation) {
    float2x2 inv_rotation_scale = invert2x2(transformation.rotation_scale);
    return mul(world - transformation.translation, inv_rotation_scale);
}

    uint transform_index;
    uint pad;
StructuredBuffer<TransformationMatrix> quad_transforms: register(t2);
StructuredBuffer<SceneTransform> quad_context_transforms: register(t3);
    TransformationMatrix transform = quad_transforms[quad_id];
    float4 device_position = to_device_position_transformed(unit_vertex, quad.bounds, transform, quad.transform_index, quad_context_transforms);
    float4 clip_distance = distance_from_clip_rect_transformed_with_context(unit_vertex, quad.bounds, quad.content_mask, transform, quad.transform_index, quad_context_transforms);
    TransformationMatrix transform = quad_transforms[input.quad_id];
    float2 visual_world = apply_context_transform_inverse(input.position.xy, quad.transform_index, quad_context_transforms);
    float2 local_position = to_local_position(visual_world, transform);
    float4 background_color = gradient_color(quad.background, local_position, quad.bounds,
    float2 the_point = local_position - quad.bounds.origin;
    uint transform_index;
    uint pad;
StructuredBuffer<TransformationMatrix> shadow_transforms: register(t2);
StructuredBuffer<SceneTransform> shadow_context_transforms: register(t3);
    TransformationMatrix transform = shadow_transforms[shadow_id];
    float4 device_position = to_device_position_transformed(unit_vertex, bounds, transform, shadow.transform_index, shadow_context_transforms);
    float4 clip_distance = distance_from_clip_rect_transformed_with_context(unit_vertex, bounds, shadow.content_mask, transform, shadow.transform_index, shadow_context_transforms);
    TransformationMatrix transform = shadow_transforms[input.shadow_id];
    float2 visual_world = apply_context_transform_inverse(input.position.xy, shadow.transform_index, shadow_context_transforms);
    float2 local_position = to_local_position(visual_world, transform);
    float2 point0 = local_position - center;
    uint transform_index;
    uint _pad;
    uint transform_index;
StructuredBuffer<TransformationMatrix> underline_transforms: register(t2);
StructuredBuffer<SceneTransform> underline_context_transforms: register(t3);
    TransformationMatrix transform = underline_transforms[underline_id];
    float4 device_position = to_device_position_transformed(unit_vertex, underline.bounds, transform, underline.transform_index, underline_context_transforms);
    float4 clip_distance = distance_from_clip_rect_transformed_with_context(unit_vertex, underline.bounds,
                                                    underline.content_mask, transform, underline.transform_index, underline_context_transforms);
    TransformationMatrix transform = underline_transforms[input.underline_id];
    float2 visual_world = apply_context_transform_inverse(input.position.xy, underline.transform_index, underline_context_transforms);
    float2 local_position = to_local_position(visual_world, transform);
        float2 st = ((local_position - origin) / underline.bounds.size.y) - float2(0., 0.5);
    uint transform_index;
StructuredBuffer<SceneTransform> mono_sprite_context_transforms: register(t3);
        to_device_position_transformed(unit_vertex, sprite.bounds, sprite.transformation, sprite.transform_index, mono_sprite_context_transforms);
    float4 clip_distance = distance_from_clip_rect_transformed_with_context(unit_vertex, sprite.bounds, sprite.content_mask, sprite.transformation, sprite.transform_index, mono_sprite_context_transforms);
    uint transform_index;
StructuredBuffer<SceneTransform> poly_sprite_context_transforms: register(t3);
    float4 device_position = to_device_position_with_context(unit_vertex, sprite.bounds, sprite.transform_index, poly_sprite_context_transforms);
    float4 clip_distance = distance_from_clip_rect_with_context(unit_vertex, sprite.bounds,
                                                    sprite.content_mask, sprite.transform_index, poly_sprite_context_transforms);
    float2 local_position = apply_context_transform_inverse(input.position.xy, sprite.transform_index, poly_sprite_context_transforms);
    float distance = quad_sdf(local_position, sprite.bounds, sprite.corner_radii);