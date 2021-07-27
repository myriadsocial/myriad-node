FROM paritytech/ci-linux:3d4ca6a9-20210707 as builder

WORKDIR /build

COPY ./nodes /build/nodes
COPY ./pallets /build/pallets
COPY ./runtimes /build/runtimes
COPY ./Cargo.lock /build/Cargo.lock
COPY ./Cargo.toml /build/Cargo.toml

RUN cargo build --release

FROM debian:buster-slim

# metadata
ARG VCS_REF
ARG BUILD_DATE

LABEL social.myriad.image.title="myriadsocial/myriad-node" \
  social.myriad.image.description="myriad-node" \
  social.myriad.image.authors="myriad.dev@blocksphere.id" \
  social.myriad.image.vendor="Blocksphere" \
  social.myriad.image.revision="${VCS_REF}" \
  social.myriad.image.created="${BUILD_DATE}" \
  social.myriad.image.source="https://github.com/myriadsocial/myriad-node/blob/${VCS_REF}/.maintain/docker/Dockerfile" \
  social.myriad.image.documentation="https://github.com/myriadsocial/myriad-node/tree/${VCS_REF}"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y \
  libssl1.1 \
  ca-certificates \
  curl && \
  # apt cleanup
  apt-get autoremove -y && \
  apt-get clean && \
  find /var/lib/apt/lists/ -type f -not -name lock -delete; \
  # add user
  useradd -m -u 1000 -U -s /bin/sh -d /myriad myriad && \
  # manage folder data
  mkdir -p /myriad/.local/share && \
  mkdir /data && \
  chown -R myriad:myriad /data && \
  ln -s /data /myriad/.local/share/myriad

# add binary to docker image
COPY --from=builder /build/target/release/myriad /usr/local/bin
COPY --from=builder /build/target/release/myriad-parachain /usr/local/bin
COPY --from=builder /build/target/release/myriad-appchain /usr/local/bin

USER myriad

# check if executable works in this container
RUN /usr/local/bin/myriad --version
RUN /usr/local/bin/myriad-parachain --version
RUN /usr/local/bin/myriad-appchain --version

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

CMD ["/usr/local/bin/myriad"]
