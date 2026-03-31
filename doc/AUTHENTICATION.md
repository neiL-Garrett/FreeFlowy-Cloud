# Authentication and Billing Behavior (Self-host)

This fork uses GoTrue for authentication and includes self-host compatible billing behavior.

## Authentication defaults

Configuration is driven primarily by `deploy.env` (for compose) and `dev.env` (for local development).

Key variables:

- `GOTRUE_ADMIN_EMAIL`, `GOTRUE_ADMIN_PASSWORD`
- `GOTRUE_JWT_SECRET`, `GOTRUE_JWT_EXP`
- `GOTRUE_DISABLE_SIGNUP`
- `GOTRUE_MAILER_AUTOCONFIRM`
- `GOTRUE_SMTP_*`
- `GOTRUE_EXTERNAL_*` (Google, GitHub, Discord, Apple, SAML)

## Recommended self-host settings

### Local/dev

- `GOTRUE_DISABLE_SIGNUP=false`
- `GOTRUE_MAILER_AUTOCONFIRM=true`
- OAuth providers disabled unless explicitly testing OAuth

This allows quick account creation without SMTP setup.

### Production

- `GOTRUE_DISABLE_SIGNUP=false` (or `true` for invite-only)
- `GOTRUE_MAILER_AUTOCONFIRM=false`
- Configure all required `GOTRUE_SMTP_*` values
- Rotate and secure `GOTRUE_JWT_SECRET`
- Enable only the OAuth providers you actually use

## Self-host billing and plan behavior in this fork

This fork is intentionally tuned for self-host parity:

- Self-hosted runtime paths avoid hosted upgrade/paywall interruptions.
- Compatibility billing/usage endpoints are provided so clients can operate without hosted plan enforcement.
- Core workspace and collaboration workflows should remain usable without hosted subscription state.

### Practical outcome

- You should not rely on hosted billing checks to gate normal self-host usage.
- If you need strict plan enforcement in your own deployment, implement it as a custom policy layer.

## Related docs

- Deployment flow: [`DEPLOYMENT.md`](./DEPLOYMENT.md)
- EC2 quick guide: [`EC2_SELF_HOST_GUIDE.md`](./EC2_SELF_HOST_GUIDE.md)
- Deferred items: [`KNOWN_LIMITATIONS.md`](./KNOWN_LIMITATIONS.md)
