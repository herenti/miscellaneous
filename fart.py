import random

def fart():
    choice = ['thbt','tttbbbbtt', 'tttttthhhhhhbbbbbttt', 'flubflublbblb','brrrrrraapppp','brrrurnnnntttt', ' ']
    _num = random.randrange(1, 5)
    choice = [random.choice(choice) for x in range(_num)]
    choice = ''.join(choice)
    if choice == ' ':
        choice = '   - It was a silent one....'
    return choice
