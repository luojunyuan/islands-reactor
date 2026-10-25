macro_rules! islands_relation_backend_methods {
    () => {
    fn append_child(&mut self, parent: ControlId, child: ControlId) {
        let index = self
            .parent_children
            .borrow()
            .get(&parent)
            .map_or(0, Vec::len);
        self.insert_child(parent, index, child);
    }

    fn remove_child(&mut self, parent: ControlId, index: usize) {
        let visual = self.visual_index(parent, index);
        if let Some(list) = self.parent_children.borrow_mut().get_mut(&parent)
            && index < list.len()
        {
            list.remove(index);
        }
        self.visual_remove_at(parent, visual);
    }

    fn replace_child(&mut self, parent: ControlId, index: usize, new: ControlId) {
        let visual = self.visual_index(parent, index);
        if let Some(list) = self.parent_children.borrow_mut().get_mut(&parent)
            && index < list.len()
        {
            list[index] = new;
        }
        self.visual_set_at(parent, visual, new);
    }

    fn move_child(&mut self, parent: ControlId, from: usize, to: usize) {
        let child = {
            let mut parents = self.parent_children.borrow_mut();
            let Some(list) = parents.get_mut(&parent) else {
                return;
            };
            if from >= list.len() || to >= list.len() {
                return;
            }
            let child = list.remove(from);
            list.insert(to, child);
            child
        };
        self.visual_remove_at(parent, from);
        self.visual_insert_at(parent, to, child);
    }

    fn insert_child(&mut self, parent: ControlId, index: usize, child: ControlId) {
        let visual = self.visual_index(parent, index);
        self.parent_children
            .borrow_mut()
            .entry(parent)
            .or_default()
            .insert(index, child);
        self.visual_insert_at(parent, visual, child);
    }
    };
}
