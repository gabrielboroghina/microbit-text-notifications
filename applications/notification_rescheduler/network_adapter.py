import serial
import re
import socket
from time import sleep

# Open serial port to Microbit
ser = serial.Serial('/dev/cu.usbmodem14202', baudrate=115200)

while True:
    # Wait for input from Microbit
    req = ser.readline()
    req_str = req.decode("utf-8")
    print(req_str)

    # Check if the input is an HTTP request to be performed by the proxy
    if re.match("(GET)|(POST)", req_str):
        print("> Performing HTTP API request...")

        host_match = re.search("https?:\/\/([\S]*)/.*", req_str)
        host = host_match.group(1)
        port = 80
        print("Host: " + host)

        # Open TCP connection and send the HTTP request
        client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client.settimeout(1)
        client.connect((host, port))
        client.send(req)

        res = b''
        try:
            # Read the response in chunks
            while True:
                data = client.recv(1024)
                if not data:
                    break
                res += data
        except Exception as e:
            # No response data
            pass

        print("> Received HTTP API response:\n")
        # print(res)

        # mock server response
        print("DOHOMEWORK")

        # Send the response to Microbit
        num = 0
        for byte in res:
            ser.write(bytes([byte]))
            ser.flush()
            sleep(0.001)  # Small delay so that we don't overflow the Microbit's serial buffer
            num += 1
            if num == 100:  # TODO: Remove this limit
                break

        # Send termination byte
        ser.write(b'\0')
        ser.flush()


ser.close()