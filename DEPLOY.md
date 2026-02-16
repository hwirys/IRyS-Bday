# Deployment Guide — Synology NAS + Docker + Nginx/Cloudflare

## Architecture

```
Internet → Cloudflare (DDoS + CDN + TLS)
         → Synology NAS
           → Nginx (reverse proxy, port 443/80)
             → Docker: irys-bday (port 8080, internal)
```

---

## Step 1: Build & Run Docker on Synology

### Option A: Build on Synology (if it has enough RAM/CPU)

SSH into your Synology and clone the repo:

```bash
git clone https://github.com/hwirys/IRyS-Bday.git
cd IRyS-Bday
docker build -t irys-bday .
```

### Option B: Build on your Mac, transfer to Synology

```bash
# On your Mac
cd /path/to/IRyS-Bday
docker build --platform linux/amd64 -t irys-bday .
docker save irys-bday | gzip > irys-bday.tar.gz

# Transfer to Synology
scp irys-bday.tar.gz user@synology-ip:/tmp/

# On Synology (SSH)
docker load < /tmp/irys-bday.tar.gz
```

> Use `linux/amd64` for Intel-based Synology (most models).
> Use `linux/arm64` for ARM-based Synology (e.g., DS223j).

### Run the container

```bash
docker run -d \
  --name irys-bday \
  --restart unless-stopped \
  -p 127.0.0.1:8080:8080 \
  -e ALLOWED_ORIGIN=https://your-domain.com \
  -e WORKERS=4 \
  -e MAX_CONNECTIONS=4096 \
  -e RATE_LIMIT_BURST=30 \
  -e KEEP_ALIVE_SECS=10 \
  -e POWERED_BY_NAME=HyunJa \
  -e POWERED_BY_LINK=https://x.com/hyunja_0423 \
  irys-bday
```

> Binding to `127.0.0.1:8080` so only nginx can reach it — not exposed to LAN directly.

---

## Step 2: Nginx Reverse Proxy

### Install Nginx on Synology

Option 1: Use Synology's built-in **Web Station** + reverse proxy settings in DSM.

Option 2: Run Nginx as another Docker container (recommended):

Create `nginx.conf`:

```nginx
worker_processes auto;

events {
    worker_connections 2048;
}

http {
    # Rate limiting zone — 10 req/s per IP
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_conn_zone $binary_remote_addr zone=conn:10m;

    # Timeouts to prevent slowloris
    client_body_timeout 5s;
    client_header_timeout 5s;
    send_timeout 10s;
    keepalive_timeout 15s;

    # Max body size (no uploads needed)
    client_max_body_size 1k;

    # Hide nginx version
    server_tokens off;

    # Gzip (double compression with backend, nginx handles it efficiently)
    gzip on;
    gzip_types text/css application/javascript application/json;
    gzip_min_length 256;

    upstream backend {
        server host.docker.internal:8080;
        # If not using Docker for nginx, use:
        # server 127.0.0.1:8080;
    }

    server {
        listen 80;
        server_name your-domain.com;

        # Redirect HTTP to HTTPS (when using Cloudflare or own certs)
        return 301 https://$host$request_uri;
    }

    server {
        listen 443 ssl http2;
        server_name your-domain.com;

        # TLS certs (use Cloudflare origin certs or Let's Encrypt)
        ssl_certificate /etc/nginx/certs/cert.pem;
        ssl_certificate_key /etc/nginx/certs/key.pem;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers HIGH:!aNULL:!MD5;

        # Connection limit per IP
        limit_conn conn 20;

        # API endpoint — rate limited
        location /api/ {
            limit_req zone=api burst=20 nodelay;
            proxy_pass http://backend;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header Host $host;
        }

        # Static files — aggressive caching
        location /assets/ {
            proxy_pass http://backend;
            proxy_set_header Host $host;
            expires 1y;
            add_header Cache-Control "public, immutable";
        }

        # Root
        location / {
            proxy_pass http://backend;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header Host $host;
        }

        # Block common attack paths
        location ~ /\. {
            deny all;
        }
    }
}
```

### Run Nginx Docker container

```bash
docker run -d \
  --name nginx-proxy \
  --restart unless-stopped \
  -p 80:80 \
  -p 443:443 \
  -v /path/to/nginx.conf:/etc/nginx/nginx.conf:ro \
  -v /path/to/certs:/etc/nginx/certs:ro \
  --add-host=host.docker.internal:host-gateway \
  nginx:alpine
```

### Or use Docker Compose

Create `docker-compose.yml` in the project root:

```yaml
services:
  app:
    build: .
    container_name: irys-bday
    restart: unless-stopped
    expose:
      - "8080"
    environment:
      - ALLOWED_ORIGIN=https://your-domain.com
      - WORKERS=4
      - MAX_CONNECTIONS=4096
      - RATE_LIMIT_BURST=30
      - KEEP_ALIVE_SECS=10
      - POWERED_BY_NAME=HyunJa
      - POWERED_BY_LINK=https://x.com/hyunja_0423

  nginx:
    image: nginx:alpine
    container_name: nginx-proxy
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./certs:/etc/nginx/certs:ro
    depends_on:
      - app
```

> When using compose, change `host.docker.internal:8080` to `app:8080` in `nginx.conf`.

---

## Step 3: Cloudflare (Free Tier)

### DNS Setup

1. Add your domain to Cloudflare
2. Point an A record to your Synology NAS public IP
3. Enable the orange cloud (proxy) icon

### Cloudflare Settings

Go to your domain dashboard and configure:

**SSL/TLS → Overview:**
- Set to **Full (Strict)**

**SSL/TLS → Origin Server:**
- Create an **Origin Certificate**
- Download `cert.pem` and `key.pem`
- Place them in your Synology certs directory for nginx

**Security → WAF:**
- Enable **Bot Fight Mode**
- Enable **Browser Integrity Check**

**Security → DDoS:**
- Enabled by default on free tier

**Security → Settings:**
- Security Level: **Medium** or **High**
- Challenge Passage: **30 minutes**

**Caching → Configuration:**
- Caching Level: **Standard**
- Browser Cache TTL: **1 day**

**Speed → Optimization:**
- Enable **Auto Minify** (JS, CSS, HTML)
- Enable **Brotli** compression

**Rules → Page Rules (optional):**
- `your-domain.com/assets/*` → Cache Level: Cache Everything, Edge TTL: 1 month

### Cloudflare Firewall Rules (free tier allows 5)

1. **Block non-GET requests:**
   - If `http.request.method` is not `GET` → Block

2. **Rate limit API:**
   - If `URI Path` contains `/api/` and rate exceeds 30 req/10s → Block

3. **Block known bad bots:**
   - If `Known Bots` is false and `Threat Score` > 30 → Challenge

### Port Forwarding on Router

Forward these ports to your Synology NAS internal IP:
- **80** → Synology:80
- **443** → Synology:443

> Do NOT forward port 8080 — nginx handles all external traffic.

---

## Summary

| Layer | Protection |
|---|---|
| **Cloudflare** | DDoS mitigation, CDN caching, WAF, bot detection, TLS |
| **Nginx** | Rate limiting, connection limits, slowloris prevention, static file caching |
| **Backend** | Per-IP rate limiting, CSP, security headers, pre-serialized responses, gzip/brotli |

### Quick Verification

```bash
# Test from outside your network
curl -I https://your-domain.com
# Should see: Cloudflare headers + security headers

curl https://your-domain.com/api/messages
# Should see: JSON array of messages
```
