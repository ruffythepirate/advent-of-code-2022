namespace Domain;

using System.Numerics;

using System.Text.RegularExpressions;

public class Monkey {

    public Monkey(int id, List<BigInteger> items, string operation, string test, int trueTarget, int falseTarget) {
        Id = id;
        Items = items;
        Operation = operation;
        Test = test;
        TrueTarget = trueTarget;
        FalseTarget = falseTarget;
        InspectScore = 0;

        TestDivisor = int.Parse(TestRegex.Match(Test).Groups[1].Value);
    }

    public int Id {get; internal set;}

    public List<BigInteger> Items {get; internal set;}
    public string Operation { get; internal set; }
    public string Test { get; internal set; }
    public int TrueTarget { get; internal set; }
    public int FalseTarget { get; internal set; }

    public int TestDivisor { get; internal set; }

    public int InspectScore { get; internal set; }

    private static readonly Regex MonkeyRegex = new Regex(@"Monkey (\d+):");
    public static Monkey fromStrings(List<string> lines) {
        var id = int.Parse(MonkeyRegex.Match(lines[0]).Groups[1].Value);
        var items = lines[1].Split(":")[1].Split(",").Select(BigInteger.Parse).ToList();
        var operation = lines[2].Split(":")[1].Trim();
        var test = lines[3].Split(":")[1].Trim();
        var trueTarget = int.Parse(lines[4].Split(":")[1].Split(" ")[4]);
        var falseTarget = int.Parse(lines[5].Split(":")[1].Split(" ")[4]);

        return new Monkey(id, items, operation, test, trueTarget, falseTarget);
    }

    public BigInteger? ExamineItem() {
        if( Items.Count == 0) return null;
        InspectScore++;
        var firstItem = Items.First();
        Items.RemoveAt(0);
        return PerformOperation(firstItem);
    }

    private static readonly Regex TestRegex = new Regex(@"divisible by (\d+)");
    public bool TestItem(BigInteger item) {
        return item % TestDivisor == 0;
    }

    public void AddItem(BigInteger item) {
        Items.Add(item);
    }

    private static readonly Regex OperationRegex = new Regex(@"new = (.*) ([-+*/]) (.*)");
    private BigInteger PerformOperation(BigInteger item) {
        try {

        var operationMatch = OperationRegex.Match(Operation);
        var operand1 = operationMatch.Groups[1].Value;
        var operation = operationMatch.Groups[2].Value;
        var operand2 = operationMatch.Groups[3].Value;

        if(operand1 == "old") operand1 = item.ToString();
        if(operand2 == "old") operand2 = item.ToString();

        return ApplyOperation(BigInteger.Parse(operand1), BigInteger.Parse(operand2), operation);
        } catch(Exception e) {
            Console.WriteLine($"Item: {item}, Operation: {Operation}");
            throw e;
        }
    }

    private static BigInteger ApplyOperation(BigInteger operand1, BigInteger operand2, string operation) {
        return operation switch {
            "+" => operand1 + operand2,
            "-" => operand1 - operand2,
            "*" => operand1 * operand2,
            "/" => operand1 / operand2,
            _ => throw new Exception("Unknown operation")
        };
    }

    public override string ToString()
    {
        return $"Monkey {Id}: {string.Join(",", Items)}";
    }
}
