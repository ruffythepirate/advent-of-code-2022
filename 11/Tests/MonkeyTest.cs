namespace Domain;
public class MonkeyTest
{
    private Monkey CreateDefaultMonkey(string operation = "Operation: new = old * 19", string test = "Test: divisible by 23") {
        var lines = new List<string> {
            "Monkey 0:",
            "  Starting items: 79, 98",
            $"  {operation}",
            $"  {test}",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
        };

        var monkey = Monkey.fromStrings(lines);
        return monkey;
    }

    [Fact]
    public void ShouldCreateMonkeyFromLines()
    {
        var lines = new List<string> {
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
        };

        var monkey = Monkey.fromStrings(lines);

        Assert.Equal(0, monkey.Id);
        Assert.Equal(new List<int> {79, 98}, monkey.Items);
        Assert.Equal("new = old * 19", monkey.Operation);
        Assert.Equal("divisible by 23", monkey.Test);
        Assert.Equal(2, monkey.TrueTarget);
        Assert.Equal(3, monkey.FalseTarget);
    }

    [Fact]
    public void ShouldReturnItemsOneAfterAnother() {
        var monkey = CreateDefaultMonkey();
        Assert.Equal(79 * 19, monkey.ExamineItem());
        Assert.Equal(98 * 19, monkey.ExamineItem());
        Assert.Null(monkey.ExamineItem());
    }

    [Fact]
    public void ShouldPerformOperationWhenReturningItem() {
        var monkey = CreateDefaultMonkey("Operation: new = old * 2");
        Assert.Equal(79 * 2, monkey.ExamineItem());
        Assert.Equal(98 * 2, monkey.ExamineItem());
        Assert.Null(monkey.ExamineItem());
    }

    [Fact]
    public void ShouldTestWhichOperation() {
        var monkey = CreateDefaultMonkey(test: "Test: divisible by 21");
        Assert.True(monkey.TestItem(21 * 2));
        Assert.False(monkey.TestItem(22 * 2));
    }
}
