import requests
from bs4 import BeautifulSoup

def update_most_common_ua():
    url = "https://www.whatismybrowser.com/guides/the-latest-user-agent/chrome"

    print("Getting...")
    r = requests.get(url)

    if r.status_code != 200:
        print(f"Non 200 status code; abort with status code: " + str(r.status_code))
        import sys; sys.exit()

    # parse with bs4
    parsed = BeautifulSoup(r.text, "html.parser")

    code_blocks = parsed.find_all("span", attrs={"class": "code"})

    most_common = None
    for block in code_blocks:
        if "Mozilla" in block.text or "Chrome" in block.text or "KHTML" in block.text or "Windows" in block.text or "x64" in block.text:
            most_common = block.text
            break

    if most_common is None:
        print("No most common user agent found; abort")
        import sys; sys.exit()

    print("Gotten: " + str(most_common))

    return most_common

ua = update_most_common_ua()

with open("most_common_ua.txt", "w") as f:
    f.write(ua)