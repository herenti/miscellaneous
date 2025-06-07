import time
import urllib.request
import urllib.parse
import random
import re
import select
import socket



def regex(pattern, x, default): return re.search(pattern, x).group(1) if re.search(pattern, x) else default


def Auth(user, password):
    data = urllib.parse.urlencode({"user_id": user, "password": password, "storecookie": "on", "checkerrors": "yes"}).encode()
    return regex('auth.chatango.com=(.*?);', urllib.request.urlopen("http://chatango.com/login", data).getheader('Set-Cookie'), None)

manager = dict()
server = 'c1.chatango.com'
port = 5222

def start(username, password):
        cumsock = socket.socket()
        cumsock.connect((server, port))
        manager["socket"] = cumsock
        auth = Auth(username, password)
        if auth:
          login(auth)
        else:
          print("Invalid login: " + username)
          time.sleep(0.1)


def login(auth):
        send('tlogin', auth, '2')
        manager["socket"].close()
        print("logged in "+username)


def send(*x):
        data = ':'.join(x).encode()
        manager["socket"].send(data+b'\x00')



try:
  accounts = []
  with open("accounts.txt", "r", encoding="latin-1") as file:
    for x in file.readlines():
      password = x.strip()
      accounts.append(password)
except Exception as error:
  accounts = []
  print(error)

if len(accounts) > 0:
  fail = []
  for x in accounts:
    data = x.split(": ")
    if len(data) > 1:
      username = data[0]
      password = data[1]
      start(username, password)
      time.sleep(0.1)

