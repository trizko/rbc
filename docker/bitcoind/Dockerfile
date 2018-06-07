FROM debian:9.4

ADD laanwj-releases.asc /root/

ENV GPG_KEY 0x90C8019E36C2E964
ENV BITCOIN_CORE_VERSION 0.16.0

WORKDIR /root/
RUN set -ex \
		&& buildDeps=' \
				ca-certificates \
				dirmngr \
				gpg \
				wget \
		' \
		&& apt-get update && apt-get install -y $buildDeps --no-install-recommends \
		&& rm -rf /var/lib/apt/lists/* \
		&& wget \
				-O /root/bitcoin-${BITCOIN_CORE_VERSION}-x86_64-linux-gnu.tar.gz \
				"https://bitcoin.org/bin/bitcoin-core-${BITCOIN_CORE_VERSION}/bitcoin-${BITCOIN_CORE_VERSION}-x86_64-linux-gnu.tar.gz" \
		&& wget \
				-O /root/SHA256SUMS.asc \
				"https://bitcoin.org/bin/bitcoin-core-0.16.0/SHA256SUMS.asc" \
		&& export GNUPGHOME="$(mktemp -d)" \
		&& gpg --import laanwj-releases.asc \
		&& sha256sum --check SHA256SUMS.asc 2>&1 | grep OK \
		&& gpg --verify SHA256SUMS.asc \
		&& rm -rf "$GNUPGHOME" SHA256SUMS.asc \
		&& apt-get purge -y --auto-remove $buildDeps \
		&& tar xzf bitcoin-${BITCOIN_CORE_VERSION}-x86_64-linux-gnu.tar.gz \
		&& install -m 0755 -o root -g root -t /usr/local/bin bitcoin-${BITCOIN_CORE_VERSION}/bin/*

ADD entrypoint.sh /root/

ENTRYPOINT ["./entrypoint.sh"]
