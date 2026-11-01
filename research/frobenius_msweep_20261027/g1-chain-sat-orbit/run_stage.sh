#!/usr/bin/env bash
# One stage of cell G1. Usage:
#   run_stage.sh STAGE SEED DEFAULT_CAP NATURAL PLANTED COSET "n:m:k[:CAP] n:m:k[:CAP] ..."
# A cell token may carry its own conflict cap (`n:m:k:CAP`, as printed by the
# `cap` subcommand); `n:m:k:IMPEDIMENT` and `n:m:k:NODATA` skip the cell and record it. Otherwise
# DEFAULT_CAP applies. Optional environment: ARMS (default "P O R Q"), AA (1 = also run
# the A/A job, default 1), ANALYZE (1 = run `analyze` at the end, default 1).
# Needs RUN (UTC stamp). Writes reports/run-$RUN/. The resume-checkpoint gate
# (RUN_CARD.md step 0) is a session-side launcher gate: the session runs it
# before submitting; this script never invokes it (it may run in a remote job).
set -euo pipefail
cd "$(dirname "$0")/../../.."
STAGE=${1:?stage}; SEED=${2:?seed}; DCAP=${3:?default cap}; NAT=${4:?natural}; PL=${5:?planted}; CO=${6:?coset}; CELLS=${7-}
: "${RUN:?set RUN to the UTC stamp, e.g. 20261028T000000Z}"
# an empty cell list is an error (GNU xargs without -r would run once on nothing)
if [ -z "${CELLS//[[:space:]]/}" ]; then echo "empty cell list: nothing to run (check the caps.txt filter for this stage)" >&2; exit 2; fi
ARMS=${ARMS:-"P O R Q"}; AA=${AA:-1}; ANALYZE=${ANALYZE:-1}
G1=research/frobenius_msweep_20261027/g1-chain-sat-orbit
BIN=target/release/examples/frobenius_msweep_g1_chain_sat_orbit
OUT=$G1/reports/run-$RUN/$STAGE
mkdir -p "$OUT"
COMMON="--a 0 --seed $SEED --natural $NAT --planted $PL --coset $CO"

# 0. provenance, written once per stage before any job
{ echo "commit $(git rev-parse HEAD)"; echo "dirty_paths_in_g1:"; git status --short -- "$G1" examples/frobenius_msweep_g1_chain_sat_orbit.rs;
  echo "binary_sha256 $(sha256sum "$BIN")"; echo "rustc $(rustc --version)"; echo "host $(hostname)"; echo "nproc $(nproc)";
  echo "utc $(date -u +%FT%TZ)"; echo "args $*"; echo "ARMS=$ARMS AA=$AA"; } > "$OUT/provenance.txt"

# cells with their caps; IMPEDIMENT cells are recorded and dropped
LIVE=""
for c in $CELLS; do IFS=: read -r n m k cap <<<"$c"; cap=${cap:-$DCAP}
  case "$cap" in IMPEDIMENT|NODATA) ;; ''|*[!0-9]*|0) echo "bad or missing cap in cell token '$c' (use n:m:k:CAP from the cap subcommand)" >&2; exit 2;; esac
  if [ "$cap" = IMPEDIMENT ]; then echo "O-IMPEDIMENT(budget) $n $m $k" >> "$OUT/failures.txt"; continue; fi
  if [ "$cap" = NODATA ]; then echo "O-IMPEDIMENT(no-pilot-data) $n $m $k" >> "$OUT/failures.txt"; continue; fi
  LIVE="$LIVE $n:$m:$k:$cap"
done

if [ -z "${LIVE// /}" ]; then echo "every cell was skipped (IMPEDIMENT or NODATA); see $OUT/failures.txt" >&2; exit 2; fi

# 1. exact oracle, once per cell, shared by every arm
for c in $LIVE; do IFS=: read -r n m k cap <<<"$c"; echo "$n $m $k"; done |
  xargs -r -P4 -L1 bash -c 'timeout 7200 '"$BIN"' oracle --n $0 --m $1 --k $2 '"$COMMON"' --out '"$OUT"'/oracle'

# 2. one job per (cell, arm, conj); order shuffled with a fixed seed so arms interleave
for c in $LIVE; do IFS=: read -r n m k cap <<<"$c"
  for arm in $ARMS; do echo "$n $m $k $cap $arm 0"; done
  if [ "$AA" = 1 ]; then echo "$n $m $k $cap P 1"; fi   # A/A arm: plain on the sigma-conjugate target
done | shuf --random-source=<(yes "$SEED") |
  xargs -r -P4 -L1 bash -c 'ID=$(printf "a0-n%s-m%s-k%s-s%s" $0 $1 $2 '"$SEED"');
    timeout 14400 '"$BIN"' run --n $0 --m $1 --k $2 --arm $4 --conj $5 '"$COMMON"' \
      --conflicts $3 --tieout 3 --run-id '"$RUN"'-'"$STAGE"' \
      --oracle '"$OUT"'/oracle/oracle-$ID.jsonl --out '"$OUT"'/rows || echo "TIMEOUT-OR-FAIL $0 $1 $2 $4 $5" >> '"$OUT"'/failures.txt'

# 3a. independent certificate re-verification (native verifier crate, no solver, no shared code, no rerun);
#     a failure is recorded
VERIFY="$G1/verify_certs/target/release/g1-verify-certs"
[ -x "$VERIFY" ] || cargo build --release --offline --manifest-path "$G1/verify_certs/Cargo.toml" --target-dir "$G1/verify_certs/target" >&2
"$VERIFY" --rows "$OUT/rows" --manifests "$OUT/oracle" > "$OUT/verify_certs.txt" 2>&1 || echo "CERT-VERIFY-FAIL (see verify_certs.txt)" >> "$OUT/failures.txt"

# 3. analysis (not for the pilot, which has no A/A and only arms P and O)
if [ "$ANALYZE" = 1 ]; then "$BIN" analyze --rows "$OUT/rows" --out "$OUT/analysis"; fi
