import random


slot_machine = {'Anpan': [1,100000],
                'Dildo': [20,10000],
                'Fart wad': [25,6000],
                'Squirrel': [30,7000],
                'Croissant': [33,6000],
                '❀': [35,5000],
                '♥︎': [40,3000],
                '✪': [80,1500],
                'O-Chinchin': [450,500],
                'O-Manko': [450,500],
                'Derptoid': [650,100],
                'Boobies': [650,50],
                '糞': [550,5]
                }



def _slotmachine():
    choice_pool = []
    for i in slot_machine:
        _num = slot_machine[i][0]
        [choice_pool.append(i) for x in range(_num)]
    outcome = []
    for i in [1,2,3]:
        outcome.append(random.choice(choice_pool))
    result_num = []
    for i in slot_machine:
        result_num.append([i, outcome.count(i)])
    for i in result_num:
        if i[1] < 2:
            result_num.remove(i)
    final_results = '['+', '.join(outcome)+'] '
    for i in result_num:
        if i[1] == 3:
            amount = slot_machine[i[0]][1]
            if i == 'Anpan':
                return final_results+'You hit the big Anpan jackpot!! You win '+str(amount)+' points.'
            else:
                return final_results+'You hit a three of a kind with ['+i[0]+']!!! You win '+str(amount)+' points.'
        elif i[1] == 2:
            outcome.remove(i[0])
            outcome.remove(i[0])
            amount = round((slot_machine[i[0]][1]*(1/3))+(slot_machine[outcome[0]][1]*(1/20)))
            if amount < 1:
                amount = 1
            return final_results+'You hit a two of a kind with ['+i[0]+']!!! You win '+str(amount)+' points.'
        else:
            continue
    amount = [slot_machine[i][1]*(1/20) for i in outcome]
    a,b,c = amount
    amount = round(a+b+c)
    if amount < 1:
        amount = 1
    return 'The results are '+final_results+'You win '+str(amount)+' points.'
