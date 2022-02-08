from flask import Flask, render_template, send_file
import waitress

app = Flask(__name__)


@app.route("/")
@app.route("/index")
def index():
    return render_template("index.html")


@app.route("/downloads/openxgauthenticator-linux")
def linux():
    return send_file("../target/release/openxgauthenticator", as_attachment=True)


@app.route("/downloads/openxgauthenticator.exe")
def windows():
    return send_file("../target/x86_64-pc-windows-gnu/release/openxgauthenticator.exe", as_attachment=True)


waitress.serve(app, port=4264)
