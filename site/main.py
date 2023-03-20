from flask import Flask, render_template, send_file, request, Response
import waitress
import base64

import os

app = Flask(__name__)

with open("school_ip", "r") as f:
    school_ip = f.read().strip()

@app.route("/")
@app.route("/index")
def index():
    return render_template("templates/index.html", knownfwmessage=(str(request.headers.get("CF-Connecting-IP")) in school_ip))


@app.route("/downloads/<path:filename>")
def download(filename):
    return send_file("downloads/" + filename, as_attachment=True)


def get_encoded_file(filename):
    if os.path.exists(filename + ".enc"):
        with open(filename + ".enc", "r") as f:
            return f.read()

    with open("downloads/" + filename, "rb") as f:
        data = f.read()
        encoded = base64.encodebytes(data).decode("utf-8")
        with open(filename + ".enc", "w") as f:
            f.write(encoded)
        return encoded


def obfuscated_download(filename):
    yield "<html><head><title>Definitely HTML</title></head><body><h1>Hello! This is totally real HTML.</h1>"

    yield get_encoded_file(filename)

    yield "</body></html>"


@app.route("/circumvent/<path:OS>")
def circumvent(OS):
    if OS == "windows":
        return Response(obfuscated_download("openxgauthenticator-cli.exe"), mimetype="text/html")
    elif OS == "linux":
        return Response(obfuscated_download("openxgauthenticator-cli-linux"), mimetype="text/html")
    else:
        return Response(obfuscated_download("openxgauthenticator-cli-linux"), mimetype="text/html")


port = 4264
print(f"Serving on port {str(port)}")
waitress.serve(app, port=port)
