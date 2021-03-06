use bit_set::BitSet;

#[derive(Debug)]
pub struct TrackResource {
    pub inserted: BitSet<u32>,
    pub modified: BitSet<u32>,
    pub removed: BitSet<u32>,
    pub component_added: BitSet<u32>,
    pub component_removed: BitSet<u32>,
}

impl TrackResource {
    pub fn new() -> TrackResource {
        TrackResource {
            inserted: BitSet::new(),
            modified: BitSet::new(),
            removed: BitSet::new(),
            component_removed: BitSet::new(),
            component_added: BitSet::new(),
        }
    }

    pub fn insert(&mut self, set: usize) {
        // If previously removed/modified we don't need to know that anymore.
        self.removed.remove(set);
        self.modified.remove(set);
        self.inserted.insert(set);
    }

    pub fn remove(&mut self, set: usize) {
        // Don't need to know that it was inserted/modified if it was subsequently
        // removed.
        self.inserted.remove(set);
        self.modified.remove(set);
        self.removed.insert(set);
    }

    pub fn modify(&mut self, set: usize) {
        self.modified.insert(set);
    }

    pub fn component_add(&mut self, set: usize) {
        self.component_added.insert(set);
    }

    pub fn component_unset(&mut self, set: usize) {
        self.component_removed.insert(set);
    }

    pub fn clear(&mut self) {
        self.inserted.clear();
        self.modified.clear();
        self.removed.clear();
    }

    pub(crate) fn remove_if_any_contains(&mut self, identifier: usize) -> bool {
        if self.removed.contains(identifier) {
            self.removed.remove(identifier);
            return true;
        }
        if self.modified.contains(identifier) {
            self.modified.remove(identifier);
            return true;
        }
        if self.inserted.contains(identifier) {
            self.inserted.remove(identifier);
            return true;
        }

        if self.component_added.contains(identifier) {
            self.component_added.remove(identifier);
            return true;
        }
        if self.component_removed.contains(identifier) {
            self.component_removed.remove(identifier);
            return false;
        }

        return false;
    }
}

impl Clone for TrackResource {
    fn clone(&self) -> Self {
        TrackResource {
            inserted: self.inserted.clone(),
            removed: self.removed.clone(),
            modified: self.removed.clone(),
            component_added: self.component_added.clone(),
            component_removed: self.component_removed.clone(),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::TrackResource;

    #[test]
    fn update_insert_test() {
        let mut resource = TrackResource::new();
        resource.modified.insert(1);
        resource.removed.insert(1);

        resource.insert(1);

        assert!(!resource.removed.contains(1));
        assert!(!resource.modified.contains(1));
        assert!(resource.inserted.contains(1));
    }

    #[test]
    fn update_modified_test() {
        let mut resource = TrackResource::new();
        resource.inserted.insert(1);
        resource.removed.insert(1);

        resource.modify(1);

        assert!(resource.modified.contains(1));
    }

    #[test]
    fn update_remove_test() {
        let mut resource = TrackResource::new();
        resource.inserted.insert(1);
        resource.modified.insert(1);

        resource.remove(1);

        assert!(!resource.inserted.contains(1));
        assert!(!resource.modified.contains(1));
        assert!(resource.removed.contains(1));
    }

    #[test]
    fn clear_test() {
        let mut resource = TrackResource::new();
        resource.inserted.insert(1);
        resource.modified.insert(1);
        resource.removed.insert(1);

        resource.clear();

        assert!(!resource.inserted.contains(1));
        assert!(!resource.modified.contains(1));
        assert!(!resource.removed.contains(1));
    }
}
