FROM paritytech/ci-linux:42a94a57-20220730@sha256:9666da1a0ad5f4d127e3cbd7fcf2aae0165a33ecff4fb0efd9baf03f24093483 AS chef
RUN cargo install cargo-chef --version 0.1.31
WORKDIR /app

FROM chef AS planner
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./runtime /app/runtime
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

# Rebuild the source code only when needed
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./runtime /app/runtime
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo build --release

# Production image, copy all the files and run
FROM debian:buster-20210816-slim@sha256:bdb7370812bd65a465f929fd8e61af294fe5988a954ea3c373511f125697603d AS runner
LABEL social.myriad.image.authors="dev@myriad.social"
# Create user and set ownership and permissions as required
RUN useradd -m -u 1001 -U -s /bin/sh -d /home/myriad myriad && \
  # manage folder data
  mkdir -p /home/myriad/.local/share && \
  mkdir /data && \
  chown -R myriad:myriad /data && \
  ln -s /data /home/myriad/.local/share/myriad
# Copy from builder stage
COPY --from=builder /app/target/release/myriad /usr/local/bin
# Set to a non-root built-in user
USER myriad
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/myriad"]
