FROM rust:alpine AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine

COPY --from=builder /app/target/release/polymorphism_vehicles .

CMD ["./polymorphism_vehicles"]
