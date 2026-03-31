# FreeFlowy Cloud on AWS EC2 (Ubuntu)

This is a practical EC2 path that mirrors the current FreeFlowy self-host deployment flow.

## 1) Provision EC2

- Ubuntu 22.04+ instance
- Security group ports:
  - `22` (SSH)
  - `80` (HTTP)
  - `443` (HTTPS)

## 2) Install Docker

```bash
sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo $VERSION_CODENAME) stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo usermod -aG docker $USER
```

Log out/in once so group membership applies.

## 3) Configure FreeFlowy Cloud

```bash
git clone https://github.com/neiL-Garrett/FreeFlowy-Cloud.git
cd FreeFlowy-Cloud
cp deploy.env .env
```

Edit `.env` and set at least:

- `FQDN`
- `SCHEME=https` and `WS_SCHEME=wss` when TLS is enabled
- `POSTGRES_PASSWORD`
- `GOTRUE_JWT_SECRET`
- storage credentials (`AWS_ACCESS_KEY`, `AWS_SECRET`, or your S3 values)
- SMTP values if using email confirmation (`GOTRUE_MAILER_AUTOCONFIRM=false`)

## 4) Launch

```bash
docker compose up -d
docker compose ps
```

## 5) Validate

- `curl -f http://localhost/api/health`
- open `http://<your-domain>/app` (or HTTPS endpoint)

## 6) Operations

- Follow logs: `docker compose logs -f appflowy_cloud gotrue nginx`
- Update: pull repo updates, then run `docker compose pull && docker compose up -d`

## Notes

- For active local source iteration, use the local flow in [`DEPLOYMENT.md`](./DEPLOYMENT.md).
- For deferred self-host gaps, see [`KNOWN_LIMITATIONS.md`](./KNOWN_LIMITATIONS.md).
