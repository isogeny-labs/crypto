#!/bin/bash
# ECC2K-95 class walk (gpu/ecc2k) launch geometry on one GPU.
#   1. gate: build the selftest at each batch width W and require the device walk
#      to match the host stepper bit for bit, with device DPs replaying on the host;
#   2. screen: every (block size, min blocks) build x variant x W, two passes;
#   3. final: the TOP screened configurations and the reference (main's default:
#      128-thread blocks, no min-blocks bound, lowmem, W = 8) alternating REPS times.
set -uo pipefail
cd "${WORK:-/work}/benchmarks/ecc2k-geometry/tree" || exit 1
R=${RESULTS:-/results}; mkdir -p "$R"
REPS=${REPS:-5}; TOP=${TOP:-3}; ITERS=${ITERS:-2048}
CAP=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d '. ')
{ nvidia-smi --query-gpu=name,driver_version,clocks.max.sm,power.limit,compute_cap --format=csv,noheader
  nvcc --version | tail -1; echo "source: ${SOURCE_REV:-unknown}"; echo "sm_$CAP reps $REPS top $TOP iters $ITERS"; } | tee "$R/host.txt"
sha256sum f2m.cuh koblitz.cuh rho2k.cuh kernels2k.cuh bench2k.cu > "$R/source-files.sha256"
python3 ecref2k.py params --curve ecc2k95 > curve_ecc2k95.h
nvb() { nvcc -O3 -std=c++17 -arch=sm_$CAP -lineinfo -Xptxas -v -DGPU_ECC2K_CURVE_HEADER='"curve_ecc2k95.h"' "$@"; }
tick() { nvidia-smi --query-gpu=clocks.sm,power.draw,temperature.gpu --format=csv,noheader; }
WIDTHS=${WIDTHS:-"2 4 8 16"}
fail=0
echo "##### gate"
for w in $WIDTHS; do
  nvb -DSELFTEST_W=$w -o st_$w bench2k.cu 2> "$R/ptxas-selftest-w$w.txt" || { echo "BUILD FAILED selftest W=$w"; fail=1; continue; }
  ./st_$w selftest > "$R/selftest-w$w.txt" 2>&1
  if grep -q "selftest OK" "$R/selftest-w$w.txt"; then echo "selftest W=$w OK: $(grep -E 'identical|replay|lowmem' "$R/selftest-w$w.txt" | tr '\n' ' ')"
  else echo "selftest W=$w FAILED"; grep -E "FAIL" "$R/selftest-w$w.txt" | head -5; fail=1; WIDTHS=$(echo " $WIDTHS " | sed "s/ $w / /"); fi
done
[ -z "$(echo $WIDTHS)" ] && { echo "no width passed the gate"; exit 1; }
echo "##### builds"
BUILDS=${BUILDS:-"128:0 128:4 128:6 128:8 256:0 256:2 256:3 256:4"}
OK=()
for b in $BUILDS; do
  blk=${b%%:*}; mb=${b##*:}; name="b${blk}m${mb}"
  if nvb -DR2K_BLOCK=$blk -DR2K_MIN_BLOCKS=$mb -o bench_$name bench2k.cu 2> "$R/ptxas-$name.txt"; then
    echo "$name: $(grep -A1 'k2k_rho_walk_lowmemILi8' "$R/ptxas-$name.txt" | grep -oE 'Used [0-9]+ registers|[0-9]+ bytes spill stores' | tr '\n' ' ')"
    OK+=("$name")
  else echo "BUILD FAILED $name"; fi
done
rate() { ./bench_$1 rho --w $3 --variant $2 --iters $ITERS 2>&1 | grep -oE '[0-9.]+ Gstep/s' | grep -oE '[0-9.]+'; }
echo "##### screen"
for pass in 1 2; do
  for n in "${OK[@]}"; do for v in lowmem reg; do for w in $WIDTHS; do
    r=$(rate $n $v $w); echo "screen $n $v w$w pass $pass: ${r:-0} Gstep/s  $(tick)" | tee -a "$R/screen.txt"
  done; done; done
done
python3 - "$R" "$TOP" <<'PY' | tee "$R/ranked.txt"
import re, sys, statistics
r, top = sys.argv[1], int(sys.argv[2]); rates = {}
for l in open(r + "/screen.txt"):
    m = re.match(r"screen (\S+) (\S+) (w\d+) pass \d: ([0-9.]+)", l)
    if m: rates.setdefault((m.group(1), m.group(2), m.group(3)), []).append(float(m.group(4)))
ref = ("b128m0", "lowmem", "w8")
rows = sorted(((statistics.median(v), k) for k, v in rates.items()), reverse=True)
base = statistics.median(rates[ref]) if ref in rates else None
for med, k in rows: print("%.4f %s %s %s%s" % (med, *k, "  / ref %.4f" % (med / base) if base else ""))
PY
FINAL=("b128m0 lowmem 8")
while read -r med n v w rest; do
  cand="$n $v ${w#w}"; [ "$cand" = "b128m0 lowmem 8" ] && continue
  FINAL+=("$cand"); [ ${#FINAL[@]} -gt "$TOP" ] && break
done < "$R/ranked.txt"
printf '%s\n' "${FINAL[@]}" > "$R/finalists.txt"
echo "##### final"
for rep in $(seq 1 "$REPS"); do
  for f in "${FINAL[@]}"; do set -- $f
    echo "final $1 $2 w$3 rep $rep: $(rate $1 $2 $3) Gstep/s  $(tick)" | tee -a "$R/final.txt"
  done
done
python3 - "$R" <<'PY' | tee "$R/summary.txt"
import re, statistics, sys
r = sys.argv[1]; s = {}; order = []
for l in open(r + "/final.txt"):
    m = re.match(r"final (\S+) (\S+) (w\d+) rep (\d+): ([0-9.]+)", l)
    if m:
        k = m.group(1, 2, 3)
        if k not in s: order.append(k)
        s.setdefault(k, []).append(float(m.group(5)))
ref = order[0]
for k in order:
    pairs = [a / b for a, b in zip(s[k], s[ref])]
    print("%-8s %-6s %-4s median %.4f Gstep/s  / ref %.4f  pairs %s  all>1 %s" % (*k, statistics.median(s[k]),
          statistics.median(s[k]) / statistics.median(s[ref]), ["%.4f" % p for p in pairs], all(p > 1 for p in pairs)))
PY
echo "=== done"; exit $fail
