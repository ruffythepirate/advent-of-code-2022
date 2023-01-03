namespace Logic.Tests {


    using Logic;

    [TestClass]
    public class LogicServiceTests {

        [TestMethod]
        public void ShouldInitGridBasedOnMaxValues() {
            var logicService = new LogicService();

            var points = new List<Point3D> {
                new Point3D(1, 1, 5),
                new Point3D(6, 1, 5),
                new Point3D(1, 7, 5),
            };

            var grid = logicService.InitGrid(points);

            Assert.AreEqual(6 + 1, grid.width);
            Assert.AreEqual(7 + 1, grid.height);
            Assert.AreEqual(5 + 1, grid.depth);
        }
    }


}
