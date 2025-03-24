mod actions;
mod error;
pub mod index;

use crate::task::{ExecutionOrder, Task};
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
#[derive(Debug, Default)]
pub struct TaskList {
    /// The name of the task list
    name: String,
    /// The primary linear collection of tasks
    pub(crate) tasks: Vec<Task>,
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
    pub(crate) fn get_task(&self, pos: usize) -> Result<&Task, TaskListError> {
        if pos >= self.len() {
            return Err(TaskListError::TaskOutOfBoundsError);
        }

        Ok(&self.tasks[pos])
    }

    fn get_mut_task(&mut self, pos: usize) -> Result<&mut Task, TaskListError> {
        if pos >= self.len() {
            return Err(TaskListError::TaskOutOfBoundsError);
        }

        Ok(&mut self.tasks[pos])
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

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn setup_task_list() -> TaskList {
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
}
