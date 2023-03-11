import fileinput
import re

class Monkey:
    def __init__(self, name, operation):
        self.name = name
        self.operation = operation
        pattern = r'(\w+)\s*([+\-*/])\s*(\w+)'
        match = re.match(pattern, self.operation)
        self.required_value = None

        if match:
            self.operand1 = match.group(1)
            self.operator = match.group(2)
            self.operand2 = match.group(3)

    def resolve_required_values(self, all_monkeys):
        print(self.name, self.operation, self.required_value)
        if self.operation == 'None':
            return
        first_value = all_monkeys[self.operand1].resolve(all_monkeys)
        second_value = all_monkeys[self.operand2].resolve(all_monkeys)
        if first_value is None:
            all_monkeys[self.operand1].required_value = self.find_required_value_first_operand(second_value)
            all_monkeys[self.operand1].resolve_required_values(all_monkeys)
        if second_value is None:
            all_monkeys[self.operand2].required_value = self.find_required_value_second_operand(first_value)
            all_monkeys[self.operand2].resolve_required_values(all_monkeys)

    def find_required_value_first_operand(self, other_value):
        if self.operator == '+':
            return self.required_value - other_value
        elif self.operator == '-':
            return self.required_value + other_value
        elif self.operator == '*':
            return self.required_value / other_value
        elif self.operator == '/':
            return self.required_value * other_value
        elif self.operator == '=':
            return other_value
        else:
            return None

    def find_required_value_second_operand(self, other_value):
        if self.operator == '+':
            return self.required_value - other_value
        elif self.operator == '-':
            return - self.required_value + other_value
        elif self.operator == '*':
            return self.required_value / other_value
        elif self.operator == '/':
            return other_value / self.required_value
        elif self.operator == '=':
            return other_value
        else:
            return None

    def resolve(self, all_monkeys):
        # check if operation is number
        if self.operation == 'None':
            return None

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
            if value1 is not None and value2 is not None:
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
all_monkeys['root'].operator = '='
all_monkeys['humn'].operation = 'None'
all_monkeys['root'].resolve_required_values(all_monkeys)
result = all_monkeys['humn'].required_value

print(result)


