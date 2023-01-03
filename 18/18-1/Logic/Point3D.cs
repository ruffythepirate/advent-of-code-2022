namespace Logic;

public class Point3D {
    public int X { get; private set; }
    public int Y { get; private set; }
    public int Z { get; private set; }

    public Point3D(int x, int y, int z) {
        this.X = x;
        this.Y = y;
        this.Z = z;
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