namespace Domain;

public class RoundLogic {
    public static void PerformRound( List<Monkey> monkeys, long highestCommonDivisor) {
        foreach (var monkey in monkeys) {
            var item = monkey.ExamineItem();
            while(item != null) {
                var itemValue = item.Value % highestCommonDivisor;
                var target = monkey.TestItem(itemValue) ? monkey.TrueTarget : monkey.FalseTarget;
                monkeys[target].AddItem(itemValue);
                item = monkey.ExamineItem();
            }
        }
    }
}