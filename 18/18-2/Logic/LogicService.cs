namespace Logic {

    public class LogicService {

        public List<Point3D> ReadPoints(List<string> allLines) {
            var points = new List<Point3D>();
            foreach(var line in allLines) {
                points.Add(Point3D.fromLine(line));
            }
            return points;
        }


        public Grid3D InitGrid(List<Point3D> points) {
            var maxX = points.Max(p => p.X);
            var maxY = points.Max(p => p.Y);
            var maxZ = points.Max(p => p.Z);
            var grid = new Grid3D(maxX + 1, maxY + 1, maxZ + 1);
            foreach(var point in points) {
                grid.populateCell(point.X, point.Y, point.Z);
            }
            return grid;
        }


    }
}
