# Stage 1: Build frontend
FROM node:22-alpine AS frontend-build
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

# Stage 2: Build backend
FROM rust:1-alpine AS backend-build
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY backend/ ./backend/
COPY data/ ./data/
COPY --from=frontend-build /app/frontend/dist ./frontend/dist
RUN cargo build --release --manifest-path backend/Cargo.toml

# Stage 3: Runtime
FROM alpine:3.21
RUN apk add --no-cache ca-certificates
RUN adduser -D -u 1000 appuser
WORKDIR /app

COPY --from=backend-build /app/backend/target/release/backend ./backend
COPY --from=frontend-build /app/frontend/dist ./static/
RUN chown -R appuser:appuser /app

USER appuser

ENV BIND_ADDR=0.0.0.0:8080
ENV STATIC_DIR=/app/static
ENV ALLOWED_ORIGIN=http://localhost:8080
ENV RATE_LIMIT_SECS=1
ENV RATE_LIMIT_BURST=30
ENV MAX_CONNECTIONS=4096
ENV KEEP_ALIVE_SECS=10
ENV REQUEST_TIMEOUT_SECS=5
ENV WORKERS=4
ENV MAX_PAYLOAD_BYTES=1024
ENV POWERED_BY_NAME=HyunJa
ENV POWERED_BY_LINK=https://x.com/hyunja_0423

EXPOSE 8080

CMD ["./backend"]
