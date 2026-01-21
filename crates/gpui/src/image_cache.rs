    pub(crate) fn identity_eq(&self, other: &AnyImageCache) -> bool {
        self.image_cache.entity_id() == other.image_cache.entity_id()
            && self.image_cache.entity_type() == other.image_cache.entity_type()
            && std::ptr::fn_addr_eq(self.load_fn, other.load_fn)
    }

    type RequestLayoutState = ();
            for child in &mut self.children {
                child.request_layout(window, cx);
            }
            let layout_id = window.request_layout(style, [], cx);
            (layout_id, ())