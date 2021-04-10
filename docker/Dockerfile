# ===== FIRST STAGE ======
# Original code from https://github.com/paritytech/polkadot

FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Hydra-DX. Here we create the binary."

ARG PROFILE=release
WORKDIR /hydra-dx-node

COPY . /hydra-dx-node

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Hydra-DX binary."
ARG PROFILE=release
COPY --from=builder /hydra-dx-node/target/$PROFILE/hydra-dx /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /hydra-dx-node hydra && \
    mkdir -p /hydra-dx-node/.local/share && \
    mkdir /data && \
    chown -R hydra:hydra /data && \
    ln -s /data /hydra-dx-node/.local/share/polkadot && \
    rm -rf /usr/bin /usr/sbin

USER hydra
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/hydra-dx"]