use crate::taffy::TaffyLayoutEngine;
    Action, AnyDrag, AnyElement, AnyImageCache, AnyTooltip, AnyView, App, AppContext, Asset,
    AsyncWindowContext, AvailableSpace, Background, BorderStyle, Bounds, BoxShadow,
    Capslock, Context, Corners, CursorStyle, Decorations, DevicePixels, DirtyFlags, DisplayId,
    Edges, Effect, Entity, EntityId, EventEmitter, FiberEffects, FileDropEvent, FontId, Global,
    GlobalElementId, GlyphId, GpuSpecs, Hsla, InputHandler, IsZero, KeyBinding, KeyContext,
    KeyDispatcher, KeyDownEvent, KeyEvent, Keystroke, KeystrokeEvent, LayoutId, Modifiers,
    ModifiersChangedEvent, MonochromeSprite, MouseButton, MouseEvent, MouseMoveEvent, MouseUpEvent,
    Path, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformWindow, Point, PolychromeSprite, Priority, PromptButton, PromptLevel, Quad,
    ReconcileReport, Render, RenderGlyphParams, RenderImage, RenderImageParams, RenderSvgParams,
    ScaledPixels, Scene, SceneSegmentPool, Shadow, SharedString, Size, StrikethroughStyle, Style,
    SubpixelSprite, SubscriberSet, Subscription, SystemWindowTab, SystemWindowTabController, Task,
    TextRenderingMode, TextStyle, TextStyleRefinement, TransformationMatrix, Underline,
    UnderlineStyle, WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControls,
    WindowDecorations, WindowOptions, WindowParams, WindowTextSystem, Transform2D, TransformId,
use slotmap::{DefaultKey, SlotMap};
    ops::DerefMut,
use taffy::tree::NodeId;
pub(crate) mod context;
use crate::fiber::FiberRuntime;
#[cfg(debug_assertions)]
use crate::fiber::debug_assert_active_list_matches_map;
pub(crate) use crate::fiber::{FiberRuntimeHandle, FiberRuntimeHandleRef, has_mouse_effects};
        inner.dirty = true;
        if matches!(inner.draw_phase, DrawPhase::None | DrawPhase::Event) {
            cx.push_effect(Effect::Notify { emitter: entity });
            true
        } else {
            false
        }
    }

    pub fn mark_dirty(&self, entity: EntityId, cx: &mut App) -> bool {
        let mut inner = self.inner.borrow_mut();
        inner.dirty = true;
        if matches!(inner.draw_phase, DrawPhase::None | DrawPhase::Event) {
    pub fn phase(&self) -> DrawPhase {
        self.inner.borrow().draw_phase
    }

        matches!(
            self.inner.borrow().draw_phase,
            DrawPhase::None | DrawPhase::Event
        )
    pub fn debug_assert_layout_or_prepaint(&self) {
            matches!(
                self.inner.borrow().draw_phase,
                DrawPhase::Layout | DrawPhase::Prepaint
            ),
            "this method can only be called during layout or prepaint"
    pub fn debug_assert_prepaint_or_paint(&self) {
            "this method can only be called during prepaint or paint"
        );
    }

    #[track_caller]
    pub fn debug_assert_layout_or_prepaint_or_paint(&self) {
        debug_assert!(
            matches!(
                self.inner.borrow().draw_phase,
                DrawPhase::Layout | DrawPhase::Prepaint | DrawPhase::Paint
            ),
            "this method can only be called during layout, prepaint, or paint"
fn snapshot_hitboxes_into_map(
    fiber_tree: &crate::fiber::FiberTree,
    map: &mut FxHashMap<HitboxId, HitboxSnapshot>,
) {
    map.clear();
    let Some(root) = fiber_tree.root else {
        return;
    };
    let mut stack: Vec<GlobalElementId> = vec![root];
    while let Some(fiber_id) = stack.pop() {
        let children: SmallVec<[GlobalElementId; 8]> = fiber_tree.children(&fiber_id).collect();
        for child in children {
            stack.push(child);
        let Some(data) = fiber_tree
            .hitbox_state
            .get(fiber_id.into())
            .and_then(|state| state.hitbox.as_ref())
        else {
            continue;
        };
        map.insert(
            fiber_id.into(),
            HitboxSnapshot {
                transform_id: data.transform_id,
                bounds: data.bounds,
                content_mask: data.content_mask.clone(),
                behavior: data.behavior,
            },
        );
fn resolve_hitbox_for_hit_test(window: &Window, fiber_id: &GlobalElementId) -> Option<HitboxSnapshot> {
    let data = window
        .fiber
        .tree
        .hitbox_state
        .get((*fiber_id).into())
        .and_then(|state| state.hitbox.as_ref())?;

    Some(HitboxSnapshot {
        transform_id: data.transform_id,
        bounds: data.bounds,
        content_mask: data.content_mask.clone(),
        behavior: data.behavior,
    })
/// This is provided when subscribing for `Context::on_focus_out` events.
pub struct FocusOutEvent {
    /// A weak focus handle representing what was blurred.
    pub blurred: WeakFocusHandle,
slotmap::new_key_type! {
    /// A globally unique identifier for a focusable element.
    pub struct FocusId;
        window.focus_contains(*self, other)
        if let Some(fiber_id) = window.fiber.tree.focusable_fibers.get(&self.id).copied() {
            window.dispatch_action_on_node(fiber_id, action, cx)
#[derive(Clone, Debug)]
pub(crate) struct HitboxSnapshot {
    pub(crate) transform_id: TransformId,
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) content_mask: ContentMask<Pixels>,
    pub(crate) behavior: HitboxBehavior,
}

#[derive(Clone, Copy, Debug)]
struct TransformStackFrame {
    id: TransformId,
    offset: Point<Pixels>,
}

/// Manages the current transform context during prepaint and paint.
pub(crate) struct TransformStack {
    frames: Vec<TransformStackFrame>,
}

impl Default for TransformStack {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformStack {
    pub(crate) fn new() -> Self {
        Self {
            frames: vec![TransformStackFrame {
                id: TransformId::ROOT,
                offset: Point::default(),
            }],
        }
    }

    pub(crate) fn depth(&self) -> usize {
        self.frames.len()
    }

    pub(crate) fn truncate(&mut self, depth: usize) {
        self.frames.truncate(depth.max(1));
    }

    pub(crate) fn set_local_offset(&mut self, offset: Point<Pixels>) {
        if let Some(frame) = self.frames.last_mut() {
            frame.offset = offset;
        }
    }

    /// Get the current transform ID.
    pub(crate) fn current(&self) -> TransformId {
        self.frames
            .last()
            .map(|frame| frame.id)
            .unwrap_or(TransformId::ROOT)
    }

    /// Get the current offset within this transform context.
    pub(crate) fn local_offset(&self) -> Point<Pixels> {
        self.frames
            .last()
            .map(|frame| frame.offset)
            .unwrap_or_default()
    }

    /// Push a simple offset (accumulates into the current frame's offset).
    pub(crate) fn push_offset(&mut self, offset: Point<Pixels>) {
        if let Some(frame) = self.frames.last_mut() {
            frame.offset.x += offset.x;
            frame.offset.y += offset.y;
        }
    }

    /// Pop a simple offset.
    pub(crate) fn pop_offset(&mut self, offset: Point<Pixels>) {
        if let Some(frame) = self.frames.last_mut() {
            frame.offset.x -= offset.x;
            frame.offset.y -= offset.y;
        }
    }

    /// Push an existing transform context.
    ///
    /// This resets the local offset for the child context to 0, so that primitives can be stored
    /// in the transform's local coordinate space.
    pub(crate) fn push_existing_transform(&mut self, id: TransformId) {
        self.frames.push(TransformStackFrame {
            id,
            offset: Point::default(),
        });
    }

    /// Pop the current transform context.
    pub(crate) fn pop_transform(&mut self) {
        if self.frames.len() > 1 {
            self.frames.pop();
        }
    }
}

pub struct HitboxId(NodeId);
    /// Checks if the hitbox with this ID is currently hovered.
        window.hitbox_is_hovered(self)
    /// Checks if the hitbox contains the mouse and should handle scroll events.
        window.hitbox_should_handle_scroll(self)
    }
}

impl std::ops::Deref for HitboxId {
    type Target = NodeId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NodeId> for HitboxId {
    fn from(id: NodeId) -> Self {
        Self(id)
    }
}

impl From<HitboxId> for NodeId {
    fn from(id: HitboxId) -> Self {
        id.0
}
impl From<HitboxId> for DefaultKey {
    fn from(id: HitboxId) -> Self {
        id.0.into()
    pub(crate) fn next(&mut self) -> TooltipId {
        let id = self.0;
        self.0 = self.0.wrapping_add(1);
        TooltipId(id)
    }

/// Tracks an active fiber-backed overlay (tooltip, prompt, or drag) for painting.
#[derive(Clone, Copy)]
pub(crate) struct ActiveOverlay {
    /// The fiber root for this overlay.
    pub(crate) fiber_id: GlobalElementId,
    /// The absolute offset to paint at.
    pub(crate) offset: Point<Pixels>,
    /// The view context for painting.
    pub(crate) view_id: EntityId,
}

    pub(crate) id: TooltipId,
    pub(crate) tooltip: AnyTooltip,
    pub(crate) current_view: EntityId,
    pub(crate) priority: usize,
    pub(crate) text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) element: Option<AnyElement>,
    pub(crate) fiber_id: Option<GlobalElementId>,
    pub(crate) reference_fiber: Option<GlobalElementId>,
    pub(crate) local_offset: Point<Pixels>,
    /// Whether this deferred draw needs its own layout pass.
    ///
    /// `false` for deferred drawing of an already-laid-out fiber subtree (e.g. `deferred(...)`).
    /// `true` for detached overlay trees created via `Window::defer_draw`.
    pub(crate) requires_layout: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct HitboxesSnapshotEpoch {
    structure_epoch: u64,
    hitbox_epoch: u64,
    pub(crate) focus_path: SmallVec<[FocusId; 8]>,
    pub(crate) hitboxes: FxHashMap<HitboxId, HitboxSnapshot>,
    hitboxes_epoch: Option<HitboxesSnapshotEpoch>,
    impl Frame {
    pub(crate) fn new() -> Self {
            focus_path: SmallVec::new(),
            hitboxes: FxHashMap::default(),
            hitboxes_epoch: None,
        self.scene.clear_transient();
        self.focus_path.clear();
    pub(crate) fn hit_test(&self, window: &Window, position: Point<Pixels>) -> HitTest {

        let viewport = window.viewport_size();
        if position.x < px(0.)
            || position.y < px(0.)
            || position.x >= viewport.width
            || position.y >= viewport.height
        {
            return hit_test;
        }

        let scale_factor = window.scale_factor();
        let transforms = &window.segment_pool.transforms;
        let world_scaled = Point::new(
            ScaledPixels(position.x.0 * scale_factor),
            ScaledPixels(position.y.0 * scale_factor),
        );

        // Returns true if we should stop (BlockMouse behavior hit)
        let mut push_hitbox = |hit_test: &mut HitTest,
                               set_hover_hitbox_count: &mut bool,
                               hitbox_id: HitboxId,
                               hitbox: &HitboxSnapshot| {
            if !hitbox.content_mask.bounds.contains(&position) {
                return false;
            }

            let local_scaled = transforms.world_to_local_no_cache(hitbox.transform_id, world_scaled);
            let local_point = Point::new(
                Pixels(local_scaled.x.0 / scale_factor),
                Pixels(local_scaled.y.0 / scale_factor),
            );

            if hitbox.bounds.contains(&local_point) {
                hit_test.ids.push(hitbox_id);
                if !*set_hover_hitbox_count
                    *set_hover_hitbox_count = true;
                    return true;
                }
            }
            false
        };

        let should_visit_subtree = |fiber_id: GlobalElementId| {
            window
                .fiber
                .tree
                .hitbox_state
                .get(fiber_id.into())
                .and_then(|state| state.hitbox_subtree_bounds)
                .is_some_and(|subtree| {
                    let local_scaled =
                        transforms.world_to_local_no_cache(subtree.transform_id, world_scaled);
                    let local_point = Point::new(
                        Pixels(local_scaled.x.0 / scale_factor),
                        Pixels(local_scaled.y.0 / scale_factor),
                    );
                    subtree.bounds.contains(&local_point)
                })
        };

        // Defered fibers (via `deferred(...)`) paint after all non-deferred content, regardless of
        // their position in the tree. Hit-testing must mirror this paint order so overlays remain
        // interactive even when declared before the content they cover.
        let mut deferred_roots: Vec<(GlobalElementId, usize)> = window
            .fiber
            .active_deferred_draws
            .members
            .iter()
            .filter_map(|fiber_id| {
                window
                    .fiber
                    .tree
                    .deferred_priorities
                    .get((*fiber_id).into())
                    .map(|&priority| (*fiber_id, priority))
            })
            .collect();
        deferred_roots.sort_by_key(|(_, priority)| *priority);

        let mut process_hitbox = |fiber_id: GlobalElementId,
                                  hit_test: &mut HitTest,
                                  set_hover_hitbox_count: &mut bool| {
            if let Some(hitbox) = self.hitboxes.get(&fiber_id.into()) {
                push_hitbox(hit_test, set_hover_hitbox_count, fiber_id.into(), hitbox)
            } else if let Some(hitbox) = resolve_hitbox_for_hit_test(window, &fiber_id) {
                push_hitbox(hit_test, set_hover_hitbox_count, fiber_id.into(), &hitbox)
            } else {
                false
            }
        };

        // Process deferred roots first (topmost-first): higher priority is painted later (on top).
        'outer: for (deferred_root, _priority) in deferred_roots.iter().rev().copied() {
            let mut stack: Vec<(GlobalElementId, bool)> = vec![(deferred_root, true)];
            while let Some((fiber_id, entering)) = stack.pop() {
                if entering {
                    if !should_visit_subtree(fiber_id) {
                        continue;
                    }
                    stack.push((fiber_id, false));
                    for child_id in window.fiber.tree.children_slice(&fiber_id) {
                        stack.push((*child_id, true));
                    }
                } else if process_hitbox(fiber_id, &mut hit_test, &mut set_hover_hitbox_count) {
                    break 'outer;
                }
            }
        }

        // Then process the main tree, skipping deferred subtrees (they were handled above).
        if let Some(root) = window.fiber.tree.root {
            let mut stack: Vec<(GlobalElementId, bool)> = vec![(root, true)];
            while let Some((fiber_id, entering)) = stack.pop() {
                if entering {
                    if !should_visit_subtree(fiber_id) {
                        continue;
                    }
                    if window
                        .fiber
                        .tree
                        .deferred_priorities
                        .contains_key(fiber_id.into())
                    {
                        continue;
                    }
                    stack.push((fiber_id, false));
                    for child_id in window.fiber.tree.children_slice(&fiber_id) {
                        stack.push((*child_id, true));
                    }
                } else if process_hitbox(fiber_id, &mut hit_test, &mut set_hover_hitbox_count) {


    pub(crate) fn focus_path(&self) -> &SmallVec<[FocusId; 8]> {
        &self.focus_path
    pub(crate) fn finish(
        &mut self,
        segment_pool: &mut SceneSegmentPool,
    ) -> crate::scene::SceneFinishStats {
        self.scene.finish(segment_pool)

/// Diagnostic counters for the most recently completed frame.
///
/// Enable in release builds via the `diagnostics` feature.
#[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FrameDiagnostics {
    /// Frame number for which these counters were recorded.
    pub frame_number: u64,
    /// `FiberTree` structure epoch at frame start.
    pub structure_epoch: u64,
    /// `FiberTree` hitbox epoch at frame end.
    pub hitbox_epoch: u64,
    /// Number of fibers that executed a prepaint this frame (not replayed).
    pub prepaint_fibers: usize,
    /// Number of subtrees whose prepaint state was replayed.
    pub prepaint_replayed_subtrees: usize,
    /// Number of fibers that executed paint this frame (not replayed).
    pub paint_fibers: usize,
    /// Number of subtrees whose paint output was replayed.
    pub paint_replayed_subtrees: usize,
    /// Whether the scene segment order was rebuilt.
    pub segment_order_rebuilt: bool,
    /// Scene segment order length when rebuilt.
    pub scene_segment_order_len: usize,
    /// Whether the hitbox snapshot map was rebuilt.
    pub hitboxes_snapshot_rebuilt: bool,
    /// Hitbox count in the snapshot map.
    pub hitboxes_in_snapshot: usize,
    /// Total allocated segments in the segment pool.
    pub total_pool_segments: usize,
    /// Segments mutated in the current scene mutation epoch.
    pub mutated_pool_segments: usize,
    /// Whether the transient segment was mutated.
    pub transient_segment_mutated: bool,
    /// Total path primitives in the scene.
    pub paths: usize,
    /// Total shadow primitives in the scene.
    pub shadows: usize,
    /// Total quad primitives in the scene.
    pub quads: usize,
    /// Total underline primitives in the scene.
    pub underlines: usize,
    /// Total monochrome sprite primitives in the scene.
    pub monochrome_sprites: usize,
    /// Total subpixel sprite primitives in the scene.
    pub subpixel_sprites: usize,
    /// Total polychrome sprite primitives in the scene.
    pub polychrome_sprites: usize,
    /// Total surface primitives in the scene.
    pub surfaces: usize,
    /// Estimated bytes uploaded for instance buffers if the entire scene is re-uploaded.
    pub estimated_instance_upload_bytes: usize,
    /// Number of fibers that had layout computed (cache miss).
    pub layout_fibers: usize,
    /// Time spent in the reconcile phase.
    pub reconcile_time: std::time::Duration,
    /// Time spent in the intrinsic sizing phase.
    pub intrinsic_sizing_time: std::time::Duration,
    /// Time spent in the layout phase.
    pub layout_time: std::time::Duration,
    /// Time spent in the prepaint phase.
    pub prepaint_time: std::time::Duration,
    /// Time spent in the paint phase.
    pub paint_time: std::time::Duration,
    /// Time spent in end-of-frame cleanup (clearing work flags, descendant tracking, etc.).
    pub cleanup_time: std::time::Duration,
    /// Total frame time.
    pub total_time: std::time::Duration,
}

    pub(crate) layout_engine: TaffyLayoutEngine,
    key_dispatch: KeyDispatcher,
    pub(crate) fiber: FiberRuntime,
    pub(crate) transform_stack: TransformStack,
    scroll_transforms: FxHashMap<GlobalElementId, TransformId>,
    scene_culling_disabled_depth: usize,
    /// Shared storage for fiber scene segments. Persists across frame swaps so that
    /// segment IDs allocated during paint remain valid when frames are swapped.
    pub(crate) segment_pool: SceneSegmentPool,
    /// Active overlay state for fiber-backed overlays (tooltip/prompt/drag).
    /// Stores the offset used during prepaint for use during paint.
    pub(crate) active_overlay: Option<ActiveOverlay>,
    render_layers: FxHashMap<ElementId, RenderLayerRegistration>,
    next_render_layer_seq: usize,
    pending_view_accesses: FxHashMap<GlobalElementId, FxHashSet<EntityId>>,
    pub(crate) mouse_hit_test: HitTest,
    pending_mouse_hit_test_refresh: bool,
    #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
    pub(crate) frame_diagnostics: FrameDiagnostics,
    #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
    completed_frame_diagnostics: FrameDiagnostics,
type RenderLayerBuilder = Arc<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>;

#[derive(Clone)]
struct RenderLayerRegistration {
    order: i32,
    seq: usize,
    build: RenderLayerBuilder,
}

    Reconcile,
    Layout,
    Event,
    fn update_platform_input_handler(&mut self, cx: &App) {
        if !self.invalidator.not_drawing() {
            return;
        }
        if let Some(input_handler) = self.fibers().latest_input_handler(cx) {
            self.platform_window.set_input_handler(input_handler);
        } else {
            let _ = self.platform_window.take_input_handler();
        }
    }
    pub(crate) fn fibers(&mut self) -> FiberRuntimeHandle<'_> {
        FiberRuntimeHandle { window: self }
    }

    pub(crate) fn fibers_ref(&self) -> FiberRuntimeHandleRef<'_> {
        FiberRuntimeHandleRef { window: self }
    }

    /// Returns true if the hitbox is currently hovered.
    pub fn hitbox_is_hovered(&self, hitbox_id: HitboxId) -> bool {
        self.mouse_hit_test
            .ids
            .iter()
            .take(self.mouse_hit_test.hover_hitbox_count)
            .any(|id| *id == hitbox_id)
    }

    /// Returns true if the hitbox should handle scroll events.
    pub fn hitbox_should_handle_scroll(&self, hitbox_id: HitboxId) -> bool {
        self.mouse_hit_test.ids.contains(&hitbox_id)
    }

    fn resolve_hitbox_bounds_world(&self, data: &crate::fiber::HitboxData) -> Bounds<Pixels> {
        if data.transform_id.is_root() {
            return data.bounds;
        }

        let scale_factor = self.scale_factor();
        let local_scaled = data.bounds.scale(scale_factor);
        let world = self
            .segment_pool
            .transforms
            .get_world_no_cache(data.transform_id);
        let origin_scaled = world.apply(local_scaled.origin);
        let size_scaled = Size::new(
            ScaledPixels(local_scaled.size.width.0 * world.scale),
            ScaledPixels(local_scaled.size.height.0 * world.scale),
        );

        Bounds::new(
            Point::new(
                Pixels(origin_scaled.x.0 / scale_factor),
                Pixels(origin_scaled.y.0 / scale_factor),
            ),
            Size::new(
                Pixels(size_scaled.width.0 / scale_factor),
                Pixels(size_scaled.height.0 / scale_factor),
            ),
        )
    }

    pub(crate) fn resolve_hitbox(&self, fiber_id: &GlobalElementId) -> Option<Hitbox> {
        self.invalidator.debug_assert_prepaint_or_paint();
        let data = self
            .fiber
            .tree
            .hitbox_state
            .get((*fiber_id).into())
            .and_then(|state| state.hitbox.as_ref())?;

        let bounds = self.resolve_hitbox_bounds_world(data);

        Some(Hitbox {
            id: (*fiber_id).into(),
            bounds,
            content_mask: data.content_mask.clone(),
            behavior: data.behavior,
        })
    }

    pub(crate) fn resolve_hitbox_for_event(&self, fiber_id: &GlobalElementId) -> Option<Hitbox> {
        let data = self
            .fiber
            .tree
            .hitbox_state
            .get((*fiber_id).into())
            .and_then(|state| state.hitbox.as_ref())?;

        Some(Hitbox {
            id: (*fiber_id).into(),
            bounds: self.resolve_hitbox_bounds_world(data),
            content_mask: data.content_mask.clone(),
            behavior: data.behavior,
        })
    }

    pub(crate) fn get_fiber_effects(&self, fiber_id: &GlobalElementId) -> Option<&FiberEffects> {
        self.fiber.tree.effects.get((*fiber_id).into())
    }

    /// Inserts a hitbox associated with a fiber.
    pub fn insert_hitbox_with_fiber(
        &mut self,
        bounds: Bounds<Pixels>,
        behavior: HitboxBehavior,
        fiber_id: GlobalElementId,
    ) -> Hitbox {
        self.fibers()
            .insert_hitbox_with_fiber(bounds, behavior, fiber_id)
    }

    pub(crate) fn register_fiber_effects(
        &mut self,
        fiber_id: &GlobalElementId,
    ) -> Option<&mut FiberEffects> {
        let entry = self.fiber.tree.effects.entry((*fiber_id).into())?;
        let effects = entry.or_insert_with(FiberEffects::new);
        Some(effects)
    }

    pub(crate) fn update_active_mouse_listeners(&mut self, fiber_id: &GlobalElementId) {
        let effects = self.fiber.tree.effects.get((*fiber_id).into());
        // Get interactivity from render node
        let interactivity = self
            .fiber
            .tree
            .render_nodes
            .get((*fiber_id).into())
            .and_then(|node| node.interactivity());
        if has_mouse_effects(interactivity, effects) {
            self.fiber.active_mouse_listeners.insert(*fiber_id);
        } else {
            self.fiber.active_mouse_listeners.remove(fiber_id);
        }
    }

    pub(crate) fn clear_fiber_mouse_effects(&mut self, fiber_id: &GlobalElementId) {
        if let Some(effects) = self.fiber.tree.effects.get_mut((*fiber_id).into()) {
            effects.click_listeners.clear();
            effects.any_mouse_listeners.clear();
            effects.mouse_down_listeners.clear();
            effects.mouse_up_listeners.clear();
            effects.mouse_move_listeners.clear();
            effects.mouse_pressure_listeners.clear();
            effects.scroll_wheel_listeners.clear();
            effects.drag_listener = None;
            effects.drop_listeners.clear();
            effects.can_drop_predicate = None;
            effects.hover_listener = None;
            effects.tooltip = None;
            effects.cursor_style = None;
        }
        self.fiber.active_mouse_listeners.remove(fiber_id);
    }

                                window.draw(cx);
            layout_engine: TaffyLayoutEngine::new(),
            key_dispatch: KeyDispatcher::new(cx.keymap.clone()),
            fiber: FiberRuntime::new(),
            transform_stack: TransformStack::new(),
            scroll_transforms: FxHashMap::default(),
            scene_culling_disabled_depth: 0,
            rendered_frame: Frame::new(),
            next_frame: Frame::new(),
            segment_pool: SceneSegmentPool::default(),
            active_overlay: None,
            render_layers: FxHashMap::default(),
            next_render_layer_seq: 0,
            pending_view_accesses: FxHashMap::default(),
            pending_mouse_hit_test_refresh: false,
            #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
            frame_diagnostics: FrameDiagnostics::default(),
            #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
            completed_frame_diagnostics: FrameDiagnostics::default(),
    pub(crate) fn mark_view_dirty(&mut self, view_id: EntityId) {
        self.fibers().mark_view_dirty(view_id);
        self.ensure_view_root_fiber(view.entity_id());
    /// Schedule a redraw without forcing a full refresh/reconciliation path.
    ///
    /// This is useful for paint-only invalidation driven by runtime state (hover, scroll offset,
    /// etc.) in the retained fiber architecture.
    pub(crate) fn request_redraw(&mut self) {
        if self.invalidator.not_drawing() {
            self.invalidator.set_dirty(true);
        }
    }

    /// Mark a specific fiber as needing paint and schedule a redraw.
    pub(crate) fn invalidate_fiber_paint(&mut self, fiber_id: GlobalElementId) {
        self.fiber.tree.mark_dirty(&fiber_id, DirtyFlags::NEEDS_PAINT);
        self.request_redraw();
    }

    /// Mark a specific fiber as having a transform-only change and schedule a redraw.
    pub(crate) fn invalidate_fiber_transform(&mut self, fiber_id: GlobalElementId) {
        self.fiber
            .tree
            .mark_dirty(&fiber_id, DirtyFlags::TRANSFORM_CHANGED);
        self.request_redraw();
    }

    pub(crate) fn ensure_scroll_transform(
        &mut self,
        fiber_id: GlobalElementId,
        parent: TransformId,
        scroll_offset: Point<Pixels>,
    ) -> TransformId {
        let scale_factor = self.scale_factor();
        let offset = Point::new(
            ScaledPixels(scroll_offset.x.0 * scale_factor),
            ScaledPixels(scroll_offset.y.0 * scale_factor),
        );
        let transform = Transform2D {
            offset,
            scale: 1.0,
            parent,
        };

        if let Some(id) = self.scroll_transforms.get(&fiber_id).copied() {
            self.segment_pool.transforms.insert(id, transform);
            id
        } else {
            let id = self.segment_pool.transforms.push(transform);
            self.scroll_transforms.insert(fiber_id, id);
            id
        }
    }

    /// Mark a scroll container as having a transform-only change and update its scroll transform in
    /// O(1).
    pub(crate) fn invalidate_fiber_scroll(
        &mut self,
        fiber_id: GlobalElementId,
        scroll_offset: Point<Pixels>,
        cx: &mut App,
    ) {
        self.invalidate_fiber_transform(fiber_id);

        if let Some(transform_id) = self.scroll_transforms.get(&fiber_id).copied() {
            let scale_factor = self.scale_factor();
            let scaled = Point::new(
                ScaledPixels(scroll_offset.x.0 * scale_factor),
                ScaledPixels(scroll_offset.y.0 * scale_factor),
            );
            self.segment_pool
                .transforms
                .update_offset(transform_id, scaled);
        }

        self.pending_mouse_hit_test_refresh = true;
        self.apply_pending_mouse_hit_test_refresh(cx);
    }

    pub(crate) fn apply_pending_mouse_hit_test_refresh(&mut self, cx: &mut App) {
        if !self.pending_mouse_hit_test_refresh {
            return;
        }
        self.pending_mouse_hit_test_refresh = false;

        let previous_hit_test = HitTest {
            ids: self.mouse_hit_test.ids.clone(),
            hover_hitbox_count: self.mouse_hit_test.hover_hitbox_count,
        };

        let current_hit_test = self.rendered_frame.hit_test(self, self.mouse_position);
        if current_hit_test == self.mouse_hit_test {
            return;
        }

        self.mouse_hit_test = current_hit_test;
        self.reset_cursor_style(cx);
        self.update_hover_states_for_hit_test_change(&previous_hit_test, cx);
    }

    fn update_hover_states_for_hit_test_change(&mut self, previous_hit_test: &HitTest, cx: &mut App) {
        let current_hit_test = &self.mouse_hit_test;

        let mut target_hitboxes: SmallVec<[HitboxId; 16]> = SmallVec::new();
        let mut seen: FxHashSet<HitboxId> = FxHashSet::default();

        for id in current_hit_test
            .ids
            .iter()
            .take(current_hit_test.hover_hitbox_count)
            .copied()
        {
            if seen.insert(id) {
                target_hitboxes.push(id);
            }
        }
        for id in previous_hit_test
            .ids
            .iter()
            .take(previous_hit_test.hover_hitbox_count)
            .copied()
        {
            if seen.insert(id) {
                target_hitboxes.push(id);
            }
        }

        for hitbox_id in target_hitboxes {
            let fiber_id: GlobalElementId = hitbox_id.into();
            if self.fiber.tree.get(&fiber_id).is_none() {
                continue;
            }

            let (has_hover_style, group_hover) = {
                let Some(interactivity) = self
                    .fiber
                    .tree
                    .render_nodes
                    .get(fiber_id.into())
                    .and_then(|node| node.interactivity())
                else {
                    continue;
                };
                (
                    interactivity.hover_style.is_some(),
                    interactivity
                        .group_hover_style
                        .as_ref()
                        .map(|group_hover| group_hover.group.clone()),
                )
            };

            if has_hover_style {
                let _ = self.with_element_state_in_event::<crate::InteractiveElementState, _>(
                    &fiber_id,
                    |element_state, window| {
                        let mut element_state = element_state.unwrap_or_default();
                        let hover_state = element_state
                            .hover_state
                            .get_or_insert_with(Default::default)
                            .clone();
                        let hovered = window.hitbox_is_hovered(hitbox_id);
                        let mut hover_state = hover_state.borrow_mut();
                        if hovered != hover_state.element {
                            hover_state.element = hovered;
                            drop(hover_state);
                            window.invalidate_fiber_paint(fiber_id);
                        }
                        ((), element_state)
                    },
                );
            }

            if let Some(group) = group_hover {
                if let Some(group_hitbox_id) = crate::GroupHitboxes::get(&group, cx) {
                    let _ = self.with_element_state_in_event::<crate::InteractiveElementState, _>(
                        &fiber_id,
                        |element_state, window| {
                            let mut element_state = element_state.unwrap_or_default();
                            let hover_state = element_state
                                .hover_state
                                .get_or_insert_with(Default::default)
                                .clone();
                            let group_hovered = window.hitbox_is_hovered(group_hitbox_id);
                            let mut hover_state = hover_state.borrow_mut();
                            if group_hovered != hover_state.group {
                                hover_state.group = group_hovered;
                                drop(hover_state);
                                window.invalidate_fiber_paint(fiber_id);
                            }
                            ((), element_state)
                        },
                    );
                }
            }
        }
    }

    /// Close this window.
    pub fn remove_window(&mut self) {
        self.removed = true;
    fn focus_contains(&self, parent: FocusId, child: FocusId) -> bool {
        self.fibers_ref().focus_contains(parent, child)
    }

        self.update_platform_input_handler(cx);
        if self.invalidator.not_drawing() {
            let _ = self.platform_window.take_input_handler();
        }
        if let Some(handle) = self.fibers_ref().next_tab_stop(self.focus.as_ref()) {
        if let Some(handle) = self.fibers_ref().prev_tab_stop(self.focus.as_ref()) {
        // Get the current view without phase assertion - this can be called during reconciliation
        // when views are being rendered (expand_view_fibers phase)
        let entity = if let Some(id) = self.rendered_entity_stack.last().copied() {
            id
        } else {
            self.root.as_ref().map(|root| root.entity_id()).expect(
                "Window::request_animation_frame called with no rendered view and no root view",
            )
        };
            .render_to_image(&self.rendered_frame.scene, &self.segment_pool)
        _element_id: ElementId,
        let global_id = self.ensure_fiber_for_current_id();
        f(&global_id, self)
    /// This method must only be called as part of element layout or drawing.
        self.invalidator.debug_assert_layout_or_prepaint_or_paint();
    /// Register a window-global render layer.
    ///
    /// Render layers are invoked once per window per frame, after the root view
    /// has been prepainted (so they can rely on layout-bound state) and before
    /// hit-testing is finalized.
    ///
    /// Layers are painted after the root view and before deferred draws,
    /// prompts, and tooltips. Ordering between layers is controlled by `order`
    /// (lower first). Ties are broken by first-registration order.
    pub fn register_render_layer<F>(&mut self, key: impl Into<ElementId>, order: i32, build: F)
    where
        F: Fn(&mut Window, &mut App) -> AnyElement + 'static,
    {
        let key = key.into();
        let build: RenderLayerBuilder = Arc::new(build);

        if let Some(registration) = self.render_layers.get_mut(&key) {
            registration.order = order;
            registration.build = build;
            return;
        }

        let seq = self.next_render_layer_seq;
        self.next_render_layer_seq = self.next_render_layer_seq.saturating_add(1);
        self.render_layers
            .insert(key, RenderLayerRegistration { order, seq, build });
    }

    /// Unregister a render layer by key.
    pub fn unregister_render_layer(&mut self, key: &ElementId) {
        self.render_layers.remove(key);
    }

    /// Returns true if the given render layer key is registered.
    pub fn has_render_layer(&self, key: &ElementId) -> bool {
        self.render_layers.contains_key(key)
    }

        self.fibers_ref()
            .is_action_available_for_node(action, node_id)
        self.fibers_ref()
            .is_action_available_for_node(action, node_id)
    /// Hit-test the rendered frame at the given window position.
    ///
    /// Returns hitbox IDs from topmost to bottommost.
    pub fn hit_test_ids(&self, position: Point<Pixels>) -> SmallVec<[HitboxId; 8]> {
        self.rendered_frame.hit_test(self, position).ids
    }

    /// Returns diagnostic counters for the most recently completed frame.
    #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
    #[allow(clippy::misnamed_getters)]
    pub fn frame_diagnostics(&self) -> FrameDiagnostics {
        self.completed_frame_diagnostics
    }

    pub fn draw(&mut self, cx: &mut App) {
        self.prepare_frame(cx);
        if !cx.mode.skip_drawing() {
            self.draw_roots(cx);
        }
        for segment_id in self.fiber.tree.take_removed_scene_segments() {
            self.segment_pool.remove_segment(segment_id);
        }
        self.rebuild_scene_segment_order_if_needed();
        self.cleanup_removed_fibers();
        self.fibers().rebuild_collection_ordering();
        #[cfg(debug_assertions)]
        self.debug_assert_incremental_collections();
        self.finalize_frame(cx);
    }
    fn prepare_frame(&mut self, cx: &mut App) {
        self.fiber.layout_bounds_cache.clear();
        self.fiber.hitbox_stack.clear();
        #[cfg(any(test, feature = "test-support"))]
        self.next_frame
            .debug_bounds
            .clone_from(&self.rendered_frame.debug_bounds);

        self.next_frame.scene.begin_frame();
        self.platform_window.take_input_handler();
    }

    fn finalize_frame(&mut self, cx: &mut App) {
        if let Some(input_handler) = self.fibers().latest_input_handler(cx) {
            self.platform_window.set_input_handler(input_handler);
        let scene_finish_stats = {
            let fiber_tree = &self.fiber.tree;
            let hitboxes_epoch = HitboxesSnapshotEpoch {
                structure_epoch: fiber_tree.structure_epoch,
                hitbox_epoch: fiber_tree.hitbox_epoch(),
            };
            if self.next_frame.hitboxes_epoch != Some(hitboxes_epoch) {
                snapshot_hitboxes_into_map(fiber_tree, &mut self.next_frame.hitboxes);
                self.next_frame.hitboxes_epoch = Some(hitboxes_epoch);
                #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
                {
                    self.frame_diagnostics.hitboxes_snapshot_rebuilt = true;
                }
            }
            #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
            {
                self.frame_diagnostics.hitboxes_in_snapshot = self.next_frame.hitboxes.len();
                self.frame_diagnostics.hitbox_epoch = hitboxes_epoch.hitbox_epoch;
            }

            self.next_frame.finish(&mut self.segment_pool)
        };
        #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
        {
            self.frame_diagnostics.total_pool_segments = scene_finish_stats.total_pool_segments;
            self.frame_diagnostics.mutated_pool_segments = scene_finish_stats.mutated_pool_segments;
            self.frame_diagnostics.transient_segment_mutated = scene_finish_stats.transient_mutated;

            let scene = &self.next_frame.scene;
            let pool = &self.segment_pool;
            self.frame_diagnostics.paths = scene.paths_len(pool);
            self.frame_diagnostics.shadows = scene.shadows_len(pool);
            self.frame_diagnostics.quads = scene.quads_len(pool);
            self.frame_diagnostics.underlines = scene.underlines_len(pool);
            self.frame_diagnostics.monochrome_sprites = scene.monochrome_sprites_len(pool);
            self.frame_diagnostics.subpixel_sprites = scene.subpixel_sprites_len(pool);
            self.frame_diagnostics.polychrome_sprites = scene.polychrome_sprites_len(pool);
            self.frame_diagnostics.surfaces = scene.surfaces_len(pool);

            use std::mem::size_of;
            self.frame_diagnostics.estimated_instance_upload_bytes = self
                .frame_diagnostics
                .shadows
                .saturating_mul(size_of::<crate::scene::Shadow>() + size_of::<TransformationMatrix>())
                .saturating_add(
                    self.frame_diagnostics
                        .quads
                        .saturating_mul(size_of::<crate::scene::Quad>() + size_of::<TransformationMatrix>()),
                )
                .saturating_add(
                    self.frame_diagnostics.underlines.saturating_mul(
                        size_of::<crate::scene::Underline>() + size_of::<TransformationMatrix>(),
                    ),
                )
                .saturating_add(
                    self.frame_diagnostics.monochrome_sprites.saturating_mul(
                        size_of::<crate::scene::MonochromeSprite>(),
                    ),
                )
                .saturating_add(
                    self.frame_diagnostics.subpixel_sprites.saturating_mul(
                        size_of::<crate::scene::SubpixelSprite>(),
                    ),
                )
                .saturating_add(
                    self.frame_diagnostics.polychrome_sprites.saturating_mul(
                        size_of::<crate::scene::PolychromeSprite>() + size_of::<TransformationMatrix>(),
                    ),
                )
                .saturating_add(
                    self.frame_diagnostics.paths.saturating_mul(
                        size_of::<crate::Path<ScaledPixels>>(),
                    ),
                )
                .saturating_add(
                    self.frame_diagnostics
                        .surfaces
                        .saturating_mul(size_of::<crate::scene::PaintSurface>()),
                );
        }
        let _ = scene_finish_stats;
        self.next_frame.focus_path = self.fibers_ref().focus_path_for(self.focus);
        let previous_focus_path = self.rendered_frame.focus_path().clone();
        let current_focus_path = self.rendered_frame.focus_path().clone();
            self.dispatch_focus_change_events(
                previous_focus_path,
                current_focus_path,
                previous_window_active,
                current_window_active,
                cx,
            );
        #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
        {
            self.completed_frame_diagnostics = self.frame_diagnostics;
        }
    }

    /// Dispatch any pending focus change events without performing a full draw cycle.
    ///
    /// This is useful when you need focus change listeners to be notified immediately,
    /// such as between programmatic keystrokes where focus may change and subsequent
    /// keystrokes depend on the new focus state (e.g., vim mode).
    ///
    /// In the fiber architecture, the focus path can be computed at any time from the
    /// persistent fiber tree, enabling this lightweight focus event dispatch without
    /// requiring a full layout/paint cycle.
    pub fn dispatch_pending_focus_events(&mut self, cx: &mut App) {
        let current_focus_path = self.fibers_ref().focus_path_for(self.focus);
        let previous_focus_path = self.rendered_frame.focus_path.clone();

        if previous_focus_path != current_focus_path {
            self.dispatch_focus_change_events(
                previous_focus_path,
                current_focus_path.clone(),
                self.rendered_frame.window_active,
                self.rendered_frame.window_active,
                cx,
            );
            self.rendered_frame.focus_path = current_focus_path;
        }
    }

    fn dispatch_focus_change_events(
        &mut self,
        previous_focus_path: SmallVec<[FocusId; 8]>,
        current_focus_path: SmallVec<[FocusId; 8]>,
        previous_window_active: bool,
        current_window_active: bool,
        cx: &mut App,
    ) {
        if !previous_focus_path.is_empty() && current_focus_path.is_empty() {
            self.focus_lost_listeners
                .clone()
                .retain(&(), |listener| listener(self, cx));
        }

        let event = WindowFocusEvent {
            previous_focus_path: if previous_window_active {
                previous_focus_path
            } else {
                Default::default()
            },
            current_focus_path: if current_window_active {
                current_focus_path
            } else {
                Default::default()
            },
        };
        self.focus_listeners
            .clone()
            .retain(&(), |listener| listener(&event, self, cx));
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn snapshot_hitboxes_into_rendered_frame(&mut self) {
        let fiber_tree = &self.fiber.tree;
        snapshot_hitboxes_into_map(fiber_tree, &mut self.rendered_frame.hitboxes);
        self.rendered_frame.hitboxes_epoch = Some(HitboxesSnapshotEpoch {
            structure_epoch: fiber_tree.structure_epoch,
            hitbox_epoch: fiber_tree.hitbox_epoch(),
        });
    #[cfg(debug_assertions)]
    fn debug_assert_incremental_collections(&self) {
        debug_assert_active_list_matches_map(
            "tooltips",
            &self.fiber.active_tooltips,
            &self.fiber.tree.tooltips,
        );
        debug_assert_active_list_matches_map(
            "cursor_styles",
            &self.fiber.active_cursor_styles,
            &self.fiber.tree.cursor_styles,
        );
        debug_assert_active_list_matches_map(
            "deferred_draws",
            &self.fiber.active_deferred_draws,
            &self.fiber.tree.deferred_draws,
        );
        debug_assert_active_list_matches_map(
            "input_handlers",
            &self.fiber.active_input_handlers,
            &self.fiber.tree.input_handlers,
        );
        for fiber_id in self.fiber.active_mouse_listeners.members.iter() {
            let effects = self.fiber.tree.effects.get((*fiber_id).into());
            // Get interactivity from render node
            let interactivity = self
                .fiber
                .tree
                .render_nodes
                .get((*fiber_id).into())
                .and_then(|node| node.interactivity());
            debug_assert!(
                has_mouse_effects(interactivity, effects),
                "active mouse list contains fiber without mouse effects: {fiber_id:?}"
            );
        }
        for (key, focus_ids) in self.fiber.tree.tab_stops.iter() {
            let fiber_id = GlobalElementId::from(key);
            for focus_id in focus_ids.iter() {
                debug_assert!(
                    self.fiber.rendered_tab_stops.contains(focus_id),
                    "tab stop {focus_id:?} missing from rendered map for fiber {fiber_id:?}"
                );
            }
        }
    }

        self.platform_window
            .draw(&self.rendered_frame.scene, &self.segment_pool);
    fn prepaint_render_layers(
        &mut self,
        root_size: Size<Pixels>,
        cx: &mut App,
    ) -> Vec<(ElementId, AnyElement)> {
        if self.render_layers.is_empty() {
            return Vec::new();
        }

        context::PrepaintCx::new(self).prepaint_render_layers(root_size, cx)
    }

    fn paint_render_layers(&mut self, elements: &mut [(ElementId, AnyElement)], cx: &mut App) {
        context::PaintCx::new(self).paint_render_layers(elements, cx)
    }

    fn with_root_view_context<R>(&mut self, f: impl FnOnce(&mut Window) -> R) -> R {
        // Many elements expect to run under a rendering view context (e.g. image caches
        // consult `Window::current_view()`), so ensure a view ID is present.
        if let Some(root_view_id) = self.root.as_ref().map(|v| v.entity_id()) {
            self.with_rendered_view(root_view_id, f)
        } else {
            f(self)
        }
    }

    fn rebuild_scene_segment_order_if_needed(&mut self) {
        let structure_epoch = self.fiber.tree.structure_epoch;
        if self.next_frame.scene.segment_order_epoch() == structure_epoch {
            return;
        }
        let order = self
            .fiber
            .tree
            .root
            .map(|root| self.fiber.tree.scene_segment_order(root))
            .unwrap_or_default();
        #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
        {
            self.frame_diagnostics.segment_order_rebuilt = true;
            self.frame_diagnostics.scene_segment_order_len = order.len();
        }
        self.next_frame
            .scene
            .set_segment_order(order, structure_epoch);
    }

    fn cleanup_removed_fibers(&mut self) {
        let removed_tab_stops = self.fiber.tree.take_removed_tab_stops();
        for (owner_id, focus_id) in removed_tab_stops {
            self.fiber
                .rendered_tab_stops
                .remove_if_owned_by(&focus_id, owner_id);
        }
        for fiber_id in self.fiber.tree.take_removed_fibers() {
            self.fiber.active_tooltips.remove(&fiber_id);
            self.fiber.active_cursor_styles.remove(&fiber_id);
            self.fiber.active_deferred_draws.remove(&fiber_id);
            self.fiber.active_input_handlers.remove(&fiber_id);
            self.fiber.active_mouse_listeners.remove(&fiber_id);
            if let Some(transform_id) = self.scroll_transforms.remove(&fiber_id) {
                self.segment_pool.transforms.remove(transform_id);
            }
        }
    }

    fn finalize_dirty_flags(&mut self) {
        TaffyLayoutEngine::finalize_dirty_flags(self);
    }

    /// Reconcile the fiber tree against the current view state.
    ///
    /// This is the single entry point for all structure changes and node updates.
    /// After this call, the fiber tree is stable and ready for layout/prepaint/paint.
    ///
    /// Returns a `ReconcileReport` indicating what changed during reconciliation,
    /// and the root fiber ID.
    fn reconcile_frame(
        &mut self,
        root_size: Size<Pixels>,
        cx: &mut App,
    ) -> (ReconcileReport, GlobalElementId) {
        self.invalidator.set_phase(DrawPhase::Reconcile);
        let mut report = ReconcileReport::default();

        // Check for viewport change
        let viewport_changed = self
            .layout_engine
            .last_layout_viewport_size
            .is_none_or(|size| size != root_size);

        // Branch 1: Root is completely clean with cached output - skip reconciliation
        if !self.refreshing
            && !viewport_changed
            && self.fiber.tree.root.as_ref().is_some_and(|root_id| {
                self.fiber.tree.get(root_id).is_some_and(|_| {
                    self.fiber.tree.dirty_flags(root_id).is_subtree_clean()
                        && self.fiber.tree.has_cached_output(root_id)
                })
            })
        {
            let root_fiber = self.fiber.tree.root.unwrap();
            // Check if layout is needed (for return value)
            report.needs_layout = viewport_changed;
            return (report, root_fiber);
        }

        // Branch 2: Root exists but has dirty views - expand views only
        if !self.refreshing && !viewport_changed && self.fiber.tree.root.is_some() {
            let root_fiber = self.fiber.tree.root.unwrap();

            // Check if the root fiber itself is dirty and needs re-rendering.
            // The root fiber doesn't have view_data, so expand_view_fibers skips it.
            // We need to handle root view re-rendering here.
            let root_needs_rerender = self.fiber.tree.dirty_flags(&root_fiber).needs_work();

            if root_needs_rerender {
                let root_view = self.root.as_ref().unwrap().clone();
                cx.entities.push_access_scope();
                cx.entities.record_access(root_view.entity_id());
                let mut root_element_tree = self
                    .with_rendered_view(root_view.entity_id(), |window| {
                        root_view.render_element(window, cx)
                    });
                let root_accessed_entities = cx.entities.pop_access_scope();

                self.hydrate_view_children(&mut root_element_tree);
                self.record_pending_view_accesses(&root_fiber, root_accessed_entities);

                // Reconcile the new element tree into the existing fiber structure
                self.fiber
                    .tree
                    .reconcile(&root_fiber, &root_element_tree, false);

                // Update view_roots mapping for any new nested views
                self.map_view_roots_from_element(&root_fiber, &root_element_tree, &mut Vec::new());

                // Cache descriptor payloads
                report.views_rendered = 1;
                self.cache_fiber_payloads(&root_fiber, &mut root_element_tree, cx);
            }

            self.expand_view_fibers(root_fiber, &mut report, cx);

            // Check dirty flags after view expansion
            if self.fiber.tree.get(&root_fiber).is_some() {
                let dirty = self.fiber.tree.dirty_flags(&root_fiber);
                report.needs_layout = dirty.has_layout_work();
                report.needs_paint = dirty.needs_paint();
            }

            // Track fibers removed during view expansion
            report.fibers_removed = self.fiber.tree.removed_fibers_count();

            return (report, root_fiber);
        }

        // Branch 3: Full reconciliation (first render, refreshing, or viewport changed)
        report.structure_changed = true;

        let root_view = self.root.as_ref().unwrap().clone();
        cx.entities.push_access_scope();
        cx.entities.record_access(root_view.entity_id());
        let mut root_element_tree = self.with_rendered_view(root_view.entity_id(), |window| {
            root_view.render_element(window, cx)
        });
        let root_accessed_entities = cx.entities.pop_access_scope();

        self.hydrate_view_children(&mut root_element_tree);

        // Get or create root fiber.
        //
        // Prefer the existing `FiberTree.root` over `view_roots` because the window root view is
        // rendered directly (not as an `AnyView` element), so the `view_roots` map is not always
        // populated for it. Using `FiberTree.root` ensures state (e.g. scroll offsets) survives
        // full refreshes triggered by transient UI events like mouse down/up.
        let fibers_before = self.fiber.tree.fibers.len();
        let root_fiber = self
            .fiber
            .tree
            .root
            .filter(|fiber_id| self.fiber.tree.get(fiber_id).is_some())
            .or_else(|| {
                self.fiber
                    .tree
                    .get_view_root(root_view.entity_id())
                    .filter(|fiber_id| self.fiber.tree.get(fiber_id).is_some())
            })
            .unwrap_or_else(|| self.fiber.tree.create_fiber_for(&root_element_tree));
        let root_fiber = self.fiber.tree.ensure_root(&root_element_tree, root_fiber);

        // Slow path: disable bailout to force full reconciliation with fresh constraints
        self.fiber
            .tree
            .reconcile(&root_fiber, &root_element_tree, false);
        self.fiber.tree.view_roots.clear();
        self.map_view_roots_from_element(&root_fiber, &root_element_tree, &mut Vec::new());
        self.fiber
            .tree
            .set_view_root(root_view.entity_id(), root_fiber);

        // Don't set view_data for root fiber - the root view is handled specially
        // via direct rendering (root_view.render_element()) and shouldn't go through
        // expand_view_fibers to avoid double-rendering
        // if let Some(fiber) = self.fiber.tree.get_mut(&root_fiber) {
        //     fiber.view_data = Some(ViewData::new(root_view.clone()));
        // }

        self.record_pending_view_accesses(&root_fiber, root_accessed_entities);

        // Update last viewport size if it changed
        if viewport_changed {
            // Mark the root fiber as needing layout so the layout pass can track which fibers
            // changed bounds under the new viewport constraints. This enables correct
            // SIZE_CHANGED/POSITION_CHANGED propagation and prevents stale prepaint/paint replay
            // (e.g. cached line layouts) across resizes.
            //
            // Also mark the root as needing paint so that we do a full-tree repaint on the first
            // frame under a new viewport. This avoids replaying cached primitives that may depend
            // on viewport-relative state (content masks, pixel alignment, platform surfaces) when
            // the drawable size changes.
            self.fiber
                .tree
                .mark_dirty(&root_fiber, DirtyFlags::NEEDS_LAYOUT | DirtyFlags::NEEDS_PAINT);
            self.layout_engine.last_layout_viewport_size = Some(root_size);
        }

        // Cache descriptor payloads in fibers for iterative layout/paint
        // Count the root view as rendered
        report.views_rendered = 1;
        self.cache_fiber_payloads(&root_fiber, &mut root_element_tree, cx);
        self.expand_view_fibers(root_fiber, &mut report, cx);

        // Compute report stats
        let fibers_after = self.fiber.tree.fibers.len();
        if fibers_after > fibers_before {
            report.fibers_created = fibers_after - fibers_before;
        }
        report.fibers_removed = self.fiber.tree.removed_fibers_count();

        // Check dirty flags
        if self.fiber.tree.get(&root_fiber).is_some() {
            report.needs_layout =
                viewport_changed
                    || self
                        .fiber
                        .tree
                        .dirty_flags(&root_fiber)
                        .has_layout_work();
            report.needs_paint = self
                .fiber
                .tree
                .dirty_flags(&root_fiber)
                .needs_paint();
        } else {
            report.needs_layout = viewport_changed;
        }

        (report, root_fiber)
    }

        let frame_start = std::time::Instant::now();
        self.pending_view_accesses.clear();
        self.fiber.frame_number = self.fiber.frame_number.wrapping_add(1);
        self.fiber.tree.begin_frame(self.fiber.frame_number);
        #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
        {
            self.frame_diagnostics = FrameDiagnostics {
                frame_number: self.fiber.frame_number,
                structure_epoch: self.fiber.tree.structure_epoch,
                ..Default::default()
            };
        }
        // Phase 1: Reconcile (single entry point for all structure changes and node updates)
        // reconcile_frame sets DrawPhase::Reconcile internally
        let reconcile_start = std::time::Instant::now();
        let (reconcile_report, root_fiber) = self.reconcile_frame(root_size, cx);
        #[cfg(any(test, feature = "test-support"))]
        if reconcile_report.structure_changed {
            self.next_frame.debug_bounds.clear();
        }
        let reconcile_time = reconcile_start.elapsed();

        // Phase 2: Intrinsic sizing
        self.invalidator.set_phase(DrawPhase::Layout);
        let sizing_start = std::time::Instant::now();
        let any_size_changed = self.compute_intrinsic_sizes(root_fiber, cx);
        let sizing_time = sizing_start.elapsed();

        let dirty_islands = self.fiber.tree.collect_dirty_layout_islands();

        // Layout is needed if:
        // - Any island has explicit layout dirtiness (position/style/structure), or
        // - Intrinsic sizing determined any intrinsic sizes actually changed, or
        // - Reconciliation reported layout work (viewport/etc).
        let needs_layout = reconcile_report.needs_layout || any_size_changed || !dirty_islands.is_empty();

        // Phase 3: Layout (if needed)
        let layout_start = std::time::Instant::now();
        let layout_fibers = if needs_layout {
            self.invalidator.set_phase(DrawPhase::Layout);
            self.compute_layout_islands(root_fiber, root_size.into(), &dirty_islands, cx)
        } else {
            0
        };

        if needs_layout {
            self.finalize_dirty_flags();
        }
        let layout_time = layout_start.elapsed();

        // Phase 4: Prepaint
        let prepaint_start = std::time::Instant::now();
        self.invalidator.set_phase(DrawPhase::Prepaint);
        self.with_absolute_element_offset(Point::default(), |window| {
            context::PrepaintCx::new(window).prepaint_fiber_tree(root_fiber, cx)
        });

        let mut render_layer_elements = self.prepaint_render_layers(root_size, cx);

        let mut deferred_draws = {
            let mut prepaint_cx = context::PrepaintCx::new(self);
            prepaint_cx.collect_deferred_draw_keys()
        };
        deferred_draws.sort_by_key(|draw| (draw.priority, draw.sequence));
        {
            let mut prepaint_cx = context::PrepaintCx::new(self);
            prepaint_cx.prepaint_deferred_draws(&deferred_draws, cx);
        // Clear active overlay from previous frame.
        self.active_overlay = None;

        // Prepaint overlays in priority order: prompt > drag > tooltip.
        // Only one can be active at a time. All use fiber-backed rendering.
        let has_overlay = self.prepaint_prompt(root_size, cx)
            || self.prepaint_active_drag(cx)
            || self.prepaint_tooltip(cx);
        self.mouse_hit_test = self.next_frame.hit_test(self, self.mouse_position);
        let prepaint_time = prepaint_start.elapsed();

        // Phase 5: Paint
        let paint_start = std::time::Instant::now();
        self.fiber.tree.ensure_preorder_indices();
        context::PaintCx::new(self).paint_fiber_tree(root_fiber, cx);

        self.paint_render_layers(&mut render_layer_elements, cx);
        {
            let mut paint_cx = context::PaintCx::new(self);
            paint_cx.paint_deferred_draws(&deferred_draws, cx);
        }
        if has_overlay {
            // Paint fiber-backed overlay (prompt, drag, or tooltip).
            self.paint_overlay(cx);
        let paint_time = paint_start.elapsed();

        // Phase 6: Cleanup
        // Clear work flags and properly recompute HAS_DIRTY_DESCENDANT.
        // This ensures fibers are in a clean state for the next frame's caching decisions.
        let cleanup_start = std::time::Instant::now();
        self.fiber.tree.end_of_frame_cleanup();
        let cleanup_time = cleanup_start.elapsed();
        let total_time = frame_start.elapsed();

        #[cfg(any(debug_assertions, feature = "diagnostics", feature = "test-support"))]
        {
            self.frame_diagnostics.layout_fibers = layout_fibers;
            self.frame_diagnostics.reconcile_time = reconcile_time;
            self.frame_diagnostics.intrinsic_sizing_time = sizing_time;
            self.frame_diagnostics.layout_time = layout_time;
            self.frame_diagnostics.prepaint_time = prepaint_time;
            self.frame_diagnostics.paint_time = paint_time;
            self.frame_diagnostics.cleanup_time = cleanup_time;
            self.frame_diagnostics.total_time = total_time;
        }
        #[cfg(not(any(debug_assertions, feature = "diagnostics", feature = "test-support")))]
        let _ = layout_fibers;

    /// Phase 2: Compute intrinsic sizes for dirty elements.
    ///
    /// Returns true if any fiber's computed intrinsic size changed.
    fn compute_intrinsic_sizes(&mut self, root_fiber: GlobalElementId, cx: &mut App) -> bool {
        self.fiber.tree.rebuild_layout_islands_if_needed();
        let dirty_sizing_islands = self.fiber.tree.collect_dirty_sizing_islands();
        if dirty_sizing_islands.is_empty() {
            return false;
        }

        let rem_size = self.rem_size();
        let scale_factor = self.scale_factor();

        #[derive(Clone, Copy)]
        struct StackState {
            text_style_len: usize,
            image_cache_len: usize,
            rendered_entity_len: usize,
        }

        impl StackState {
            fn capture(window: &Window) -> Self {
                Self {
                    text_style_len: window.text_style_stack.len(),
                    image_cache_len: window.image_cache_stack.len(),
                    rendered_entity_len: window.rendered_entity_stack.len(),
                }
            }

            fn restore(self, window: &mut Window) {
                window.text_style_stack.truncate(self.text_style_len);
                window.image_cache_stack.truncate(self.image_cache_len);
                window
                    .rendered_entity_stack
                    .truncate(self.rendered_entity_len);
            }
        }

        struct Frame {
            fiber_id: GlobalElementId,
            dirty: DirtyFlags,
            stack_state: StackState,
            node_frame: Option<crate::LayoutFrame>,
        }

        let mut any_changed = false;

        for island_root in dirty_sizing_islands {
            if self.fiber.tree.get(&island_root).is_none() {
            }

            let mut stack: Vec<(GlobalElementId, bool)> = vec![(island_root, true)];
            let mut frame_stack: Vec<Frame> = Vec::new();

            while let Some((fiber_id, entering)) = stack.pop() {
                if entering {
                    let Some(_fiber) = self.fiber.tree.get(&fiber_id) else {
                        continue;
                    };

                    let dirty = self.fiber.tree.dirty_flags(&fiber_id);
                    if !dirty.has_sizing_work() {
                        continue;
                    }

                    let stack_state = StackState::capture(self);
                    let mut node_frame: Option<crate::LayoutFrame> = None;

                    if self.fiber.tree.render_nodes.get(fiber_id.into()).is_some() {
                        let mut render_node = self.fiber.tree.render_nodes.remove(fiber_id.into());
                        if let Some(ref mut node) = render_node {
                            let mut layout_ctx = crate::LayoutCtx {
                                fiber_id,
                                rem_size,
                                scale_factor,
                                window: self,
                                cx,
                            };
                            node_frame = Some(node.layout_begin(&mut layout_ctx));
                        }
                        if let Some(node) = render_node {
                            self.fiber.tree.render_nodes.insert(fiber_id.into(), node);
                        }
                    }

                    frame_stack.push(Frame {
                        fiber_id,
                        dirty,
                        stack_state,
                        node_frame,
                    });

                    stack.push((fiber_id, false));

                    let children: SmallVec<[GlobalElementId; 8]> =
                        self.fiber.tree.children(&fiber_id).collect();
                    for child_id in children.into_iter().rev() {
                        if self.fiber.tree.outer_island_root_for(child_id) == island_root {
                            let child_dirty = self.fiber.tree.dirty_flags(&child_id);
                            if child_dirty.has_sizing_work() {
                                stack.push((child_id, true));
                            }
                        }
                    }
                } else {
                    let Some(frame) = frame_stack.pop() else {
                        continue;
                    };

                    debug_assert_eq!(frame.fiber_id, fiber_id);

                    if frame.dirty.needs_sizing() {
                        if self.compute_fiber_intrinsic_size(fiber_id, rem_size, scale_factor, cx) {
                            any_changed = true;
                            self.fiber.tree.mark_intrinsic_size_changed(&fiber_id);
                        }
                    }

                    self.fiber.tree.clear_sizing_flags(&fiber_id);

                    if let Some(node_frame) = frame.node_frame {
                        let mut render_node = self.fiber.tree.render_nodes.remove(fiber_id.into());
                        if let Some(ref mut node) = render_node {
                            let mut layout_ctx = crate::LayoutCtx {
                                fiber_id,
                                rem_size,
                                scale_factor,
                                window: self,
                                cx,
                            };
                            node.layout_end(&mut layout_ctx, node_frame);
                        }
                        if let Some(node) = render_node {
                            self.fiber.tree.render_nodes.insert(fiber_id.into(), node);
                        }
                    }

                    frame.stack_state.restore(self);
                }
            }
        }

        let _ = root_fiber;
        any_changed
    }

    fn compute_layout_islands(
        &mut self,
        root_fiber: GlobalElementId,
        root_space: Size<AvailableSpace>,
        dirty_islands: &collections::FxHashSet<GlobalElementId>,
        cx: &mut App,
    ) -> usize {
        self.fiber.tree.rebuild_layout_islands_if_needed();

        // Update taffy styles and run layout for dirty islands only.
        self.layout_engine.fibers_layout_changed.clear();
        let island_roots: Vec<GlobalElementId> = self.fiber.tree.layout_island_roots().to_vec();

        let mut layout_calls = 0;
        for island_root in island_roots {
            if !dirty_islands.contains(&island_root) {
                continue;
            }

            let available_space = if island_root == root_fiber {
                root_space
            } else if let Some(bounds) = self.fiber.tree.bounds.get(island_root.into()).copied() {
                Size {
                    width: AvailableSpace::Definite(bounds.size.width),
                    height: AvailableSpace::Definite(bounds.size.height),
                }
            } else {
                // Fallback for first layout of detached roots. Use min-size constraints.
                AvailableSpace::min_size()

            TaffyLayoutEngine::setup_taffy_from_fibers(self, island_root, cx);
            layout_calls += self.compute_layout_for_fiber(island_root, available_space, cx);
        }

        layout_calls
    }

    fn compute_fiber_intrinsic_size(
        &mut self,
        fiber_id: GlobalElementId,
        rem_size: Pixels,
        scale_factor: f32,
        cx: &mut App,
    ) -> bool {
        let slot_key: DefaultKey = fiber_id.into();

        let mut render_node = self.fiber.tree.render_nodes.remove(slot_key);
        if render_node.as_ref().is_some_and(|node| !node.uses_intrinsic_sizing_cache()) {
            if let Some(layout_state) = self.fiber.tree.layout_state.get_mut(slot_key) {
                layout_state.intrinsic_size = None;
            }
            if let Some(node) = render_node {
                self.fiber.tree.render_nodes.insert(slot_key, node);
            }
            return false;
        }

        let result = if let Some(ref mut node) = render_node {
            let mut ctx = crate::SizingCtx {
                fiber_id,
                window: self,
                cx,
                rem_size,
                scale_factor,
            };
            Some(node.compute_intrinsic_size(&mut ctx))
        } else {
            None
        };

        if let Some(node) = render_node {
            self.fiber.tree.render_nodes.insert(slot_key, node);
        }

        let Some(result) = result else {
            return false;
        };

        let cached = self.fiber.tree.get_intrinsic_size(&fiber_id);
        let changed = cached.map(|c| c.size != result.size).unwrap_or(true);

        self.fiber
            .tree
            .set_intrinsic_size(&fiber_id, result.size);

        changed
    }

    /// Paint the active fiber-backed overlay (tooltip, prompt, or drag).
    fn paint_overlay(&mut self, cx: &mut App) {
        let Some(overlay) = self.active_overlay else {
            return;
        };

        self.with_rendered_view(overlay.view_id, |window| {
            let mut paint_cx = context::PaintCx::new(window);
            paint_cx.with_absolute_element_offset(overlay.offset, |window| {
                window
                    .fibers()
                    .paint_fiber_tree_internal(overlay.fiber_id, cx, true)
            });
        });
    }

    fn prepaint_tooltip(&mut self, cx: &mut App) -> bool {
        let tooltip_requests = self.fibers().collect_tooltip_requests();
        for tooltip_request in tooltip_requests.into_iter().rev() {
            let current_view = self.current_view();
            // Get or create the tooltip fiber root.
            let fiber_id = if let Some(existing) = self.fiber.tooltip_overlay_root {
                existing
            } else {
                let new_root = self.fiber.tree.create_placeholder_fiber();
                self.fiber.tooltip_overlay_root = Some(new_root);
                new_root
            };

            // Expand wrapper elements BEFORE reconciliation.
            element.expand_wrappers(self, cx);

            // Reconcile the tooltip element into the fiber.
            self.fiber.tree.reconcile(&fiber_id, &element, true);

            // Install retained nodes.
            self.fibers()
                .cache_fiber_payloads_overlay(&fiber_id, &mut element, cx);

            // Layout the tooltip using min-size constraints.
            crate::taffy::TaffyLayoutEngine::setup_taffy_from_fibers(self, fiber_id, cx);
            self.compute_layout_for_fiber(fiber_id, AvailableSpace::min_size(), cx);

            // Get the computed size from the fiber bounds.
            let tooltip_size = self
                .fiber
                .tree
                .bounds
                .get(fiber_id.into())
                .map(|b| b.size)
                .unwrap_or_default();

            // Position the tooltip.
            // Check visibility.
            // Prepaint the fiber tree at the computed offset.
            self.with_rendered_view(current_view, |window| {
                let mut prepaint_cx = context::PrepaintCx::new(window);
                prepaint_cx.with_absolute_element_offset(tooltip_bounds.origin, |window| {
                    window
                        .fibers()
                        .prepaint_fiber_tree_internal(fiber_id, cx, true)
                });
            // Store state for painting.
            self.active_overlay = Some(ActiveOverlay {
                fiber_id,
                offset: tooltip_bounds.origin,
                view_id: current_view,
            });
            return true;
        false
    /// Prepaint the prompt overlay using the fiber-backed pipeline.
    /// Returns true if a prompt was prepainted.
    fn prepaint_prompt(&mut self, root_size: Size<Pixels>, cx: &mut App) -> bool {
        let Some(prompt) = self.prompt.take() else {
            return false;
        };
        let mut element = prompt.view.any_view().into_any();
        let current_view = self.current_view();
        // Get or create the prompt fiber root.
        let fiber_id = if let Some(existing) = self.fiber.prompt_overlay_root {
            existing
        } else {
            let new_root = self.fiber.tree.create_placeholder_fiber();
            self.fiber.prompt_overlay_root = Some(new_root);
            new_root
        };
        // Expand wrapper elements BEFORE reconciliation.
        element.expand_wrappers(self, cx);
        // Reconcile the prompt element into the fiber.
        self.fiber.tree.reconcile(&fiber_id, &element, true);
        // Install retained nodes.
        self.fibers()
            .cache_fiber_payloads_overlay(&fiber_id, &mut element, cx);
        // Layout the prompt using root size constraints.
        crate::taffy::TaffyLayoutEngine::setup_taffy_from_fibers(self, fiber_id, cx);
        self.compute_layout_for_fiber(fiber_id, root_size.into(), cx);
        // Prepaint the fiber tree at the origin.
        self.with_rendered_view(current_view, |window| {
            let mut prepaint_cx = context::PrepaintCx::new(window);
            prepaint_cx.with_absolute_element_offset(Point::default(), |window| {
                window
                    .fibers()
                    .prepaint_fiber_tree_internal(fiber_id, cx, true)
            });
        });
        // Store state for painting.
        self.active_overlay = Some(ActiveOverlay {
            fiber_id,
            offset: Point::default(),
            view_id: current_view,
        });
        // Restore the prompt.
        self.prompt = Some(prompt);
        true
    /// Prepaint the active drag overlay using the fiber-backed pipeline.
    /// Returns true if a drag was prepainted.
    fn prepaint_active_drag(&mut self, cx: &mut App) -> bool {
        let Some(active_drag) = cx.active_drag.take() else {
            return false;
        };
        let mut element = active_drag.view.clone().into_any();
        let offset = self.mouse_position() - active_drag.cursor_offset;
        let current_view = self.current_view();

        // Get or create the drag fiber root.
        let fiber_id = if let Some(existing) = self.fiber.drag_overlay_root {
            existing
        } else {
            let new_root = self.fiber.tree.create_placeholder_fiber();
            self.fiber.drag_overlay_root = Some(new_root);
            new_root
        };

        // Expand wrapper elements BEFORE reconciliation.
        element.expand_wrappers(self, cx);

        // Reconcile the drag element into the fiber.
        self.fiber.tree.reconcile(&fiber_id, &element, true);

        // Install retained nodes.
        self.fibers()
            .cache_fiber_payloads_overlay(&fiber_id, &mut element, cx);

        // Layout the drag using min-size constraints.
        crate::taffy::TaffyLayoutEngine::setup_taffy_from_fibers(self, fiber_id, cx);
        self.compute_layout_for_fiber(fiber_id, AvailableSpace::min_size(), cx);

        // Prepaint the fiber tree at the computed offset.
        self.with_rendered_view(current_view, |window| {
            let mut prepaint_cx = context::PrepaintCx::new(window);
            prepaint_cx.with_absolute_element_offset(offset, |window| {
                window
                    .fibers()
                    .prepaint_fiber_tree_internal(fiber_id, cx, true)
            });
        });

        // Store state for painting.
        self.active_overlay = Some(ActiveOverlay {
            fiber_id,
            offset,
            view_id: current_view,
        });

        // Restore the active drag.
        cx.active_drag = Some(active_drag);
        true
        self.invalidator.debug_assert_prepaint_or_paint();
        self.fibers().set_cursor_style(style, hitbox);
        self.with_fiber_cx(|fiber| fiber.set_window_cursor_style(style));
        self.invalidator.debug_assert_layout_or_prepaint();
        self.with_fiber_cx(|fiber| fiber.set_tooltip(tooltip))
        self.invalidator.debug_assert_prepaint_or_paint();
            // Transform mask to world coordinates if inside a transform context
            let world_mask = self.transform_mask_to_world(mask);
            let intersected = world_mask.intersect(&self.content_mask());
            self.content_mask_stack.push(intersected);
    /// scrolling. This method should only be called during element drawing.
        self.invalidator.debug_assert_prepaint_or_paint();
    /// element drawing.
        self.invalidator.debug_assert_prepaint_or_paint();
        let current = self.element_offset();
        let delta = offset - current;
        self.transform_stack.push_offset(delta);
        self.transform_stack.pop_offset(delta);
    pub(crate) fn push_unculled_scene(&mut self) {
        self.scene_culling_disabled_depth = self.scene_culling_disabled_depth.saturating_add(1);
    }

    pub(crate) fn pop_unculled_scene(&mut self) {
        self.scene_culling_disabled_depth = self.scene_culling_disabled_depth.saturating_sub(1);
    }

    pub(crate) fn should_cull_scene_primitives(&self) -> bool {
        self.scene_culling_disabled_depth == 0
    }

    /// Executes the given closure with an additional element opacity multiplier.
    ///
    /// This is used to implement inherited opacity for custom elements that paint directly
    /// via window APIs.
    ///
    /// This method should only be called during the prepaint or paint phase of element drawing.
    pub fn with_element_opacity<R>(
        self.invalidator.debug_assert_prepaint_or_paint();
        self.invalidator.debug_assert_layout_or_prepaint();
        self.fibers().transact(f)
        self.invalidator.debug_assert_layout_or_prepaint();
        self.invalidator.debug_assert_layout_or_prepaint();
                        let _ = cx.update(|window, cx| {
                            window.request_redraw();
    /// Obtain the current element offset. This method should only be called during element drawing.
        self.invalidator.debug_assert_prepaint_or_paint();
        self.transform_stack.local_offset()
        self.invalidator.debug_assert_prepaint_or_paint();
        self.invalidator.debug_assert_prepaint_or_paint();
    /// Transform a content mask from local coordinates to world coordinates.
    /// If we're inside a scroll transform context, the mask bounds need to be
    /// transformed so they match the coordinate space used by the shader for
    /// clipping comparisons.
    #[inline]
    pub(crate) fn transform_mask_to_world(
        &self,
        mask: ContentMask<Pixels>,
    ) -> ContentMask<Pixels> {
        let transform_id = self.transform_stack.current();
        if transform_id.is_root() {
            return mask;
        }

        let scale_factor = self.scale_factor();
        let world_transform = self.segment_pool.transforms.get_world_no_cache(transform_id);

        // Convert origin to ScaledPixels, apply transform, convert back to Pixels
        let origin_scaled = Point::new(
            ScaledPixels(mask.bounds.origin.x.0 * scale_factor),
            ScaledPixels(mask.bounds.origin.y.0 * scale_factor),
        );
        let world_origin = world_transform.apply(origin_scaled);

        // Scale the size by the transform's scale factor
        let world_size = Size {
            width: Pixels(mask.bounds.size.width.0 * world_transform.scale),
            height: Pixels(mask.bounds.size.height.0 * world_transform.scale),
        };

        ContentMask {
            bounds: Bounds {
                origin: Point::new(
                    Pixels(world_origin.x.0 / scale_factor),
                    Pixels(world_origin.y.0 / scale_factor),
                ),
                size: world_size,
            },
        }
    }

        _element_id: impl Into<ElementId>,
        f(self)
    /// Updates or initializes state for the given element id, stored on the fiber as long as it
    /// persists across frames. The state returned by the closure will be stored and reused the next
    /// time this fiber is drawn. This method should only be called as part of element drawing.
    ///
        // Allow layout phase for legacy elements that use element state during request_layout
        self.invalidator.debug_assert_layout_or_prepaint_or_paint();
        self.with_element_state_inner(global_id, f)
    }
    pub(crate) fn with_element_state_in_event<S, R>(
        &mut self,
        global_id: &GlobalElementId,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: 'static,
    {
        self.with_element_state_inner(global_id, f)
    }
    fn with_element_state_inner<S, R>(
        &mut self,
        global_id: &GlobalElementId,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: 'static,
    {
        let type_id = TypeId::of::<S>();
        let slot_key: DefaultKey = (*global_id).into();
        let mut state_map = self
            .fiber
            .tree
            .remove(slot_key)
            .unwrap_or_default();

        let result = if let Some(any) = state_map.remove(&type_id) {
            state_map.insert(
                type_id,
            state_map.insert(
                type_id,
        };

        self.fiber.tree.element_states.insert(slot_key, state_map);
        result
    }

    pub(crate) fn with_input_handler_mut<R>(
        &mut self,
        fiber_id: GlobalElementId,
        cx: &mut App,
        f: impl FnOnce(&mut dyn InputHandler, &mut Window, &mut App) -> R,
    ) -> Option<R> {
        let slot_key: DefaultKey = fiber_id.into();
        let mut handler = self.fiber.tree.input_handlers.remove(slot_key)?;
        let result = f(handler.as_mut(), self, cx);
        self.fiber.tree.input_handlers.insert(slot_key, handler);
        Some(result)
        // Allow layout phase for legacy elements that use element state during request_layout
        self.invalidator.debug_assert_layout_or_prepaint_or_paint();
            self.with_element_state(global_id, |state, window| {
                let (result, state) = f(Some(state), window);
        self.fibers().with_tab_group(index, f)
    }

    /// Begins a tab group scope. Must be paired with `end_tab_group`.
    /// This is useful for retained node implementations where children are painted
    /// between begin and end calls.
    pub fn begin_tab_group(&mut self, index: isize) {
        self.invalidator.debug_assert_paint();
        self.fiber.rendered_tab_stops.begin_group(index);
    }

    /// Ends a tab group scope started by `begin_tab_group`.
    pub fn end_tab_group(&mut self) {
        self.invalidator.debug_assert_paint();
        self.fiber.rendered_tab_stops.end_group();
    }

    /// Creates a fiber for a dynamically rendered element.
    /// This is used by virtualized lists and other elements that create children dynamically.
    /// Returns the global element ID that can be used with `with_element_context`.
    pub fn create_element_fiber(&mut self, element: &AnyElement) -> GlobalElementId {
        self.fiber.tree.create_fiber_for(element)
    }

    /// Checks if a fiber exists for the given element ID.
    /// This is useful for virtualized lists to check if an item's fiber is still valid.
    pub fn element_fiber_exists(&self, id: &GlobalElementId) -> bool {
        self.fiber.tree.get(id).is_some()
    }

    /// Removes a fiber for a dynamically rendered element.
    /// This should be called when a dynamic element is no longer needed.
    pub fn remove_element_fiber(&mut self, id: &GlobalElementId) {
        self.fiber.tree.remove(id);
    }

    /// Executes the given closure within the context of a specific element fiber.
    /// This sets up the element ID stack so that child elements are properly associated
    /// with the parent fiber.
    pub fn with_element_context<R>(
        &mut self,
        fiber_id: GlobalElementId,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.push_fiber_id(fiber_id);
        let result = f(self);
        self.pop_fiber_id();
        result
    }

    /// Measures an element's size using the fiber-backed layout pipeline.
    ///
    /// This is the preferred way to measure elements for sizing probes (e.g., virtualized
    /// lists measuring item heights). Unlike `AnyElement::layout_as_root`, this goes through
    /// the retained fiber/node pipeline, ensuring that RenderNode::measure is used for
    /// leaf sizing and that layout context (text style, image cache) is properly inherited.
    ///
    /// The measurement is performed in a temporary fiber subtree that:
    /// - Does NOT affect focus state (focusable_fibers is not modified)
    /// - Does NOT register view roots
    /// - Does NOT run prepaint or paint
    /// - Is automatically cleaned up after measurement
    ///
    /// # Limitations
    ///
    /// If the element tree contains `VKey::View` elements, this falls back to the legacy
    /// `layout_as_root` pipeline. Views require special handling (rendering their content,
    /// managing view_roots) that isn't yet implemented for measurement mode. This is fine
    /// for typical sizing probes which don't contain views.
    ///
    /// Returns the computed size of the element.
    pub(crate) fn measure_element_via_fibers(
        &mut self,
        element: &mut AnyElement,
        available_space: Size<AvailableSpace>,
        cx: &mut App,
    ) -> Size<Pixels> {
        // Measurement currently cannot safely traverse across view boundaries, because
        // reconciliation has special behavior for `VKey::View` that can reuse existing
        // view fibers via `tree.view_roots`. Fall back to the legacy `layout_as_root`
        // pipeline in that case.
        //
        // This keeps measurement isolated until overlays are fully fiber-backed.
        let contains_view_key = {
            let mut stack: Vec<&AnyElement> = vec![element];
            let mut found = false;
            while let Some(current) = stack.pop() {
                if matches!(current.key(), crate::VKey::View(_)) {
                    found = true;
                    break;
                }
                stack.extend(current.children().iter());
            }
            found
        };
        if contains_view_key {
            return element.layout_as_root(available_space, self, cx);
        }

        // Create a temporary measurement root fiber.
        //
        // Use a placeholder fiber so we don't accidentally participate in keyed
        // view-root bookkeeping (`tree.view_roots`) during cleanup.
        let measure_root = self.fiber.tree.create_placeholder_fiber();

        // Expand wrapper elements BEFORE reconciliation.
        element.expand_wrappers(self, cx);

        // Reconcile the element subtree into the measurement root
        self.fiber
            .tree
            .reconcile_wrapper(&measure_root, element, false);

        // Save the current structure epoch so transient measurement fibers don't
        // force incremental collections (mouse listeners, tab stops, segment order)
        // to rebuild for the main rendered tree.
        let saved_structure_epoch = self.fiber.tree.structure_epoch;

        // Install render nodes using measurement-safe variant (no focus/view mutations)
        self.fibers()
            .cache_fiber_payloads_measurement(&measure_root, element, cx);

        // Scope layout engine state so we don't clobber the main frame's state
        let saved_fibers_layout_changed =
            std::mem::take(&mut self.layout_engine.fibers_layout_changed);
        let saved_pending_measure_calls =
            std::mem::take(&mut self.layout_engine.pending_measure_calls);

        // Setup taffy styles from fibers (calls RenderNode::layout_begin/end)
        TaffyLayoutEngine::setup_taffy_from_fibers(self, measure_root, cx);

        // Compute layout
        self.compute_layout_for_fiber(measure_root, available_space, cx);

        // Read the computed size
        let layout_id = TaffyLayoutEngine::layout_id(&measure_root);
        let bounds = self.with_layout_engine(|layout_engine, window| {
            layout_engine.layout_bounds(window, layout_id)
        });

        // Restore layout engine state
        self.layout_engine.fibers_layout_changed = saved_fibers_layout_changed;
        self.layout_engine.pending_measure_calls = saved_pending_measure_calls;

        // Clean up the temporary measurement subtree
        self.fiber.tree.remove(&measure_root);

        // Restore structure epoch to avoid perturbing the main tree's incremental
        // ordering/caching mechanisms.
        self.fiber.tree.structure_epoch = saved_structure_epoch;

        bounds.size
    }

    /// Creates a fiber for a dynamically-created element in a legacy layout context.
    ///
    /// When a legacy element (like PopoverMenu) creates children dynamically during
    /// `request_layout`, those children may be fiber-only elements (like Div). This
    /// method creates a fiber for such elements so they can participate in layout.
    ///
    /// Returns the layout ID (which is the fiber's GlobalElementId) that can be used
    /// by taffy to establish the layout hierarchy.
    ///
    /// Panics if called outside of a legacy layout context (i.e., when
    /// `fiber.legacy_layout_parent` is None).
    pub(crate) fn layout_element_in_legacy_context(
        &mut self,
        element: &mut AnyElement,
        cx: &mut App,
    ) -> LayoutId {
        let parent_fiber_id = self
            .fiber
            .legacy_layout_parent
            .expect("layout_element_in_legacy_context called outside legacy layout context");

        // Generate a unique fiber ID for this child.
        // We use the parent's ID plus a counter to create a stable child ID.
        let child_index = self.fiber.legacy_layout_child_counter;
        self.fiber.legacy_layout_child_counter += 1;

        // Create a child fiber ID using the parent and index.
        // We need a unique ID - use the parent fiber's namespace with a child suffix.
        let child_fiber_id = self.fiber.tree.create_child_fiber(parent_fiber_id, child_index);

        // Expand wrapper elements BEFORE reconciliation.
        element.expand_wrappers(self, cx);

        // Reconcile the element into the child fiber.
        self.fiber.tree.reconcile(&child_fiber_id, element, false);

        // Install render nodes.
        self.fibers()
            .cache_fiber_payloads_overlay(&child_fiber_id, element, cx);

        // The layout ID is just the fiber ID.
        TaffyLayoutEngine::layout_id(&child_fiber_id)
    }

    /// Draws an element using the fiber-backed rendering pipeline.
    ///
    /// This is similar to `measure_element_via_fibers` but also performs prepaint and paint.
    /// It supports views (VKey::View) by using the full reconciliation path with view expansion.
    ///
    /// Used by test utilities to render elements through the retained node pipeline.
    ///
    /// Returns the computed bounds of the rendered element.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn draw_element_via_fibers(
        &mut self,
        element: &mut AnyElement,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut App,
    ) -> Bounds<Pixels> {
        // Phase 1: Reconcile
        // Set the phase for reconciliation (required by debug assertions in cache_fiber_payloads and expand_view_fibers)
        self.invalidator.set_phase(DrawPhase::Reconcile);

        // Create a temporary root fiber for this draw call.
        let draw_root = self.fiber.tree.create_placeholder_fiber();

        // Reconcile the element subtree into the draw root
        self.fiber
            .tree
            .reconcile_wrapper(&draw_root, element, false);

        // Save the current structure epoch so transient draw fibers don't
        // force incremental collections to rebuild for the main rendered tree.
        let saved_structure_epoch = self.fiber.tree.structure_epoch;

        // Install render nodes. This registers view roots so expand_view_fibers can find them.
        let mut report = ReconcileReport::default();
        self.fibers().cache_fiber_payloads(&draw_root, element, cx);
        self.expand_view_fibers(draw_root, &mut report, cx);

        // Phase 2: Layout
        // Scope layout engine state so we don't clobber the main frame's state
        let saved_fibers_layout_changed =
            std::mem::take(&mut self.layout_engine.fibers_layout_changed);
        let saved_pending_measure_calls =
            std::mem::take(&mut self.layout_engine.pending_measure_calls);

        // Setup taffy styles from fibers (calls RenderNode::layout_begin/end)
        self.invalidator.set_phase(DrawPhase::Layout);
        TaffyLayoutEngine::setup_taffy_from_fibers(self, draw_root, cx);

        // Compute layout
        self.compute_layout_for_fiber(draw_root, available_space, cx);

        // Read the computed bounds
        let layout_id = TaffyLayoutEngine::layout_id(&draw_root);
        let bounds = self.with_layout_engine(|layout_engine, window| {
            layout_engine.layout_bounds(window, layout_id)
        });

        // Prepaint at the specified origin
        self.invalidator.set_phase(DrawPhase::Prepaint);
        self.with_absolute_element_offset(origin, |window| {
            context::PrepaintCx::new(window).prepaint_fiber_tree(draw_root, cx)
        });

        // Ensure preorder indices are set before paint
        self.fiber.tree.ensure_preorder_indices();

        // Paint
        self.invalidator.set_phase(DrawPhase::Paint);
        context::PaintCx::new(self).paint_fiber_tree(draw_root, cx);

        // Snapshot hitboxes
        self.snapshot_hitboxes_into_rendered_frame();

        // Reset phase
        self.invalidator.set_phase(DrawPhase::None);

        // Restore layout engine state
        self.layout_engine.fibers_layout_changed = saved_fibers_layout_changed;
        self.layout_engine.pending_measure_calls = saved_pending_measure_calls;

        // Clean up the temporary draw subtree
        self.fiber.tree.remove(&draw_root);

        // Restore structure epoch to avoid perturbing the main tree's incremental
        // ordering/caching mechanisms.
        self.fiber.tree.structure_epoch = saved_structure_epoch;

        // Return bounds offset by origin
        Bounds {
            origin,
            size: bounds.size,
    /// Registers a focus handle as a tab stop for the current frame.
    ///
    /// This method should only be called during the paint phase of element drawing.
    pub fn register_tab_stop(&mut self, focus_handle: &FocusHandle, tab_index: isize) {
        self.invalidator.debug_assert_paint();
        self.with_fiber_cx(|fiber| fiber.register_tab_stop(focus_handle, tab_index));
    }

    #[track_caller]
        self.invalidator.debug_assert_layout_or_prepaint();
        let callsite = core::panic::Location::caller();
        self.with_fiber_cx(|fiber| fiber.defer_draw(element, absolute_offset, priority, callsite));
    /// for performance reasons. Bounds are used only to skip creating empty layers.
        let content_mask = self.content_mask().scale(scale_factor);
        let local_bounds = bounds.scale(scale_factor);
        let world_transform = self
            .segment_pool
            .transforms
            .get_world_no_cache(self.transform_stack.current());
        let world_bounds = Bounds {
            origin: world_transform.apply(local_bounds.origin),
            size: Size {
                width: ScaledPixels(local_bounds.size.width.0 * world_transform.scale),
                height: ScaledPixels(local_bounds.size.height.0 * world_transform.scale),
            },
        };

        let clipped_bounds = world_bounds.intersect(&content_mask.bounds);
        let pushed = !clipped_bounds.is_empty();
        if pushed {
            self.next_frame.scene.push_layer(&mut self.segment_pool);
        if pushed {
            self.next_frame.scene.pop_layer(&mut self.segment_pool);
    ) {
        self.paint_shadows_with_transform(
            bounds,
            corner_radii,
            shadows,
            TransformationMatrix::unit(),
        );
    }

    /// Paint one or more drop shadows with an explicit visual transform.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_shadows_with_transform(
        &mut self,
        bounds: Bounds<Pixels>,
        corner_radii: Corners<Pixels>,
        shadows: &[BoxShadow],
        transform: TransformationMatrix,
        let transform_index = self.transform_stack.current().as_u32();
        let transform = self.scale_transform_for_scene(transform);
        let cull = self.should_cull_scene_primitives();
            self.next_frame.scene.insert_primitive(
                &mut self.segment_pool,
                (
                    Shadow {
                        order: 0,
                        blur_radius: shadow.blur_radius.scale(scale_factor),
                        transform_index,
                        pad: 0,
                        bounds: shadow_bounds.scale(scale_factor),
                        content_mask: content_mask.scale(scale_factor),
                        corner_radii: corner_radii.scale(scale_factor),
                        color: shadow.color.opacity(opacity),
                    },
                    transform,
                ),
                cull,
            );
        self.paint_quad_with_transform(quad, TransformationMatrix::unit());
    }

    /// Paint one or more quads with an explicit visual transform.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_quad_with_transform(&mut self, quad: PaintQuad, transform: TransformationMatrix) {
        let transform_index = self.transform_stack.current().as_u32();
        let transform = self.scale_transform_for_scene(transform);
        let cull = self.should_cull_scene_primitives();

        self.next_frame.scene.insert_primitive(
            &mut self.segment_pool,
            (
                Quad {
                    order: 0,
                    transform_index,
                    pad: 0,
                    bounds: quad.bounds.scale(scale_factor),
                    content_mask: content_mask.scale(scale_factor),
                    background: quad.background.opacity(opacity),
                    border_color: quad.border_color.opacity(opacity),
                    corner_radii: quad.corner_radii.scale(scale_factor),
                    border_widths: quad.border_widths.scale(scale_factor),
                    border_style: quad.border_style,
                },
                transform,
            ),
            cull,
        );
        path.transform_index = self.transform_stack.current().as_u32();
        let cull = self.should_cull_scene_primitives();
            .insert_primitive(
                &mut self.segment_pool,
                path.scale(scale_factor),
                cull,
            );
    ) {
        self.paint_underline_with_transform(origin, width, style, TransformationMatrix::unit());
    }

    /// Paint an underline with an explicit visual transform.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_underline_with_transform(
        &mut self,
        origin: Point<Pixels>,
        width: Pixels,
        style: &UnderlineStyle,
        transform: TransformationMatrix,
        let transform_index = self.transform_stack.current().as_u32();
        let transform = self.scale_transform_for_scene(transform);
        let cull = self.should_cull_scene_primitives();
        self.next_frame.scene.insert_primitive(
            &mut self.segment_pool,
            (
                Underline {
                    order: 0,
                    transform_index,
                    bounds: bounds.scale(scale_factor),
                    content_mask: content_mask.scale(scale_factor),
                    color: style.color.unwrap_or_default().opacity(element_opacity),
                    thickness: style.thickness.scale(scale_factor),
                    wavy: if style.wavy { 1 } else { 0 },
                },
                transform,
            ),
            cull,
        );
    ) {
        self.paint_strikethrough_with_transform(origin, width, style, TransformationMatrix::unit());
    }

    /// Paint a strikethrough with an explicit visual transform.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_strikethrough_with_transform(
        &mut self,
        origin: Point<Pixels>,
        width: Pixels,
        style: &StrikethroughStyle,
        transform: TransformationMatrix,
        let transform_index = self.transform_stack.current().as_u32();
        let transform = self.scale_transform_for_scene(transform);
        let cull = self.should_cull_scene_primitives();
        self.next_frame.scene.insert_primitive(
            &mut self.segment_pool,
            (
                Underline {
                    order: 0,
                    transform_index,
                    bounds: bounds.scale(scale_factor),
                    content_mask: content_mask.scale(scale_factor),
                    thickness: style.thickness.scale(scale_factor),
                    color: style.color.unwrap_or_default().opacity(opacity),
                    wavy: 0,
                },
                transform,
            ),
            cull,
        );
    ) -> Result<()> {
        self.paint_glyph_with_transform(
            origin,
            font_id,
            glyph_id,
            font_size,
            color,
            TransformationMatrix::unit(),
        )
    }

    /// Paints a monochrome glyph with an explicit visual transform.
    pub fn paint_glyph_with_transform(
        &mut self,
        origin: Point<Pixels>,
        font_id: FontId,
        glyph_id: GlyphId,
        font_size: Pixels,
        color: Hsla,
        transform: TransformationMatrix,
        let transform_index = self.transform_stack.current().as_u32();
        let transform = self.scale_transform_for_scene(transform);
            let cull = self.should_cull_scene_primitives();
                self.next_frame.scene.insert_primitive(
                    &mut self.segment_pool,
                    SubpixelSprite {
                        order: 0,
                        transform_index,
                        bounds,
                        content_mask,
                        color: color.opacity(element_opacity),
                        tile,
                        transformation: TransformationMatrix::unit(),
                    },
                    cull,
                );
                self.next_frame.scene.insert_primitive(
                    &mut self.segment_pool,
                    MonochromeSprite {
                        order: 0,
                        transform_index,
                        bounds,
                        content_mask,
                        color: color.opacity(element_opacity),
                        tile,
                        transformation: transform,
                    },
                    cull,
                );
    ) -> Result<()> {
        self.paint_emoji_with_transform(
            origin,
            font_id,
            glyph_id,
            font_size,
            TransformationMatrix::unit(),
        )
    }

    /// Paints an emoji glyph with an explicit visual transform.
    pub fn paint_emoji_with_transform(
        &mut self,
        origin: Point<Pixels>,
        font_id: FontId,
        glyph_id: GlyphId,
        font_size: Pixels,
        transform: TransformationMatrix,
        let transform_index = self.transform_stack.current().as_u32();
        let transform = self.scale_transform_for_scene(transform);
            let cull = self.should_cull_scene_primitives();

            self.next_frame.scene.insert_primitive(
                &mut self.segment_pool,
                (
                    PolychromeSprite {
                        order: 0,
                        transform_index,
                        grayscale: false,
                        bounds,
                        corner_radii: Default::default(),
                        content_mask,
                        tile,
                        opacity,
                    },
                    transform,
                ),
                cull,
            );
        let transform_index = self.transform_stack.current().as_u32();
        let cull = self.should_cull_scene_primitives();
        self.next_frame.scene.insert_primitive(
            &mut self.segment_pool,
            MonochromeSprite {
                order: 0,
                transform_index,
                bounds: svg_bounds
                    .map_origin(|origin| origin.round())
                    .map_size(|size| size.ceil()),
                content_mask,
                color: color.opacity(element_opacity),
                tile,
                transformation,
            },
            cull,
        );
    ) -> Result<()> {
        self.paint_image_with_transform(
            bounds,
            corner_radii,
            data,
            frame_index,
            grayscale,
            TransformationMatrix::unit(),
        )
    }

    /// Paint an image with an explicit visual transform.
    pub fn paint_image_with_transform(
        &mut self,
        bounds: Bounds<Pixels>,
        corner_radii: Corners<Pixels>,
        data: Arc<RenderImage>,
        frame_index: usize,
        grayscale: bool,
        transform: TransformationMatrix,
        let transform_index = self.transform_stack.current().as_u32();
        let transform = self.scale_transform_for_scene(transform);
        let cull = self.should_cull_scene_primitives();

        self.next_frame.scene.insert_primitive(
            &mut self.segment_pool,
            (
                PolychromeSprite {
                    order: 0,
                    transform_index,
                    grayscale,
                    bounds: bounds
                        .map_origin(|origin| origin.floor())
                        .map_size(|size| size.ceil()),
                    content_mask,
                    corner_radii,
                    tile,
                    opacity,
                },
                transform,
            ),
            cull,
        );
    fn scale_transform_for_scene(&self, transform: TransformationMatrix) -> TransformationMatrix {
        if transform.is_unit() {
            return transform;
        }
        let scale_factor = self.scale_factor();
        let mut scaled = transform;
        scaled.translation[0] *= scale_factor;
        scaled.translation[1] *= scale_factor;
        scaled
    }

        let transform_index = self.transform_stack.current().as_u32();
        let cull = self.should_cull_scene_primitives();
        self.next_frame.scene.insert_primitive(
            &mut self.segment_pool,
            PaintSurface {
                order: 0,
                transform_index,
                bounds,
                content_mask,
                image_buffer,
            },
            cull,
        );
    pub(crate) fn push_fiber_id(&mut self, id: GlobalElementId) {
        self.fibers().push_fiber_id(id);
    }

    pub(crate) fn pop_fiber_id(&mut self) {
        self.fibers().pop_fiber_id();
    }

    pub(crate) fn current_fiber_id(&self) -> Option<GlobalElementId> {
        self.fibers_ref().current_fiber_id()
    }

    pub(crate) fn with_element_id_stack<R>(
        &mut self,
        fiber_id: &GlobalElementId,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.push_fiber_id(*fiber_id);
        let result = f(self);
        self.pop_fiber_id();
        result
    }

    /// Ensure a fiber exists for the current fiber scope, creating one if necessary.
    pub(crate) fn ensure_fiber_for_current_id(&mut self) -> GlobalElementId {
        self.fibers().ensure_fiber_for_current_id()
    }

    /// Register a view's entity ID with the current fiber.
    /// This enables view-level dirty tracking.
    pub(crate) fn register_view_fiber(&mut self, entity_id: EntityId) -> GlobalElementId {
        self.fibers().register_view_fiber(entity_id)
    }

    /// Ensure a pending fiber exists for a view root outside of render traversal.
    pub(crate) fn ensure_view_root_fiber(&mut self, view_id: EntityId) -> GlobalElementId {
        self.fibers().ensure_view_root_fiber(view_id)
    }

    pub(crate) fn record_pending_view_accesses(
        &mut self,
        fiber_id: &GlobalElementId,
        accessed: FxHashSet<EntityId>,
    ) {
        if accessed.is_empty() {
            return;
        }
        self.pending_view_accesses
            .entry(*fiber_id)
            .or_insert_with(FxHashSet::default)
            .extend(accessed);
    }

    pub(crate) fn take_pending_view_accesses(
        &mut self,
        fiber_id: &GlobalElementId,
    ) -> Option<FxHashSet<EntityId>> {
        self.pending_view_accesses.remove(&fiber_id)
    }

    pub(crate) fn hydrate_view_children(&self, element: &mut AnyElement) {
        // Recursively hydrate children
        for child in element.children_mut() {
            self.hydrate_view_children(child);
        }
    }

    fn map_view_roots_from_element(
        &mut self,
        fiber_id: &GlobalElementId,
        element: &AnyElement,
        new_view_fibers: &mut Vec<GlobalElementId>,
    ) {
        self.fibers()
            .map_view_roots_from_element(fiber_id, element, new_view_fibers);
    }

    pub(crate) fn should_render_view_fiber(&self, fiber_id: &GlobalElementId) -> bool {
        self.fibers_ref().should_render_view_fiber(fiber_id)
    }

    fn expand_view_fibers(
        &mut self,
        root_fiber: GlobalElementId,
        report: &mut ReconcileReport,
        cx: &mut App,
    ) {
        self.fibers().expand_view_fibers(root_fiber, report, cx);
    }

    pub(crate) fn fiber_view_id(
        &self,
        fiber_id: &GlobalElementId,
        fiber: &crate::Fiber,
    ) -> Option<EntityId> {
        match &fiber.key {
            crate::VKey::View(view_id) => Some(*view_id),
            _ => self
                .fiber
                .tree
                .view_state
                .get((*fiber_id).into())
                .and_then(|state| state.view_data.as_ref())
                .map(|view| view.view.entity_id()),
        }
    }

    pub(crate) fn paint_svg_paths(
        &mut self,
        bounds: Bounds<Pixels>,
        svg_path: Option<&SharedString>,
        svg_external_path: Option<&SharedString>,
        svg_transformation: Option<crate::Transformation>,
        color: Hsla,
        cx: &mut App,
    ) {
        let transformation = svg_transformation
            .map(|transformation| transformation.into_matrix(bounds.center(), self.scale_factor()))
            .unwrap_or_default();

        if let Some(path) = svg_path {
            self.paint_svg(bounds, path.clone(), None, transformation, color, cx)
                .log_err();
            return;
        }

        let Some(path) = svg_external_path else {
            return;
        };
        let Some(bytes) = self
            .use_asset::<crate::elements::SvgAsset>(path, cx)
            .and_then(|asset| asset.log_err())
        else {
            return;
        };

        self.paint_svg(
            bounds,
            path.clone(),
            Some(&bytes),
            transformation,
            color,
            cx,
        )
        .log_err();
    }

    fn cache_fiber_payloads(
        &mut self,
        fiber_id: &GlobalElementId,
        element: &mut AnyElement,
        cx: &mut App,
    ) {
        self.fibers().cache_fiber_payloads(fiber_id, element, cx);
    }

    pub(crate) fn remove_rendered_tab_stops_for_fiber(
        &mut self,
        owner_id: GlobalElementId,
        focus_ids: impl IntoIterator<Item = FocusId>,
    ) {
        for focus_id in focus_ids {
            self.fiber
                .rendered_tab_stops
                .remove_if_owned_by(&focus_id, owner_id);
        }
    }

    pub(crate) fn layout_bounds_cached(
        &self,
        global_id: &GlobalElementId,
        scale_factor: f32,
        cache: &mut FxHashMap<GlobalElementId, Bounds<Pixels>>,
    ) -> Bounds<Pixels> {
        crate::taffy::layout_bounds(self, global_id, scale_factor, cache)
    }

    /// Temporarily take the layout engine out of self, use it via the closure, and restore it.
    /// This pattern is needed because layout engine methods require both `&mut self` on the engine
    /// and `&mut Window`.
    fn with_layout_engine<R>(
        &mut self,
        f: impl FnOnce(&mut TaffyLayoutEngine, &mut Self) -> R,
    ) -> R {
        let mut layout_engine =
            std::mem::replace(&mut self.layout_engine, TaffyLayoutEngine::new());
        let result = f(&mut layout_engine, self);
        self.layout_engine = layout_engine;
        result
    }

    /// Add a node to the layout tree for the current frame, using the current fiber scope.
    /// This method is called during [`Element::request_layout`] and enables any element
    /// to participate in layout. Children are implicit in the fiber tree.
    /// This method should only be called as part of the request_layout or prepaint phase
    /// of element drawing.
        let fiber_id = self.ensure_fiber_for_current_id();
        self.invalidator.debug_assert_layout_or_prepaint();
        let children: Vec<_> = children.into_iter().collect();
        self.with_layout_engine(|layout_engine, window| {
            layout_engine.request_layout(window, fiber_id, style, children, cx)
        })
    /// For better performance with caching, use `request_measured_layout_cached` instead.
        let fiber_id = self.ensure_fiber_for_current_id();
        self.invalidator.debug_assert_layout_or_prepaint();
        self.with_layout_engine(|layout_engine, window| {
            layout_engine.request_measured_layout(window, fiber_id, style, measure)
        })
    }
    /// Request a measured layout with caching support.
    ///
    /// This method should only be called as part of the request_layout or prepaint phase of element drawing.
    pub fn request_measured_layout_cached<F>(
        &mut self,
        style: Style,
        content_hash: u64,
        measure: F,
    ) -> LayoutId
    where
        F: Fn(Size<Option<Pixels>>, Size<AvailableSpace>, &mut Window, &mut App) -> Size<Pixels>
            + 'static,
    {
        let fiber_id = self.ensure_fiber_for_current_id();
        self.invalidator.debug_assert_layout_or_prepaint();
        self.with_layout_engine(|layout_engine, window| {
            layout_engine.request_measured_layout_cached(
                window,
                fiber_id,
                style,
                content_hash,
                measure,
            )
        })
    ) -> usize {
        self.invalidator.debug_assert_layout_or_prepaint();
        self.with_layout_engine(|layout_engine, window| {
            layout_engine.compute_layout(window, layout_id, available_space, cx)
        })
    }
    pub(crate) fn compute_layout_for_fiber(
        &mut self,
        fiber_id: GlobalElementId,
        available_space: Size<AvailableSpace>,
        cx: &mut App,
    ) -> usize {
        self.invalidator.debug_assert_layout_or_prepaint();
        self.with_layout_engine(|layout_engine, window| {
            layout_engine.compute_layout_for_fiber(window, fiber_id, available_space, cx)
        })
        self.invalidator.debug_assert_layout_or_prepaint();
        self.with_layout_engine(|layout_engine, window| {
            layout_engine.layout_bounds(window, layout_id)
        })
        self.invalidator.debug_assert_layout_or_prepaint();
        self.with_fiber_cx(|fiber| fiber.insert_hitbox(bounds, behavior))
        self.with_fiber_cx(|fiber| fiber.set_key_context(context));
    pub fn set_focus_handle(&mut self, focus_handle: &FocusHandle, cx: &App) {
        self.invalidator.debug_assert_layout_or_prepaint();
        let _ = cx;
        self.with_fiber_cx(|fiber| fiber.set_focus_handle(focus_handle));
    }

    /// Sets the focus handle for a specific element identified by its global element id.
    /// This is used when the element's focus handle needs to be registered with a specific fiber.
    ///
    /// This method should only be called as part of the prepaint phase of element drawing.
    pub fn set_focus_handle_for(&mut self, global_id: GlobalElementId, focus_handle: &FocusHandle) {
        self.invalidator.debug_assert_layout_or_prepaint();
        self.with_fiber_cx_for(global_id, |fiber| fiber.set_focus_handle(focus_handle));
    }

    /// Registers a focus handle as a tab stop for the current element.
    /// The focus handle should already have its tab stop configuration set.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn register_tab_stop_handle(&mut self, focus_handle: &FocusHandle) {
        self.invalidator.debug_assert_paint();
        self.with_fiber_cx(|fiber| fiber.register_tab_stop_handle(focus_handle));
        self.invalidator.debug_assert_layout_or_prepaint();
        let _ = view_id;
        self.invalidator.debug_assert_layout_or_prepaint_or_paint();
        if let Some(id) = self.rendered_entity_stack.last().copied() {
            return id;
        }

        // Render layers and other out-of-tree rendering can legitimately run
        // outside a view's `Element` implementation. When that happens, fall
        // back to the window root view so subsystems like image caching can
        // still associate work with a view.
        self.root
            .as_ref()
            .map(|root| root.entity_id())
            .expect("Window::current_view called with no rendered view and no root view")
    /// Execute `f` while treating `id` as the "current view".
    ///
    /// This is primarily intended for render layers and other out-of-tree
    /// rendering that needs a stable view identity for subsystems like image
    /// caching and view-local state.
    pub fn with_rendered_view<R>(&mut self, id: EntityId, f: impl FnOnce(&mut Self) -> R) -> R {
        self.with_fiber_cx(|fiber| fiber.handle_input(focus_handle, input_handler, cx));
        self.invalidator.debug_assert_prepaint_or_paint();
        self.with_fiber_cx(|fiber| fiber.on_mouse_event(listener));
        self.with_fiber_cx(|fiber| fiber.on_key_event(listener))
        self.with_fiber_cx(|fiber| fiber.on_modifiers_changed(listener))
    pub(crate) fn reset_cursor_style(&mut self, cx: &mut App) {
            let style = self.fibers().cursor_style_for_frame().unwrap_or(CursorStyle::Arrow);
        let prefer_character_input = keystroke.key_char.is_some();
                prefer_character_input,
            input_handler.dispatch_input(&input);
                self.mouse_position = mouse_exited.position;
        let _event_phase = context::EventPhaseScope::new(self.invalidator.clone());
        context::EventCx::new(self).dispatch_mouse_event(event, cx);
        let event_phase = context::EventPhaseScope::new(self.invalidator.clone());
            self.draw(cx);
            event_phase.reassert();
        let mut node_id = self.focus_node_id_in_rendered_frame(self.focus);
        let mut context_stack = self.fibers_ref().context_stack_for_node(node_id);
        if context_stack.is_empty() && self.invalidator.not_drawing() {
            self.draw(cx);
            node_id = self.focus_node_id_in_rendered_frame(self.focus);
            context_stack = self.fibers_ref().context_stack_for_node(node_id);
            event_phase.reassert();
        }
            self.finish_dispatch_key_event(event, node_id, context_stack, cx);
        self.dispatch_keystroke_interceptors(event, context_stack.clone(), cx);
            self.finish_dispatch_key_event(event, node_id, context_stack, cx);
        let had_pending = !currently_pending.keystrokes.is_empty();
        let pending_keystrokes = currently_pending.keystrokes.clone();
        let match_result =
            self.key_dispatch
                .dispatch_key(pending_keystrokes, keystroke, &context_stack);
                    let accepts = input_handler.accepts_text_input();
                        let context_stack = window.fibers_ref().context_stack_for_node(node_id);
                            .key_dispatch
                            .flush_dispatch(currently_pending.keystrokes, &context_stack);
        let prefer_character_input =
            event
                .downcast_ref::<KeyDownEvent>()
                .is_some_and(|key_down_event| {
                    key_down_event.prefer_character_input
                        && key_down_event.keystroke.key_char.is_some()
                });
                        let accepts = input_handler.accepts_text_input();
        if (skip_bindings || prefer_character_input) && had_pending {
            self.pending_input = Some(currently_pending);
            self.pending_input_changed(cx);
            cx.propagate_event = false;
            return;
        }

        self.finish_dispatch_key_event(event, node_id, match_result.context_stack, cx);
        node_id: GlobalElementId,
        self.dispatch_key_down_up_event(event, node_id, cx);
        self.dispatch_modifiers_changed_event(event, node_id, cx);
        node_id: GlobalElementId,
        self.fibers().dispatch_key_listeners(event, node_id, cx);
        node_id: GlobalElementId,
        self.fibers().dispatch_modifiers_listeners(event, node_id, cx);
            self.dispatch_key_down_up_event(&event, node_id, cx);
                input_handler.dispatch_input(&input);
    fn focus_node_id_in_rendered_frame(&self, focus_id: Option<FocusId>) -> GlobalElementId {
        self.fibers_ref().focus_node_id_in_rendered_frame(focus_id)
        node_id: GlobalElementId,
        if !self.fibers().dispatch_window_action_listeners(action, node_id, cx) {
            return;
        self.on_next_frame(|window, _cx| {
                if let Some(bounds) = input_handler.selected_bounds() {
        self.fibers_ref().context_stack_for_node(node_id)
        let mut actions = Vec::<Box<dyn Action>>::new();
        let mut current = Some(node_id);
        while let Some(fiber_id) = current {
            if let Some(effects) = self.get_fiber_effects(&fiber_id) {
                for (action_type, _) in &effects.action_listeners {
                    if let Err(ix) =
                        actions.binary_search_by_key(action_type, |a| a.as_any().type_id())
                    {
                        // Intentionally silence these errors without logging.
                        // If an action cannot be built by default, it's not available.
                        let action = cx.actions.build_action_type(action_type).ok();
                        if let Some(action) = action {
                            actions.insert(ix, action);
                        }
                    }
                }
            }
            current = self.fibers_ref().parent_for(&fiber_id);
        }
        self.key_dispatch
            .bindings_for_action(action, &self.context_stack())
        self.key_dispatch
            .highest_precedence_binding_for_action(action, &self.context_stack())
        self.key_dispatch.bindings_for_action(action, &[context])
        self.key_dispatch
            .highest_precedence_binding_for_action(action, &[context])
        self.key_dispatch
            .bindings_for_action(action, &context_stack)
        self.key_dispatch
            .highest_precedence_binding_for_action(action, &context_stack)
        self.key_dispatch
        self.fibers_ref()
            .context_stack_for_focus_handle(focus_handle)
        self.with_fiber_cx(|fiber| fiber.on_action(action_type, listener));
            self.on_action(action_type, listener);
        self.invalidator.debug_assert_layout_or_prepaint();
        self.invalidator.debug_assert_prepaint_or_paint();
                && let Some(hitbox) = self.resolve_hitbox(&hitbox_id)
    pub(crate) fn handle_inspector_mouse_event(&mut self, event: &dyn Any, cx: &mut App) {


#[cfg(test)]
mod tests;