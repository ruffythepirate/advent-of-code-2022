namespace Logic {

    public class Point3D {
        public int X { get; private set; }
        public int Y { get; private set; }
        public int Z { get; private set; }

        public Point3D(int x, int y, int z) {
            this.X = x;
            this.Y = y;
            this.Z = z;
        }

        override public bool Equals(object obj) {
            if(obj == null) {
                return false;
            }
            if(obj.GetType() != typeof(Point3D)) {
                return false;
            }
            var point = (Point3D)obj;
            return this.X == point.X && this.Y == point.Y && this.Z == point.Z;
        }

        override public int GetHashCode() {
            return $"{this.X}-{this.Y}-{this.Z}".GetHashCode();
        }

        static public Point3D fromLine(string line) {
            var parts = line.Split(',');
            return new Point3D(
                int.Parse(parts[0]),
                int.Parse(parts[1]),
                int.Parse(parts[2])
            );
        }
    }
}
