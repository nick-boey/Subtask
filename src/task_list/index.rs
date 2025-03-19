use crate::task_list::TaskList;

impl TaskList {
    /// Rebuilds all the indices in the list
    pub(crate) fn rebuild_all_indices(&mut self) -> &mut Self {
        self.rebuild_depth_index();
        self.rebuild_title_index();
        self
    }

    /// Rebuilds the depth index of the list.
    fn rebuild_depth_index(&mut self) -> &mut Self {
        self.depth_index.clear();
        for (i, task) in self.tasks.iter().enumerate() {
            let Some(depth_tasks) = self.depth_index.get_mut(&task.depth) else {
                self.depth_index.insert(task.depth, vec![i]);
                continue;
            };
            depth_tasks.push(i);
        }
        self
    }

    /// Rebuilds the title index of the list.
    fn rebuild_title_index(&mut self) -> &mut Self {
        self.title_index.clear();
        for (i, task) in self.tasks.iter().enumerate() {
            self.title_index.insert(task.title.clone(), i);
        }
        self
    }
}
