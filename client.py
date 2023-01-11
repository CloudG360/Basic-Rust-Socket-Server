import socket

def send(target, data):
    target.send(bytearray(data.encode()))

ip = "127.0.0.1"
port = 28000
addr = (ip, port)

print("Starting Test Client")

client = socket.socket(type=socket.SOCK_DGRAM)
client.connect(addr)

send(client, "test 1")
send(client, "test 2")
send(client, "test 3")
send(client, "exit")

while True:
    pass

