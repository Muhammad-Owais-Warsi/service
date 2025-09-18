
FROM rustlang/rust:nightly as builder
WORKDIR /app


COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked || true



COPY . .
RUN cargo build --release


FROM rust:1.82-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/service-2 /usr/local/bin/service-2


EXPOSE 8080


CMD ["service-2"]
