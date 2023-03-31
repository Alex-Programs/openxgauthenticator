import requests
from bs4 import BeautifulSoup

def update_most_common_ua():
    url = "https://www.whatismybrowser.com/guides/the-latest-user-agent/chrome"

    r = requests.get(url)

    if r.status_code != 200:
        print(f"Non 200 status code; abort ({str(r.status_code)})")
        import sys; sys.exit()

    # parse with bs4
    parsed = BeautifulSoup(r.text, "html.parser")

    code_blocks = parsed.find_all("span", attrs={"class": "code"})

    print(code_blockcs)

    return code_blocks

ua = update_most_common_ua()

with open("most_common_ua.txt", "w") as f:
    f.write(ua)