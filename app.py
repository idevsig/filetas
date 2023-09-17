# -*- coding: utf-8 -*-
import requests
import os
import urllib.parse
from flask import Flask, Response, abort, request, stream_with_context, send_from_directory, render_template
from flask_basicauth import BasicAuth

app = Flask(__name__)

# BasicAuth
USERNAME = os.getenv('USERNAME', '')
PASSWORD = os.getenv('PASSWORD', '')
BASIC_AUTH_FORCE = bool(USERNAME) or bool(PASSWORD)

app.config['BASIC_AUTH_USERNAME'] = USERNAME
app.config['BASIC_AUTH_PASSWORD'] = PASSWORD
app.config['BASIC_AUTH_FORCE'] = BASIC_AUTH_FORCE  # 整个站点都验证
basic_auth = BasicAuth(app)


# 将Flask应用程序包装成 WSGI 可调用对象
wsgi_app = app.wsgi_app

# 读取环境变量
HOST = os.getenv('HOST', '0.0.0.0')
PORT = int(os.getenv('PORT', 5000))
DEBUG = bool(os.getenv('DEBUG', False))
CHUNK_SIZE = int(os.getenv('CHUNK_SIZE', 10240))
TITLE = os.getenv('TITLE', '文件加速下载')
FILE_EXT = os.getenv('FILE_EXT', '')

FILE_EXT_LIST = FILE_EXT.split(',')


@app.route('/')
def index():
    return render_template('index.html', title=TITLE)


@app.route('/favicon.ico')
def favicon():
    if os.path.exists('static/favicon.ico'):
        return send_from_directory(app.static_folder, 'favicon.ico')
    else:
        abort(404)


@app.route('/robots.txt')
def robots_txt():
    content = 'User-agent:*\nDisallow:/'
    return Response(content, content_type='text/plain')


@app.route('/<path:uri>')
@stream_with_context
def proxy(uri):
    # 若 URL 不含 'https://', 'http://'，则添加 https://
    if not uri.startswith(('https://', 'http://')):
        if len(FILE_EXT_LIST) > 0 and is_download_url(uri):
            uri = 'https://' + uri

    # 判断不为下载链接
    if not uri.startswith(('https://', 'http://')):
        info = '下载链接必须含 "https://" 或 "http://"'
        return render_template('error.html', info=info)

    # 构建要发送的请求对象
    req = requests.Request(method='GET', url=uri)

    # 发送请求并获取响应
    with requests.Session() as sess:
        resp = sess.send(req.prepare(), stream=True)

    # 构建流式响应对象
    def generate():
        for chunk in resp.iter_content(chunk_size=CHUNK_SIZE):
            if chunk:
                yield chunk

    # 返回流式响应
    headers = dict(resp.headers)
    headers['Access-Control-Allow-Origin'] = '*'
    headers['Access-Control-Allow-Methods'] = 'GET'
    headers['Access-Control-Allow-Headers'] = 'Content-Type, Authorization'
    return Response(generate(), headers=headers, status=resp.status_code)


def is_download_url(url):
    url_path = urllib.parse.urlparse(url).path
    filename, extension = os.path.splitext(url_path)
    if extension == '':
        return False
    return extension.lower() in FILE_EXT_LIST


if __name__ == '__main__':
    app.run(host=HOST, port=PORT, debug=DEBUG)
