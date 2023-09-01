import dotenv
import ssl
import smtplib
import os
from email.message import EmailMessage

dotenv.load_dotenv()
port = 465  # For SSL
addr = os.environ["EMAIL_ADDR"]
password = os.environ["EMAIL_PW"]

# Create a secure SSL context
context = ssl.create_default_context()

with open("message.txt", encoding="utf8") as file:
    msg = file.read()

print(msg)
confirm = input("\nWill send the above. Proceed? ")
if len(confirm) == 0 or confirm[0] != "y":
    raise KeyboardInterrupt("Action aborted!")

with smtplib.SMTP_SSL("smtp.gmail.com", port, context=context) as server:
    server.login(addr, password)
    with open("recipients.csv") as recipients:
        for recipient in recipients.readlines():
            name, email = recipient.split(",")
            formattedMsg = EmailMessage()
            formattedMsg['Subject'] = "Ethan's life update!"
            formattedMsg['From'] = addr
            formattedMsg['To'] = email
            formattedMsg.set_content(msg.replace("$name", name))

            server.send_message(formattedMsg)

print("All mail sent!")
