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

# Stage 3: Runtime — nginx serves static files, proxies API to backend
FROM nginx:alpine

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=backend-build /app/backend/target/release/backend ./backend
COPY --from=frontend-build /app/frontend/dist /app/static/
COPY nginx/nginx.conf /etc/nginx/nginx.conf
COPY nginx/entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh

# Create tmp dirs for nginx
RUN mkdir -p /tmp/nginx_body /tmp/nginx_proxy /tmp/nginx_fastcgi /tmp/nginx_uwsgi /tmp/nginx_scgi

ENV BIND_ADDR=0.0.0.0:3000
ENV ALLOWED_ORIGIN=https://birthday5th.irys.best
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

ENTRYPOINT ["./entrypoint.sh"]
