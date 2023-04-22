# Build Stage
FROM rust:1.66.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/gat-api
COPY . .

RUN cargo install --path .

# Running stage
FROM gcr.io/distroless/cc-debian10 as run

COPY ./key.json ./key.json
COPY --from=build /usr/local/cargo/bin/gat-api /usr/local/bin/gat-api

CMD ["gat-api"]


