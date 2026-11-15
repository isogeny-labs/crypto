#!/usr/bin/env bash
# EXP-PKM-DENSE-DEC-001 Amendment 3: the Amendment 1 cells lost with pod y8fnixt44o3ziv, rerun so that
# pod loss costs at most the target in flight. Runs in WORK on a network volume
# (tools/runpod_resilient/supervisor.py restarts it on a replacement pod). Idempotent:
#   * a cell with a .done marker is skipped;
#   * a cell without one resumes: the oracle (--out --resume 1) redraws the same targets from the same
#     seed and skips every index already in the cell's .jsonl;
#   * each attempt appends {attempt, rc, maxrss_kb, wall_s, pod} to the cell's .rss.jsonl.
set -uo pipefail
cd "$(dirname "$0")"; export RAYON_NUM_THREADS=4
R=runs/large; mkdir -p $R; sha256sum pkm_algebraic_oracle > $R/binary.sha256
P1=1399680001; P3=3221225473; SEED=20261114; DMAX=1048576
cat > wrap.py <<'PY'
import json, os, resource, subprocess, sys, time
name, *argv = sys.argv[1:]
t0 = time.time()
with open(f"runs/large/{name}.log", "a") as log:
    rc = subprocess.run(argv, stdout=subprocess.DEVNULL, stderr=log).returncode
ru = resource.getrusage(resource.RUSAGE_CHILDREN)
with open(f"runs/large/{name}.rss.jsonl", "a") as f:
    f.write(json.dumps({"cell": name, "rc": rc, "maxrss_kb": ru.ru_maxrss, "wall_s": time.time() - t0,
                        "pod": os.environ.get("RUNPOD_POD_ID")}) + "\n")
sys.exit(rc)
PY
deg() { local s="" i; for i in $(seq 1 "$1"); do s="$s,2"; done; echo "${s#,}"; }
cell() {  # name p m t membership vkind planted random budget
  [ -e "$R/$1.done" ] && return
  echo "attempt $1 $(date -u +%FT%TZ) pod ${RUNPOD_POD_ID:-?}" >> $R/progress.txt
  if python3 wrap.py "$1" ./pkm_algebraic_oracle --p "$2" --m "$3" --degrees "$(deg "$4")" --membership "$5" \
      --vkind "$6" --planted "$7" --random "$8" --seed $SEED --dmax $DMAX --budget "$9" --cell "$1" \
      --out "$R/$1.jsonl" --resume 1; then
    touch "$R/$1.done"; echo "done $1 $(date -u +%FT%TZ)" >> $R/progress.txt
  else
    echo "failed $1 $(date -u +%FT%TZ)" >> $R/progress.txt
  fi
}
echo "start $(date -u +%FT%TZ) pod ${RUNPOD_POD_ID:-?}" >> $R/progress.txt
( cell "m2-t10-both" $P1 2 10 both coset 4 2 7200
  cell "m2-t10-random" $P1 2 10 dense random 4 2 7200
  cell "P3-m2-t12-random" $P3 2 12 dense random 4 2 1800 ) &
( for t in 13 14 15; do cell "P3-m2-t$t-dense" $P3 2 $t dense coset 4 2 1800; cell "P3-m2-t$t-random" $P3 2 $t dense random 4 2 1800; done ) &
( cell "m3-t6-dense" $P1 3 6 dense coset 4 2 7200; cell "m3-t6-random" $P1 3 6 dense random 4 2 7200 ) &
( cell "m3-t7-dense" $P1 3 7 dense coset 1 0 86400 ) &
wait
echo "end $(date -u +%FT%TZ)" >> $R/progress.txt; touch $R/COMPLETE
