# ===== FIRST STAGE ======
FROM paritytech/ci-linux:94420526-20210215 as builder
LABEL description="This is the build stage for Myriad. Here we create the binary."

WORKDIR /build

COPY ./node /build/node
COPY ./pallets /build/pallets
COPY ./runtime /build/runtime
COPY ./Cargo.lock /build/Cargo.lock
COPY ./Cargo.toml /build/Cargo.toml

RUN cargo build --release

# ===== SECOND STAGE ======
FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Myriad binary."
COPY --from=builder /build/target/release/myriad /usr/local/bin

RUN apt update && apt install curl -y && \
	useradd -m -u 1000 -U -s /bin/sh -d /myriad myriad && \
	mkdir -p /myriad/.local/share && \
	mkdir /data && \
	chown -R myriad:myriad /data && \
	ln -s /data /myriad/.local/share/myriad && \
	rm -rf /usr/bin /usr/sbin

USER myriad
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/myriad"]
