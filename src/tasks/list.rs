use super::{enums::ExecutionOrder, Task};
use std::collections::HashMap;
use uuid::Uuid;

enum TaskListError {
    TaskOutOfBoundsError,
    TaskNotFoundError,
}

enum Direction {
    None,
    Up,
    Down,
    Value(isize),
}

/// A list of tasks, acting as a container for the tasks held.
struct TaskList {
    /// The name of the task list
    name: String,
    /// The primary linear collection of tasks
    tasks: Vec<Task>,
    /// An index of all the tasks
    title_index: HashMap<String, usize>,
    /// An index of all the tasks at a specific depth
    depth_index: HashMap<i8, Vec<usize>>,
}

impl TaskList {
    /// Create a new TaskList
    pub fn new(name: &str) -> TaskList {
        TaskList {
            name: name.to_string(),
            tasks: vec![],
            title_index: HashMap::new(),
            depth_index: HashMap::new(),
        }
    }

    /// Gets the number of tasks
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Get the relative position given a certain direction
    fn get_rel_pos(dir: &Direction) -> isize {
        match *dir {
            Direction::None => 0,
            Direction::Up => -1,
            Direction::Down => 1,
            Direction::Value(value) => value,
        }
    }

    /// Get the absolute position given a certain direction from a provided position
    fn get_pos(&self, pos: usize, dir: &Direction) -> usize {
        let pos: isize = pos.try_into().unwrap();
        let rel_pos = TaskList::get_rel_pos(dir);
        let neighbour_pos = (pos + rel_pos).try_into().unwrap_or(0);

        if neighbour_pos >= self.tasks.len() {
            // Return the end of the array if the bounds are outside the array
            return self.tasks.len();
        }

        neighbour_pos
    }

    /// Gets a task from the list at a specific position.
    fn get_task(&self, pos: usize) -> Result<&Task, TaskListError> {
        if pos >= self.len() {
            return Err(TaskListError::TaskOutOfBoundsError);
        }

        Ok(&self.tasks[pos])
    }

    /// Gets the depth of a neighbouring task relative to the provided position.
    /// Return 0 if the task list is empty and returns the same depth as the task if the task is
    /// at the end of the list.
    pub fn neighbour_depth(&self, pos: usize, dir: &Direction) -> i8 {
        if self.tasks.is_empty() {
            return 0;
        }
        let neighbour_pos = self.get_pos(pos, dir);
        self.tasks[neighbour_pos].depth
    }

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
        let depth = self.neighbour_depth(pos, &Direction::Up);
        let task = Task::new(title, depth + 1);
        let task_pos = self.get_pos(pos, &Direction::None);

        self.tasks.insert(task_pos + 1, task);
        self.rebuild_all_indices();
        self
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

    /// Rebuilds all the indices in the list
    fn rebuild_all_indices(&mut self) -> &mut Self {
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

    /// Delete a task from the task list. Deletes all the subtasks as well.
    fn delete_task(&mut self, pos: usize) -> &mut Self {
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

    fn has_subtasks(&self, pos: usize) -> bool {
        // Check that the task is not at the end of the list, and that the task below it is not at the same depth or less.
        if pos >= self.len() - 1 {
            return false;
        }
        if self.neighbour_depth(pos, &Direction::Down) <= self.tasks[pos].depth {
            return false;
        }
        true
    }

    /// Get all the start and end indices of the range of subtasks for the selected subtask.
    /// Returns the same index if the task has no subtasks.
    fn get_last_subtask_pos(&self, pos: usize) -> usize {
        // Check that the task is not at the end of the list, and that the task below it is not at the same depth or less.
        if self.has_subtasks(pos) {
            return pos;
        }

        // To get all the subtasks, look through the depth index for the same depth or less.
        // Every task between the selected task and the next task of the same depth or less is a subtask.
        if let Ok(task) = self.get_task(pos) {
            let depth = task.depth;
            // Loop through the depth index backwards
            for i in depth..=0 {
                let Some(depth_tasks) = self.depth_index.get(&i) else {
                    continue;
                };

                // Find the next task after the current selected position.
                let end = depth_tasks.iter().find(|&&x| x > pos);
                if let Some(&end) = end {
                    // If there is an index, return the end position
                    return end;
                }
            }
        }
        // If no end index is found, the entire task list below the current task is a subtask to the current task
        self.len() - 1
    }

    /// Get all the direct child subtasks of the selected subtask. Returns an empty Vec if there are no subtasks.
    fn get_direct_subtasks(&self, pos: usize) -> Vec<usize> {
        // Check that the task is not at the end of the list, and that the task below it is not at the same depth or less.
        if self.has_subtasks(pos) {
            return vec![];
        }

        // To get the direct subtasks, get the entire size of the subtask list, the look for subtasks in the
        // depth index at a depth one greater than the current subtask and within the range of the subtasks.
        let subtask_end = self.get_last_subtask_pos(pos);
        let Ok(task) = self.get_task(pos) else {
            return vec![];
        };

        let depth = task.depth;
        let Some(subtask_depth_tasks) = self.depth_index.get(&(depth + 1)) else {
            return vec![];
        };

        // Collect the subtasks that in the greater depth that are within the range of the subtasks and return them.
        let subtasks = subtask_depth_tasks
            .iter()
            .filter(|&&x| (x > pos) && (x <= subtask_end))
            .cloned()
            .collect();

        subtasks
    }

    /// Promote a task by making all of its siblings below it children to this task.
    fn promote_task(&mut self, task_id: &Uuid) -> &mut Self {
        todo!();
    }

    /// Gets the position of a subtask within it's parent tasks list.
    fn get_child_position(&self, task_id: &Uuid) -> Result<usize, TaskListError> {
        todo!();
    }

    /// Demote a task by making it a child of the task above it.
    /// If the task above it is a subtask, make it a sibling of that subtask.
    fn demote_task(&mut self, task_id: &Uuid) -> &mut Self {
        todo!();
    }

    /// Calculate the total duration of the task and its subtasks depending on their execution order.
    fn calculate_task_duration(&self, task_id: &Uuid) -> i32 {
        todo!();
        /*
        if let Some(task) = self.tasks.get(task_id) {
            // If there are no subtasks, the expected duration of the subtask is as has been currently set
            if task.subtasks.len() == 0 {
                return task.expected_duration.unwrap_or(0);
            }

            // If there is are subtasks, calculate the duration as a maximum or sum of the subtask depending on the execution order
            // This calls calculate_duration recursively
            match task.execution_order {
                // When in series, the duration is the sum of the subtasks
                ExecutionOrder::Series => task
                    .subtasks
                    .iter()
                    .map(|subtask| self.calculate_task_duration(subtask))
                    .sum::<i32>(),
                // When in parallel, the duration is the maximum of the subtasks
                ExecutionOrder::Parallel => task
                    .subtasks
                    .iter()
                    .map(|subtask| self.calculate_task_duration(subtask))
                    .max()
                    .unwrap_or(0),
            }
        } else {
            // Return 0 if the task can't be found
            return 0;
        }
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_new_root_task_at_end_increases_count() {
        let mut task_list = TaskList::new(&String::from("Task List"));
        // Test task creation
        assert_eq!(task_list.len(), 0);
        task_list.add_new_root_task_at_end(&String::from("Task 1"));
        assert_eq!(task_list.len(), 1);
        task_list.add_new_root_task_at_end(&String::from("Task 2"));
        assert_eq!(task_list.len(), 2);
    }

    #[test]
    fn add_new_task_adds_at_correct_position() {
        let mut task_list = TaskList::new(&String::from("Task List"));

        // Adds at end of list
        task_list.add_new_task(&String::from("Task 1"), 0);
        // Adds at start of list, pushing Task 1 down
        task_list.add_new_task(&String::from("Task 2"), 0);
        // Adds at middle lof list, pushing Task 1 down
        task_list.add_new_task(&String::from("Task 3"), 1);
        // Resulting order should be Task 2, Task 3, Task 1

        let mut task_string = String::new();
        for task in task_list.tasks.iter() {
            task_string.push_str(&task.title);
            task_string.push_str(";")
        }
        assert_eq!(task_string, "Task 2;Task 3;Task 1;");
    }

    #[test]
    fn add_new_subtask_adds_at_correct_position() {
        let mut task_list = TaskList::new(&String::from("Task List"));

        // Adds at end of list
        task_list.add_new_root_task_at_end(&String::from("Task 1"));
        // Adds at end of list
        task_list.add_new_root_task_at_end(&String::from("Task 2"));
        // Add subtask to Task 1
        task_list.add_new_subtask(&String::from("Task 1.1"), 0);

        let mut task_string = String::new();
        for task in task_list.tasks.iter() {
            task_string.push_str(&format!(
                "{} ({});",
                task.title.clone(),
                task.depth.to_string()
            ));
        }
        assert_eq!(task_string, "Task 1 (0);Task 1.1 (1);Task 2 (0);");
    }
}
