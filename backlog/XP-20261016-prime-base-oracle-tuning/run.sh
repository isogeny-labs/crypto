#!/bin/sh
# XP-20261016-prime-base-oracle-tuning: thin orchestration of PROTOCOL.md.
# Every measurement, audit and statistic is native `ecbench`; this script only
# sequences stages, skips completed sessions and fills the holdout templates
# from the train band (jq).  DRAFT: the protocol is not frozen.
#
#   sh backlog/XP-20261016-prime-base-oracle-tuning/run.sh STAGE OUT [--resume]
#
# STAGE: host | identity | train | select | holdout | kummer | audit |
#        analyse | all | verify.  Run from the repository root.  OUT is a new
# directory (research/prime_base_oracle_tuning_<run-date> when landing).
# --resume (or rerunning the same command) skips every session whose
# session.json says `complete`; an incomplete session directory is renamed
# <dir>.interrupted-<unix time> and kept, then rerun whole (PROTOCOL.md §8).
set -eu
STAGE=${1:?stage}; OUT=${2:?out}
CARD=backlog/XP-20261016-prime-base-oracle-tuning
E=${ECBENCH:-target/release/ecbench}
CPU=${PBO_CPU:-31}            # one core; `--cpus LIST` exits silently on some hosts
SIZES=${PBO_SIZES:-20 24 28 32}   # override only for a dry test, never in a measured run
mkdir -p "$OUT/sessions" "$OUT/specs" "$OUT/select" "$OUT/audits" "$OUT/analysis" "$OUT/host"
log() { printf '%s %s\n' "$(date -u +%FT%TZ)" "$*" | tee -a "$OUT/progress.txt"; }

session() { # SPEC NAME
  d="$OUT/sessions/$2"
  if [ -f "$d/session.json" ] && [ "$(jq -r .status "$d/session.json")" = complete ]; then
    log "skip $2 (complete)"; return 0
  fi
  [ -e "$d" ] && mv "$d" "$d.interrupted-$(date +%s)" && log "kept interrupted $2"
  log "run $2 spec $(sha256sum "$1" | cut -c1-16)"
  RAYON_NUM_THREADS=1 taskset -c "$CPU" "$E" run --spec "$1" --out "$d" \
    --cpus none --allow-busy --quiet
  [ "$(jq -r .status "$d/session.json")" = complete ] || { log "FAILED $2"; exit 1; }
}

ninc() { case $1 in 20) echo 32;; 24) echo 162;; 28) echo 407;; 32) echo 1024;; esac; }
nk() { awk -v n="$1" -v k="$2" 'BEGIN{printf "%d\n", int(n*2^(k/2)+0.5)}'; }

stage_host() {
  [ -z "$(git status --porcelain --untracked-files=no)" ] || { log "dirty tree"; exit 1; }
  git rev-parse HEAD > "$OUT/host/commit.txt"
  sha256sum "$E" > "$OUT/host/ecbench.sha256"
  rustc --version > "$OUT/host/rustc.txt"
  "$E" host --json > "$OUT/host/host.json"
  { lscpu; free -g; nproc; uname -a; } > "$OUT/host/machine.txt" 2>&1
  "$E" methods > "$OUT/host/methods.txt"
  log "host recorded: commit $(cat "$OUT/host/commit.txt")"
}

stage_identity() { # §5.1: the incumbent's frozen sessions replay exactly
  for b in $SIZES; do
    r="$OUT/audits/identity-frontier-P$b.json"
    [ -f "$r" ] && continue
    taskset -c "$CPU" "$E" verify --dir "research/ic_frontier_campaign_20261015/phase1/sessions/P$b" \
      --replay-all --exit-code --out "$r" > /dev/null || { log "IDENTITY FAILED P$b"; exit 1; }
    log "identity P$b ok"
  done
}

stage_train() {
  for b in $SIZES; do session "$CARD/specs/train-P$b.json" "train-P$b"; done
  for b in $SIZES; do extend "$b"; done
}

band_rows() { # BITS -> eligible rows as JSON lines {fam,n,s0,s1,s2,arm}
  dirs=$(ls -d "$OUT/sessions/train-P$1" "$OUT/sessions/train-ext-P$1-"* 2>/dev/null | grep -v interrupted)
  # shellcheck disable=SC2086
  "$E" band --dir $dirs --json "$OUT/select/band-train-P$1.json" > "$OUT/select/band-train-P$1.md"
  jq -c '.rows[] | select(.arm != "rho-neg" and .runs == .verified and .runs > 0
           and (.uncounted | length) == 0)
         | {arm, fam: (.arm | split("-n")[0]), n: (.arm | split("-n")[1] | tonumber),
            s0: .mean_s[0], s1: .mean_s[1], s2: .mean_s[2]}' "$OUT/select/band-train-P$1.json"
}

extend() { # §4.3 edge rule, per oracle family, at most k = ±4
  b=$1; n0=$(ninc "$b")
  while :; do
    more=0
    for fam in mitm2 batch2 mitm3 rfmitm3 rfsub3; do
      rows=$(band_rows "$b" | jq -sc --arg f "$fam" 'map(select(.fam == $f))')
      [ "$(echo "$rows" | jq length)" -ge 2 ] || continue
      best=$(echo "$rows" | jq 'min_by(.s1) | .n'); lo=$(echo "$rows" | jq 'map(.n) | min'); hi=$(echo "$rows" | jq 'map(.n) | max')
      for k in 3 4 -3 -4; do
        n=$(nk "$n0" $k)
        { [ "$best" = "$hi" ] && [ $k -gt 0 ] && [ "$n" -gt "$hi" ]; } || { [ "$best" = "$lo" ] && [ $k -lt 0 ] && [ "$n" -lt "$lo" ] && [ "$n" -ge 2 ]; } || continue
        name="train-ext-P$b-$fam-n$n"; spec="$OUT/specs/$name.json"
        [ -d "$OUT/sessions/$name" ] && [ -f "$OUT/sessions/$name/session.json" ] && continue
        jq --arg f "$fam" --argjson n "$n" '
          first(.arms[] | select(.name | startswith($f + "-n"))) as $t
          | .label += " (edge extension)"
          | .arms = [.arms[0], ($t | .name = "\($f)-n\($n)" | .role = "candidate"
                     | .method.params.factor_base = "prime-abscissa:size=\($n)")]' \
          "$CARD/specs/train-P$b.json" > "$spec"
        session "$spec" "$name"; more=1; break
      done
    done
    [ $more = 1 ] || break
  done
}

stage_select() { # §4.4; writes the holdout specs, then nothing may change
  for b in $SIZES; do
    rows=$(band_rows "$b" | jq -s .)
    pick() { echo "$rows" | jq -c "$1 | sort_by(.s1, (if .fam == \"mitm2\" then 0 else 1 end), .n) | .[0]"; }
    sel=$(pick '.'); size=$(pick 'map(select(.fam == "mitm2"))')
    oracle=$(jq -r --arg a "$(echo "$sel" | jq -r .arm)" '.arms[] | select(.name == $a) | .method.params.oracle' \
      "$CARD/specs/train-P$b.json" $(ls "$OUT/specs/train-ext-P$b-"*.json 2>/dev/null) | head -1)
    nsel=$(echo "$sel" | jq .n); nsize=$(echo "$size" | jq .n)
    jq -n --argjson b "$b" --argjson sel "$sel" --argjson size "$size" --arg o "$oracle" \
      --arg band "$(sha256sum "$OUT/select/band-train-P$b.json" | cut -d' ' -f1)" \
      '{schema: "xp-pbo.selection/v1", bits: $b, selected: ($sel + {oracle: $o}), size_only: $size,
        rule: "min mean S at w=0.1 over cells verified 8/8 with no uncounted counter; ties: incumbent oracle, then smaller N",
        band_train_sha256: $band}' > "$OUT/select/selection-P$b.json"
    sed -e "s|@SEL_FB@|prime-abscissa:size=$nsel|" -e "s|@SIZE_FB@|prime-abscissa:size=$nsize|" \
        -e "s|@STRIDE_FB@|prime-abscissa:size=$nsel,stride=7919|" -e "s|@SEL_ORACLE@|$oracle|g" \
        "$CARD/specs/holdout-P$b.template.json" > "$OUT/specs/holdout-P$b.json"
    "$E" plan --spec "$OUT/specs/holdout-P$b.json" > "$OUT/specs/holdout-P$b.plan.txt"
    log "selected P$b: $(echo "$sel" | jq -r .arm) ($oracle); size-only n$nsize; holdout spec $(sha256sum "$OUT/specs/holdout-P$b.json" | cut -c1-16)"
  done
  (cd "$OUT" && sha256sum select/selection-P*.json specs/holdout-P*.json > select/FROZEN.sha256)
  log "selection frozen: $(sha256sum "$OUT/select/FROZEN.sha256" | cut -c1-16)"
}

stage_holdout() {
  [ -f "$OUT/select/FROZEN.sha256" ] || { log "select first"; exit 1; }
  (cd "$OUT" && sha256sum -c --quiet select/FROZEN.sha256) || { log "selection changed after freeze"; exit 1; }
  for b in $SIZES; do session "$OUT/specs/holdout-P$b.json" "holdout-P$b"; done
}

stage_kummer() { for b in 22 24; do session "$CARD/specs/kummer-P$b.json" "kummer-P$b"; done; }

stage_audit() { # §6: every measured run re-executed, identical counts required
  for d in "$OUT"/sessions/*; do
    case $d in *.interrupted-*) continue;; esac
    s=$(basename "$d"); r="$OUT/audits/$s.json"
    [ -f "$r" ] && continue
    taskset -c "$CPU" "$E" verify --dir "$d" --replay-all --exit-code --out "$r" > /dev/null \
      || { log "AUDIT FAILED $s (receipt kept)"; exit 1; }
    log "audit $s ok $(sha256sum "$r" | cut -c1-16)"
  done
}

stage_analyse() { # §7
  A="$OUT/analysis"
  "$E" band --dir "$OUT"/sessions/holdout-P* --ranks --json "$A/band-holdout.json" > "$A/band-holdout.md"
  "$E" band --dir "$OUT"/sessions/kummer-P* --ranks --json "$A/band-kummer.json" > "$A/band-kummer.md"
  "$E" table --dir "$OUT"/sessions/holdout-P* > "$A/table-holdout.md"
  if ! "$E" compare --help | grep -q -- '--weight'; then
    log "BLOCKED: ecbench compare has no --weight (PROTOCOL.md §7.2); paired band intervals not computed"
    exit 3
  fi
  for b in $SIZES; do
    d="$OUT/sessions/holdout-P$b"
    for w in 0 0.1 1; do
      for pair in "inc sel" "inc size" "inc stride" "inc inc-aa" "rho-neg inc" "rho-neg sel" "rho-neg rho-neg-aa"; do
        set -- $pair
        "$E" compare --dir "$d" --a "$1" --b "$2" --weight "$w" --json > "$A/compare-P$b-$1__$2-w$w.json"
      done
    done
  done
  log "analysis written"
}

stage_verify() { # the card's outputs.verify: an independent re-check of a finished run
  V="$OUT/verify"; [ -e "$V" ] && { echo "$V exists; use a fresh copy of OUT" >&2; exit 2; }
  mkdir -p "$V"
  (cd "$OUT" && sha256sum -c --quiet select/FROZEN.sha256) || { echo "selection or holdout spec changed" >&2; exit 1; }
  bad=0
  for d in "$OUT"/sessions/*; do
    case $d in *.interrupted-*) continue;; esac
    s=$(basename "$d")
    "$E" verify --dir "$d" --replay-all --exit-code --out "$V/$s.json" > /dev/null || { echo "replay FAILED $s" >&2; bad=1; }
    n=$(jq -s '[.[] | select(.warmup == false and .outcome.matches_target != true)] | length' "$d/records.jsonl")
    [ "$n" = 0 ] || { echo "$s: $n measured runs without a verified scalar (kept; blocks a claim on that arm)" >&2; bad=1; }
  done
  [ $bad = 0 ] && echo "verify ok: every session replayed identically and every measured scalar verified"
  exit $bad
}

case $STAGE in
  all) stage_host; stage_identity; stage_train; stage_select; stage_holdout; stage_kummer; stage_audit; stage_analyse;;
  host|identity|train|select|holdout|kummer|audit|analyse|verify) "stage_$STAGE";;
  *) echo "unknown stage $STAGE" >&2; exit 2;;
esac
