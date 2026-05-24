# rust-playground

Full-stack portfolio and auth application. Rust/Actix-web backend, Vue 3 frontend.

## Architecture

```
rust-playground/
├── src/              # Rust backend (Actix-web)
│   ├── main.rs                      # HTTP server, route/middleware wiring
│   ├── schema.rs                    # Diesel-generated DB schema
│   ├── db.rs                        # DB connection pool setup
│   ├── email_queue.rs               # Background email worker (Resend API)
│   ├── helpers/
│   │   ├── auth/
│   │   │   ├── mod.rs
│   │   │   └── paseto.rs            # PASETO token generation/validation
│   │   └── email/
│   │       └── mod.rs               # Email sending helpers
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── paseto_auth.rs           # Auth middleware (validates PASETO cookie)
│   ├── models/
│   │   ├── mod.rs
│   │   ├── otp_code/mod.rs          # OTP code model
│   │   ├── password_history/mod.rs  # Password history model (last 5 hashes)
│   │   └── user_diesel/mod.rs       # User model
│   └── routes/
│       ├── mod.rs
│       ├── auth/
│       │   ├── mod.rs               # Registers /api/auth/* routes
│       │   ├── login.rs             # POST /api/auth/login
│       │   ├── logout.rs            # POST /api/auth/logout (auth-guarded)
│       │   ├── register.rs          # POST /api/auth/register
│       │   ├── verify_email.rs      # GET  /api/auth/verify-email
│       │   └── verify_otp.rs        # POST /api/auth/verify-otp
│       └── settings/
│           ├── mod.rs               # Registers /api/settings/* routes
│           ├── change_password.rs   # POST /api/settings/change-password (auth-guarded)
│           ├── get_settings.rs      # GET  /api/settings
│           └── toggle_2fa.rs        # POST /api/settings/2fa
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
├── Cargo.toml
└── diesel.toml
```

## Backend

- **Framework**: Actix-web 4
- **Database**: PostgreSQL via Diesel 2 (ORM + migrations)
- **Auth**: PASETO tokens stored as HTTP-only cookies; `paseto_auth` middleware guards protected routes
- **Email**: Background queue worker using Resend API; ed25519-dalek signs email verification tokens
- **2FA**: TOTP OTP codes via the `otp_codes` table
- **Port**: 8080

### API Routes

All routes are under the `/api` prefix.

| Method | Path | Auth required | Description |
|--------|------|---------------|-------------|
| `POST` | `/api/auth/register` | No | Create account, sends verification email |
| `POST` | `/api/auth/login` | No | Login; returns PASETO cookie (or prompts OTP if 2FA enabled) |
| `POST` | `/api/auth/verify-otp` | No | Submit OTP code to complete login |
| `GET` | `/api/auth/verify-email` | No | Confirm email via token in link |
| `POST` | `/api/auth/logout` | Yes | Clear auth cookie |
| `GET` | `/api/settings` | Yes | Fetch user settings |
| `POST` | `/api/settings/2fa` | Yes | Toggle 2FA on/off |
| `POST` | `/api/settings/change-password` | Yes | Change password; rejects last 5 used passwords |

### DB Schema

**`users`** — `id` (UUID), `email`, `password_hash`, `created_at`, `updated_at`, `otp_enabled` (bool), `is_verified` (bool), `verification_public_key` (nullable), `is_admin` (bool)

**`otp_codes`** — `id` (UUID), `user_id` (UUID → users), `code`, `expires_at`, `used` (bool), `created_at`

**`password_history`** — `id` (UUID), `user_id` (UUID → users), `password_hash`, `created_at`; pruned to 5 most recent entries per user on each password change

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
docker-compose -f docker-compose.local.yml up
```
