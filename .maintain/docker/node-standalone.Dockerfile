FROM debian:buster-slim

ARG VCS_REF
ARG BUILD_DATE

LABEL social.myriad.node.image.authors="1@myriad.social" \
	social.myriad.node.image.vendor="Myriadsocial" \
	social.myriad.node.image.title="myriadsocial/myriad-node" \
	social.myriad.node.image.description="Myriad is a web3 layer on top of web2 social media" \
	social.myriad.node.image.revision="${VCS_REF}" \
	social.myriad.node.image.created="${BUILD_DATE}" \
  social.myriad.node.image.source="https://github.com/myriadsocial/myriad-node/blob/${VCS_REF}/.maintain/docker/node-standalone.Dockerfile" \
	social.myriad.node.image.documentation="https://github.com/myriadsocial/myriad-node/tree/${VCS_REF}"

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
COPY ./myriad /usr/local/bin

USER myriad

# check if executable works in this container
RUN /usr/local/bin/myriad --version

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

CMD ["/usr/local/bin/myriad"]
