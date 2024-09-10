using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Subchain.Tasks
{
    /// <summary>
    /// The status of a task.
    /// </summary>
    public enum TaskStatus
    {
        /// <summary>
        /// The task has not been started.
        /// </summary>
        NotStarted,
        /// <summary>
        /// The task is currently in progress.
        /// </summary>
        InProgress,
        /// <summary>
        /// The task is complete.
        /// </summary>
        Complete
    }

    /// <summary>
    /// The order that subtasks should be executed in.
    /// </summary>
    public enum ExecutionOrder
    {
        /// <summary>
        /// Tasks are to be executed one after the other, in series.
        /// </summary>
        Series,
        /// <summary>
        /// Tasks can be executed simultaneously, in parallel.
        /// </summary>
        Parallel
    }
}
