FROM python:3.10.1-bullseye

WORKDIR /usr/src/app

#COPY requirements.txt ./
#RUN pip install --no-cache-dir -r requirements.txt

COPY . .

EXPOSE 12345

CMD [ "python", "server.py" ]
