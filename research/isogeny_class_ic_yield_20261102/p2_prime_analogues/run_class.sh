#!/bin/bash
# One (class, m) cell of the P2 study: spec -> ecbench session -> audit -> analysis.
# Thin orchestration only; every number comes from the Rust binaries.
#   run_class.sh <class id, e.g. c24A> <m: 2|3> <outdir> [extra p2tool spec flags]
# Environment: ECBENCH (default ./target/release/ecbench), P2TOOL (default p2tool/target..),
#              ALLOW_BUSY=1 (smoke only: record below L2), ISOLATION (default L2; ecbench refuses a busy host at L2), REPLAYS (default 12).
set -eu
cd "$(dirname "$0")/../../.."
D=research/isogeny_class_ic_yield_20261102/p2_prime_analogues
E=${ECBENCH:-./target/release/ecbench}
P=${P2TOOL:-$D/p2tool/target/release/p2tool}
cls=$1; m=$2; out=$3; shift 3
mkdir -p "$out"
Y=0.02; [ "$m" = 3 ] && Y=0.1
$P spec --classes $D/classes.json --class "$cls" --m "$m" --y "$Y" --isolation "${ISOLATION:-L2}" --out "$out/spec.json" "$@"
$E run --wait ${ALLOW_BUSY:+--allow-busy} --spec "$out/spec.json" --out "$out/session" > "$out/run.log" 2>&1
$E verify --dir "$out/session" --replay "${REPLAYS:-12}" --out "$out/audit.json" > "$out/verify.log" 2>&1 || echo "verify exit $? (see $out/verify.log)"
$P analyze --classes $D/classes.json --class "$cls" --m "$m" --session "$out/session" --out "$out/analysis.json"
