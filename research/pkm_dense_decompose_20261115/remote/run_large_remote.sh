#!/usr/bin/env bash
# EXP-PKM-DENSE-DEC-001 Amendment 1: larger S on a RunPod CPU pod. Four lanes, 4 threads each.
set -uo pipefail
cd /work; export RAYON_NUM_THREADS=4
mkdir -p runs/large; sha256sum pkm_algebraic_oracle > runs/large/binary.sha256
P1=1399680001; P3=3221225473; SEED=20261114; DMAX=1048576
cat > wrap.py <<'EOF'
import json, resource, subprocess, sys, time
name, *argv = sys.argv[1:]
t0 = time.time()
with open(f"runs/large/{name}.jsonl", "w") as out, open(f"runs/large/{name}.err", "w") as err:
    rc = subprocess.run(argv, stdout=out, stderr=err).returncode
ru = resource.getrusage(resource.RUSAGE_CHILDREN)
json.dump({"cell": name, "rc": rc, "maxrss_kb": ru.ru_maxrss, "wall_s": time.time() - t0},
          open(f"runs/large/{name}.rss.json", "w"))
EOF
deg() { local s="" i; for i in $(seq 1 "$1"); do s="$s,2"; done; echo "${s#,}"; }
cell() {  # name p m t membership vkind planted random budget
  python3 wrap.py "$1" ./pkm_algebraic_oracle --p "$2" --m "$3" --degrees "$(deg "$4")" --membership "$5" \
    --vkind "$6" --planted "$7" --random "$8" --seed $SEED --dmax $DMAX --budget "$9" --cell "$1"
  echo "done $1 $(date -u +%FT%TZ)" >> runs/large/progress.txt
}
echo "start $(date -u +%FT%TZ)" >> runs/large/progress.txt
( for t in 9 10; do cell "m2-t$t-both" $P1 2 $t both coset 4 2 7200; cell "m2-t$t-random" $P1 2 $t dense random 4 2 7200; done ) &
( cell "P3-m2-t8-both" $P3 2 8 both coset 4 2 1800
  for t in 11 12 13 14 15; do cell "P3-m2-t$t-dense" $P3 2 $t dense coset 4 2 1800; cell "P3-m2-t$t-random" $P3 2 $t dense random 4 2 1800; done ) &
( cell "m3-t6-dense" $P1 3 6 dense coset 4 2 7200; cell "m3-t6-random" $P1 3 6 dense random 4 2 7200 ) &
( cell "m3-t7-dense" $P1 3 7 dense coset 1 0 86400 ) &
wait
echo "end $(date -u +%FT%TZ)" >> runs/large/progress.txt
