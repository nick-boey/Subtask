using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace Subchain.Tasks
{
    public class Role
    {
        #region Members
        private static readonly JsonSerializerOptions _jsonOptions = new()
        {
            WriteIndented = true,
            DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
        };
        #endregion

        #region Properties
        /// <summary>
        /// The unique identifier of the role.
        /// </summary>
        public Guid Id { get; set; }

        /// <summary>
        /// The name of the role.
        /// </summary>
        public string Name { get; set; } = "";

        /// <summary>
        /// The root task of the role, managed by the role.
        /// </summary>
        public Task RootTask { get; init; } = new Task("_root");
        #endregion

        #region Constructors
        /// <summary>
        /// Creates a new role with no name.
        /// </summary>
        public Role()
        {
            Id = Guid.NewGuid();

        }

        /// <summary>
        /// Creates a new role with a name.
        /// </summary>
        /// <param name="name">The name of the role.</param>
        public Role(string name) : this()
        {
            Name = name;
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
