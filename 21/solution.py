import fileinput
import re

class Monkey:
    def __init__(self, name, operation):
        self.name = name
        self.operation = operation

    def resolve(self, all_monkeys):
        # check if operation is number
        try:
            return int(self.operation)
        except ValueError:
            pass
        return self.resolve_operation(all_monkeys)

    def resolve_operation(self, all_monkeys):
        pattern = r'(\w+)\s*([+\-*/])\s*(\w+)'

        match = re.match(pattern, self.operation)

        if match:
            operand1 = match.group(1)
            operator = match.group(2)
            operand2 = match.group(3)
            value1 = all_monkeys[operand1].resolve(all_monkeys)
            value2 = all_monkeys[operand2].resolve(all_monkeys)
            return self.apply_operation(value1, operator, value2)
        return None

    def apply_operation(self, operand1, operator, operand2):
        if operator == '+':
            return operand1 + operand2
        elif operator == '-':
            return operand1 - operand2
        elif operator == '*':
            return operand1 * operand2
        elif operator == '/':
            return operand1 / operand2

def read_input():
    all_monkeys = []
    for line in fileinput.input():
        monkey = parse_monkey(line)
        all_monkeys.append(monkey)
    return {monkey.name: monkey for monkey in all_monkeys}

def parse_monkey(line):
    pattern = r'(\w+):\s(.*)'

    match = re.match(pattern, line)

    if match:
        name = match.group(1)
        operation = match.group(2)
        return Monkey(name, operation)
    raise Exception('Invalid monkey')

all_monkeys = read_input()
print
result = all_monkeys['root'].resolve(all_monkeys)

print(result)


