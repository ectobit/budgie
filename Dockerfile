# syntax=docker/dockerfile:1.4
FROM rust:1.65.0 AS builder

ARG TARGETPLATFORM

WORKDIR /root

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} \
    cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/root/target,id=${TARGETPLATFORM} \
    cargo build --release && \
    cargo strip && \
    mv /root/target/release/budgie /root


FROM gcr.io/distroless/cc-debian11

COPY --from=builder /root/budgie /

ENTRYPOINT ["./budgie"]

EXPOSE 3000
