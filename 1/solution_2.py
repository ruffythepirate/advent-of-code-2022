import fileinput


def is_among_greatest_sums(current_sum, greatest_sums, number_of_greatest_sums):
    if(len(greatest_sums) < number_of_greatest_sums):
        return True
    for i in range(number_of_greatest_sums):
        if current_sum > greatest_sums[i]:
            return True
    return False

def insert_in_greatest_sums(current_sum, greatest_sums, number_of_greatest_sums):
    if(len(greatest_sums) < number_of_greatest_sums):
        greatest_sums.append(current_sum)
        return
    for i in range(number_of_greatest_sums):
        if current_sum > greatest_sums[i]:
            greatest_sums.insert(i, current_sum)
            greatest_sums.pop()
            return

def sum_greatest_sums(greatest_sums):
    sum = 0
    for i in range(len(greatest_sums)):
        sum += greatest_sums[i]
    return sum

current_sum = 0
greatest_sums = []
for line in fileinput.input():
    if line == "\n":
        if is_among_greatest_sums(current_sum, greatest_sums, 3):
            insert_in_greatest_sums(current_sum, greatest_sums, 3)
        elif current_sum < 0:
            print("Error: current_sum < 0")
        current_sum = 0
        continue
    current_sum += int(line)

print(sum_greatest_sums(greatest_sums))
