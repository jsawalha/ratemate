
# # stage 1
# FROM rust:1.88 as builder
# WORKDIR /app
# COPY Cargo.toml Cargo.lock ./
# RUN mkdir src && echo 'fn main() {}' > src/main.rs
# RUN cargo build --release
# COPY src ./src
# RUN touch src/main.rs
# RUN cargo build --release

# #stage 2
# FROM debian:bookworm-slim
# WORKDIR /app
# COPY --from=builder /app/target/release/ratemate .
# EXPOSE 8000
# CMD ["./ratemate"]

#stage 1
FROM rust:1.88 as builder
WORKDIR /app
COPY . .
RUN cargo build --release


#stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/ratemate .
COPY static ./static
EXPOSE 8000
CMD ["./ratemate"]