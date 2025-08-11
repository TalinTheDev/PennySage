# Use archlinux as a base image
FROM archlinux:base

# Update system
RUN pacman -Syu --noconfirm

# Install packages
RUN pacman -S --noconfirm --needed nvim
RUN pacman -S --noconfirm --needed git
RUN pacman -S --noconfirm --needed openssh
RUN pacman -S --noconfirm --needed fish
RUN pacman -S --noconfirm --needed yazi
RUN pacman -S --noconfirm --needed ripgrep
RUN pacman -S --noconfirm --needed fd
RUN pacman -S --noconfirm --needed unzip
RUN pacman -S --noconfirm --needed sudo
RUN pacman -S --noconfirm --needed base-devel
RUN pacman -S --noconfirm --needed rustup
RUN pacman -S --noconfirm --needed npm
RUN pacman -S --noconfirm --needed nodejs
RUN pacman -S --noconfirm --needed less
RUN pacman -S --noconfirm --needed sqlite


# Create a dev user
ARG USER=talin
ARG HOME=/home/$USER
ARG USER_UID=1000
ARG USER_GID=1000

RUN groupadd --gid $USER_GID $USER && \
    useradd --uid $USER_UID --gid $USER_GID -m $USER

RUN chsh -s /usr/bin/fish $USER

# Set up container
USER $USER
WORKDIR $HOME

# User dev packages
RUN rustup toolchain install stable
RUN rustup default stable
RUN cargo install sqlx-cli --no-default-features --features sqlite
RUN rustup component add rustfmt

USER root
RUN pacman -S --noconfirm --needed pnpm

USER $USER

# Expose any required ports (e.g., frontend, backend)
EXPOSE 5173
