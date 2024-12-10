FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
LABEL maintainer="Jetsung Chan<jetsungchan@gmail.com>"

# RUN apt-get update && apt-get install -y locales libssl3 libssl-dev && rm -rf /var/lib/apt/lists/* \
# 	&& localedef -i zh_CN -c -f UTF-8 -A /usr/share/locale/locale.alias zh_CN.UTF-8
# ENV LANG zh_CN.utf8
RUN apt update && \
    apt install -y deborphan openssl libcurl4 libssl-dev && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /var/cache/apt/archives/* && \
    deborphan | xargs apt -y remove --purge

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/filetas /usr/local/bin/filetas
COPY --from=builder /app/static /app/static
COPY --from=builder /app/templates /app/templates


EXPOSE 8000
CMD [ "filetas" ]
