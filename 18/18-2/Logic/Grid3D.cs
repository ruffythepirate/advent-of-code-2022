namespace Logic {
    using System.Collections.Generic;

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

        static private List<Point3D> AdjacentPoints = new List<Point3D>() {
            new Point3D(-1, 0, 0),
            new Point3D(1, 0, 0),
            new Point3D(0, -1, 0),
            new Point3D(0, 1, 0),
            new Point3D(0, 0, -1),
            new Point3D(0, 0, 1),
        };

        public List<Point3D> FindConnectedFreeCells(int x, int y, int z, HashSet<Point3D> visited) {
            List<Point3D> freeCells = new List<Point3D>();
            if(this.isPopulated(x, y, z)) {
                return freeCells;
            }
            var point = new Point3D(x, y, z);
            freeCells.Add(point);


            foreach(var direction in AdjacentPoints) {
                var i = direction.X;
                var j = direction.Y;
                var k = direction.Z;
                var adjacentPoint = new Point3D(x + i, y + j, z + k);
                if(this.IsWithinBounds(point) && !this.isPopulated(x + i, y + j, z + k) && !visited.Contains(new Point3D(x + i, y + j, z + k))) {
                    visited.Add(adjacentPoint);
                    var connectedFreeCells = this.FindConnectedFreeCells(x + i, y + j, z + k, visited);
                    freeCells.AddRange(connectedFreeCells);
                }
            }
            return freeCells;
        }

        public void FillEncapsulatedSpots() {
            var encapsulatedPoints = this.FindEncapsulatedSpots();
            Console.WriteLine($"Found {encapsulatedPoints.Count} encapsulated points");
            foreach(var point in encapsulatedPoints) {
                this.populateCell(point.X, point.Y, point.Z);
            }
        }

        public List<Point3D> FindEncapsulatedSpots() {
            List<Point3D> result = new List<Point3D>();
            HashSet<Point3D> visited = new HashSet<Point3D>();
            for(int x = 0; x < this.width; x++) {
                for(int y = 0; y < this.height; y++) {
                    for(int z = 0; z < this.depth; z++) {
                        if(!this.isPopulated(x, y, z) && !visited.Contains(new Point3D(x, y, z))) {
                            var freeCells = this.FindConnectedFreeCells(x, y, z, visited);
                            if(x == 2 && y == 2 && z == 5) {
                                Console.WriteLine($"Found {freeCells.Count} free cells");
                            }
                            if(ArePointsEncapsulated(freeCells)) {
                                result.AddRange(freeCells);
                            }
                        }
                    }
                }
            }
            return result;
        }

        public bool IsWithinBounds(Point3D point) {
            return point.X >= 0 && point.X < this.width && point.Y >= 0 && point.Y < this.height && point.Z >= 0 && point.Z < this.depth;
        }
        
        public bool ArePointsEncapsulated(List<Point3D> points) {
            foreach(var point in points) {
                if(point.X == 0 || point.X >= this.width - 1) {
                    return false;
                } else if (point.Y == 0 || point.Y >= this.height - 1) {
                    return false;
                } else if (point.Z == 0 || point.Z >= this.depth - 1) {
                    return false;
                }
            }
            return true;
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
}
