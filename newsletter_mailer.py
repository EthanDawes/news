import requests

from datetime import date, timedelta

from mailer import send_personalized_mail


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
    response = requests.get("https://ethandawes.github.io/news/" + period)
    if response.status_code != 200:
        raise ConnectionError("Page returned status " + str(response.status_code))
    return response.text


msg = get_newsletter()
period = get_mailing_period().strftime("%B %Y")
subject = f"Ethan's {period} life update!"
send_personalized_mail(msg, subject)
