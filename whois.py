def _whois(self, string):
    a = []
    for i in uids:
        i = json.loads(uids[i])
        if string in i:
            a += i
    return list(set(a))

def whois(self, string):
    a = [string]
    while True:
        l = len(a)
        for n in a:
            i = self._whois(n)
            if len(i) > 0: a += i
            else: a = ['no accounts for that user']; break
            a = list(set(a))
        if l == len(a):
            break
    return sorted(a)

def delwhois(self, string):
    string = string.split()
    for x in string:
        for i in uids:
            n = json.loads(uids[i])
            if x in n:
                n.remove(x)
                uids[i] = json.dumps(n)
    return 'done'


def rUids(k, v):
    key, value = k.lower(), v.lower()
    if key not in uids:
        uids[key] = json.dumps([value])
    else:
        values = json.loads(uids[key])
        if value not in values:
            values.append(value)
            uids[key] = json.dumps(values)
