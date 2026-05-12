# rust-playground

Full-stack portfolio and auth application. Rust/Actix-web backend, Vue 3 frontend.

## Architecture

```
rust-playground/
├── src/              # Rust backend (Actix-web)
│   ├── main.rs       # HTTP server, route definitions
│   └── schema.rs     # Diesel-generated DB schema
├── migrations/       # Diesel SQL migrations
├── frontend/         # Vue 3 SPA
│   └── src/
│       ├── main.js         # App entry, Vuetify + Router setup
│       ├── router.ts       # Vue Router route definitions
│       ├── App.vue          # Root component (v-app + RouterView)
│       ├── assets/
│       │   └── main.css    # Global CSS variables and base styles
│       └── components/
│           ├── NavBar.vue   # v-app-bar + v-navigation-drawer
│           ├── Home.vue     # Portfolio page (Hero, About, Projects, Contact)
│           ├── Hero.vue
│           ├── About.vue
│           ├── Projects.vue
│           ├── Contact.vue
│           └── Auth.vue     # Login + Register (shared, route-driven)
├── Dockerfile
├── docker-compose.local.yml
├── Cargo.toml
└── diesel.toml
```

## Backend

- **Framework**: Actix-web 4
- **Database**: PostgreSQL via Diesel 2 (ORM + migrations)
- **Port**: 8080
- **Routes**: all under `/api` prefix
  - `GET /api/hello`
  - `POST /api/echo`
  - `GET /api/hey`
- **DB schema**: `users` table — `id` (UUID), `username`, `email`, `password_hash`, `created_at`, `updated_at`

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
| Path | Component |
|------|-----------|
| `/` | `Home.vue` |
| `/login` | `Auth.vue` |
| `/register` | `Auth.vue` |

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
