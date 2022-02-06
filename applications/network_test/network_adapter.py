import serial
import re
import socket
from time import sleep

# Open serial port to Microbit
ser = serial.Serial('/dev/ttyACM0', baudrate=115200)

while True:
    # Wait for input from Microbit
    req = ser.readline()
    req_str = req.decode("utf-8")
    print(req_str.strip())

    # Check if the input is an HTTP request to be performed by the proxy
    if re.match("(GET)|(POST)", req_str):
        if re.match("(POST)", req_str):
            # Wait for the body
            req += ser.readline()
            req += ser.readline()
            req += ser.readline()
            req += ser.readline()

        print("> Performing HTTP API request...")
        print(req)

        host_match = re.search("https?:\/\/([^\s:]*)(\:([0-9]*))?.*", req_str)
        if host_match is None:
            print("> Error: Invalid URL")
            continue
        host = host_match.group(1)
        port = int(host_match.group(3) or 80)
        print("Host:", host, "Port:", port)

        # Open TCP connection and send the HTTP request
        client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client.settimeout(1)

        res = b''
        try:
            client.connect((host, port))
            client.send(req + b'\r\n')

            # Read the response in chunks
            while True:
                data = client.recv(1024)
                if not data:
                    break
                res += data
        except Exception as e:
            # No response data
            print("> Error:", e)
            pass

        print("> Received HTTP API response:\n")
        print(res)

        # Send the response to Microbit
        num = 0
        for byte in res:
            ser.write(bytes([byte]))
            ser.flush()
            sleep(0.001)  # Small delay so that we don't overflow the Microbit's serial buffer
            num += 1

        # Send termination byte
        ser.write(b'\0')
        ser.flush()


ser.close()