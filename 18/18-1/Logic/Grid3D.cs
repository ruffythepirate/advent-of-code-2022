namespace Logic;

public class Grid3D {

    public int width { get; private set; }
    public int height { get; private set; }
    public int depth { get; private set; }
    private bool[,,] cells;


    public Grid3D(int width, int height, int depth) {
        this.width = width;
        this.height = height;
        this.depth = depth;
        this.cells = new bool[width, height, depth];
    }

    public void populateCell(int x, int y, int z) {
        this.cells[x, y, z] = true;
    }

    public bool isPopulated(int x, int y, int z) {
        if(x < 0 || x >= this.width) {
            return false;
        }
        if(y < 0 || y >= this.height) {
            return false;
        }
        if(z < 0 || z >= this.depth) {
            return false;
        }
        return this.cells[x, y, z];
    }

    public int FindAdjacentFreeSpots(int x, int y, int z) {
        var freeSpots = 0;
        freeSpots += this.isPopulated(x - 1, y, z) ? 0 : 1;
        freeSpots += this.isPopulated(x + 1, y, z) ? 0 : 1;
        freeSpots += this.isPopulated(x, y - 1, z) ? 0 : 1;
        freeSpots += this.isPopulated(x, y + 1, z) ? 0 : 1;
        freeSpots += this.isPopulated(x, y, z - 1) ? 0 : 1;
        freeSpots += this.isPopulated(x, y, z + 1) ? 0 : 1;
        return freeSpots;
    }
}
