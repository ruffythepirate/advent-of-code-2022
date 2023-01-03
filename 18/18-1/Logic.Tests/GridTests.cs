namespace Logic.Tests {

    using Logic;

    [TestClass]
    public class GridTests
    {
        [TestMethod]
        public void shouldPopulateGrid()
        {
            var grid = new Grid3D(3, 3, 3);
            grid.populateCell(1, 1, 1);

            Assert.IsTrue(grid.isPopulated(1, 1, 1));
            Assert.IsFalse(grid.isPopulated(0, 0, 0));
        }

        [TestMethod]
        public void ShouldFindAdjacentFreeSpots() {
            var grid = new Grid3D(3, 3, 3);
            grid.populateCell(1, 1, 1);
            grid.populateCell(1, 2, 1);

            Assert.AreEqual(5, grid.FindAdjacentFreeSpots(1, 1, 1));
        }
    }

}
