#!/usr/bin/env bash
# correctness matrix: integrated binaries vs baseline
S=${SCRATCH}
B=$S/baseline/bin; N=$S/integrated/bin; O=$S/integrated/matrix
FP="python3 $S/leads/f4-parallel/fps.py"
declare -A ARMS=(
 [default]=""
 [par_old]="F4_FP_PAR_OLD=1"
 [mem_old]="F4_FP_MEM_OLD=1"
 [mem_a0]="F4_FP_MEM_A=0"
 [mem_b0]="F4_FP_MEM_B=0"
 [mem_c0]="F4_FP_MEM_C=0"
 [fl_tiles]="F4_FP_PAR_FL_MODE=tiles"
 [fl_rows]="F4_FP_PAR_FL_MODE=rows"
 [m4ri_old]="KIC_M4RI_OLD=1"
 [all_old]="F4_FP_PAR_OLD=1 F4_FP_MEM_OLD=1 KIC_M4RI_OLD=1"
 [mem_old_tiles]="F4_FP_MEM_OLD=1 F4_FP_PAR_FL_MODE=tiles"
 [fl_blocked0]="F4_FP_FL_BLOCKED=0"
)
for t in 1 4; do
  RAYON_NUM_THREADS=$t $B/f4_fp_bench 1 large > $O/f4_base_t$t.json 2>/dev/null
  $FP $O/f4_base_t$t.json > $O/f4_base_t$t.fp
  RAYON_NUM_THREADS=$t F4_FP_FL_BLOCKED=0 $B/f4_fp_bench 1 large > $O/f4_base_fl_blocked0_t$t.json 2>/dev/null
  $FP $O/f4_base_fl_blocked0_t$t.json > $O/f4_base_fl_blocked0_t$t.fp
  for a in "${!ARMS[@]}"; do
    env RAYON_NUM_THREADS=$t ${ARMS[$a]} $N/f4_fp_bench 1 large > $O/f4_${a}_t$t.json 2>/dev/null
    $FP $O/f4_${a}_t$t.json > $O/f4_${a}_t$t.fp
    ref=$O/f4_base_t$t.fp; [ $a = fl_blocked0 ] && ref=$O/f4_base_fl_blocked0_t$t.fp
    if cmp -s $ref $O/f4_${a}_t$t.fp; then echo "f4 t$t $a SAME ($(wc -l < $O/f4_${a}_t$t.fp) cases)"; else echo "f4 t$t $a DIFF"; diff $ref $O/f4_${a}_t$t.fp | head -5; fi
  done
done
# f4 at 2 threads, tiles forced (fused tiled path with several tiles)
for a in default fl_tiles mem_old; do
  env RAYON_NUM_THREADS=2 ${ARMS[$a]} $N/f4_fp_bench 1 large > $O/f4_${a}_t2.json 2>/dev/null
  $FP $O/f4_${a}_t2.json > $O/f4_${a}_t2.fp
  cmp -s $O/f4_base_t4.fp $O/f4_${a}_t2.fp && echo "f4 t2 $a SAME (vs base t4)" || { echo "f4 t2 $a DIFF"; diff $O/f4_base_t4.fp $O/f4_${a}_t2.fp | head -5; }
done
# GF(2)
strip() { python3 -c '
import sys
for l in open(sys.argv[1]):
    if l.startswith("|"):
        cells=l.rstrip("\n").split("|")
        print("|".join(c for i,c in enumerate(cells) if i not in drop))
        continue
    print(l,end="")
' "$1"; }
stripxb() { python3 - "$1" <<'PY'
import sys
drop=set()
for l in open(sys.argv[1]):
    if l.startswith("|"):
        cells=l.rstrip("\n").split("|")
        if any("wall" in c for c in cells):
            drop={i for i,c in enumerate(cells) if "wall" in c}
        print("|".join(c for i,c in enumerate(cells) if i not in drop))
    else:
        drop=set() if not l.strip() else drop
        print(l,end="")
PY
}
kern() { python3 -c '
import sys,json
for l in open(sys.argv[1]):
    if l.startswith("{"):
        j=json.loads(l)
        for k in ("median_ms","samples_ms"): j.pop(k,None)
        print(json.dumps(j,sort_keys=True))
' "$1"; }
for t in 1 4; do
  RAYON_NUM_THREADS=$t $B/crossbred_bench 9 11 13 > $O/xb_base_t$t.txt 2>&1; stripxb $O/xb_base_t$t.txt > $O/xb_base_t$t.s
  for s in x sym; do RAYON_NUM_THREADS=$t $B/koblitz_f4_kernel_bench 5 $s > $O/k${s}_base_t$t.txt 2>&1; kern $O/k${s}_base_t$t.txt > $O/k${s}_base_t$t.s; done
  for a in default m4ri_old all_old par_old mem_old; do
    env RAYON_NUM_THREADS=$t ${ARMS[$a]} $N/crossbred_bench 9 11 13 > $O/xb_${a}_t$t.txt 2>&1; stripxb $O/xb_${a}_t$t.txt > $O/xb_${a}_t$t.s
    cmp -s $O/xb_base_t$t.s $O/xb_${a}_t$t.s && echo "xb t$t $a SAME" || { echo "xb t$t $a DIFF"; diff $O/xb_base_t$t.s $O/xb_${a}_t$t.s | head -6; }
    for s in x sym; do
      env RAYON_NUM_THREADS=$t ${ARMS[$a]} $N/koblitz_f4_kernel_bench 5 $s > $O/k${s}_${a}_t$t.txt 2>&1; kern $O/k${s}_${a}_t$t.txt > $O/k${s}_${a}_t$t.s
      cmp -s $O/k${s}_base_t$t.s $O/k${s}_${a}_t$t.s && echo "kern $s t$t $a SAME" || { echo "kern $s t$t $a DIFF"; diff $O/k${s}_base_t$t.s $O/k${s}_${a}_t$t.s | head -4; }
    done
  done
done
