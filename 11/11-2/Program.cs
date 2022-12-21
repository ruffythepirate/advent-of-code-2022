// See https://aka.ms/new-console-template for more information

using Domain;

class Program
{
    static void Main(string[] args)
    {
        var monkeys = ReadAllMonkeys();
        var highestCommonDivisor = monkeys.Select(m => m.TestDivisor).Aggregate((a, b) => a * b);
        for(int i = 0; i < 10000; i++) {
            RoundLogic.PerformRound(monkeys, highestCommonDivisor);
        }

        var worstMonkeys = monkeys.OrderByDescending(m => m.InspectScore).Take(2).ToList();
        Console.WriteLine($"{worstMonkeys[0].InspectScore} {worstMonkeys[1].InspectScore}");
        var monkeyBusniness = new Decimal(worstMonkeys[0].InspectScore) * new Decimal(worstMonkeys[1].InspectScore);
        Console.WriteLine($"{monkeyBusniness}");
    }

    static List<Monkey> ReadAllMonkeys() {
        var monkeys = new List<Monkey>();
        string? line = "";
        while(line != null) {
            var lines = new List<string>();
            while ((line = Console.ReadLine()) != null && line != "")
            {
                lines.Add(line);
            }
            var monkey = Monkey.fromStrings(lines);
            monkeys.Add(monkey);
        }
        return monkeys;

    }
}
