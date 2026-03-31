# FreeFlowy Cloud Deployment

This guide documents the current FreeFlowy self-host flow used in this fork.

## Choose a deployment path

- **Local source iteration (recommended for development):** run core infra with Docker Compose, then run Cloud/Web from local source images.
- **VPS all-in-one (recommended for simple production-like setup):** run the bundled `docker-compose.yml` stack with `deploy.env` values.

## 1) Local source iteration

This path is best when you are actively changing `FreeFlowy`, `FreeFlowy-Web`, or `FreeFlowy-Cloud`.

### Prerequisites

- Docker + Docker Compose
- `git`
- `node` + `pnpm` for `FreeFlowy-Web` builds

### Step A: Start infra dependencies

Run in `FreeFlowy-Cloud/`:

```bash
docker compose --env-file dev.env -f docker-compose-dev.yml up -d postgres redis gotrue minio
```

### Step B: Build and run FreeFlowy Cloud API from source

Run in `FreeFlowy-Cloud/`:

```bash
docker build -t freeflowy-cloud-local:dev -f Dockerfile .
docker rm -f freeflowy-cloudapi 2>/dev/null || true
docker run -d \
  --name freeflowy-cloudapi \
  --network freeflowy-cloud_default \
  -p 8000:8000 \
  -e RUST_LOG=info \
  -e APPFLOWY_DATABASE_URL=postgres://postgres:password@postgres:5432/postgres \
  -e APPFLOWY_REDIS_URI=redis://redis:6379 \
  -e APPFLOWY_GOTRUE_BASE_URL=http://gotrue:9999 \
  -e APPFLOWY_GOTRUE_JWT_SECRET=hello456 \
  -e APPFLOWY_S3_CREATE_BUCKET=true \
  -e APPFLOWY_S3_USE_MINIO=true \
  -e APPFLOWY_S3_MINIO_URL=http://minio:9000 \
  -e APPFLOWY_S3_ACCESS_KEY=minioadmin \
  -e APPFLOWY_S3_SECRET_KEY=minioadmin \
  -e APPFLOWY_S3_BUCKET=appflowy \
  -e APPFLOWY_WEB_URL=http://localhost:3000 \
  -e APPFLOWY_BASE_URL=http://localhost:3000 \
  freeflowy-cloud-local:dev
```

### Step C: Build and run FreeFlowy Web from source

Run in `FreeFlowy-Web/`:

```bash
docker build -t freeflowy-web-local:dev -f docker/Dockerfile.ssr .
docker rm -f freeflowy-webui 2>/dev/null || true
docker run -d \
  --name freeflowy-webui \
  --network freeflowy-cloud_default \
  -e APPFLOWY_BASE_URL=http://localhost:3000 \
  -e APPFLOWY_GOTRUE_BASE_URL=http://localhost:3000/gotrue \
  -e APPFLOWY_WS_BASE_URL=ws://localhost:3000/ws/v2 \
  freeflowy-web-local:dev
```

### Step D: Start local gateway

The gateway routes `localhost:3000` to web, API, auth, and websocket services.

```bash
docker rm -f freeflowy-web-gateway 2>/dev/null || true
docker run -d \
  --name freeflowy-web-gateway \
  --network freeflowy-cloud_default \
  -p 3000:80 \
  -v "$(pwd)/docker/local-web-gateway.conf:/etc/nginx/conf.d/default.conf:ro" \
  nginx:stable-alpine
```

Run this command from `FreeFlowy-Cloud/` so `$(pwd)/docker/local-web-gateway.conf` resolves correctly.

### Verify

- `http://localhost:3000/login`
- `http://localhost:3000/app`
- `http://localhost:8000/api/health`

### Tear down

```bash
docker rm -f freeflowy-web-gateway freeflowy-webui freeflowy-cloudapi
docker compose --env-file dev.env -f docker-compose-dev.yml down
```

Use `down -v` only if you want to delete persisted DB/storage volumes.

## 2) VPS all-in-one deployment

This path uses the bundled production-oriented compose stack.

### Step A: Prepare host

- Ubuntu 22.04+ recommended
- Install Docker Engine and Docker Compose plugin
- Open TCP ports `80` and `443`

### Step B: Configure environment

Run in `FreeFlowy-Cloud/`:

```bash
cp deploy.env .env
```

Edit `.env` at minimum:

- `FQDN`
- `SCHEME` / `WS_SCHEME`
- `GOTRUE_JWT_SECRET`
- `POSTGRES_PASSWORD`
- `AWS_ACCESS_KEY` / `AWS_SECRET` (or your S3 credentials)
- SMTP values if email verification is required

### Step C: Launch services

```bash
docker compose up -d
```

### Step D: Verify

- `docker compose ps`
- `curl -f http://localhost/api/health`
- Visit `http://<your-domain>/app`

## Known limitations (current fork)

See [`KNOWN_LIMITATIONS.md`](./KNOWN_LIMITATIONS.md) for currently deferred behaviors and optional services.
