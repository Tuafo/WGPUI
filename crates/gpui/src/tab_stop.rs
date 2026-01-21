use crate::{FocusHandle, FocusId, GlobalElementId};
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct TabStopOrderKey {
    pub(crate) preorder_index: u64,
    pub(crate) sequence: u32,
impl TabStopOrderKey {
    pub(crate) fn new(preorder_index: u64, sequence: u32) -> Self {
        Self {
            preorder_index,
            sequence,
/// Represents a collection of focus handles using the tab-index APIs.
#[derive(Debug)]
pub(crate) struct TabStopMap {
    current_path: TabStopPath,
    by_id: FxHashMap<FocusId, TabStopEntry>,
    order: SumTree<TabStopEntry>,
    last_structure_epoch: u64,
}

    /// Order key used to stabilize sibling ordering.
    order_key: TabStopOrderKey,
#[derive(Clone, Debug)]
struct TabStopEntry {
    node: TabStopNode,
    handle: FocusHandle,
    owner_id: GlobalElementId,
}

            .then(self.order_key.cmp(&other.order_key))
        Some(self.cmp(other))
            last_structure_epoch: u64::MAX,
    pub fn insert_with_order(
        &mut self,
        owner_id: GlobalElementId,
        focus_handle: &FocusHandle,
        order_key: TabStopOrderKey,
    ) {
        let node = TabStopNode {
            order_key,
        let entry = TabStopEntry {
            node,
            handle: focus_handle.clone(),
            owner_id,
        };
        if let Some(existing) = self.by_id.remove(&focus_handle.id) {
            self.order.remove(&existing.node, ());
        }
        self.by_id.insert(focus_handle.id, entry.clone());
        self.order.insert_or_replace(entry, ());
    }

    pub fn remove(&mut self, focus_id: &FocusId) {
        if let Some(entry) = self.by_id.remove(focus_id) {
            self.order.remove(&entry.node, ());
        }
    }

    pub(crate) fn remove_if_owned_by(&mut self, focus_id: &FocusId, owner_id: GlobalElementId) {
        let should_remove = self
            .by_id
            .get(focus_id)
            .is_some_and(|entry| entry.owner_id == owner_id);
        if should_remove {
            self.remove(focus_id);
        }
    }

    #[cfg(any(test, debug_assertions))]
    pub(crate) fn contains(&self, focus_id: &FocusId) -> bool {
        self.by_id.contains_key(focus_id)
    pub fn clear_groups(&mut self) {
    }

    pub fn rebuild_order_if_needed<F>(&mut self, structure_epoch: u64, mut order_for_owner: F)
    where
        F: FnMut(GlobalElementId) -> u64,
    {
        if self.last_structure_epoch == structure_epoch {
            return;
        }

        self.last_structure_epoch = structure_epoch;
        let mut order = SumTree::new(());
        for entry in self.by_id.values_mut() {
            entry.node.order_key.preorder_index = order_for_owner(entry.owner_id);
            order.insert_or_replace(entry.clone(), ());
        }
        self.order = order;
            if first.node.tab_stop {
                return Some(first.handle.clone());
                    .next_inner(&first.node)
                    .map(|entry| entry.handle.clone());
            Some(item.handle.clone())
    fn next_inner(&self, node: &TabStopNode) -> Option<&TabStopEntry> {
        cursor.seek(node, Bias::Left);
            && !item.node.tab_stop
            if last.node.tab_stop {
                return Some(last.handle.clone());
                    .prev_inner(&last.node)
                    .map(|entry| entry.handle.clone());
            Some(item.handle.clone())
    fn prev_inner(&self, node: &TabStopNode) -> Option<&TabStopEntry> {
        cursor.seek(node, Bias::Left);
            && !item.node.tab_stop
        let Some(entry) = self.by_id.get(focused_id) else {
        Some(&entry.node)
    use crate::tab_stop::{TabStopEntry, TabStopNode, TabStopOrderKey, TabStopPath};
        max_order_key: TabStopOrderKey,
                max_order_key: TabStopOrderKey::default(),
            self.max_order_key = summary.max_order_key;
    impl sum_tree::KeyedItem for TabStopEntry {
        type Key = TabStopNode;
            self.node.clone()
    impl sum_tree::Item for TabStopEntry {
                max_order_key: self.node.order_key,
                max_path: self.node.path.clone(),
                tab_stops: if self.node.tab_stop { 1 } else { 0 },
            self.order_key = summary.max_order_key;
                <TabStopOrderKey as Ord>::cmp(&self.order_key, &cursor_location.order_key),
    use crate::{FocusHandle, FocusId, FocusMap, GlobalElementId, TabStopMap, TabStopOrderKey};
    fn insert_with_order(
        map: &mut TabStopMap,
        owner_id: GlobalElementId,
        next_order: &mut u32,
        handle: &FocusHandle,
    ) {
        let order_key = TabStopOrderKey::new(0, *next_order);
        *next_order += 1;
        map.insert_with_order(owner_id, handle, order_key);
    }

        let owner_id = GlobalElementId::from(0u64);
        let mut next_order = 0;
            insert_with_order(&mut tab_index_map, owner_id, &mut next_order, handle);
        let owner_id = GlobalElementId::from(0u64);
        let mut next_order = 0;
        insert_with_order(
            &mut tab_index_map,
            owner_id,
            &mut next_order,
            &tab_non_stop_1,
        );
        insert_with_order(&mut tab_index_map, owner_id, &mut next_order, &tab_stop_2);
        insert_with_order(&mut tab_index_map, owner_id, &mut next_order, &tab_stop_0);
        insert_with_order(
            &mut tab_index_map,
            owner_id,
            &mut next_order,
            &tab_non_stop_0,
        );
        owner_id: GlobalElementId,
        next_order: u32,
                owner_id: GlobalElementId::from(0u64),
                next_order: 0,
            insert_with_order(
                &mut self.tab_map,
                self.owner_id,
                &mut self.next_order,
                &handle,
            );
            insert_with_order(
                &mut self.tab_map,
                self.owner_id,
                &mut self.next_order,
                &handle,
            );