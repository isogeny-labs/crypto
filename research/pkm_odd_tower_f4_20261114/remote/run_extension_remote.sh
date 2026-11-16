#!/usr/bin/env bash
# EXP-PKM-ODD-001 Amendment 5: budget extension for the m = 3 cells that hit the frozen 1,800 s budget.
# Six systems (XB-* cells), 86,400 s each, three at a time, RAYON_NUM_THREADS=4 per system.
# Each system's peak RSS is recorded beside its rows. Separate cells: never mixed with the frozen rows.
set -uo pipefail
cd /work
export RAYON_NUM_THREADS=4
P1=1399680001; SEED=20261114; BUDGET=86400; DMAX=64
mkdir -p runs/xb
echo "xb start $(date -u +%FT%TZ)" >> runs/progress.txt

cat > xb_wrap.py <<'EOF'
import json, resource, subprocess, sys, time
name, *argv = sys.argv[1:]
t0 = time.time()
rc = subprocess.run(argv).returncode
ru = resource.getrusage(resource.RUSAGE_CHILDREN)
json.dump({"cell": name, "rc": rc, "maxrss_kb": ru.ru_maxrss, "wall_s": time.time() - t0},
          open(f"runs/xb/{name}.rss.json", "w"))
EOF

jobs=(
  "XB-M3-A2-t5-tower|./pkm_odd_tower_f4 --p $P1 --m 3 --degrees 2,2,2,2,2 --control tower --kind refute --draws 1"
  "XB-M3-A23-n4-tower|./pkm_odd_tower_f4 --p $P1 --m 3 --degrees 2,3,2,3 --control tower --kind refute --draws 1"
  "XB-B-M3-A2-t5-planted|./pkm_algebraic_oracle --p $P1 --m 3 --form kummer --degrees 2,2,2,2,2 --planted 1 --random 0"
  "XB-B-M3-A23-n4-planted|./pkm_algebraic_oracle --p $P1 --m 3 --form kummer --degrees 2,3,2,3 --planted 1 --random 0"
  "XB-B-M3-A2-t5-random|./pkm_algebraic_oracle --p $P1 --m 3 --form kummer --degrees 2,2,2,2,2 --planted 0 --random 1"
  "XB-B-M3-A23-n4-random|./pkm_algebraic_oracle --p $P1 --m 3 --form kummer --degrees 2,3,2,3 --planted 0 --random 1"
)
for j in "${jobs[@]}"; do
  name="${j%%|*}"; cmd="${j#*|}"
  # shellcheck disable=SC2086
  echo "$name $cmd --seed $SEED --budget $BUDGET --dmax $DMAX --cell $name --out runs/xb/$name.jsonl"
done | xargs -P 3 -L 1 sh -c 'n="$0"; python3 xb_wrap.py "$n" "$@" > "runs/xb/$n.stdout" 2> "runs/xb/$n.stderr"; echo "xb done $n $(date -u +%FT%TZ)" >> runs/progress.txt'
echo "xb end $(date -u +%FT%TZ)" >> runs/progress.txt
