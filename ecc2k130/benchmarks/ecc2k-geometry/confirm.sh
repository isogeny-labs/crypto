#!/bin/bash
# The new RTX PRO 6000 defaults through the real build: `make bench ARCH=sm_120`
# (256-thread blocks) and `./bench rho` with no width or variant (W = 4, reg),
# against the previous defaults (128-thread blocks, --w 8 --variant lowmem).
set -uo pipefail
cd "${WORK:-/work}/benchmarks/ecc2k-geometry/tree" || exit 1
R=${RESULTS:-/results}; mkdir -p "$R"; REPS=${REPS:-5}
{ nvidia-smi --query-gpu=name,driver_version,compute_cap --format=csv,noheader; nvcc --version | tail -1; echo "source: ${SOURCE_REV:-unknown}"; } | tee "$R/host.txt"
python3 ecref2k.py params --curve ecc2k95 > curve_ecc2k95.h
fail=0
make -s -B bench ARCH=sm_120 > "$R/build-new.log" 2>&1 && mv bench bench_new || { tail "$R/build-new.log"; exit 1; }
make -s -B bench ARCH=sm_120 R2K_BLOCK_FLAG= > "$R/build-old.log" 2>&1 && mv bench bench_old || { tail "$R/build-old.log"; exit 1; }
for b in new old; do ./bench_$b selftest > "$R/selftest-$b.txt" 2>&1; grep -q 'selftest OK' "$R/selftest-$b.txt" && echo "selftest $b OK" || { echo "selftest $b FAILED"; fail=1; }; done
./bench_new rho --iters 64 2>&1 | grep -E 'class walks|blocks x' | tee "$R/defaults-new.txt"
grep -q 'x 4), dp' "$R/defaults-new.txt" && grep -q 'variant=reg' "$R/defaults-new.txt" && grep -q 'x 256 threads' "$R/defaults-new.txt" || { echo "new defaults not in effect"; fail=1; }
./bench_old rho --iters 64 --w 8 --variant lowmem 2>&1 | grep -E 'class walks|blocks x' | tee "$R/defaults-old.txt"
grep -q 'x 128 threads' "$R/defaults-old.txt" || { echo "old build not at 128 threads"; fail=1; }
for rep in $(seq 1 "$REPS"); do
  echo "new rep $rep: $(./bench_new rho --iters 2048 | grep -oE '[0-9.]+ Gstep/s')" | tee -a "$R/bench.txt"
  echo "old rep $rep: $(./bench_old rho --iters 2048 --w 8 --variant lowmem | grep -oE '[0-9.]+ Gstep/s')" | tee -a "$R/bench.txt"
done
python3 - "$R/bench.txt" <<'PY' | tee "$R/summary.txt"
import re, statistics, sys
d = {"new": [], "old": []}
for l in open(sys.argv[1]):
    m = re.match(r"(new|old) rep \d+: ([0-9.]+)", l)
    if m: d[m.group(1)].append(float(m.group(2)))
pairs = [a / b for a, b in zip(d["new"], d["old"])]
print("new median %.4f  old median %.4f  ratio %.4f  pairs %s  all>1 %s" % (statistics.median(d["new"]), statistics.median(d["old"]),
      statistics.median(d["new"]) / statistics.median(d["old"]), ["%.4f" % p for p in pairs], all(p > 1 for p in pairs)))
PY
echo "=== done"; exit $fail
