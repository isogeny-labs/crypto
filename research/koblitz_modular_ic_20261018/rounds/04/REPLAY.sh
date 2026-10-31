#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
replay=${2:-$(mktemp -d)}
mkdir -p "$replay"
cp "$expected/gl2b-integral-polarizations.json" "$expected/phi_j_11.txt" "$replay/"
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
"$runner" --quotient-lattices "$replay"
"$runner" --natural-quotient "$replay"
"$runner" --modular-geometry "$replay"
"$runner" --modular-transport "$replay"
"$runner" --modular-transport-paired "$replay"
"$runner" --direct-modular-relations "$replay"
"$runner" --modular-interior-test "$replay"
"$runner" --verify-round04 "$replay"
for name in q1-quotient-lattices q2-natural-quotient m1-modular-base m3-modular-transport m3b-paired-transport m4-direct-modular-relations m5-interior-function; do
  cmp "$expected/$name.json" "$replay/$name.json"
done
printf 'PASS: seven outputs replay byte for byte; separate library/divisor/BigInt checks pass.\n'
