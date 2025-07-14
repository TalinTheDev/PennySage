# PennySage

## Local Development

1. Clone project
3. Create a `./.env` file using `./sample.env`. This is used when running the app in a docker container.
4. Create a `./backend/.env` file using `./backend/backend-dev.env`. This is used when running the app outside a container.
5. Open project in the dev container using the configured file (run `dev up .`)
6. In the container, run `sqlx database setup --source ./pennysage-backend-lib/migrations/` in `./backend` to create the dev database and run migrations.
7. Start backend server by running `cargo run` in the `./backend` directory.
8. Start frontend server by running `pnpm install && pnpm dev` in the `./frontend` directory.

## Docker Compose Deployment

- To run application: `docker compose up` (Optional `-d/--detach`)
- To stop application: `docker compose stop`
- To rebuild application (after changes): `docker compose build`
- To remove containers (but not volumes/data): `docker compose down`
- To remove everything: `docker compose down --remove-orphans --volumes`
