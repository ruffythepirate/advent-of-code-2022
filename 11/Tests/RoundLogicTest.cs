namespace Domain;
public class RoundLogicTest 
{
    List<Monkey> CreateMonkeys() {
        var monkeys = new List<Monkey> {
            new Monkey(0, new List<int> {}, "new = old * 19", "divisible by 2", 2, 3),
            new Monkey(1, new List<int> {}, "new = old + 1", "divisible by 23", 2, 3),
            new Monkey(2, new List<int> {}, "new = old + 1", "divisible by 23", 2, 3),
            new Monkey(3, new List<int> {1,2}, "new = old * 3", "divisible by 2", 0, 1),
        };
        return monkeys;
    }
    [Fact]
    public void ShouldPerformRound() {
        var monkeys = CreateMonkeys();

        RoundLogic.PerformRound(monkeys);

        Assert.Equal(new List<int> {2}, monkeys[0].Items);
        Assert.Equal(new List<int> {1}, monkeys[1].Items);
        Assert.Equal(new List<int> {}, monkeys[2].Items);
        Assert.Equal(new List<int> {}, monkeys[3].Items);
    }
}