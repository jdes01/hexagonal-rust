FROM rust:latest AS build

WORKDIR /usr/src/pizzeria_carlos

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/pizzeria_carlos*

COPY . .

RUN cargo build --release

# Etapa de ejecución
FROM debian:bullseye-slim

WORKDIR /usr/src/pizzeria_carlos

COPY --from=build /usr/src/pizzeria_carlos/target/release/pizzeria_carlos .

EXPOSE 8080

CMD ["./pizzeria_carlos"]
