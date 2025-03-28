use crate::task::ExecutionOrder;
use crate::task_list::TaskList;

impl TaskList {
    /// Rebuilds all the indices in the list
    pub(crate) fn rebuild_all_indices(&mut self) -> &mut Self {
        self.rebuild_depth_index();
        self.rebuild_title_index();
        self.rebuild_next_tasks();
        self
    }

    /// Rebuilds the depth index of the list.
    pub(crate) fn rebuild_depth_index(&mut self) -> &mut Self {
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
    pub(crate) fn rebuild_title_index(&mut self) -> &mut Self {
        self.title_index.clear();
        for (i, task) in self.tasks.iter().enumerate() {
            self.title_index.insert(task.title.clone(), i);
        }
        self
    }

    /// Rebuilds the index of tasks that are up next.
    pub(crate) fn rebuild_next_tasks(&mut self) -> &mut Self {
        self.next_tasks.clear();
        let Some(root_tasks) = self.depth_index.get(&0) else {
            return self;
        };

        for &task in root_tasks {
            let mut active_subtasks = self.get_active_subtasks(task);
            self.next_tasks.append(&mut active_subtasks);
        }

        self
    }

    /// Gets all the active subtasks of the current position
    fn get_active_subtasks(&self, pos: usize) -> Vec<usize> {
        let mut active_subtasks: Vec<usize> = vec![];
        let Ok(task) = self.get_task(pos) else {
            return active_subtasks;
        };

        active_subtasks.push(pos);

        if !self.has_subtasks(pos) {
            return active_subtasks;
        }

        // Get the direct subtasks of this task
        let subtasks = self.get_direct_subtasks(pos);
        match task.execution_order {
            // If in series, just add the subtasks of the first task to the active tasks
            ExecutionOrder::Series => {
                if let Some(first_subtask) = subtasks.first() {
                    active_subtasks.append(&mut self.get_active_subtasks(*first_subtask));
                }
            }
            // If in parallel, add all the subtasks of the first task to the active tasks
            ExecutionOrder::Parallel => {
                for subtask in subtasks {
                    active_subtasks.append(&mut self.get_active_subtasks(subtask));
                }
            }
        }
        active_subtasks
    }
}
