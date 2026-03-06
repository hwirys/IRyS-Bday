#!/bin/sh
set -e

# Start backend API in background
./backend &

# Run nginx in foreground (PID 1 — handles signals)
exec nginx -g 'daemon off;'
