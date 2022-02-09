from flask import Flask, render_template, send_file
import waitress

app = Flask(__name__)


@app.route("/")
@app.route("/index")
def index():
    return render_template("index.html")


@app.route("/downloads/<path:filename>")
def download(filename):
    return send_file("downloads/" + filename, as_attachment=True)


waitress.serve(app, port=4264)