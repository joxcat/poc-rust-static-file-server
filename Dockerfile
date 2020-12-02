FROM rust:slim
MAINTAINER Johan Planchon <dev@planchon.xyz>
EXPOSE 80
ENV LISTEN_ADDR=127.0.0.1
HEALTHCHECK CMD "if test $(curl -s -o /dev/null -I -w '%{http_code}' http://$LISTEN_ADDR:80/) -eq 404; then exit 0; else exit 1; fi'

RUN mkdir -p /var/www
VOLUME /var/www

RUN mkdir -p /root
COPY . /root/build

WORKDIR /root/build
RUN cargo build --release
RUN cp target/release/static-file-server ./

WORKDIR /var/www
ENTRYPOINT ["/root/build/static-file-server", "$LISTEN_ADDR:80"]