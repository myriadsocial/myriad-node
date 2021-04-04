# ===== FIRTS STAGE ======
FROM paritytech/ci-linux:production as builder
# This is the build stage for myriad. Here we create the binary.
ARG PROFILE=release
ARG VCS_REF
ARG VERSION
ARG BUILD_DATE

ENV RUST_BACKTRACE 1

LABEL social.myriad.node.image.authors="1@myriad.social" \
	social.myriad.node.image.vendor="Myriad" \
	social.myriad.node.image.title="myriadsocial/myriad-node" \
	social.myriad.node.image.description="myriad-node: a platform for web3" \
	social.myriad.node.image.source="https://github.com/myriadsocial/myriad-node/blob/${VCS_REF}/docker/Dockerfile" \
	social.myriad.node.image.revision="${VCS_REF}" \
	social.myriad.node.image.version="${VERSION}" \
	social.myriad.node.image.created="${BUILD_DATE}" \
	social.myriad.node.image.documentation="https://github.com/myriadsocial/myriad-node/tree/${VCS_REF}"
	
WORKDIR /myriad

COPY ./node /myriad/node
COPY ./pallets /myriad/pallets
COPY ./runtime /myriad/runtime
COPY ./Cargo.lock /myriad/Cargo.lock
COPY ./Cargo.toml /myriad/Cargo.toml

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======
FROM debian:buster-slim
# This is the 2nd stage: a very small image where we copy the Polkadot binary.
ARG PROFILE=release
ARG VCS_REF
ARG VERSION
ARG BUILD_DATE

ENV RUST_BACKTRACE 1

LABEL social.myriad.node.image.authors="1@myriad.social" \
	social.myriad.node.image.vendor="Myriad" \
	social.myriad.node.image.title="myriadsocial/myriad-node" \
	social.myriad.node.image.description="myriad-node: a platform for web3" \
	social.myriad.node.image.source="https://github.com/myriadsocial/myriad-node/blob/${VCS_REF}/docker/Dockerfile" \
	social.myriad.node.image.revision="${VCS_REF}" \
	social.myriad.node.image.version="${VERSION}" \
	social.myriad.node.image.created="${BUILD_DATE}" \
	social.myriad.node.image.documentation="https://github.com/myriadsocial/myriad-node/tree/${VCS_REF}"

# show backtraces
ENV RUST_BACKTRACE 1

COPY --from=builder /myriad/target/$PROFILE/myriad /usr/local/bin

RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y curl && \
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
	useradd -m -u 1000 -U -s /bin/sh -d /myriad myriad && \
	mkdir -p /myriad/.local/share && \
	mkdir /data && \
	chown -R myriad:myriad /data && \
	ln -s /data /myriad/.local/share/myriad

USER myriad

# check if executable works in this container
RUN /usr/local/bin/myriad --version

EXPOSE 30333 9933 9944 30334 9934 9945
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/myriad"]