#!/usr/bin/env bash
# Thin, resumable orchestration for protocol.json.  It performs no analysis.
set -euo pipefail

if [[ $# -ne 3 ]]; then
  echo "usage: $0 LADDER_BIN CLOSURE_BIN gen|audit-draws|identity|controls|measure" >&2
  exit 2
fi

LADDER=$1
CLOSURE=$2
STEP=$3
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
# Draws are immutable committed inputs.  Runs and checkpoints may live on a
# detached worker's durable collection volume instead of inside its checkout.
DRAWS=${DREG_DRAWS_DIR:-$HERE/draws}
OUTPUT_ROOT=${DREG_OUTPUT_ROOT:-$HERE}
RUNS=$OUTPUT_ROOT/runs
CHECKPOINTS=$OUTPUT_ROOT/checkpoints
SEED=20261101

mkdir -p "$DRAWS" "$RUNS" "$CHECKPOINTS"
export F4_F2_MAX_ROWS=50000000
export F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=4
export DREG_CKPT_SECONDS=900

cells=("5:11:2" "5:13:3" "6:11:2")

cell_file() {
  local m=$1 n=$2 ell=$3
  printf '%s/m%s-n%s-l%s.jsonl' "$DRAWS" "$m" "$n" "$ell"
}

draw_indices() {
  local file=$1 kind=$2 limit=$3
  if [[ $kind == sat ]]; then
    jq -r 'select(.outcome.kind == "satisfiable" and .solutions > 0) | .draw' "$file" | sed -n "1,${limit}p"
  else
    jq -r 'select(.outcome.kind != "satisfiable" and .solutions == 0) | .draw' "$file" | sed -n "1,${limit}p"
  fi
}

audit_one() {
  local m=$1 n=$2 ell=$3 file
  file=$(cell_file "$m" "$n" "$ell")
  [[ -s $file ]] || { echo "missing draw file: $file" >&2; return 1; }
  jq -e -c . "$file" >/dev/null
  local u s
  u=$(draw_indices "$file" unsat 4 | wc -l)
  s=$(draw_indices "$file" sat 2 | wc -l)
  [[ $u -eq 4 && $s -eq 2 ]] || {
    echo "m=$m n=$n ell=$ell draw gate failed: unsat=$u sat=$s" >&2
    return 1
  }
  echo "m=$m n=$n ell=$ell draw gate passed: unsat=4 sat=2"
}

run_closure() {
  local name=$1 file=$2 draw=$3 mode=$4 max_cols=$5
  local out=$RUNS/$name.d$draw.jsonl
  local log=$RUNS/$name.d$draw.log
  local status=$RUNS/$name.d$draw.status
  local ckpt=$CHECKPOINTS/$name-d$draw
  if [[ -s $out && -s $status && $(<"$status") == 0 ]]; then
    echo "$name draw $draw already complete"
    return 0
  fi
  local flags=()
  if [[ $mode == sat ]]; then flags+=(--satisfiable); fi
  set +e
  (
    cd "$REPO"
    prlimit --as=32000000000 \
      timeout --signal=TERM --kill-after=1800 86400 \
      "$CLOSURE" "${flags[@]}" --to 4 --max-cols "$max_cols" \
      --draw "$draw" --checkpoint-dir "$ckpt" "$file"
  ) >"$out" 2>>"$log"
  local rc=$?
  set -e
  printf '%s\n' "$rc" >"$status"
  if [[ $rc -ne 0 ]]; then
    echo "$name draw $draw exited $rc; retained as unmeasured/resumable" >&2
  fi
  return 0
}

audit_identity() {
  local got=$RUNS/identity-m4-n12-l5.d24.jsonl
  local status=$RUNS/identity-m4-n12-l5.d24.status
  local expected=$REPO/research/dreg_closure_driver_20261016/runs/fresh-m4-12-5.d24.jsonl
  [[ -s $got && -s $status && $(<"$status") == 0 ]] || {
    echo "identity replay is incomplete" >&2
    return 1
  }
  if ! diff -u \
      <(jq -S 'del(.source,.checkpoint_segments) | .scan |= map(del(.secs))' "$expected") \
      <(jq -S 'del(.source,.checkpoint_segments) | .scan |= map(del(.secs))' "$got") \
      >"$RUNS/identity.diff"; then
    echo "identity replay differs in an algebraic field" >&2
    return 1
  fi
  echo "identity replay passed"
}

audit_controls() {
  for c in "${cells[@]}"; do
    IFS=: read -r m n ell <<<"$c"
    local file
    file=$(cell_file "$m" "$n" "$ell")
    while read -r draw; do
      local stem=$RUNS/control-m$m-n$n-l$ell.d$draw
      [[ -s $stem.jsonl && -s $stem.status && $(<"$stem.status") == 0 ]] || {
        echo "control m=$m n=$n ell=$ell draw=$draw is incomplete" >&2
        return 1
      }
      jq -e '.solutions > 0 and .refuted_satisfiable == false and .measured_to == 4' \
        "$stem.jsonl" >/dev/null || {
          echo "control m=$m n=$n ell=$ell draw=$draw failed soundness/completeness" >&2
          return 1
        }
    done < <(draw_indices "$file" sat 2)
  done
  echo "all six satisfiable controls passed"
}

case "$STEP" in
  gen)
    (cd "$REPO" && sha256sum -c "$HERE/sources.sha256")
    for c in "${cells[@]}"; do
      IFS=: read -r m n ell <<<"$c"
      file=$(cell_file "$m" "$n" "$ell")
      [[ ! -e $file ]] || { echo "refusing to overwrite $file" >&2; exit 1; }
      (cd "$REPO" && "$LADDER" --cells "$n:$ell:3" --m "$m" \
        --unsat 4 --ffd-max 3 --seed "$SEED" --max-draws 4096) \
        >"$file" 2>"${file%.jsonl}.log"
    done
    ;;
  audit-draws)
    for c in "${cells[@]}"; do
      IFS=: read -r m n ell <<<"$c"
      audit_one "$m" "$n" "$ell"
    done
    ;;
  identity)
    (cd "$REPO" && sha256sum -c "$HERE/sources.sha256")
    run_closure identity-m4-n12-l5 \
      research/dreg_closure_driver_20261016/draws/m4-12-5.jsonl 24 unsat 155000
    audit_identity
    ;;
  controls)
    for c in "${cells[@]}"; do
      IFS=: read -r m n ell <<<"$c"
      audit_one "$m" "$n" "$ell"
      file=$(cell_file "$m" "$n" "$ell")
      while read -r draw; do
        run_closure "control-m$m-n$n-l$ell" "$file" "$draw" sat 400000
      done < <(draw_indices "$file" sat 2)
    done
    audit_controls
    ;;
  measure)
    audit_identity
    audit_controls
    for c in "${cells[@]}"; do
      IFS=: read -r m n ell <<<"$c"
      audit_one "$m" "$n" "$ell"
      file=$(cell_file "$m" "$n" "$ell")
      while read -r draw; do
        run_closure "unsat-m$m-n$n-l$ell" "$file" "$draw" unsat 400000
      done < <(draw_indices "$file" unsat 4)
    done
    ;;
  *)
    echo "unknown step: $STEP" >&2
    exit 2
    ;;
esac

