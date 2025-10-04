import requests

from datetime import date, timedelta

from mailer import send_personalized_mail

URL_PREFIX = "https://ethandawes.github.io/news/"

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


def get_newsletter():
    """
    Get the most recent newsletter
    """
    period = get_mailing_period().strftime("%Y/%b")
    response = requests.get(URL_PREFIX + period)
    if response.status_code != 200:
        raise ConnectionError("Could not get " + URL_PREFIX + period + ". Status " + str(response.status_code))
    return response.text


period = get_mailing_period().strftime("%B %Y")
try:
    msg = get_newsletter()
except ConnectionError as err:
    print(err)
    msg = requests.get(input("URL of page to send: ")).text
    period = input("For which month is this newsletter? ")
subject = f"Ethan's {period} life update!"
send_personalized_mail(msg, subject)
