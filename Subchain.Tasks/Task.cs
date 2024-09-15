using System.Text.Json;
using System.Text.Json.Serialization;

namespace Subchain.Tasks
{
    /// <summary>
    /// A task.
    /// </summary>
    public class Task
    {
        #region Members
        private readonly List<Task> _subtasks = [];
        private bool _isNext = false;
        private bool _isCritical = false;
        private readonly DateTime _creationDate = DateTime.Now;
        private TaskStatus _taskStatus = TaskStatus.NotStarted;

        private static readonly JsonSerializerOptions _jsonOptions = new()
        {
            WriteIndented = true,
            DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
        };
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
        public TaskStatus Status
        {
            get => _taskStatus;
            set
            {
                // Update the start and completion dates based on the status
                switch (value)
                {
                    case TaskStatus.NotStarted:
                        StartDate = null;
                        CompletionDate = null;
                        break;

                    case TaskStatus.InProgress:
                        if (_taskStatus != TaskStatus.InProgress)
                        {
                            StartDate = DateTime.Now;
                            CompletionDate = null;
                        }
                        break;

                    case TaskStatus.Complete:
                        if (_taskStatus != TaskStatus.Complete)
                        {
                            CompletionDate = DateTime.Now;
                        }
                        break;
                }

                _taskStatus = value;
            }
        }

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
        [JsonIgnore]
        public Task? Parent { get; set; } = null;

        /// <summary>
        /// The expected duration of the task, in minutes
        /// </summary>
        public int ExpectedDuration { get; set; }

        /// <summary>
        /// The calculated duration of the task, in minutes
        /// </summary>
        [JsonIgnore]
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
        [JsonIgnore]
        public bool IsActive
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
        /// True if the task is on the critical path, false otherwise. Is set by the UpdateNext method.
        /// </summary>
        [JsonIgnore]
        public bool IsCritical { get => _isCritical; set => _isCritical = value; }

        /// <summary>
        /// The buffer time for the task, in minutes. Based on the due date.
        /// </summary>
        /// TODO: Implement buffer time calculation
        [JsonIgnore]
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
        // TODO: Consider whether this should be able to be set
        public DateTime? ActiveDate { get; set; } = null;

        /// <summary>
        /// The date that the task was completed.
        /// </summary>
        // TODO: Consider whether this should be able to be set
        public DateTime? CompletionDate { get; set; } = null;

        /// <summary>
        /// The date that the task was created.
        /// </summary>
        public DateTime CreationDate { get => _creationDate; }

        /// <summary>
        /// The amount of time the task was active for, in minutes.
        /// </summary>
        [JsonIgnore]
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
        [JsonIgnore]
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
                UpdateNextAndCritical();
            }
        }

        /// <summary>
        /// Insert a subtask at a specific index.
        /// </summary>
        /// <param name="subtask">Subtask to insert</param>
        /// <param name="index">Index to insert the subtask at</param>
        /// <exception cref="ArgumentOutOfRangeException">Thrown if the index is out of range of the Subtask list</exception>
        public void InsertSubtask(Task subtask, int index)
        {
            if (index < 0 || index > _subtasks.Count)
            {
                throw new ArgumentOutOfRangeException(nameof(index), "Index is out of range.");
            }

            if (_subtasks.Contains(subtask))
            {
                _subtasks.Insert(index, subtask);
                subtask.Parent = this;
                UpdateNextAndCritical();
            }
        }

        /// <summary>
        /// Removes a subtask from the current task.
        /// </summary>
        /// <param name="subtask"></param>
        public void RemoveSubtask(Task subtask)
        {
            _subtasks.Remove(subtask);
            subtask.Parent = null;

            // Update the subtasks
            UpdateNextAndCritical();

        }

        /// <summary>
        /// Moves all subtasks from this task to another target task.
        /// </summary>
        /// <param name="target">The target task to move all subtasks to.</param>
        /// <exception cref="NotImplementedException"></exception>
        public void MoveAllSubtasks(Task target)
        {
            throw new NotImplementedException();
        }

        /// <summary>
        /// Moves a single subtask from this task to another target task.
        /// </summary>
        /// <param name="subtask">The task to be moved.</param>
        /// <param name="target">The target task to move the subtask to.</param>
        /// <exception cref="NotImplementedException"></exception>
        public void MoveSubtask(Task subtask, Task target)
        {
            throw new NotImplementedException();
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
        /// Updates the IsNext and IsCritical property for the task and all subtasks.
        /// </summary>
        public void UpdateNextAndCritical()
        {
            // If the task has no parent, it by default is next on the execution list as the task is the root task
            if (Parent == null)
            {
                IsActive = true;
                IsCritical = true;
            }
            // If the task has a parent, the updating of IsNext is done by the parent task
            if (_subtasks.Count > 0)
            {
                if (SubtaskOrder == ExecutionOrder.Parallel)
                {
                    // The longest duration subtask is critical if the current task is critical
                    IOrderedEnumerable<Task> tasksByDuration = _subtasks.OrderByDescending(t => t.CalculatedDuration);
                    tasksByDuration.First().IsCritical = IsCritical;
                    // Otherwise, the subtasks are not critical
                    for (int i = 1; i < _subtasks.Count; i++)
                    {
                        tasksByDuration.ElementAt(i).IsCritical = false;
                    }

                    // If execution is parallel, all subtasks are next if the current task is next
                    foreach (Task t in _subtasks)
                    {
                        t.IsActive = IsActive;

                        t.UpdateNextAndCritical();
                    }
                }
                // If execution is series, only the first subtask is next if the current task is next
                // All others are off the path
                else
                {
                    // All series subtasks are critical if the current task is critical
                    foreach (var t in _subtasks)
                    {
                        t.IsCritical = IsCritical;
                    }

                    // The first subtask is next if the current task is next
                    _subtasks.First().IsActive = IsActive;
                    _subtasks.First().UpdateNextAndCritical();
                    for (int i = 1; i < _subtasks.Count; i++)
                    {
                        _subtasks.ElementAt(i).IsActive = false;
                        _subtasks.ElementAt(i).UpdateNextAndCritical();
                    }
                }
            }

        }

        /// <summary>
        /// Calculates the time buffer for the task based on the due date.
        /// </summary>
        // TODO: Calculate the buffer of time for the tasks (i.e. when the task can be started)
        public void CalculateBuffer()
        {

        }

        #endregion

        #region Serialization
        /// <summary>
        /// Serialize the task to a JSON string.
        /// </summary>
        /// <returns>A JSON string representation of the task.</returns>
        public string ToJSON()
        {
            return JsonSerializer.Serialize(this, _jsonOptions);
        }

        /// <summary>
        /// Create a task from a JSON string.
        /// </summary>
        /// <param name="json">JSON string to deserialize.</param>
        /// <returns>The task that has been created.</returns>
        public static Task FromJSON(string json)
        {
            var t = JsonSerializer.Deserialize<Task>(json, _jsonOptions);
            if (t is not null)
            {
                return t;
            }
            else
            {
                throw new JsonException("Could not deserialize the JSON string.");
            }
        }
        #endregion
    }
}
