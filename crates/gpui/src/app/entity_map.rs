    access_scopes: RefCell<Vec<FxHashSet<EntityId>>>,
            access_scopes: RefCell::new(Vec::new()),
        self.record_access(slot.entity_id);
        self.record_access(pointer.entity_id);
        self.record_access(entity.entity_id);
    pub(crate) fn record_access(&self, entity_id: EntityId) {
        self.accessed_entities.borrow_mut().insert(entity_id);
        if let Some(scope) = self.access_scopes.borrow_mut().last_mut() {
            scope.insert(entity_id);
        }
    }

    pub(crate) fn push_access_scope(&self) {
        self.access_scopes.borrow_mut().push(FxHashSet::default());
    }

    pub(crate) fn pop_access_scope(&self) -> FxHashSet<EntityId> {
        let mut scopes = self.access_scopes.borrow_mut();
        let scope = scopes.pop().unwrap_or_default();
        if let Some(parent) = scopes.last_mut() {
            parent.extend(scope.iter().copied());
        }
        scope
    }

    pub(crate) fn extend_accessed(&self, entities: &FxHashSet<EntityId>) {
        if entities.is_empty() {
            return;
        }
        if let Some(scope) = self.access_scopes.borrow_mut().last_mut() {
            scope.extend(entities.iter().copied());
        }
    pub(crate) fn clear_accessed(&mut self) {
        self.access_scopes.borrow_mut().clear();
        let mut access_scopes = self.access_scopes.borrow_mut();
                for scope in access_scopes.iter_mut() {
                    scope.remove(&entity_id);
                }