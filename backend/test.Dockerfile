FROM rust:latest

RUN apt-get update && \
    apt-get install -y \
    curl \
    wget \
    unzip \
    gnupg \
    xmlstarlet \
    && rm -rf /var/lib/apt/lists/*
    
RUN apt-get update && \
    apt-get install -y \ 
    redis-server \
    postgresql \
    postgresql-contrib \
    && rm -rf /var/lib/apt/lists/*

    
RUN apt-get update && \
    apt-get install -y default-jdk && \
    rm -rf /var/lib/apt/lists/*

RUN ALLURE_VERSION=2.27.0 && \
    wget https://github.com/allure-framework/allure2/releases/download/${ALLURE_VERSION}/allure-${ALLURE_VERSION}.zip -O /tmp/allure.zip && \
    unzip /tmp/allure.zip -d /opt/ && \
    ln -s /opt/allure-${ALLURE_VERSION}/bin/allure /usr/bin/allure && \
    rm /tmp/allure.zip
    
WORKDIR /app

COPY ./api ./api

COPY ./db ./db

COPY ./db/data /data

COPY ./.env .

COPY .run-tests.sh .
RUN chmod +x .run-tests.sh

CMD [ "./.run-tests.sh" ]
