use crate::task::Task;
use crate::task_list::{Direction, TaskList, TaskListError};

impl TaskList {
    /// Adds a new root task to the end of the list
    pub fn add_new_root_task_at_end(&mut self, title: &str) -> &mut Self {
        let task = Task::new(title, 0);
        self.tasks.push(task);
        self.rebuild_all_indices();
        self
    }

    /// Add a new task at a specific index
    pub fn add_new_task(&mut self, title: &str, pos: usize) -> &mut Self {
        // Look at the index above and match the depth
        let depth = self.neighbour_depth(pos, &Direction::Up);
        let task = Task::new(title, depth);
        let task_pos = self.get_pos(pos, &Direction::None);

        self.tasks.insert(task_pos, task);
        self.rebuild_all_indices();
        self
    }

    /// Add a new subtask to the task at a specific index
    pub fn add_new_subtask(&mut self, title: &str, pos: usize) -> &mut Self {
        // Look at the index above and match the depth
        let depth = self.neighbour_depth(pos, &Direction::None);
        let task = Task::new(title, depth + 1);
        let task_pos = self.get_pos(pos, &Direction::None);

        self.tasks.insert(task_pos + 1, task);
        self.rebuild_all_indices();
        self
    }

    /// Delete a task from the task list. Deletes all the subtasks as well.
    pub fn delete_task(&mut self, pos: usize) -> &mut Self {
        // Check that a task exists at the position.
        if pos >= self.len() {
            return self;
        }
        // Get the position of the last subtask, then remove all the tasks between the current task and the last subtask.
        let end = self.get_last_subtask_pos(pos);
        for i in (pos..=end).rev() {
            self.tasks.remove(i);
        }
        self
    }

    pub fn toggle_task_status(&mut self, pos: usize) {
        let Ok(task) = self.get_mut_task(pos) else {
            return;
        };

        task.toggle_status();
    }

    /// Moves a subtask up or down in the list
    pub fn move_task(&mut self, pos: usize, dir: &Direction) -> Result<&mut Self, TaskListError> {
        let swap_pos = self.get_pos(pos, dir);
        let task = self.get_task(pos)?;

        // A task can only be moved up/down if the item above/below it has an equal depth
        let neighbour_depth = self.neighbour_depth(pos, dir);
        if task.depth != neighbour_depth {
            return Ok(self);
        }

        self.tasks.swap(pos, swap_pos);
        self.rebuild_all_indices();
        Ok(self)
    }

    /// Promote a task by making all of its siblings by subtracting 1 from the task's depth.
    /// This makes all the siblings below it children to this task.
    pub fn promote_task(&mut self, pos: usize) -> &mut Self {
        self.change_task_depth(pos, -1)
    }

    /// Demote a task by making it a child of the task above it.
    /// If the task above it is a subtask, make it a sibling of that subtask.
    pub fn demote_task(&mut self, pos: usize) -> &mut Self {
        self.change_task_depth(pos, 1)
    }

    /// Change the depth of a task by a given quantity.
    fn change_task_depth(&mut self, pos: usize, depth_change: i8) -> &mut Self {
        let Some(task) = self.tasks.get_mut(pos) else {
            return self;
        };

        // Make sure that the depth does not go below 0
        if depth_change.is_negative() && task.depth < -depth_change {
            return self;
        }

        // Move the task
        task.depth += depth_change;
        self
    }
}

mod test {
    #[test]
    fn add_new_root_task_at_end_increases_count() {
        let mut task_list = crate::task_list::tests::setup_task_list();
        assert_eq!(task_list.len(), 9);
        task_list.add_new_root_task_at_end(&String::from("Task 4"));
        assert_eq!(task_list.len(), 10);
    }

    #[test]
    fn add_new_task_adds_at_correct_position() {
        let mut task_list = crate::task_list::tests::setup_task_list();

        // Adds at start of list, pushing Task 1 down
        task_list.add_new_task(&String::from("Task 0"), 0);
        // Adds at middle of list, becoming parent of Task 1.3
        task_list.add_new_task(&String::from("Task 4"), 4);

        let expected = "Task 0\r\nTask 1\r\n>Task 1.1\r\n>Task 1.2\r\n>Task 4\r\n>Task 1.3\r\nTask 2\r\n>Task 2.1\r\n>Task 2.2\r\n>Task 2.3\r\nTask 3\r\n";

        assert_eq!(task_list.print_debug(), expected);
    }
}
