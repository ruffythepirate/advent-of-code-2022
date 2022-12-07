import fileinput

options = [
    {"name": "rock", "score": 1, "beats": "scissors"},
    {"name": "paper", "score": 2, "beats": "rock"},
    {"name": "scissors", "score": 3, "beats": "paper"},
]

def get_option(code):
    if code == "A" or code == "X":
        return options[0]
    elif code == "B" or code == "Y":
        return options[1]
    elif code == "C" or code == "Z":
        return options[2]
    else:
        throw("Invalid code " + code)

they_score = 0
we_score = 0

for line in fileinput.input():
    if len(line) == 0:
        break
    they_choice = get_option(line[0])
    we_choice = get_option(line[2])
    they_score += they_choice["score"]
    we_score += we_choice["score"]
    if they_choice["beats"] == we_choice["name"]:
        they_score += 6
    elif we_choice["beats"] == they_choice["name"]:
        we_score += 6
    else:
        they_score += 3
        we_score += 3

print("Our score: " + str(we_score))


