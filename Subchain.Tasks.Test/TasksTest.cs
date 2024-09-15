using System.Security.Cryptography.X509Certificates;

namespace Subchain.Tasks.Test
{
    [TestClass]
    public class TasksTest
    {
        // Create simple task structure
        // Task 1 (105 mins) next critical
        // p- Task 2 (15 mins) next
        // p- Task 3 (105 mins) next critical
        //   s- Task 4 (45 mins) next critical
        //   s- Task 5 (60 mins) critical

        readonly Task t1 = new("Task 1");
        readonly Task t2 = new("Task 2");
        readonly Task t3 = new("Task 3");
        readonly Task t4 = new("Task 4");
        readonly Task t5 = new("Task 5");

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

            t1.UpdateNextAndCritical();

            Console.WriteLine(t1.ToJSON());
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
            Assert.AreEqual(true, t1.IsActive);
            Assert.AreEqual(true, t2.IsActive);
            Assert.AreEqual(true, t3.IsActive);
            Assert.AreEqual(true, t4.IsActive);
            Assert.AreEqual(false, t5.IsActive);
        }

        [TestMethod]
        public void CriticalTest()
        {
            Assert.AreEqual(true, t1.IsCritical);
            Assert.AreEqual(false, t2.IsCritical);
            Assert.AreEqual(true, t3.IsCritical);
            Assert.AreEqual(true, t4.IsCritical);
            Assert.AreEqual(true, t5.IsCritical);
        }
    }
}