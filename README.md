# PennySage

PennySage is a personal finance app I've been building to make transaction tracking easier than
keeping everything in a spreadsheet. You can enter transactions, import them from CSV, and see
how your spending changes over time. I built the frontend in Vue 3 and the API in Rust, with
SQLite storing the data.

## What it does

- Lets users register, log in, and keep their transactions separate.
- Adds, edits, and deletes income and expense records with a date, merchant, category, and
  payment source.
- Imports a CSV of transactions and saves suggestions for merchants, categories, and payment
  sources.
- Makes transactions searchable and filterable, with column sorting and bulk deletion.
- Shows category and payment-source breakdowns alongside monthly and yearly charts.

The app tracks what you put into it; it doesn't connect to a bank or know your live balance.

## How it's built

The frontend uses Vue 3, Vite, Pinia, PrimeVue, and Chart.js. It sends requests to a Rust API
built with Axum. The API handles authentication and uses SQLx to read and write an SQLite
database. Transaction queries use the authenticated user's ID, so records are scoped to the
account making the request.

Docker Compose runs the API and an nginx server for the built frontend. nginx also proxies
`/api` requests to Rust. During development, Vite provides that proxy instead.

## Run it with Docker

1. Copy [`sample.env`](sample.env) to `.env` in the repository root.
2. Replace `JWT_SECRET` and `JWT_REFRESH_SECRET` with two different random values. The sample
   values are just placeholders.
3. Run this from the repository root:

   ```sh
   docker compose up --build
   ```

Open `http://localhost:1234` and register an account. The API is also available on port `3000`.
Docker stores the SQLite file in a named volume.

## Run it from source

You'll need Rust/Cargo, Node.js, pnpm, and SQLx CLI with SQLite support. Copy
[`backend/backend-dev.env`](backend/backend-dev.env) to `backend/.env` and replace its sample
JWT secrets. Then, from `backend/`:

```sh
sqlx database setup --source ./pennysage-backend-lib/migrations/
cargo run --locked --package pennysage-backend-bin
```

SQLx checks queries when Rust compiles, so the database needs its migrations first. In another
terminal, from `frontend/`:

```sh
pnpm install --frozen-lockfile
pnpm dev
```

Vite normally serves the app at `http://localhost:5173` and proxies `/api` to Rust on port
`3000`.

## CSV format

The importer expects these column names:

```csv
description,date,amount,merchant,category,payment_source
```

Use a positive amount for income and a negative amount for an expense. A timestamp such as
`2026-11-01T12:00:00Z` works for the date field.

## Current state

This is still a work in progress. The session flow, some analytics calculations, account
deletion, and backups need more testing and cleanup. I'd use mock data while trying it out and
wouldn't rely on it as the only record of real finances yet.

## License

Copyright © 2020–2026 Talin Sharma. All rights reserved. See [LICENSE](LICENSE).
