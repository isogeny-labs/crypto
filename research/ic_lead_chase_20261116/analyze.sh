#!/usr/bin/env bash
# Frozen analysis for protocol.json (lazy-orbit column sweep vs strong rho).
# Thin orchestration only: every number comes from the native ecbench binary
# (verify / compare / band); this script computes nothing itself.
#
# Usage:
#   ECBENCH=target/release/ecbench OUT=<dir> \
#   [W_CAL=<GAE/op> W_CAL_METHOD=perf-ipc W_CAL_HOST=<host>] \
#   bash research/ic_lead_chase_20261116/analyze.sh <session-dir>...
#
# Per session (one curve each) it writes, under $OUT/<session>/:
#   verify.json                  audit + replay of every deterministic measured run
#   cmp-aa-w<w>.json             rho-frob-aa vs rho-frob            (A/A, must contain 1)
#   cmp-h2-c<c>-w<w>.json        ic-orbit-c<c>-renew-lazy vs eager  (H2: lazy/eager < 1)
#   cmp-h3-c<c>-w<w>.json        ic-orbit-c<c>-renew-lazy vs rho-strong (H3: lazy/strong)
#   band.json                    §5a band 0/0.1/1, pairwise ranks; plus the calibrated
#                                block at 0, w_cal, 10*w_cal when W_CAL is given
# for w in {0, 0.1, 1}. Verdicts are applied by the reasoning tier from these files,
# per protocol.json decision_rule, with arm labels masked; nothing here decides.
set -euo pipefail
ECBENCH=${ECBENCH:-target/release/ecbench}
: "${OUT:?set OUT to a fresh output directory}"
[[ $# -ge 1 ]] || { echo "usage: analyze.sh <session-dir>..." >&2; exit 2; }
[[ ! -e "$OUT" ]] || { echo "OUT=$OUT already exists; use a fresh directory" >&2; exit 2; }
mkdir -p "$OUT"
sha256sum "$ECBENCH" > "$OUT/ecbench.sha256"
wcal=()
if [[ -n "${W_CAL:-}" ]]; then
  wcal=(--w-cal "$W_CAL" --w-cal-method "${W_CAL_METHOD:?}" --w-cal-host "${W_CAL_HOST:?}")
fi
for dir in "$@"; do
  s=$(basename "$dir"); o="$OUT/$s"; mkdir -p "$o"
  "$ECBENCH" verify --dir "$dir" --replay-all --out "$o/verify.json" > "$o/verify.txt"
  for w in 0 0.1 1; do
    wf=(--weight "$w")
    "$ECBENCH" compare --dir "$dir" --a rho-frob --b rho-frob-aa "${wf[@]}" --json > "$o/cmp-aa-w$w.json"
    for c in 1 2 3; do
      "$ECBENCH" compare --dir "$dir" --a "ic-orbit-c$c-renew" --b "ic-orbit-c$c-renew-lazy" "${wf[@]}" --json > "$o/cmp-h2-c$c-w$w.json"
      "$ECBENCH" compare --dir "$dir" --a rho-strong --b "ic-orbit-c$c-renew-lazy" "${wf[@]}" --json > "$o/cmp-h3-c$c-w$w.json"
    done
  done
  "$ECBENCH" band --dir "$dir" --weights 0,0.1,1 --ranks --json "$o/band.json" "${wcal[@]}" > "$o/band.txt"
done
( cd "$OUT" && find . -type f ! -name MANIFEST.sha256 -print0 | sort -z | xargs -0 sha256sum > MANIFEST.sha256 )
echo "analysis written to $OUT"
