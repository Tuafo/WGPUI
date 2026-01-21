struct SceneTransform {
  float2 offset;
  float scale;
  uint parent_index;
};

struct ResolvedTransform {
  float2 offset;
  float scale;
};

ResolvedTransform resolve_transform(uint transform_index,
                                   constant SceneTransform *transforms);
float2 apply_context_transform(float2 position, uint transform_index,
                               constant SceneTransform *transforms);
float2 apply_context_transform_inverse(float2 position, uint transform_index,
                                       constant SceneTransform *transforms);

float4 to_device_position_with_context(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          uint transform_index,
                          constant SceneTransform *transforms,
                          constant Size_DevicePixels *viewport_size);
                          uint transform_index,
                          constant SceneTransform *transforms,
float2 apply_transform(float2 position, TransformationMatrix transformation);
float4 distance_from_clip_rect_with_context(float2 unit_vertex, Bounds_ScaledPixels bounds,
                               Bounds_ScaledPixels clip_bounds,
                               uint transform_index,
                               constant SceneTransform *transforms);
                               Bounds_ScaledPixels clip_bounds,
                               TransformationMatrix transformation,
                               uint transform_index,
                               constant SceneTransform *transforms);
float2 to_local_position(float2 world, TransformationMatrix transformation);
                                    constant TransformationMatrix *quad_transforms
                                    [[buffer(QuadInputIndex_Transforms)]],
                                    constant SceneTransform *context_transforms
                                    [[buffer(QuadInputIndex_ContextTransforms)]],
  TransformationMatrix transform = quad_transforms[quad_id];
      to_device_position_transformed(unit_vertex, quad.bounds, transform, quad.transform_index, context_transforms, viewport_size);
  float4 clip_distance = distance_from_clip_rect_transformed(unit_vertex, quad.bounds,
                                                 quad.content_mask.bounds, transform, quad.transform_index, context_transforms);
                              [[buffer(QuadInputIndex_Quads)]],
                              constant TransformationMatrix *quad_transforms
                              [[buffer(QuadInputIndex_Transforms)]],
                              constant SceneTransform *context_transforms
                              [[buffer(QuadInputIndex_ContextTransforms)]]) {
  TransformationMatrix transform = quad_transforms[input.quad_id];
  float2 visual_world =
      apply_context_transform_inverse(input.position.xy, quad.transform_index, context_transforms);
  float2 local_position = to_local_position(visual_world, transform);
  float4 background_color = fill_color(quad.background, local_position, quad.bounds,
  float2 point = local_position - float2(quad.bounds.origin.x, quad.bounds.origin.y);
    constant TransformationMatrix *shadow_transforms
    [[buffer(ShadowInputIndex_Transforms)]],
    constant SceneTransform *context_transforms
    [[buffer(ShadowInputIndex_ContextTransforms)]],
  TransformationMatrix transform = shadow_transforms[shadow_id];
      to_device_position_transformed(unit_vertex, bounds, transform, shadow.transform_index, context_transforms, viewport_size);
      distance_from_clip_rect_transformed(unit_vertex, bounds, shadow.content_mask.bounds, transform, shadow.transform_index, context_transforms);
                                [[buffer(ShadowInputIndex_Shadows)]],
                                constant TransformationMatrix *shadow_transforms
                                [[buffer(ShadowInputIndex_Transforms)]],
                                constant SceneTransform *context_transforms
                                [[buffer(ShadowInputIndex_ContextTransforms)]]) {
  TransformationMatrix transform = shadow_transforms[input.shadow_id];
  float2 visual_world =
      apply_context_transform_inverse(input.position.xy, shadow.transform_index, context_transforms);
  float2 local_position = to_local_position(visual_world, transform);
  float2 point = local_position - center;
    constant TransformationMatrix *underline_transforms
    [[buffer(UnderlineInputIndex_Transforms)]],
    constant SceneTransform *context_transforms
    [[buffer(UnderlineInputIndex_ContextTransforms)]],
    [[buffer(UnderlineInputIndex_ViewportSize)]]) {
  TransformationMatrix transform = underline_transforms[underline_id];
      to_device_position_transformed(unit_vertex, underline.bounds, transform, underline.transform_index, context_transforms, viewport_size);
  float4 clip_distance = distance_from_clip_rect_transformed(unit_vertex, underline.bounds,
                                                 underline.content_mask.bounds, transform, underline.transform_index, context_transforms);
                                   [[buffer(UnderlineInputIndex_Underlines)]],
                                   constant TransformationMatrix *underline_transforms
                                   [[buffer(UnderlineInputIndex_Transforms)]],
                                   constant SceneTransform *context_transforms
                                   [[buffer(UnderlineInputIndex_ContextTransforms)]]) {
  TransformationMatrix transform = underline_transforms[input.underline_id];
  float2 visual_world =
      apply_context_transform_inverse(input.position.xy, underline.transform_index, context_transforms);
  float2 local_position = to_local_position(visual_world, transform);
    float2 st = ((local_position - origin) / underline.bounds.size.height) -
    constant SceneTransform *context_transforms [[buffer(SpriteInputIndex_ContextTransforms)]],
      to_device_position_transformed(unit_vertex, sprite.bounds, sprite.transformation, sprite.transform_index, context_transforms, viewport_size);
                                                 sprite.content_mask.bounds, sprite.transformation, sprite.transform_index, context_transforms);
    constant SceneTransform *context_transforms [[buffer(SpriteInputIndex_ContextTransforms)]],
      to_device_position_with_context(unit_vertex, sprite.bounds, sprite.transform_index, context_transforms, viewport_size);
  float4 clip_distance = distance_from_clip_rect_with_context(unit_vertex, sprite.bounds,
                                                 sprite.content_mask.bounds, sprite.transform_index, context_transforms);
    constant SceneTransform *context_transforms [[buffer(SpriteInputIndex_ContextTransforms)]],
  float2 local_position =
      apply_context_transform_inverse(input.position.xy, sprite.transform_index, context_transforms);
      quad_sdf(local_position, sprite.bounds, sprite.corner_radii);
  constant SceneTransform *context_transforms [[buffer(SpriteInputIndex_ContextTransforms)]],
      to_device_position_with_context(unit_vertex, sprite.bounds, sprite.transform_index, context_transforms, viewport_size);
  screen_position = apply_context_transform(screen_position, sprite.transform_index, context_transforms);
    constant SceneTransform *context_transforms [[buffer(SurfaceInputIndex_ContextTransforms)]],
      to_device_position_with_context(unit_vertex, surface.bounds, surface.transform_index, context_transforms, viewport_size);
  float4 clip_distance = distance_from_clip_rect_with_context(unit_vertex, surface.bounds,
                                                 surface.content_mask.bounds, surface.transform_index, context_transforms);
float4 to_device_position_with_context(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          uint transform_index,
                          constant SceneTransform *transforms,
                          constant Size_DevicePixels *input_viewport_size) {
  float2 position =
      unit_vertex * float2(bounds.size.width, bounds.size.height) +
      float2(bounds.origin.x, bounds.origin.y);
  position = apply_context_transform(position, transform_index, transforms);
  float2 viewport_size = float2((float)input_viewport_size->width,
                                (float)input_viewport_size->height);
  float2 device_position =
      position / viewport_size * float2(2., -2.) + float2(-1., 1.);
  return float4(device_position, 0., 1.);
}

                          uint transform_index,
                          constant SceneTransform *transforms,
  // Apply context transform
  transformed_position = apply_context_transform(transformed_position, transform_index, transforms);

float4 distance_from_clip_rect_with_context(float2 unit_vertex, Bounds_ScaledPixels bounds,
                               Bounds_ScaledPixels clip_bounds,
                               uint transform_index,
                               constant SceneTransform *transforms) {
  float2 position =
      unit_vertex * float2(bounds.size.width, bounds.size.height) +
      float2(bounds.origin.x, bounds.origin.y);
  position = apply_context_transform(position, transform_index, transforms);
  return float4(position.x - clip_bounds.origin.x,
                clip_bounds.origin.x + clip_bounds.size.width - position.x,
                position.y - clip_bounds.origin.y,
                clip_bounds.origin.y + clip_bounds.size.height - position.y);
}

                               Bounds_ScaledPixels clip_bounds, TransformationMatrix transformation,
                               uint transform_index,
                               constant SceneTransform *transforms) {
  // Apply context transform
  transformed_position = apply_context_transform(transformed_position, transform_index, transforms);


float2x2 invert2x2(float2x2 m) {
  float det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
  return float2x2(
    m[1][1] / det, -m[0][1] / det,
    -m[1][0] / det, m[0][0] / det
  );
}

float2 to_local_position(float2 world, TransformationMatrix transformation) {
  float2x2 rotation_scale = float2x2(
    transformation.rotation_scale[0][0], transformation.rotation_scale[0][1],
    transformation.rotation_scale[1][0], transformation.rotation_scale[1][1]
  );
  float2 translation = float2(transformation.translation[0], transformation.translation[1]);
  float2x2 inv_rotation_scale = invert2x2(rotation_scale);
  return inv_rotation_scale * (world - translation);
}

ResolvedTransform resolve_transform(uint transform_index,
                                   constant SceneTransform *transforms) {
  ResolvedTransform out;
  out.offset = float2(0.0);
  out.scale = 1.0;
  uint current = transform_index;
  for (uint i = 0u; i < 16u && current != 0u; i++) {
    SceneTransform t = transforms[current];
    out.offset = out.offset * t.scale + t.offset;
    out.scale = out.scale * t.scale;
    current = t.parent_index;
  }
  return out;
}

float2 apply_context_transform(float2 position, uint transform_index,
                               constant SceneTransform *transforms) {
  ResolvedTransform t = resolve_transform(transform_index, transforms);
  return position * t.scale + t.offset;
}

float2 apply_context_transform_inverse(float2 position, uint transform_index,
                                       constant SceneTransform *transforms) {
  ResolvedTransform t = resolve_transform(transform_index, transforms);
  return (position - t.offset) / t.scale;
}