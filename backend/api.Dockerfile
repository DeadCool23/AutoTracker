FROM rust:latest

RUN apt-get update && apt-get install -y make

WORKDIR /app

COPY ./api .
COPY .run-api.sh .
RUN chmod +x .run-api.sh

CMD [ "./.run-api.sh" ]