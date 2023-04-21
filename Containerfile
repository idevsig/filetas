FROM python:slim
LABEL maintainer="Jetsung Chan<jetsungchan@gmail.com>"

ARG PIP_MIRROR_URL="https://pypi.org/simple"

WORKDIR /app

COPY . .

RUN pip install -i $PIP_MIRROR_URL -r requirements.txt --no-cache --no-cache-dir

EXPOSE 8000

CMD [ "gunicorn", "-b", "0.0.0.0:8000","app:app" ]
