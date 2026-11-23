#!/usr/bin/env bash
# T9: select the PKM isogeny tower seed by J / columns. PROTOCOL.md (frozen before data).
# Executes exactly the planned measurement runs, strictly one heavy process at a time:
#   (a) census analysis       - no new runs (scripts/select_seeds.py on the parent census; re-run here for provenance)
#   (b) engine, k=20 t=6 G=4  - 3 f4cost runs (seed 0, 1093, 1308), 100 targets, target seed 2026111506
#   (c) end-to-end ecbench    - 1 session (ecbench/spec.json), then band, paired compares, replay verify
# Writes only under ../data (and ../analysis for the census table). No git. No rebuilds.
# Usage: scripts/run.sh            (all stages)
#        STAGES="b" scripts/run.sh (subset; stage letters a, b, c)
# Precondition (main session, not this script): .resume/RESUME.md filled and pushed
# (scripts/checkpoint.sh init), AGENTS.md "Resume checkpoints in S3 before every run".
set -euo pipefail
D="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$D/data"
FAM=${FAM:-/home/user/ctarget/release/curve25519_family}
ECB=${ECB:-/home/user/ctarget/release/ecbench}
# Frozen binary identities (sha256 at protocol freeze, 2026-11-03).
WANT_FAM=ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef
WANT_ECB=54737834d048b22e77399160fa9e159782b1124ea8190d84dba185309229641b
WANT_SPEC=f5b110d552cc6ed6eb56e6e7663ee917b506cbc132f7bc9fc56c06fa8e0c360c
CENSUS=/home/user/crypto/research/curve25519_isogeny_seed_20261103/data
STAGES=${STAGES:-abc}
mkdir -p "$OUT"
LOG="$OUT/run.log"
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$LOG"; }

# ---- identity checks (abort before any run on mismatch) ----
h=$(sha256sum "$FAM" | cut -d' ' -f1); [ "$h" = "$WANT_FAM" ] || { log "ABORT curve25519_family hash $h != frozen"; exit 2; }
h=$(sha256sum "$ECB" | cut -d' ' -f1); [ "$h" = "$WANT_ECB" ] || { log "ABORT ecbench hash $h != frozen"; exit 2; }
h=$(sha256sum "$D/ecbench/spec.json" | cut -d' ' -f1); [ "$h" = "$WANT_SPEC" ] || { log "ABORT spec hash $h != frozen"; exit 2; }
( cd "$CENSUS" && sha256sum -c --quiet - ) <<'SUMS'
3935c968587dd631c7cebfaaa4f530b03a5ed6da4045ab9cf9ba14504699d498  k16.jsonl
4e670176d073d90c7f7dd7a6cd951f25072330128794e981eeb48181696d84cf  k18.jsonl
bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc  k20.jsonl
11903b11c6d1f2eccfadd27b058e95fb2410999b21e22afc3a9854220c770e9e  k22.jsonl
39c7543607717def3ff68ef852151f482ea6ff0a13b33a9bd00bc34a2329eba0  k24.jsonl
SUMS
{ echo "curve25519_family $WANT_FAM"; echo "ecbench $WANT_ECB"; echo "spec $WANT_SPEC"; } > "$OUT/binary.sha256"
log "identity ok; stages=$STAGES"

# ---- (a) census analysis: exact, reads existing data only ----
if [[ $STAGES == *a* ]]; then
  log "START a: select_seeds.py"
  python3 -I "$D/scripts/select_seeds.py" "$CENSUS" > "$OUT/census_t9.txt"
  cp "$D/analysis/census_t9.json" "$OUT/census_t9.json"
  log "END a"
fi

# ---- (b) engine: f4cost at k=20 t=6 G=4, 100 targets, target seed 2026111506 ----
if [[ $STAGES == *b* ]]; then
  TS=2026111506; NT=100; CAP=3600; M=2; NNZ=30000000
  f4() { # label form
    local label=$1 form=$2
    local cmd=("$FAM" f4cost 20 "$form" 6 4 "$NT" "$TS" "$CAP" "$M" full 0 "$NNZ")
    log "START b $label: ${cmd[*]}"
    set +e; "${cmd[@]}" > "$OUT/engine_$label.json" 2> "$OUT/engine_$label.stderr"; local rc=$?; set -e
    log "END b $label rc=$rc"
  }
  f4 k20_t6_g4_s0    isogeny
  f4 k20_t6_g4_s1093 isogeny@1093
  f4 k20_t6_g4_s1308 isogeny@1308
fi

# ---- (c) end-to-end ecbench session at k=20 ----
if [[ $STAGES == *c* ]]; then
  S="$OUT/ecbench/s1"
  [ -e "$S" ] && { log "ABORT c: $S exists (a session directory is never reused)"; exit 3; }
  mkdir -p "$OUT/ecbench"
  cp "$D/ecbench/spec.json" "$OUT/ecbench/spec.json"
  uptime > "$OUT/ecbench/load_start.txt"
  log "START c run: $ECB run --wait --spec $D/ecbench/spec.json --out $S"
  set +e; "$ECB" run --wait --spec "$D/ecbench/spec.json" --out "$S" > "$OUT/ecbench/run.log" 2>&1; rc=$?; set -e
  log "END c run rc=$rc"
  uptime > "$OUT/ecbench/load_end.txt"
  set +e
  "$ECB" band --dir "$S" --ranks --json "$OUT/ecbench/band.json" > "$OUT/ecbench/band.txt" 2>&1
  log "c band rc=$?"
  # Paired compares (ratio = b / a), at the three band weights.
  for pair in "pkm-iso-t6-g4 pkm-iso-t6-g4-s1308" \
              "pkm-iso-t6-g4-s1093 pkm-iso-t6-g4-s1308" \
              "pkm-iso-t6-g4 pkm-iso-t6-g4-s1093" \
              "proj-a pkm-iso-t6-g4-s1308" \
              "rho-neg pkm-iso-t6-g4-s1308"; do
    set -- $pair
    for w in 0 0.1 1; do
      f="$OUT/ecbench/compare_${2}__over__${1}_w${w}"
      "$ECB" compare --dir "$S" --a "$1" --b "$2" --weight "$w" > "$f.txt" 2>&1
      "$ECB" compare --dir "$S" --a "$1" --b "$2" --weight "$w" --json > "$f.json" 2>"$f.json.stderr"
      log "c compare $2/$1 w=$w rc=$?"
    done
  done
  # Replay audit: 16 nonce-selected measured runs (auditor seed from OS entropy, recorded in the receipt).
  "$ECB" verify --dir "$S" --replay 16 --out "$OUT/ecbench/verify.json" > "$OUT/ecbench/verify.txt" 2>&1
  log "c verify rc=$?"
  set -e
fi

( cd "$OUT" && find . -type f ! -name SHA256SUMS ! -name run.log -print0 | sort -z | xargs -0 sha256sum > SHA256SUMS )
log "done"
