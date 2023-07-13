# Rust as the base image
FROM rust:latest

# 1. Create a new empty shell project
WORKDIR /base

# 2. Copy the server manifests
RUN USER=root cargo new --bin codemafia
COPY ./codemafia/Cargo.lock ./codemafia/Cargo.lock
COPY ./codemafia/Cargo.toml ./codemafia/Cargo.toml

# 2. Copy the shared manifests and src
COPY ./shared/Cargo.lock ./shared/Cargo.lock
COPY ./shared/Cargo.toml ./shared/Cargo.toml
COPY ./shared/src ./shared/src

# 3. Build only the dependencies to cache them
RUN cd codemafia && cargo build --release
RUN cd codemafia && rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./codemafia/src ./codemafia/src

# 5. Build for release.
RUN rm ./codemafia/target/release/deps/codemafia*

# 6. Run the application.
WORKDIR /base/codemafia
CMD ["cargo", "run", "--release", "--", "--words", "/base/codemafia/src/creator/wordlist"]