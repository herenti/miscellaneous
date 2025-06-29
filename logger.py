# python 3.11.4
# chatango multithreaded login

import requests
import socket
import time
from concurrent.futures import ThreadPoolExecutor

class ChatangoLogin:
    def __init__(self, server='c1.chatango.com', port=5222, max_workers=10):
        self.server = server
        self.port = port
        self.max_workers = max_workers

    def get_auth(self, user, password):
        try:
            r = requests.get(
                f"https://st.chatango.com/script/setcookies?pwd={password}&sid={user}",
                headers={"User-Agent": "Mozilla/5.0"}
            )
            return r.headers.get("Set-Cookie", "").split("auth.chatango.com=")[1].split(";")[0]
        except requests.exceptions.RequestException as e: # useless
            print(f"[!] Authentication request error: {e}")
            return None

    def connect_to_server(self, user, auth_token):
        try:
            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            s.connect((self.server, self.port))
            login_msg = f"tlogin:{auth_token}:2:\x00".encode()
            s.sendall(login_msg)
            time.sleep(1)
            s.close()
            return f"[✓] Login successful: {user}"
        except socket.error as e: # useless
            print(f"[!] Socket error: {e}")
            return f"[x] Login failed: {user}"

    def process_account(self, line):
        if ':' not in line and '=' not in line:
            return None
        user, password = line.replace("=", ":").split(":", 1)
        auth = self.get_auth(user, password)
        if auth:
            return self.connect_to_server(user, auth)
        else:
            return f"[x] Failed to get auth token: {user}"

    def login_from_file(self, file_path="account.txt"):
        with open(file_path, "r") as f:
            lines = f.read().splitlines()

        with ThreadPoolExecutor(max_workers=self.max_workers) as executor: # I'm the boss, I have workers now :x
            results = list(executor.map(self.process_account, lines))

        for result in results:
            if result:
                print(result)

# Run
if __name__ == "__main__":
    bot = ChatangoLogin()
    bot.login_from_file("account.txt")

# original was made by herenti
