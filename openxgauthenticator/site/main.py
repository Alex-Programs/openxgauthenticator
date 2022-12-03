from flask import Flask, render_template, send_file, request, Response
import waitress
import base64
import hashlib

app = Flask(__name__)


@app.route("/")
@app.route("/index")
def index():
    return render_template("index.html")


@app.route("/downloads/<path:filename>")
def download(filename):
    return send_file("downloads/" + filename, as_attachment=True)


def obfuscated_download(filename):
    yield "<html><head><title>Definitely HTML</title></head><body><h1>Hello! This is totally real HTML.</h1>"

    with open("downloads/" + filename, "rb") as f:
        data = f.read()
        encoded = base64.encodebytes(data).decode("utf-8")
        yield encoded

    yield "</body></html>"

@app.route("/circumvent/<path:OS>")
def circumvent(OS):
    if OS == "windows":
        return Response(obfuscated_download("openxgauthenticator.exe"), mimetype="text/html")
    elif OS == "linux":
        return Response(obfuscated_download("openxgauthenticator-linux"), mimetype="text/html")
    else:
        return Response(obfuscated_download("openxgauthenticator-linux"), mimetype="text/html")

port = 4264
print(f"Serving on port {str(port)}")
waitress.serve(app, port=port)
