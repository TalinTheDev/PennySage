# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.84
ARG APP_NAME=pennysage-backend

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Add DATABASE_URL env var
ARG CARGO_INSTALL_ROOT=/usr/local/cargo-packages
ARG DATABASE_DIR
ENV DATABASE_DIR=${DATABASE_DIR}
ARG DATABASE_FILE
ENV DATABASE_FILE=${DATABASE_FILE}
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Install host build dependencies.
# RUN apk add --no-cache clang lld musl-dev git
RUN apk add clang lld musl-dev git

RUN cargo install sqlx-cli --no-default-features --features sqlite --root ${CARGO_INSTALL_ROOT}
RUN echo "Cargo Bin: ${CARGO_INSTALL_ROOT}" && ls ${CARGO_INSTALL_ROOT}/bin
ENV PATH=${CARGO_INSTALL_ROOT}/bin:$PATH

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# for git repository dependencies, and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=migrations,target=migrations \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    mkdir ${DATABASE_DIR} && \
    touch ${DATABASE_FILE} && \
    sqlx database create && sqlx migrate run && \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the alpine image as the foundation for running the app.
# By specifying the "3.18" tag, it will use version 3.18 of alpine. If
# reproducibility is important, consider using a digest
# (e.g., alpine@sha256:664888ac9cfd28068e062c991ebcff4b4c7307dc8dd4df9e728bedde5c449d91).
FROM alpine:3.18 AS final

ARG DATABASE_FILE
ENV DATABASE_FILE=${DATABASE_FILE}

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# Copy the database & executable from the "build" stage.\
COPY --from=build ${DATABASE_FILE} ${DATABASE_FILE}
COPY --from=build /bin/server /bin/server

# Expose the port that the application listens on.
EXPOSE 3000

# Update permissions for /data folder
RUN chown -R appuser /data

# Switch to non-privileged user
USER appuser

# What the container should run when it is started.
CMD ["/bin/server"]
