# Save Sync Server

Self-hosted HTTP backend for the Save Sync PS Vita app. Handles save upload/download, manifest tracking, and device pairing with token-based auth.

## Setup

```bash
cp .env.example .env
# Edit .env with your values
pnpm install
pnpm run build
pnpm run start
```

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `USER_TOKEN` | (required) | Bearer token shared between server and Vita clients |
| `USER_NAME` | `default` | Used in storage paths under `/data/vita-save-sync/users/` |
| `DATA_DIR` | `./data` | Where save archives and manifests are stored |
| `PORT` | `3000` | HTTP listen port |

## API

### `GET /api/status`
Health check, no auth required.

### `POST /api/pair`
Register a device. Body: `{ "token": "<token>", "deviceName": "<name>" }`.

### `GET /api/manifest`
Returns the user's cloud manifest. Auth: `Authorization: Bearer <token>`.

### `PUT /api/manifest`
Update manifest metadata. Auth required.

### `PUT /api/save/:titleId`
Upload a zipped save. Auth required.
Headers: `X-Save-Hash`, `X-Save-Timestamp`, `X-Device-Id`.
Body: multipart form with `file` field.

### `GET /api/save/:titleId`
Download the latest save zip for a title. Auth required.

## Storage layout

```
$DATA_DIR/users/<username>/
  manifest.json
  devices/
    vita-oled.json
    vita-slim.json
  saves/
    <TITLEID>/
      current.zip
      versions/
        2026-06-21T15-42-00Z.zip
```

## Production deployment

Put behind Nginx or Caddy with HTTPS:

```nginx
server {
    listen 443 ssl;
    server_name vita-sync.example.com;
    ssl_certificate /etc/letsencrypt/live/vita-sync.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/vita-sync.example.com/privkey.pem;
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
    }
}
```
