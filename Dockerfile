###############################################################################
## Builder
###############################################################################
FROM rust:alpine3.20 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

RUN apk add --update --no-cache \
            autoconf \
            gcc \
            gdb \
            make \
            musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release && \
    cp /app/target/release/den /app/den

###############################################################################
## Final image
###############################################################################
FROM alpine:3.20

RUN apk add --update --no-cache \
            tzdata~=2024 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*

# Copy our build
COPY --from=builder /app/den /app/

# Set the work dir
WORKDIR /app

CMD ["/app/den"]
