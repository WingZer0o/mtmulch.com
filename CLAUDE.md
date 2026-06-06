# rust-playground

Full-stack portfolio and auth application. Rust/Actix-web backend, Vue 3 frontend.

## Architecture

```
rust-playground/
├── src/              # Rust backend (Actix-web)
│   ├── main.rs                          # HTTP server, route/middleware wiring
│   ├── schema.rs                        # Diesel-generated DB schema
│   ├── db.rs                            # DB connection pool setup
│   ├── blob_storage.rs                  # Azure Blob Storage client factory
│   ├── email_queue.rs                   # Background email worker (Resend API)
│   ├── helpers/
│   │   ├── auth/
│   │   │   ├── mod.rs
│   │   │   └── paseto.rs                # PASETO token generation/validation
│   │   └── email/
│   │       └── mod.rs                   # Email sending helpers
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── paseto_auth.rs               # Auth middleware (validates PASETO cookie)
│   ├── models/
│   │   ├── mod.rs
│   │   ├── failed_login_attempt/mod.rs  # Failed login attempt model
│   │   ├── otp_code/mod.rs              # OTP code model
│   │   ├── password_history/mod.rs      # Password history model (last 5 hashes)
│   │   ├── recording/mod.rs             # Recording model (blob_key, metadata)
│   │   └── user_diesel/mod.rs           # User model
│   └── routes/
│       ├── mod.rs
│       ├── auth/
│       │   ├── mod.rs                   # Registers /api/auth/* routes
│       │   ├── login.rs                 # POST /api/auth/login
│       │   ├── logout.rs                # POST /api/auth/logout (auth-guarded)
│       │   ├── register.rs              # POST /api/auth/register
│       │   ├── unlock_account.rs        # GET  /api/auth/unlock-account
│       │   ├── verify_email.rs          # GET  /api/auth/verify-email
│       │   └── verify_otp.rs            # POST /api/auth/verify-otp
│       ├── recordings/
│       │   ├── mod.rs                   # Registers /api/recordings/* routes
│       │   ├── upload.rs                # POST /api/recordings/upload (streaming block-blob)
│       │   ├── get.rs                   # GET  /api/recordings/{id} (streaming download)
│       │   ├── list.rs                  # GET  /api/recordings
│       │   └── delete.rs                # DELETE /api/recordings/{id}
│       └── settings/
│           ├── mod.rs                   # Registers /api/settings/* routes
│           ├── change_password.rs       # POST /api/settings/change-password (auth-guarded)
│           ├── get_settings.rs          # GET  /api/settings
│           └── toggle_2fa.rs            # POST /api/settings/2fa
├── migrations/       # Diesel SQL migrations
├── frontend/         # Vue 3 SPA
│   └── src/
│       ├── main.js                  # App entry, Vuetify + Router setup
│       ├── router.ts                # Vue Router route definitions
│       ├── api.ts                   # Axios/fetch API client
│       ├── App.vue                  # Root component (v-app + RouterView)
│       ├── assets/
│       │   └── main.css            # Global CSS variables and base styles
│       └── pages/
│           ├── auth/
│           │   ├── Auth.vue         # Login + Register (shared, route-driven)
│           │   └── types/
│           │       ├── login-request.ts
│           │       └── register-request.ts
│           ├── home/
│           │   ├── Home.vue         # Portfolio page
│           │   └── components/
│           │       ├── Hero.vue
│           │       ├── About.vue
│           │       ├── Projects.vue
│           │       └── Contact.vue
│           └── settings/
│               └── Settings.vue     # User settings (2FA toggle, change password)
│       └── components/
│           └── NavBar.vue           # v-app-bar + v-navigation-drawer
├── Dockerfile
├── docker-compose.local.yml
├── docker-compose.debug.yml
├── Cargo.toml
└── diesel.toml
```

## Backend

- **Framework**: Actix-web 4
- **Database**: PostgreSQL via Diesel 2 (ORM + migrations)
- **Auth**: PASETO tokens stored as HTTP-only cookies; `paseto_auth` middleware guards protected routes
- **Email**: Background queue worker using Resend API; ed25519-dalek signs email verification and unlock tokens
- **2FA**: TOTP OTP codes via the `otp_codes` table
- **Blob storage**: Azure Blob Storage via `azure_storage_blobs` 0.21; local dev uses Azurite emulator
- **Port**: 8080

### API Routes

All routes are under the `/api` prefix.

| Method | Path | Auth required | Description |
|--------|------|---------------|-------------|
| `POST` | `/api/auth/register` | No | Create account, sends verification email |
| `POST` | `/api/auth/login` | No | Login; returns PASETO cookie (or prompts OTP if 2FA enabled); locks after 5 failures |
| `POST` | `/api/auth/verify-otp` | No | Submit OTP code to complete login |
| `GET` | `/api/auth/verify-email` | No | Confirm email via token in link |
| `GET` | `/api/auth/unlock-account` | No | Verify ed25519 token from lockout email to unlock account |
| `POST` | `/api/auth/logout` | Yes | Clear auth cookie |
| `GET` | `/api/settings` | Yes | Fetch user settings |
| `POST` | `/api/settings/2fa` | Yes | Toggle 2FA on/off |
| `POST` | `/api/settings/change-password` | Yes | Change password; rejects last 5 used passwords |
| `POST` | `/api/recordings/upload` | Yes | Stream-upload a FLAC; stored as block blob, metadata in DB |
| `GET` | `/api/recordings` | Yes | List recordings (metadata only) |
| `GET` | `/api/recordings/{id}` | Yes | Stream-download a recording |
| `DELETE` | `/api/recordings/{id}` | Yes | Delete blob from Azure then remove DB row |

### DB Schema

**`users`** — `id` (UUID), `email`, `password_hash`, `created_at`, `updated_at`, `otp_enabled` (bool), `is_verified` (bool), `verification_public_key` (nullable), `is_admin` (bool), `is_locked` (bool), `unlock_public_key` (nullable)

**`otp_codes`** — `id` (UUID), `user_id` (UUID → users), `code`, `expires_at`, `used` (bool), `created_at`

**`password_history`** — `id` (UUID), `user_id` (UUID → users), `password_hash`, `created_at`; pruned to 5 most recent entries per user on each password change

**`failed_login_attempts`** — `id` (UUID), `user_id` (UUID → users), `attempted_at`; cleared on successful login or account unlock; triggers lockout at 5 entries

**`recordings`** — `id` (UUID), `user_id` (UUID → users), `filename` (TEXT), `size_bytes` (BIGINT), `blob_key` (TEXT), `created_at`; blob data lives in Azure Blob Storage, not the DB

### Blob storage

`src/blob_storage.rs` reads two env vars and builds a `ContainerClient`:

- **`AZURE_STORAGE_CONNECTION_STRING`** — determines the backend:
  - Contains `BlobEndpoint=http://…` → Azurite emulator; host/port parsed from the URL, well-known dev credentials used automatically
  - Contains `AccountName` + `AccountKey` (no custom endpoint) → production Azure Blob Storage
- **`AZURE_STORAGE_CONTAINER`** — container name (default: `recordings`)

The `ContainerClient` is injected as `web::Data<ContainerClient>` into all recording route handlers.

Upload uses the block-blob API (4 MiB blocks streamed as they arrive; no full buffering). Download streams the pageable response body directly to the HTTP client.

### Lockout flow

1. Each wrong password inserts a row into `failed_login_attempts`.
2. On the 5th failure: `users.is_locked` → `true`, an ed25519 keypair is generated, the verifying key is stored in `users.unlock_public_key`, and a lockout email is sent with a signed link (`uid`, `ts`, `sig`).
3. `GET /api/auth/unlock-account` verifies the signature (1-hour expiry), sets `is_locked` → `false`, clears `unlock_public_key`, and deletes all `failed_login_attempts` for the user.
4. Locked accounts are rejected at login before any password work is done.

### Running the backend
```bash
cargo run
```

### Migrations
```bash
diesel migration run
diesel migration revert
```

## Frontend

- **Framework**: Vue 3 with Vite
- **UI library**: Vuetify 3 (requires `<v-app>` at root)
- **Icons**: MDI (`@mdi/font`)
- **Router**: Vue Router 4 (`createWebHistory`)
- **Node manager**: Volta

### Routes

| Path | Component | Guard |
|------|-----------|-------|
| `/` | `pages/home/Home.vue` | None |
| `/login` | `pages/auth/Auth.vue` | None |
| `/register` | `pages/auth/Auth.vue` | None |
| `/settings` | `pages/settings/Settings.vue` | `localStorage.logged_in === '1'` → redirect `/login` |

### Auth.vue pattern
`Auth.vue` is shared between `/login` and `/register`. Mode is derived from `useRoute()` — no props needed. Toggle buttons inside the form use `router.push()` to keep the URL in sync.

### CSS variables (main.css)
| Variable | Value |
|----------|-------|
| `--bg` | `#0f1117` |
| `--bg-card` | `#1a1d27` |
| `--border` | `#2a2d3e` |
| `--accent` | `#7c6af7` |
| `--accent-hover` | `#9585f8` |
| `--text` | `#e2e8f0` |
| `--text-muted` | `#8892a4` |
| `--mono` | JetBrains Mono |

### Running the frontend
```bash
cd frontend
npm run dev    # dev server
npm run build  # production build
```

## Docker
```bash
docker-compose -f docker-compose.local.yml up   # includes Azurite
docker-compose -f docker-compose.debug.yml up   # debug build + gdbserver + Azurite
```

Both compose files include the Azurite service (`mcr.microsoft.com/azure-storage/azurite`) and wire the correct `AZURE_STORAGE_CONNECTION_STRING` (pointing at `http://azurite:10000`) into the app container. Azurite data is persisted in a named `azurite_data` volume.

For production, set `AZURE_STORAGE_CONNECTION_STRING` to your real Azure connection string (no `BlobEndpoint` key) and `AZURE_STORAGE_CONTAINER` to the target container name.
