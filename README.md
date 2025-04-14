# PennySage

## Local Development

1. Clone project into WSL.
2. Open project using VSCode.
3. Create a `./.env` file using `./sample.env`. This is used when running the app in a docker container.
4. Create a `./backend/.env` file using `./backend/backend-dev.env`. This is used when running the app outside a container.
5. Open project in the dev container using the configured file.
6. In the container, run `sqlx database setup --source ./pennysage-backend-lib/migrations/` in `./backend` to create the dev database and run migrations.
7. Start backend server by running `cargo run` in the `./backend` directory.
8. Start frontend server by running `pnpm dev` in the `./frontend` directory.

## Notes

- DO NOT RUN any git commands from within the dev container. Doing so will mess up file permissions.
