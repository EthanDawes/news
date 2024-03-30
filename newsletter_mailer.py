import dotenv
import ssl
import smtplib
import os
from tqdm import tqdm
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

# Create a secure SSL context
context = ssl.create_default_context()

# Word sometimes produces invalid unicode, so ignore it
with open("message.html", encoding="utf8", errors='ignore') as file:
    msg = file.read()

print(msg)
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
            period = get_mailing_period().strftime("%B %Y")
            formattedMsg['Subject'] = f"Ethan's {period} life update!"
            formattedMsg['From'] = addr
            formattedMsg['To'] = email
            formattedMsg.attach(MIMEText(msg.replace("$name", name), "html", "utf-8"))

            server.send_message(formattedMsg)

print("All mail sent!")
