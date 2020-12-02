FROM rust:slim
MAINTAINER Johan Planchon <dev@planchon.xyz>

RUN mkdir -p /var/www
VOLUME /var/www

RUN mkdir -p /root
COPY . /root/build

WORKDIR /root/build
RUN cargo build --release
RUN cp target/release/static-file-server /root/build/static-file-server

EXPOSE 8080
ENV LISTEN_ADDR [::1]

#HEALTHCHECK CMD "if test $(curl -s -o /dev/null -I -w '%{http_code}' http://${LISTEN_ADDR}:80/) -neq 404; then exit 1; fi"

WORKDIR /var/www
ENTRYPOINT ["/root/build/static-file-server", "127.0.0.1:8080"]
