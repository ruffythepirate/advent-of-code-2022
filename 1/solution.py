import fileinput

current_sum = 0
greatest_sum = -1

def update_greatest_sum():
    global greatest_sum
    global current_sum
    if current_sum > greatest_sum:
        greatest_sum = current_sum
    elif current_sum < 0:
        print("Error: current_sum < 0")

for line in fileinput.input():
    if line == "\n":
        update_greatest_sum()
        current_sum = 0
        continue
    current_sum += int(line)

update_greatest_sum()

print(greatest_sum)

