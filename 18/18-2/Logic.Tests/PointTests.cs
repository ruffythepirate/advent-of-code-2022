namespace Logic.Tests
{
    using Logic;
    using Microsoft.VisualStudio.TestTools.UnitTesting;

    [TestClass]
    public class PointTests
    {
        [TestMethod]
        public void shouldParsePointFromLine()
        {
            var point = Point3D.fromLine("1,2,3");

            Assert.AreEqual(1, point.X);
            Assert.AreEqual(2, point.Y);
            Assert.AreEqual(3, point.Z);
        }
    }
}