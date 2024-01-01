FROM caddy:latest

RUN apk add curl git
RUN useradd -ms /bin/sh user
USER user

ADD .. /cheetah-src
WORKDIR /cheetah-src