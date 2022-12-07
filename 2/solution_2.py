import fileinput

options = [
    {"name": "rock", "score": 1, "beats": "scissors", "beats_index": 2, "loses_to_index": 1},
    {"name": "paper", "score": 2, "beats": "rock", "beats_index": 0, "loses_to_index": 2},
    {"name": "scissors", "score": 3, "beats": "paper", "beats_index": 1, "loses_to_index": 0}
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

def get_strategy_option(their_option, strategy_for_round):
    if strategy_for_round == "Y":
        return their_option
    elif strategy_for_round == "X":
        return options[their_option["beats_index"]]
    elif strategy_for_round == "Z":
        return options[their_option["loses_to_index"]]
    else:
        throw("Invalid strategy " + strategy_for_round)

def get_win_score(their_option, my_option):
    if their_option["beats"] == my_option["name"]:
        return 0
    elif their_option["name"] == my_option["name"]:
        return 3
    else:
        return 6

they_score = 0
we_score = 0

for line in fileinput.input():
    if len(line) == 0:
        break
    their_option = get_option(line[0])
    our_option = get_strategy_option(their_option, line[2])
    we_score += our_option["score"]
    we_score += get_win_score(their_option, our_option)

print("Our score: " + str(we_score))


