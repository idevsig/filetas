FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12
LABEL maintainer="Jetsung Chan<i@jetsung.com"

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/filetas /usr/local/bin/filetas
COPY --from=builder /app/static /app/static
COPY --from=builder /app/templates /app/templates

EXPOSE 8000

ENTRYPOINT ["filetas"]
