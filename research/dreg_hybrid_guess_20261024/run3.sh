#!/usr/bin/env bash
# (run2.sh: the same functions, tiers reordered by value after wall time proved ~20 min per branch; see Amendment 2)
# Hybrid guess-and-determine study (PREREGISTRATION.md).  Orchestration only:
# every computation is examples/dreg_closure --fix (branches), examples/dreg_hybrid
# (plant, score, model) and examples/dreg_ladder (fresh draws).
#
#   research/dreg_hybrid_guess_20261024/run.sh BIN_DIR TIER
#
# BIN_DIR holds dreg_closure, dreg_ladder, dreg_hybrid.  TIER is 0..6 (see the
# pre-registration).  One process per branch, each under the heavy-compute lock
# for its own duration; a branch whose output is non-empty is skipped on restart.
set -u
BIN=$1
TIER=${2:-0}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
RUNS=$HERE/runs
DRAWS=$HERE/draws
PLANTS=$HERE/plants
OLD=research/dreg_closure_scale_20261016
SPEC=research/dreg_closure_specialise_20261017/runs
LOCK=/tmp/claude-0/-home-user/c976b2e1-1199-51cb-8240-55b8b723e598/scratchpad/tasks/heavy.lock
mkdir -p "$RUNS" "$DRAWS" "$PLANTS"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }

# rule variable lists (m = 3): R1 top coordinate of each summand; R2 lowest intermediate coordinates
rule_vars() { # rule ell
  local r=$1 l=$2
  if [ "$r" = R1 ]; then echo "$((l-1)) $((2*l-1)) $((3*l-1))"; else echo "$((3*l)) $((3*l+1)) $((3*l+2))"; fi
}
fixlist() { # rule ell k b
  local vars; read -r -a vars <<< "$(rule_vars "$1" "$2")"
  local k=$3 b=$4 out="" j
  for ((j=0;j<k;j++)); do out+="${vars[j]}=$(( (b>>j)&1 )),"; done
  echo "${out%,}"
}

# branch NAME FILE DRAW MAXCOLS RULE ELL K B [--satisfiable]
branch() {
  local name=$1 file=$2 d=$3 mc=$4 rule=$5 ell=$6 k=$7 b=$8; shift 8
  local stem=$RUNS/$name.d$d.$rule.k$k.b$b
  if [ -s "$stem.jsonl" ]; then log "$name d$d $rule k$k b$b: done already, skipped"; return 0; fi
  local fix; fix=$(fixlist "$rule" "$ell" "$k" "$b")
  local fixarg=(); [ -n "$fix" ] && fixarg=(--fix "$fix")
  log "$name d$d $rule k$k b$b: start fix=${fix:-none} $*"
  local start; start=$(date +%s)
  local lk=(flock "$LOCK"); [ "${NOLOCK:-0}" = 1 ] && lk=()
  (cd "$REPO" && "${lk[@]}" prlimit --as=15000000000 timeout 3600 "$BIN/dreg_closure" --to 4 --max-cols "$mc" \
      --draw "$d" "${fixarg[@]}" "$@" "$file") > "$stem.jsonl" 2> "$stem.log"
  local rc=$?
  log "$name d$d $rule k$k b$b: exit $rc after $(( $(date +%s) - start )) s"
  [ -s "$stem.jsonl" ] || rm -f "$stem.jsonl"
  return $rc
}

# refutes NAME DRAW RULE K B -> 0 when the branch's record shows a refutation
refutes() {
  local f=$RUNS/$1.d$2.$3.k$4.b$5.jsonl
  [ -s "$f" ] && [ "$(jq -r '.closure_degree // "null"' "$f")" != null ]
}

# hybrid NAME FILE DRAW MAXCOLS RULE ELL K : all branches in order, stop at first non-refuting.
hybrid() {
  local name=$1 file=$2 d=$3 mc=$4 rule=$5 ell=$6 k=$7 b
  for ((b=0;b<(1<<k);b++)); do
    branch "$name" "$file" "$d" "$mc" "$rule" "$ell" "$k" "$b" || return 2
    refutes "$name" "$d" "$rule" "$k" "$b" || { log "$name d$d $rule k$k: fails at b$b"; return 1; }
  done
  log "$name d$d $rule k$k: all $((1<<k)) branches refute"; return 0
}

# escalate NAME FILE DRAW MAXCOLS ELL : R1, k = 1, then 2, then 3 only after a failure (rule 2)
escalate() {
  local k
  for k in 1 2 3; do
    hybrid "$1" "$2" "$3" "$4" R1 "$5" "$k" && { log "$1 d$3: k* = $k"; return 0; }
  done
  log "$1 d$3: k* > 3"
}

# unsat draws of a draws file, one index per line
unsat_draws() { jq -r 'select(.solutions==0) | .draw' "$REPO/$1"; }
sat_draws() { jq -r 'select(.solutions>0) | .draw' "$REPO/$1"; }

reuse() { # copy the predecessor's records under this study's names (provenance in runs/REUSED.txt)
  local cell d m arm rule k b src dst
  for cell in m3-19-7 m3-21-7; do
    if [ $cell = m3-19-7 ]; then ds="0 3 5 7"; else ds="0 2 3 6"; fi
    for d in $ds; do
      for m in "A0 R1 1 0" "A1 R1 1 1" "B R1 3 0" "C R2 1 0"; do
        read -r arm rule k b <<< "$m"
        src=$REPO/$SPEC/$cell.d$d.$arm
        dst=$RUNS/$cell.d$d.$rule.k$k.b$b
        if [ ! -s "$dst.jsonl" ] && [ -s "$src.jsonl" ]; then
          cp "$src.jsonl" "$dst.jsonl"; cp "$src.log" "$dst.log"
          echo "$dst <- $SPEC/$cell.d$d.$arm" >> "$RUNS/REUSED.txt"
        fi
      done
    done
  done
}

OLDF19=$OLD/draws/m3-19-7.jsonl; OLDF21=$OLD/draws/m3-21-7.jsonl
F19=research/dreg_hybrid_guess_20261024/draws/m3-19-7f.jsonl
reuse
case ${CHAIN:-A} in
A) # locked, 4 threads: the (21,7) escalation, then the k = 2 control, then fresh (21,7)
  escalate m3-21-7 $OLDF21 3 130000 7
  for b in 1; do branch m3-19-7 $OLDF19 2 130000 R1 7 1 $b --satisfiable; done
  for ((b=0;b<4;b++)); do branch m3-19-7 $OLDF19 1 130000 R1 7 2 $b --satisfiable; done
  ;;
B) # no lock, 2 threads, branches of at most about 3.1 GB: the (19,7) escalations, then fresh (19,7)
  export NOLOCK=1 RAYON_NUM_THREADS=2
  for d in 0 5; do escalate m3-19-7 $OLDF19 $d 130000 7; done
  for d in $(unsat_draws $F19); do
    stem=$RUNS/m3-19-7f.d$d.R1.k0.b0
    [ -s "$stem.jsonl" ] || branch m3-19-7f "$F19" $d 130000 R1 7 0 0
    jq -e '.closure_degree != null' "$stem.jsonl" >/dev/null && { log "m3-19-7f d$d: baseline refutes at 4, dropped"; continue; }
    escalate m3-19-7f "$F19" $d 130000 7
  done
  ;;
esac
echo "$(date -u +%FT%TZ) chain ${CHAIN:-A} finished" | tee -a "$RUNS/queue.log"
