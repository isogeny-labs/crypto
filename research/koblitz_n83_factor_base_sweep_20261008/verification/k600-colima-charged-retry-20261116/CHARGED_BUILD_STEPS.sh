#!/usr/bin/env bash
# Thin build orchestration for the committed n83.k600-m6-linux-build-plan/v1.
set -euo pipefail

mode=$1
out=$2

case "$mode" in
  rust-binaries)
    export CARGO_TARGET_DIR="$out/cargo-target"
    cargo build --release --locked --no-default-features -j 1 \
      --example koblitz_n83_mixed_orbit_worker \
      --example n83_k600_m6_native_arms \
      --example n83_k600_m6_native_preflight \
      --example n83_k600_m6_budget \
      --example koblitz_n83_factor_base_export
    install -m 755 "$CARGO_TARGET_DIR/release/examples/koblitz_n83_mixed_orbit_worker" "$out/worker"
    install -m 755 "$CARGO_TARGET_DIR/release/examples/koblitz_n83_factor_base_export" "$out/exporter"
    install -m 755 "$CARGO_TARGET_DIR/release/examples/n83_k600_m6_native_arms" "$out/arm-supervisor"
    install -m 755 "$CARGO_TARGET_DIR/release/examples/n83_k600_m6_native_preflight" "$out/preflight-supervisor"
    install -m 755 "$CARGO_TARGET_DIR/release/examples/n83_k600_m6_budget" "$out/budget-controller"
    ;;
  solver-binary)
    cms_tar=$3
    cadical_tar=$4
    cadiback_tar=$5
    mkdir -p "$out/solver-source" "$out/cadical-source" "$out/cadiback-source"
    tar -xzf "$cms_tar" -C "$out/solver-source" --strip-components=1
    tar -xzf "$cadical_tar" -C "$out/cadical-source" --strip-components=1
    tar -xzf "$cadiback_tar" -C "$out/cadiback-source" --strip-components=1
    cmake -S "$out/solver-source" -B "$out/solver-build" \
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_INSTALL_PREFIX="$out/solver-install" \
      -DFETCHCONTENT_SOURCE_DIR_CADICAL="$out/cadical-source" \
      -DFETCHCONTENT_SOURCE_DIR_CADIBACK="$out/cadiback-source" \
      -DNOBREAKID=ON -DNOMPI=ON -DBUILD_SHARED_LIBS=OFF -DSTATIC_BINARY=ON
    cmake --build "$out/solver-build" --parallel 1
    cmake --install "$out/solver-build"
    install -m 755 "$out/solver-install/bin/cryptominisat5" "$out/solver"
    "$out/solver" --version
    ;;
  runtime-image)
    commit=$3
    image_ref=$4
    context="$out/runtime-context"
    mkdir -p "$context"
    for binary in worker exporter solver arm-supervisor preflight-supervisor budget-controller; do
      install -m 755 "$out/$binary" "$context/$binary"
    done
    cat > "$context/Dockerfile" <<'DOCKERFILE'
FROM rust@sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c
ARG SOURCE_COMMIT
LABEL org.opencontainers.image.revision=$SOURCE_COMMIT
COPY worker exporter solver arm-supervisor preflight-supervisor budget-controller /usr/local/bin/
DOCKERFILE
    docker build --platform linux/arm64 --pull=false \
      --build-arg "SOURCE_COMMIT=$commit" --tag "$image_ref" "$context"
    ;;
  *)
    echo "unknown build step: $mode" >&2
    exit 2
    ;;
esac
