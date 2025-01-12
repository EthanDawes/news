import dotenv
from tqdm import tqdm
import requests

import ssl
import smtplib
import os
from datetime import date, timedelta
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from email.message import EmailMessage


def get_mailing_period():
    """
    Returns the date for which this email is for, to be displayed in the subject
    If before 21st day, return previous month. Afterwards, do this month
    """
    today = date.today()
    if today.day > 20:
        return today
    else:
        return today - timedelta(days=today.day)


dotenv.load_dotenv()
port = 465  # For SSL
addr = os.environ["EMAIL_ADDR"]
password = os.environ["EMAIL_PW"]
mail = os.environ["MAILING_ADDR"]

# Create a secure SSL context
context = ssl.create_default_context() 

def get_newsletter():
    """
    Get the most recent newsletter
    """
    period = get_mailing_period().strftime("%Y/%b")
    response = requests.get("https://funblaster22.github.io/news/" + period)
    return response.text


msg = get_newsletter()
msg = msg.replace("$address", mail)

print(msg)
period = get_mailing_period().strftime("%B %Y")
subject = f"Ethan's {period} life update!"
print("subject:", subject)
confirm = input("\nWill send the above. Proceed? ")
if len(confirm) == 0 or confirm[0] != "y":
    raise KeyboardInterrupt("Action aborted!")

with smtplib.SMTP_SSL("smtp.gmail.com", port, context=context) as server:
    server.login(addr, password)
    with open("recipients.csv") as recipients:
        for recipient in tqdm(recipients.readlines()):
            name, email = recipient.split(",")
            # Annoyingly, EmailMessage() and .set_content shows the headers and strange artifacts from Content-Transfer-Encoding: quoted-printable
            formattedMsg = MIMEMultipart("alternative")
            formattedMsg['Subject'] = subject
            formattedMsg['From'] = addr
            formattedMsg['To'] = email
            msg = msg.replace("$name", name)
            msg = msg.replace("$email", email)
            formattedMsg.attach(MIMEText(msg, "html", "utf-8"))

            server.send_message(formattedMsg)

print("All mail sent!")
