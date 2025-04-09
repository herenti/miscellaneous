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


ready = True

manager = dict()
server = 'c1.chatango.com'
port = 5222

def start(username, password):
        cumsock = socket.socket()
        cumsock.connect((server, port))
        manager["socket"] = cumsock
        manager["wbyte"] = b''
        auth = Auth(username, password)
        login(auth)


def login(auth):
        send('tlogin', auth, '2')
        ready = False
        time.sleep(1)
        manager["socket"].close()


def send(*x):
        data = ':'.join(x).encode()
        byte = b'\x00' if ready else b'\r\n\x00'
        manager["wbyte"] += data+byte
        manager["socket"].send(manager["wbyte"])
        manager["wbyte"] = b''



try:
  accounts = []
  with open("accounts.txt", "r", encoding="latin-1") as file:
    for x in file.readlines():
      password = x.strip()
      accounts.append(password)
except Exception as error: accounts = []

if len(accounts) > 0:
  fail = []
  for x in accounts:
    data = x.split(": ")
    if len(data) > 1:
      username = data[0]
      password = data[1]
      start(username, password)
      print("logged in "+username)
