# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

IRyS 5th Birthday fan message website — a celebration site where fan messages float as interactive bubbles over an animated background with a birthday cake banner. Rust backend + Svelte 5 frontend, served from a single container.

## Development Commands

### Frontend (Svelte 5 + Vite)
```bash
cd frontend && npm install    # install dependencies
cd frontend && npm run dev    # dev server at http://localhost:5173
cd frontend && npm run build  # production build to frontend/dist/
```

### Backend (Rust + Actix Web)
```bash
cd backend && cargo run       # API server at http://localhost:8080
cd backend && cargo check     # type-check without building
```

Run both simultaneously for development — Vite proxies `/api/*` requests to `localhost:8080`.

### Docker
```bash
docker build -t irys-bday .
docker run -p 8080:8080 -e ALLOWED_ORIGIN=http://localhost:8080 irys-bday
```

## Architecture

**Two-process monorepo:**
- `backend/src/main.rs` — single-file Actix Web server (~200 lines). Embeds `data/messages.csv` at compile time via `include_str!`. Pre-serializes JSON responses at startup. API only in Docker (nginx serves static files).
- `frontend/src/` — Svelte 5 SPA using runes (`$state`, `$derived`, `$props`). No TypeScript.

**API endpoints:**
- `GET /api/messages` — returns all fan messages as JSON array
- `GET /api/config` — returns powered_by name and link for footer

**Data flow:** Messages CSV → compiled into backend binary → served as JSON → frontend spawns floating bubble components with random positions, colors, and durations.

**Key frontend components** (all in `frontend/src/lib/`):
- `Header.svelte` — birthday cake banner with candles, message list overlay, YouTube video overlay
- `MessageCanvas.svelte` — manages bubble lifecycle and spawning queue
- `MessageBubble.svelte` — draggable, expandable floating card with pointer capture
- `UserGuide.svelte` — first-visit tutorial (uses localStorage flag)
- `Sparkles.svelte` — background particle effects

## Important Patterns

- **Svelte 5 runes syntax** — uses `$state()`, `$derived()`, `$props()`, not legacy `$:` or `export let`
- **CSS variables** define 10 pastel bubble colors at `:root` in `app.css`
- **Responsive sizing** uses `clamp()` throughout; mobile-first with `100dvh`
- **Backend config** is entirely via environment variables (see `main.rs` for defaults): `BIND_ADDR`, `ALLOWED_ORIGIN`, `STATIC_DIR`, `RATE_LIMIT_SECS`, `RATE_LIMIT_BURST`, `MAX_CONNECTIONS`, `KEEP_ALIVE_SECS`, `REQUEST_TIMEOUT_SECS`, `WORKERS`, `MAX_PAYLOAD_BYTES`, `POWERED_BY_NAME`, `POWERED_BY_LINK`
- **Security headers** (CSP, HSTS, X-Frame-Options, etc.) are set by both nginx (`nginx/nginx.conf` for static files) and the backend middleware (for API responses)
- **Messages CSV format:** `Name,Message` header, quote fields containing commas, validated at startup (name ≤ 200 chars, message ≤ 5,000 chars)
- **Docker container** uses nginx:alpine as runtime base. nginx serves static files on `:8080` and proxies `/api/` to the backend on `127.0.0.1:3000`. Entrypoint script (`nginx/entrypoint.sh`) starts both processes. Multi-stage build: node:22-alpine → rust:1-alpine → nginx:alpine
