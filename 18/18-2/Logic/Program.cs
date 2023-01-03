// See https://aka.ms/new-console-template for more information

using Logic;
class Program
{

    static void Main(string[] args)
    {
        var allLines = ReadAllInput();
        var logicService = new LogicService();
        var points = logicService.ReadPoints(allLines);
        var grid = logicService.InitGrid(points);

        grid.FillEncapsulatedSpots();
        var totalFreeSpots = 0;
        foreach(var point in points) {
            var freeSpots = grid.FindAdjacentFreeSpots(point.X, point.Y, point.Z);
            totalFreeSpots += freeSpots;
        }

        System.Console.WriteLine(totalFreeSpots);
    }

    static List<string> ReadAllInput() {
        string? line = "";
        var lines = new List<string>();
        while(line != null) {
            while ((line = Console.ReadLine()) != null && line != "")
            {
                lines.Add(line);
            }
        }
        return lines;
    }
}

