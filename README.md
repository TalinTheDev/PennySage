# PennySage

## Local Development

1. Clone project into WSL.
2. Open project using VSCode.
3. Create a `./.env` file using `./sample.env`. This is used when running the app in a docker container.
4. Create a `./backend/.env` file using `./backend/backend-dev.env`. This is used when running the app outside a container.
5. Open project in the dev container using the configured file.
6. In the container, run `sqlx database setup --source ./pennysage-backend-lib/migrations/` in `./backend` to create the dev database and run migrations.
7. Start backend server by running `cargo run` in the `./backend` directory.
8. Start frontend server by running `pnpm install && pnpm dev` in the `./frontend` directory.

## Docker Compose Deployment

1. Open project in WSL
2. In the project root, run `docker compose up --build` the first time
3. Run `docker compose down` to stop the servers
4. Run `docker compose up` to start the servers each subsequent time

## Notes

- DO NOT RUN any git commands from within the dev container. Doing so will mess up file permissions.
