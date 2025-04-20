FROM docker.io/nixos/nix:2.28.2@sha256:51b15336cdd00cce3608ec31eb06852a82dfc4e85429011081a2b60f7d82f055 AS builder

COPY flake.nix flake.lock Cargo.toml Cargo.lock /tmp/build/
COPY src /tmp/build/src/
WORKDIR /tmp/build

# Build with nix flake
RUN nix\
    --extra-experimental-features "nix-command flakes"\
    --option filter-syscalls false\
    build

RUN mkdir /tmp/nix-store-closure
RUN cp -R $(nix-store -qR result/) /tmp/nix-store-closure


FROM docker.io/alpine:3

# Add Tini
RUN apk add --no-cache tini
ENTRYPOINT ["/sbin/tini", "--"]

WORKDIR /app
COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /tmp/build/result /app
CMD ["/app/bin/workout_service"]
