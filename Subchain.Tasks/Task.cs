using System.Dynamic;

namespace Subchain.Tasks
{
    /// <summary>
    /// A task.
    /// </summary>
    public class Task
    {
        #region Members
        /// <summary>
        /// The subtasks of the task stored in a HashSet.
        /// </summary>
        private List<Task> _subtasks { get; set; } = [];

        /// <summary>
        /// True if the task is on the current path, false otherwise. Is set by the UpdateNext method.
        /// </summary>
        private bool _isNext = false;

        #endregion

        #region Properties
        /// <summary>
        /// The unique identifier for the task.
        /// </summary>
        public Guid Id { get; set; }

        /// <summary>
        /// The name of the task.
        /// </summary>
        public string Name { get; set; } = "";

        /// <summary>
        /// The description of the task.
        /// </summary>
        public string Description { get; set; } = "";

        /// <summary>
        /// The status of the task.
        /// </summary>
        public TaskStatus Status { get; set; } = TaskStatus.NotStarted;

        /// <summary>
        /// The subtasks of the task. Use AddSubtask and RemoveSubtask to modify the subtasks.
        /// </summary>
        public IEnumerable<Task> Subtasks => _subtasks;

        /// <summary>
        /// The order in which the subtasks should be executed.
        /// </summary>
        public ExecutionOrder SubtaskOrder { get; set; } = ExecutionOrder.Series;

        /// <summary>
        /// The parent to the current task.
        /// </summary>
        public Task? Parent { get; set; } = null;

        /// <summary>
        /// The expected duration of the task, in minutes
        /// </summary>
        public int ExpectedDuration { get; set; }

        /// <summary>
        /// The calculated duration of the task, in minutes
        /// </summary>
        public int CalculatedDuration
        {
            get
            {
                if (_subtasks.Count == 0)
                {
                    return ExpectedDuration;
                }
                if (SubtaskOrder == ExecutionOrder.Parallel)
                {
                    return _subtasks.Max(task => task.CalculatedDuration);
                }
                else
                {
                    return _subtasks.Sum(task => task.CalculatedDuration);
                }
            }
        }


        /// <summary>
        /// True if the task is on the current path, false otherwise. Is set by the UpdateNext method.
        /// </summary>
        public bool IsNext
        {
            get
            {
                return _isNext;
            }
            set
            {
                // If the task has become active, track how long the task has been active for
                if (value == true && _isNext == false)
                {
                    ActiveDate = DateTime.Now;
                }
                // If the task has become inactive, cancel the active timer
                else if (value == false)
                {
                    ActiveDate = null;
                }

                _isNext = value;
            }
        }


        /// <summary>
        /// The buffer time for the task, in minutes. Based on the due date.
        /// </summary>
        /// TODO: Implement buffer time calculation
        public int Buffer { get; } = 0;

        /// <summary>
        /// The optional due date for the task.
        /// </summary>
        public DateOnly? DueDate { get; set; } = null;

        /// <summary>
        /// The optional start date and time for the task.
        /// </summary>
        public DateTime? StartDate { get; set; } = null;

        /// <summary>
        /// The date that the task became active.
        /// </summary>
        public DateTime? ActiveDate { get; set; } = null;

        /// <summary>
        /// The date that the task was completed.
        /// </summary>
        public DateTime? CompletionDate { get; set; } = null;

        /// <summary>
        /// The amount of time the task was active for, in minutes.
        /// </summary>
        public int ActiveTime
        {
            get
            {
                if (ActiveDate == null)
                {
                    return 0;
                }
                else
                {
                    if (CompletionDate == null)
                    {
                        return (int)(DateTime.Now - ActiveDate).Value.TotalMinutes;
                    }
                    else
                    {
                        return (int)(CompletionDate - ActiveDate).Value.TotalMinutes;
                    }
                }
            }
        }

        /// <summary>
        /// The amount of time the task took to complete from the time it was started, in minutes.
        /// </summary>
        public int? CompletionTime
        {
            get
            {
                if (CompletionDate is null || StartDate is null)
                {
                    return null;
                }
                else
                {
                    return (int)(CompletionDate - StartDate).Value.TotalMinutes;
                }
            }
        }
        #endregion

        #region Constructors

        /// <summary>
        /// Creates a new instance of the Task class.
        /// </summary>
        public Task()
        {
            Id = Guid.NewGuid();
            Status = TaskStatus.NotStarted;
        }

        /// <summary>
        /// Creates a new intance of the Task class with a name.
        /// </summary>
        /// <param name="name">Task name.</param>
        public Task(string name) : this()
        {
            Name = name;
        }

        /// <summary>
        /// Creates a new instance of the Task class with a name and description.
        /// </summary>
        /// <param name="name">The name of the task.</param>
        /// <param name="description">A description of the task.</param>
        public Task(string name, string description) : this()
        {
            Name = name;
            Description = description;
        }
        #endregion

        #region Methods

        /// <summary>
        /// Get the hash code for the task.
        /// </summary>
        /// <returns>The hash code.</returns>
        public override int GetHashCode()
        {
            return Id.GetHashCode();
        }

        /// <summary>
        /// Determines whether the task is equal to the current object based on their Ids.
        /// </summary>
        /// <param name="obj">Object for comparison.</param>
        /// <returns>True if the object is a task and has the same Id.</returns>
        public override bool Equals(object? obj)
        {
            if (obj is Task t)
            {
                return Id == t.Id;
            }
            return false;
        }

        /// <summary>
        /// Add a subtask to the current task.
        /// </summary>
        /// <param name="subtask">Subtask to add to the current task.</param>
        public void AddSubtask(Task subtask)
        {
            if (!_subtasks.Contains(subtask))
            {
                _subtasks.Add(subtask);

                // If the subtask already has a parent that is not the current task, remove it from the parent
                if (subtask.Parent != null)
                {
                    if (subtask.Parent != this)
                    {
                        subtask.Parent.RemoveSubtask(this);
                    }
                }

                subtask.Parent = this;

                // Update the subtasks
                UpdateNext();
            }
        }

        public void InsertSubtask(Task subtask, int index)
        {

        }

        public void RemoveSubtask(Task subtask)
        {
        }

        public void MoveAllSubtasks(Task target)
        {

        }

        public void MoveSubtask(Task subtask, Task target)
        {

        }


        /// <summary>
        /// Updates the subtasks for this task to have the correct parent.
        /// </summary>
        public void ReconcileParents()
        {
            if (_subtasks != null)
            {
                foreach (var subtask in _subtasks)
                {
                    subtask.Parent = this;
                    subtask.ReconcileParents();
                }
            }
        }

        /// <summary>
        /// Updates the IsNext property for the task and all subtasks.
        /// </summary>
        public void UpdateNext()
        {
            // If the task has no parent, it by default is next on the execution list as the task is the root task
            if (Parent == null)
            {
                IsNext = true;
            }
            // If the task has a parent, the updating of IsNext is done by the parent task
            if (_subtasks.Count > 0)
            {
                // If execution is parallel, all subtasks are next if the current task is next
                if (SubtaskOrder == ExecutionOrder.Parallel)
                {
                    foreach (Task t in _subtasks)
                    {
                        t.IsNext = IsNext;
                        t.UpdateNext();
                    }
                }
                // If execution is series, only the first subtask is next if the current task is next
                // All others are off the path
                else
                {
                    _subtasks.First().IsNext = IsNext;
                    _subtasks.First().UpdateNext();
                    for (int i = 1; i < _subtasks.Count; i++)
                    {
                        _subtasks.ElementAt(i).IsNext = false;
                        _subtasks.ElementAt(i).UpdateNext();
                    }
                }
            }
        }

        // TODO: Calculate the buffer of time for the tasks (i.e. when the task can be started)
        public void CalculateBuffer()
        {

        }
    }

    #endregion
}
