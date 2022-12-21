namespace Domain;

public class RoundLogic {
    public static void PerformRound( List<Monkey> monkeys) {
        foreach (var monkey in monkeys) {
            var item = monkey.ExamineItem() / 3;
            while(item != null) {
                var target = monkey.TestItem(item.Value) ? monkey.TrueTarget : monkey.FalseTarget;
                monkeys[target].AddItem(item.Value);
                item = monkey.ExamineItem() / 3;
            }
        }
    }
}