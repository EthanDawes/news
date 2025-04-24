import dotenv
from tqdm import tqdm

import os
import ssl
import smtplib
from urllib import parse
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from email.message import EmailMessage

dotenv.load_dotenv()
port = 465  # For SSL
addr = os.environ["EMAIL_ADDR"]
password = os.environ["EMAIL_PW"]
mail = os.environ["MAILING_ADDR"]

# Create a secure SSL context
context = ssl.create_default_context()


def send_personalized_mail(msg, subject, recipients_file="recipients.csv"):
    msg = msg.replace("$address", mail)
    print(msg)
    print("subject:", subject)
    confirm = input("\nWill send the above. Proceed? ")
    if len(confirm) == 0 or confirm[0] != "y":
        raise KeyboardInterrupt("Action aborted!")

    with smtplib.SMTP_SSL("smtp.gmail.com", port, context=context) as server:
        server.login(addr, password)
        with open(recipients_file) as recipients:
            for recipient in tqdm(recipients.readlines()):
                name, email = recipient.split(",")
                tracking = parse.quote(name)  # URI encoded for consistent tracking urls (see #12)
                # Annoyingly, EmailMessage() and .set_content shows the headers and strange artifacts from Content-Transfer-Encoding: quoted-printable
                formattedMsg = MIMEMultipart("alternative")
                formattedMsg['Subject'] = subject
                formattedMsg['From'] = addr
                formattedMsg['To'] = email
                personal_msg = msg.replace("$name", name)
                personal_msg = personal_msg.replace("$email", email)
                personal_msg = personal_msg.replace("$tracking", tracking)
                formattedMsg.attach(MIMEText(personal_msg, "html", "utf-8"))

                server.send_message(formattedMsg)

    print("All mail sent!")


def read_file(path):
    try:
        with open(path) as file:
            return file.read()
    except FileNotFoundError:
        raise argparse.ArgumentTypeError(f"File not found: {path}")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("subject", nargs="?", default=None)
    parser.add_argument("--message", type=read_file, default="message.html", help="HTML message file")
    parser.add_argument("--recipients", type=str, default="recipients.csv", help="CSV recipients file")

    args = parser.parse_args()
    send_personalized_mail(args.message, args.subject if args.subject is not None else input("Subject: "), args.recipients)
