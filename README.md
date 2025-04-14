# PennySage

## Local Development

1. Clone project into WSL.
2. Open project using VSCode.
3. Create a `./.env` file using `./sample.env`. This is used when running the app in a docker container.
4. Create a `./backend/.env` file using `./backend/backend-dev.env`. This is used when running the app outside a container.
5. Open project in the dev container using the configured file.
6. Start backend server by running `cargo run` in the backend directory.
7. Start frontend server by running `pnpm dev` in the frontend directory.

## Notes

- DO NOT RUN any git commands from within the dev container. Doing so will mess up file permissions.
