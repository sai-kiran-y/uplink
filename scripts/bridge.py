import socket
import json
import time
import os
import stat
import shutil
import threading
import zipfile
import subprocess

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("localhost", 5050))

# Converts JSON data received over TCP into a python dictionary
def recv_action(s):
    return json.loads(s.recv(2048))

# Constructs a payload and sends it over TCP to uplink
def send_data(s, payload):
    send = json.dumps(payload) + "\n"
    print(bytes(send, encoding="utf-8"))
    s.sendall(bytes(send, encoding="utf-8"))

# Constructs a JSON `action_status` as a response to received action on completion
def action_complete(id):
    return {
        "stream": "action_status",
        "sequence": 0,
        "timestamp": int(time.time()*1000000),
        "id": id,
        "state": "Completed",
        "progress": 100,
        "errors": []
    }

def action_failed(id, reason):
    return {
        "stream": "action_status",
        "sequence": 0,
        "timestamp": int(time.time()*1000000),
        "id": id,
        "state": "Failed",
        "progress": 100,
        "errors": [reason]
    }

def update_config(action):
    payload = json.loads(action['payload'])
    print(payload)
    app = payload["name"]
    print(app)
    ver = payload["version"]
    print(ver)
    if(ver == "latest"):
        cmd = "sudo apt update && sudo apt install " + app
        print(cmd)
    resp = action_complete(action["action_id"])
    print(resp)
    send_data(s, resp)

# Reboots the device
def reboot(action):
    payload = json.loads(action['payload'])
    print(payload)
    resp = action_complete(action["action_id"])
    print(resp)
    send_data(s, resp)
    os.system('sudo reboot')

def recv_actions():
    while True:
        action = recv_action(s)
        print("Received action %s"%str(action))

        action_name = action["name"]
        action_id = action["action_id"]
        action_payload = action["payload"]
        #resp = action_failed(action_id, "Action {name} does not exist".format(name=action_name))
        try:
            if action_name == "update_firmware":
                print("update_firmware action received")
                resp = update_firmware(action_id, action_payload)
            elif action_name == "send_file":
                resp = action_complete(action_id)
            elif action_name == "update_config":
                print("update_config action received")
                print(json.loads(action['payload']))
                update_config(action)
			elif action_name == "reboot":
            print("reboot action received")
            reboot(action)
        except Exception as e: 
            print(e)
            resp = action_failed(action_id, "Failed with exception: {msg}".format(msg=str(e)))

        send_data(s, resp)

print("Starting Uplink Bridge App")
threading.Thread(target=recv_actions).start()

def send_device_shadow(s, sequence):
    t = int(time.time()*1000)
    payload = {
        "stream": "device_shadow",
        "sequence": sequence,
        "timestamp": t,
        "Status": "running" 
    }

    send_data(s, payload)

sequence = 1
while True:
    time.sleep(5)
    send_device_shadow(s, sequence)
    sequence += 1
