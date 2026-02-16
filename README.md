# IRyS Birthday Website

A birthday fan message website for IRyS. Fan messages float across the screen as animated pastel bubbles with a frosted glass birthday banner at the center.

## Stack

- **Frontend**: Svelte 5 + Vite
- **Backend**: Rust + Actix Web

## Project Structure

```
IRyS-Bday/
├── Dockerfile                 # Multi-stage Docker build
├── data/messages.csv          # Fan messages (Name,Message)
├── backend/
│   ├── Cargo.toml
│   └── src/main.rs            # API server + static file serving
└── frontend/
    ├── vite.config.js
    ├── index.html
    └── src/
        ├── app.css            # Animated gradient, keyframes, sparkles
        ├── App.svelte
        └── lib/
            ├── api.js              # API fetch helper
            ├── Sparkles.svelte     # Background particle effects
            ├── Header.svelte       # Cake banner, message list & video overlays
            ├── Footer.svelte       # Powered-by credit link
            ├── MessageCanvas.svelte # Spawns floating bubbles
            ├── MessageBubble.svelte # Draggable glassmorphism message card
            └── UserGuide.svelte    # First-visit interaction guide
```

## Development

```bash
# Terminal 1 — Backend
cd backend && cargo run

# Terminal 2 — Frontend
cd frontend && npm install && npm run dev
```

Open http://localhost:5173

## Production

```bash
cd frontend && npm run build
cd ../backend && cargo run
```

Open http://localhost:8080

## Environment Variables

All server parameters are configurable via environment variables:

| Variable | Description |
|---|---|
| `BIND_ADDR` | Listen address and port |
| `ALLOWED_ORIGIN` | CORS allowed origin |
| `STATIC_DIR` | Path to frontend dist |
| `RATE_LIMIT_SECS` | Seconds between allowed requests per IP |
| `RATE_LIMIT_BURST` | Max burst before throttling |
| `MAX_CONNECTIONS` | Max concurrent connections |
| `KEEP_ALIVE_SECS` | Keep-alive timeout |
| `REQUEST_TIMEOUT_SECS` | Client request timeout |
| `WORKERS` | Number of worker threads |
| `MAX_PAYLOAD_BYTES` | Max request body size |
| `POWERED_BY_NAME` | Footer credit name (default: HyunJa) |
| `POWERED_BY_LINK` | Footer credit link (default: https://x.com/hyunja_0423) |

## Security

### Data Handling
- Messages embedded at compile time via `include_str!` (no runtime file access or disk I/O)
- CSV field length limits enforced at parse time (name: 200 chars, message: 5,000 chars)
- Pre-serialized JSON responses cached once at startup (zero per-request serialization)

### Input Validation
- `ALLOWED_ORIGIN` must be a valid HTTP(S) URL (assert on startup)
- `POWERED_BY_LINK` must be an HTTPS URL (assert on startup)
- `env_or` logs warnings on parse failures instead of silently falling back
- Max request payload size enforced (`MAX_PAYLOAD_BYTES`, default 1 KB)

### HTTP Security Headers
- **Content-Security-Policy**: blocks inline scripts, eval, and unauthorized external resources; `object-src 'none'`, `form-action 'none'`, `base-uri 'self'`, `navigate-to` restricted
- **Strict-Transport-Security**: HSTS with 1-year max-age and includeSubDomains
- **X-Frame-Options**: DENY (prevents clickjacking)
- **X-Content-Type-Options**: nosniff (prevents MIME-type sniffing)
- **X-XSS-Protection**: enabled with block mode
- **Referrer-Policy**: strict-origin-when-cross-origin
- **Permissions-Policy**: camera, microphone, geolocation all disabled

### Network Protection
- CORS restricted to a single configured origin (GET only)
- Rate limiting per IP via actix-governor (configurable burst and interval)
- Max concurrent connection limit (`MAX_CONNECTIONS`)
- Client request timeout and keep-alive timeout enforced
- gzip compression via Actix `Compress` middleware
- ETag and Last-Modified headers for static file caching

### Frontend
- Production source maps disabled (`sourcemap: false` in Vite build)
- Footer link URL validated against allowed protocols (no `javascript:` or open redirect)
- No error objects logged to console (prevents information leakage)

## Features

- **Floating message bubbles** — fan messages rise as animated pastel cards with kawaii faces and party hats
- **Tap to expand** — tap/click a bubble to read the full message; auto-releases after 30 seconds
- **Drag to move** — drag bubbles freely around the screen
- **Birthday cake banner** — interactive cake with animated candles; click anywhere to dismiss
- **Message list** — browse all messages in a scrollable overlay panel
- **Video player** — embedded IRyStocrats birthday video
- **User guide** — first-visit overlay explaining how to interact; re-openable via help button
- **Sparkle effects** — background particle animation

## Docker

```bash
docker build -t irys-bday .
docker run -p 8080:8080 \
  -e ALLOWED_ORIGIN=http://your-host:8080 \
  -e RATE_LIMIT_BURST=20 \
  irys-bday
```

All environment variables above can be passed with `-e`.

### Production (Nginx reverse proxy)

```bash
# Build and run on localhost only (Nginx handles public traffic)
docker run -d --restart unless-stopped \
  -p 127.0.0.1:8080:8080 \
  -e ALLOWED_ORIGIN=https://your-domain.com \
  irys-bday
```

Configure Nginx to proxy ports 80/443 to `127.0.0.1:8080` with SSL (e.g. Cloudflare Origin Certificate).

## Adding Messages

Edit `data/messages.csv` (quote fields that contain commas) and recompile the backend:

```csv
Name,Message
FanName,"Happy birthday, IRyS!"
```

## Acknowledgements

I used [Claude Code](https://claude.ai/code) for front-end design.

## Author

**HyunJa** — [@hyunja_0423](https://x.com/hyunja_0423)

## License

See [LICENSE](LICENSE).
