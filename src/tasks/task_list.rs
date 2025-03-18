use crate::tasks::task::{ExecutionOrder, Render, Task};
use std::cmp::{min, Ordering};
use std::collections::HashMap;

pub enum TaskListError {
    TaskOutOfBoundsError,
    TaskNotFoundError,
}

pub enum Direction {
    None,
    Up,
    Down,
    Value(isize),
}

/// A list of tasks, acting as a container for the tasks held.
#[derive(Debug)]
pub struct TaskList {
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
        let depth = self.neighbour_depth(pos, &Direction::None);
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

    /// Check whether a task has any subtasks
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

    /// Change the depth of a task by a given quantity.
    fn change_task_depth(&mut self, pos: usize, depth_change: i8) -> &mut Self {
        let Some(task) = self.tasks.get_mut(pos) else {
            return self;
        };

        // Make sure that the depth does not go below 0
        if task.depth > depth_change {
            task.depth += depth_change
        }

        self
    }

    /// Promote a task by making all of its siblings by subtracting 1 from the task's depth.
    /// This makes all the siblings below it children to this task.
    pub fn promote_task(&mut self, pos: usize) -> &mut Self {
        self.change_task_depth(pos, -1)
    }

    /// Demote a task by making it a child of the task above it.
    /// If the task above it is a subtask, make it a sibling of that subtask.
    fn demote_task(&mut self, pos: usize) -> &mut Self {
        self.change_task_depth(pos, 1)
    }

    /// Calculate the total duration of the task and its subtasks depending on their execution order.
    /// Returns the total duration in minutes.
    fn calculate_task_duration(&self, pos: usize) -> i32 {
        let Ok(task) = self.get_task(pos) else {
            return 0;
        };

        let subtasks = self.get_direct_subtasks(pos);
        if subtasks.is_empty() {
            return task.expected_duration.unwrap_or(0);
        }

        // Calculate the task duration recursively based on the execution order.
        match task.execution_order {
            // If the execution order is in series, the total duration is just the sum of all the subtask durations.
            ExecutionOrder::Series => subtasks
                .iter()
                .map(|&subtask| self.calculate_task_duration(subtask))
                .sum::<i32>(),
            // If the execution order is in parallel, the total duration is the maximum duration of the subtasks.
            ExecutionOrder::Parallel => subtasks
                .iter()
                .map(|&subtask| self.calculate_task_duration(subtask))
                .max()
                .unwrap_or(0),
        }
    }

    /// Prints a simple debugging string representation of the task list.
    fn print_debug(&self) -> String {
        let mut result: String = String::new();
        for task in self.tasks.iter() {
            for _ in 0..task.depth {
                result.push('>');
            }
            result.push_str(&format!("{}\r\n", task.title));
        }
        result
    }
}

/// Renders a joiner between two tasks provided their depths.
/// Doesn't render joiners longer than three characters.
fn create_joiner_from_depths(depth_above: i8, depth_below: i8) -> String {
    let mut result: String = String::new();
    let depth_diff = depth_below - depth_above;

    if !(-1..=1).contains(&depth_diff) {
        return "\r\n".to_string();
    }

    result.push_str(
        " ".repeat(min(depth_above as usize, depth_below as usize) * 2)
            .as_str(),
    );
    match depth_diff {
        1 => {
            // Create a ╰─╮ joiner
            result.push_str("╰─╮\r\n");
        }
        0 => {
            // Create a │ joiner
            result.push_str(" ".repeat((depth_above * 2) as usize).as_str());
            result.push_str("│\r\n");
        }
        -1 => {
            // Create a ╭─╯ joiner
            result.push_str(" ".repeat((depth_below * 2) as usize).as_str());
            result.push_str("╭─╯\r\n");
        }
        _ => {
            return "\r\n".to_string();
        }
    }

    result
}

impl Render for TaskList {
    /// Renders the task list to a string
    fn render(&self) -> String {
        let mut result: String = String::new();

        for (pos, task) in self.tasks.iter().enumerate() {
            // Create a joiner for every task except the first
            if pos != 0 {
                result.push_str(&create_joiner_from_depths(
                    self.neighbour_depth(pos, &Direction::Up),
                    task.depth,
                ));
            }
            // Render the task
            result.push_str(&task.render());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_task_list() -> TaskList {
        let mut task_list = TaskList::new(&String::from("Task List"));
        task_list.add_new_root_task_at_end(&String::from("Task 1"));
        task_list.add_new_root_task_at_end(&String::from("Task 2"));
        task_list.add_new_subtask(&String::from("Task 1.3"), 0);
        task_list.add_new_subtask(&String::from("Task 1.2"), 0);
        task_list.add_new_subtask(&String::from("Task 1.1"), 0);
        task_list.add_new_subtask(&String::from("Task 2.3"), 4);
        task_list.add_new_subtask(&String::from("Task 2.2"), 4);
        task_list.add_new_subtask(&String::from("Task 2.1"), 4);
        task_list.add_new_root_task_at_end(&String::from("Task 3"));
        task_list
    }

    #[test]
    fn debug_string_returns_accurate_string() {
        let expected = "Task 1\r\n>Task 1.1\r\n>Task 1.2\r\n>Task 1.3\r\nTask 2\r\n>Task 2.1\r\n>Task 2.2\r\n>Task 2.3\r\nTask 3\r\n";

        assert_eq!(setup_task_list().print_debug(), expected);
    }

    #[test]
    fn add_new_root_task_at_end_increases_count() {
        let mut task_list = setup_task_list();
        assert_eq!(task_list.len(), 9);
        task_list.add_new_root_task_at_end(&String::from("Task 4"));
        assert_eq!(task_list.len(), 10);
    }

    #[test]
    fn add_new_task_adds_at_correct_position() {
        let mut task_list = setup_task_list();

        // Adds at start of list, pushing Task 1 down
        task_list.add_new_task(&String::from("Task 0"), 0);
        // Adds at middle of list, becoming parent of Task 1.3
        task_list.add_new_task(&String::from("Task 4"), 4);

        let expected = "Task 0\r\nTask 1\r\n>Task 1.1\r\n>Task 1.2\r\n>Task 4\r\n>Task 1.3\r\nTask 2\r\n>Task 2.1\r\n>Task 2.2\r\n>Task 2.3\r\nTask 3\r\n";

        assert_eq!(task_list.print_debug(), expected);
    }

    #[test]
    fn create_joiner_at_depths_correct_joiner() {
        assert_eq!(create_joiner_from_depths(0, 1), "╰─╮\r\n");
        assert_eq!(create_joiner_from_depths(1, 0), "╭─╯\r\n");
        assert_eq!(create_joiner_from_depths(0, 0), "│\r\n");
        assert_eq!(create_joiner_from_depths(1, 2), "   ╰─╮\r\n");
        assert_eq!(create_joiner_from_depths(1, 3), "\r\n");
        assert_eq!(create_joiner_from_depths(1, 3), "\r\n");
    }
}
