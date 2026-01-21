use smallvec::SmallVec;

use crate::fiber::AnchoredConfig;
use crate::geometry::IsZero;
use crate::taffy::ToTaffy;
use crate::{
    AnyElement, App, Axis, Bounds, Corner, Display, Edges, Element,
    GlobalElementId, InspectorElementId, IntoElement, ParentElement, Pixels, Point, Position, Size,
    Style, UpdateResult, VKey, Window, point, px,
};

/// An anchored element that can be used to display UI that
/// will avoid overflowing the window bounds.
pub struct Anchored {
    children: SmallVec<[AnyElement; 2]>,
    anchor_corner: Corner,
    fit_mode: AnchoredFitMode,
    anchor_position: Option<Point<Pixels>>,
    position_mode: AnchoredPositionMode,
    offset: Option<Point<Pixels>>,
}

/// anchored gives you an element that will avoid overflowing the window bounds.
/// Its children should have no margin to avoid measurement issues.
pub fn anchored() -> Anchored {
    Anchored {
        children: SmallVec::new(),
        anchor_corner: Corner::TopLeft,
        fit_mode: AnchoredFitMode::SwitchAnchor,
        anchor_position: None,
        position_mode: AnchoredPositionMode::Window,
        offset: None,
    }
}

impl Anchored {
    /// Sets which corner of the anchored element should be anchored to the current position.
    pub fn anchor(mut self, anchor: Corner) -> Self {
        self.anchor_corner = anchor;
        self
    }

    /// Sets the position in window coordinates
    /// (otherwise the location the anchored element is rendered is used)
    pub fn position(mut self, anchor: Point<Pixels>) -> Self {
        self.anchor_position = Some(anchor);
        self
    }

    /// Offset the final position by this amount.
    /// Useful when you want to anchor to an element but offset from it, such as in PopoverMenu.
    pub fn offset(mut self, offset: Point<Pixels>) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Sets the position mode for this anchored element. Local will have this
    /// interpret its [`Anchored::position`] as relative to the parent element.
    /// While Window will have it interpret the position as relative to the window.
    pub fn position_mode(mut self, mode: AnchoredPositionMode) -> Self {
        self.position_mode = mode;
        self
    }

    /// Snap to window edge instead of switching anchor corner when an overflow would occur.
    pub fn snap_to_window(mut self) -> Self {
        self.fit_mode = AnchoredFitMode::SnapToWindow;
        self
    }

    /// Snap to window edge and leave some margins.
    pub fn snap_to_window_with_margin(mut self, edges: impl Into<Edges<Pixels>>) -> Self {
        self.fit_mode = AnchoredFitMode::SnapToWindowWithMargin(edges.into());
        self
    }

    pub(crate) fn config(&self) -> AnchoredConfig {
        AnchoredConfig {
            anchor_corner: self.anchor_corner,
            fit_mode: self.fit_mode,
            anchor_position: self.anchor_position,
            position_mode: self.position_mode,
            offset: self.offset,
        }
    }
}

impl ParentElement for Anchored {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Element for Anchored {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<crate::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> (crate::LayoutId, Self::RequestLayoutState) {
        unreachable!("Anchored uses retained node path")
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        unreachable!("Anchored uses retained node path")
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        unreachable!("Anchored uses retained node path")
    }

    fn fiber_key(&self) -> VKey {
        VKey::None
    }

    fn fiber_children(&self) -> &[AnyElement] {
        &self.children
    }

    fn fiber_children_mut(&mut self) -> &mut [AnyElement] {
        &mut self.children
    }

    fn create_render_node(&mut self) -> Option<Box<dyn crate::RenderNode>> {
        Some(Box::new(crate::fiber::AnchoredNode::new(self.config())))
    }

    fn update_render_node(
        &mut self,
        node: &mut dyn crate::RenderNode,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<UpdateResult> {
        let node = node.downcast_mut::<crate::fiber::AnchoredNode>()?;
        node.config = self.config();
        Some(UpdateResult::LAYOUT_CHANGED)
    }

    fn requires_fiber_layout(&self) -> bool {
        true
    }
}

impl IntoElement for Anchored {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// Which algorithm to use when fitting the anchored element to be inside the window.
#[derive(Copy, Clone, PartialEq)]
pub enum AnchoredFitMode {
    /// Snap the anchored element to the window edge.
    SnapToWindow,
    /// Snap to window edge and leave some margins.
    SnapToWindowWithMargin(Edges<Pixels>),
    /// Switch which corner anchor this anchored element is attached to.
    SwitchAnchor,
}

/// Which algorithm to use when positioning the anchored element.
#[derive(Copy, Clone, PartialEq)]
pub enum AnchoredPositionMode {
    /// Position the anchored element relative to the window.
    Window,
    /// Position the anchored element relative to its parent.
    Local,
}

impl AnchoredPositionMode {
    fn get_position_and_bounds(
        &self,
        anchor_position: Option<Point<Pixels>>,
        anchor_corner: Corner,
        size: Size<Pixels>,
        bounds: Bounds<Pixels>,
        offset: Option<Point<Pixels>>,
    ) -> (Point<Pixels>, Bounds<Pixels>) {
        let offset = offset.unwrap_or_default();

        match self {
            AnchoredPositionMode::Window => {
                let anchor_position = anchor_position.unwrap_or(bounds.origin);
                let bounds =
                    Bounds::from_corner_and_size(anchor_corner, anchor_position + offset, size);
                (anchor_position, bounds)
            }
            AnchoredPositionMode::Local => {
                let anchor_position = anchor_position.unwrap_or_default();
                let bounds = Bounds::from_corner_and_size(
                    anchor_corner,
                    bounds.origin + anchor_position + offset,
                    size,
                );
                (anchor_position, bounds)
            }
        }
    }
}
