#!/bin/sh
# PROTOCOL.md section 3 and 6, thin orchestration only (every figure is native `ecbench`):
# band with verdicts, paired comparisons for every registered pair at w = 0, 0.1, 1,
# toy-tier exponent fits, then tables.mjs (tables and the registered verdicts) and plot.mjs.
set -eu
cd "$(dirname "$0")"
E=${ECBENCH:-../../target/release/ecbench}
mkdir -p analysis/bounds audits
# An audit that fails stops the analysis. A receipt already on disk is accepted only if it is for
# this session, ok, with no problems and every replay reproduced; a failed receipt never stays at its name.
for b in 20 24 28 32; do
  r="audits/holdout-P$b.json"
  if [ ! -f "$r" ]; then
    if "$E" verify --dir "sessions/holdout-P$b" --replay-all --exit-code --out "$r.tmp" > /dev/null 2>&1; then
      mv "$r.tmp" "$r"
    else
      echo "audit FAILED: holdout-P$b (failed receipt kept as $r.failed)" >&2
      mv "$r.tmp" "$r.failed" 2>/dev/null || true
      exit 1
    fi
  fi
  sid=$(jq -r '.session_id' "sessions/holdout-P$b/session.json")
  jq -e --arg id "$sid" '.session_id == $id and .ok == true and (.problems | length) == 0 and ([.replays[] | select(.reproduced != true)] | length) == 0' "$r" > /dev/null \
    || { echo "receipt $r is not a passing audit of session $sid" >&2; exit 1; }
done
"$E" band --dir sessions/holdout-P20 sessions/holdout-P24 sessions/holdout-P28 sessions/holdout-P32 --weights 0,0.1,1 --json analysis/band.json --ranks > analysis/band.md
for b in 20 24 28 32; do
  for pair in inc-sel:ksum5-sel inc-sel:ksum4-sel inc-w0:ksum5-sel inc-w0:ksum4-sel inc-rec:ksum5-sel inc-rec:ksum4-sel inc-rec:inc-sel ksum4-sel:ksum5-sel rho-neg:ksum5-sel rho-neg:ksum4-sel rho-neg:inc-sel bsgs-neg:ksum5-sel bsgs-neg:ksum4-sel; do
    a=${pair%%:*}; c=${pair#*:}
    case "$pair" in inc-w0:*) [ "$b" = 20 ] && continue ;; esac
    for w in 0 0.1 1; do
      "$E" compare --dir "sessions/holdout-P$b" --a "$a" --b "$c" --weight "$w" --json > "analysis/compare-P$b-${a}__${c}-w$w.json"
    done
  done
done
dirs="sessions/holdout-P20 sessions/holdout-P24 sessions/holdout-P28 sessions/holdout-P32"
audits=""; for b in 20 24 28 32; do audits="$audits --audit audits/holdout-P$b.json"; done
RULE="per-size parameters chosen on the train panel by research/prime_ksum_pooled_20261029/PROTOCOL.md (select/selection.json)"
: > analysis/bounds/fit.log
for w in 0 0.1 1; do
  for arm in rho-neg bsgs-neg inc-sel ksum4-sel ksum5-sel; do
    case "$arm" in inc-sel|ksum*) rule="$RULE" ;; *) rule="" ;; esac
    # shellcheck disable=SC2086
    if [ -n "$rule" ]; then
      "$E" bound fit --dir $dirs $audits --arm "$arm" --tier toy --weight "$w" --root ../.. --param-rule "$rule" \
        --label "prime-ksum-pooled $arm toy w=$w" --out "analysis/bounds/P-$arm-toy-w$w.json" > /dev/null 2>> analysis/bounds/fit.log \
        || echo "fit refused: $arm w=$w" >> analysis/bounds/fit.log
    else
      "$E" bound fit --dir $dirs $audits --arm "$arm" --tier toy --weight "$w" --root ../.. \
        --label "prime-ksum-pooled $arm toy w=$w" --out "analysis/bounds/P-$arm-toy-w$w.json" > /dev/null 2>> analysis/bounds/fit.log \
        || echo "fit refused: $arm w=$w" >> analysis/bounds/fit.log
    fi
  done
done
node tables.mjs
node plot.mjs
for f in ratio rho_multiple per_curve; do rsvg-convert -z 2 "$f.svg" -o "$f.png"; done
