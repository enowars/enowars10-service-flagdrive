FROM python@sha256:58a54fbf1d5320af4f6761f9c6721329320a46ea086b5504640f19d1b7467d6a
RUN apt-get update -y && apt-get upgrade -y 

# add checker user
RUN useradd -ms /bin/bash -u 1000 checker

WORKDIR /checker

COPY requirements.txt .

USER checker
RUN python3 -m pip install -r requirements.txt

COPY ./src .

ENTRYPOINT [ "/home/checker/.local/bin/gunicorn", "-c", "gunicorn.conf.py", "checker:app" ]
