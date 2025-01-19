import os

extention = input("what file extentions to rename? ")

discnum = input('disc number? ')

contents = os.listdir(os.getcwd())

contents = [i for i in contents if i.split('.')[-1] == extention]

b = {}

for i in contents:
    f = i.replace('.'+extention,'')
    f = 'Disc '+ discnum + ' ' + f
    f = f.split(' ')
    a = []
    for x in f:
        if len(x) < 2:
            if x.isdigit():
                x = x.replace(x, '0'+x)
        a.append(x)
    f = ' '.join(a)
    f = f+'.'+extention
    b[i] = f

contents = b

print(contents)

for i in contents:
    os.rename(i, contents[i])


