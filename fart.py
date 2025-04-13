import random

def fart():
    choice = ['thbt','tttbbbbtt', 'tttttthhhhhhbbbbbttt', 'flubflublbblb','brrrrrraapppp','brrrurnnnntttt', ' ']
    _num = random.randrange(1, 5)
    choice = [random.choice(choice) for x in range(_num)]
    if len(choice) == 1:
        if choice[0] == ' ':
            fart()
    return ''.join(choice)
