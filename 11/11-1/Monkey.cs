
class Monkey {

    public int Id {get; internal set;}

    private List<int> Items {get; set;}
    private string Operation { get; internal set; }
    private string Test { get; internal set; }
    private int TrueTarget { get; internal set; }
    private int FalseTarget { get; internal set; }

    static Monkey fromStrings(List<string> lines) {
        var monkey = new Monkey();
        monkey.name = lines[0];
        monkey.age = int.Parse(lines[1]);
        monkey.weight = int.Parse(lines[2]);
        return monkey;
    }
}
