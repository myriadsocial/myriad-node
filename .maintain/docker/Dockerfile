# Rebuild the source code only when needed
FROM paritytech/ci-linux:9575dfcd-20210729 AS builder
WORKDIR /build
COPY ./nodes /build/nodes
COPY ./pallets /build/pallets
COPY ./runtimes /build/runtimes
COPY ./Cargo.lock /build/Cargo.lock
COPY ./Cargo.toml /build/Cargo.toml
RUN cargo build --release

# Production image, copy all the files and run
FROM debian:buster-20210816-slim AS runner
LABEL social.myriad.image.authors="myriad.dev@blocksphere.id"
# Create user and set ownership and permissions as required
RUN useradd -m -u 1001 -U -s /bin/sh -d /home/myriad myriad && \
  # manage folder data
  mkdir -p /home/myriad/.local/share && \
  mkdir /data && \
  chown -R myriad:myriad /data && \
  ln -s /data /home/myriad/.local/share/myriad
# Copy from builder stage
COPY --from=builder /build/target/release/myriad /usr/local/bin
# Set to a non-root built-in user
USER myriad
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/myriad"]
