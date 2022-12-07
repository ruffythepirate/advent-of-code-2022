import fileinput

current_sum = 0
greatest_sum = -1
for line in fileinput.input():
    if line == "\n":
        if current_sum > greatest_sum:
            greatest_sum = current_sum
        elif current_sum < 0:
            print("Error: current_sum < 0")
        current_sum = 0
        continue
    current_sum += int(line)

print(greatest_sum)

