#!/usr/bin/env bash
# Build the Lambda package: a static musl ecdb binary (Lambda's provided.al2023
# runtime has glibc 2.34, older than a typical build host's), a bootstrap,
# the RDS CA bundle, and the curve catalogues gzipped under data/.
#
#   ecdb/deploy/build-lambda.sh [out.zip]       (from anywhere in the checkout)
#
# Also packs the directory-tree inputs (`ecdb pack`): benchmarks/ and the
# sibling checkouts' files (../cryptanalysis, ../crypto-autoresearcher,
# ../cryptanalysis-rl, ../ml-cryptanalysis) go into the zip; the ecbench
# sessions (about 70 MB gzipped, over Lambda's direct-upload limit) go to
# <out>.ecbench/ as ~2 MB parts that deploy-aws.sh sends one per invocation.
#
# Needs: rustup target x86_64-unknown-linux-musl, musl-gcc (musl-tools), zip or python3.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/../.." && pwd)
out=${1:-$repo/ecdb/target/ecdb-lambda.zip}
target_dir=${CARGO_TARGET_DIR:-$repo/ecdb/target}
pkg=$(mktemp -d)
trap 'rm -rf "$pkg"' EXIT

(cd "$repo/ecdb" && CC_x86_64_unknown_linux_musl=${CC_x86_64_unknown_linux_musl:-musl-gcc} \
  cargo build --release --locked --target x86_64-unknown-linux-musl)
install -m 0755 "$target_dir/x86_64-unknown-linux-musl/release/ecdb" "$pkg/ecdb"
install -m 0755 "$here/bootstrap" "$pkg/bootstrap"
curl -fsS -o "$pkg/rds-ca.pem" https://truststore.pki.rds.amazonaws.com/us-west-2/us-west-2-bundle.pem
git -C "$repo" rev-parse HEAD > "$pkg/SOURCE_COMMIT"

# The inputs `ecdb import all` reads (src/import/*.rs); read_bytes takes <file>.gz.
inputs=(
  docs/curves/registry.json docs/curves/traits.json docs/curves/covers.json
  docs/curves/jacobian-transfers.json docs/curves/level-certificates.json docs/curves/jacobian-certificates.json
  research/isogeny_walk_20261004/runs/p256-ell31-radius2/isogeny_routes.json
  research/isogeny_walk_20261004/runs/p224-ell61-radius2/isogeny_routes.json
  research/isogeny_walk_p192_20261004/runs/p192-ell61-radius2/isogeny_routes.json
  research/prime61_isogeny_20261016/one-hop-l13/isogeny_routes.json
  research/prime61_isogeny_20261016/one-hop-l61/isogeny_routes.json
  docs/ic/fixtures/current_head_prime_route_20261024/isogeny_routes.json
  src/cryptanalysis/curve_traits/modular_polynomials.txt
  docs/ic/leaderboard.json docs/ic/progress-timeline.json docs/ic/boundary_targets.json
)
for f in "${inputs[@]}"; do
  mkdir -p "$pkg/data/$(dirname "$f")"
  gzip -9 -n -c "$repo/$f" > "$pkg/data/$f.gz"
done

"$pkg/ecdb" pack --repo-root "$repo" --out "$pkg/data" > "$pkg/pack-report.json"
parts="${out%.zip}.ecbench"
rm -rf "$parts"; mkdir -p "$parts"
mv "$pkg"/data/ecdb-pack/ecbench-*.jsonl.gz "$parts"/
echo "ecbench parts: $(ls "$parts" | wc -l) in $parts"

mkdir -p "$(dirname "$out")"
rm -f "$out"
(cd "$pkg" && zip -q -r -9 -X "$out" .)
echo "$out ($(du -h "$out" | cut -f1)), source $(cat "$pkg/SOURCE_COMMIT")"
