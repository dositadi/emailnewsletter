FROM dhi.io/rust:1-alpine3.23-sfw-ent-dev

WORKDIR /app

COPY . .

RUN cargo fetch

