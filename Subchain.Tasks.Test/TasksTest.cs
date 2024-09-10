using System.Security.Cryptography.X509Certificates;

namespace Subchain.Tasks.Test
{
    [TestClass]
    public class TasksTest
    {
        Task t1 = new Task("Task 1");
        Task t2 = new Task("Task 2");
        Task t3 = new Task("Task 3");
        Task t4 = new Task("Task 4");
        Task t5 = new Task("Task 5");

        [TestInitialize]
        public void Initialize()
        { 
            t2.ExpectedDuration = 15;
            t4.ExpectedDuration = 45;
            t5.ExpectedDuration = 60;

            t1.AddSubtask(t2);
            t1.AddSubtask(t3);

            t3.AddSubtask(t4);
            t3.AddSubtask(t5);

            t1.SubtaskOrder = ExecutionOrder.Parallel;
            t3.SubtaskOrder = ExecutionOrder.Series;
        }

        [TestMethod]
        public void DurationTest()
        {
            Assert.AreEqual(105, t1.CalculatedDuration);
            Assert.AreEqual(15, t2.CalculatedDuration);
            Assert.AreEqual(105, t3.CalculatedDuration);
            Assert.AreEqual(45, t4.CalculatedDuration);
            Assert.AreEqual(60, t5.CalculatedDuration);
        }

        [TestMethod]
        public void NextTest()
        {
            t1.UpdateNext();

            Assert.AreEqual(true, t1.IsNext);
            Assert.AreEqual(true, t2.IsNext);
            Assert.AreEqual(true, t3.IsNext);
            Assert.AreEqual(true, t4.IsNext);
            Assert.AreEqual(false, t5.IsNext);
        }
    }
}