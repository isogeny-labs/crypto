#!/bin/bash
# Build and run the carry-less issue-pattern probe (benchmarks/clmad-pairs/probe.cu)
# on whatever GPU the container has, then dump the CLMAD neighbourhood of the
# 20 B/s build's SASS so the kernel's actual issue pattern can be read beside it.
#   modal run --detach modal_job.py --job benchmarks/clmad-pairs/gpujob.sh --out DIR
set -uo pipefail
cd "${WORK:-/work}" || exit 1
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >/dev/null 2>&1; apt-get install -y -qq make g++ python3 >/dev/null 2>&1
R=${RESULTS:-/results}; mkdir -p "$R"
{
  nvidia-smi --query-gpu=name,driver_version,clocks.max.sm,power.limit,compute_cap --format=csv,noheader
  nvcc --version | tail -2
  echo "source: ${SOURCE_REV:-unknown}"
} | tee "$R/host.txt"
CAP=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d '. ')
echo "=== build probe for sm_$CAP"
nvcc -O3 -std=c++17 -arch=sm_$CAP -Xptxas -v benchmarks/clmad-pairs/probe.cu -o probe > "$R/build.log" 2>&1 || { tail -20 "$R/build.log"; exit 1; }
grep -E "registers|spill" "$R/build.log" | head -8
cuobjdump -sass probe > "$R/probe-sass.txt" 2>&1
python3 - "$R/probe-sass.txt" <<'PY2' | tee "$R/probe-sass-counts.txt"
import re, sys
name = None; counts = {}
for line in open(sys.argv[1]):
    m = re.search(r'Function : (\S+)', line)
    if m: name = m.group(1); counts[name] = 0; continue
    if name and re.search(r'\bCLMAD', line): counts[name] += 1
for k, v in counts.items(): print("%6d CLMAD in %s" % (v, k))
PY2
for bps in 8 2; do
  for pass in 1 2; do
    echo "=== probe pass $pass, $bps blocks/SM"
    ./probe 4000 3 $bps 2>&1 | tee -a "$R/probe-bps$bps.txt"
    nvidia-smi --query-gpu=clocks.sm,power.draw,temperature.gpu --format=csv,noheader | tee -a "$R/probe-bps$bps.txt"
  done
done
if [ "${SKIP_20B:-0}" = 1 ]; then echo "=== done"; exit 0; fi
echo "=== 20 B/s build: CLMAD issue pattern in SASS"
make -s gpu-preset "PRO6000_ARCH=-gencode arch=compute_${CAP},code=sm_${CAP}" KNOBS="" > "$R/build-20b.log" 2>&1 || { tail -5 "$R/build-20b.log"; exit 1; }
cuobjdump -sass ecc2k130 > "$R/20b-sass.txt" 2>&1
python3 - "$R/20b-sass.txt" <<'PY' | tee "$R/20b-clmad-pattern.txt"
import re, sys, collections
lines = [l for l in open(sys.argv[1]) if re.search(r'/\*[0-9a-f]{4,}\*/', l)]
ins = []
for l in lines:
    m = re.search(r'\*/\s*(?:@!?P\d\s+)?([A-Z0-9_.]+)\s*([^;]*);', l)
    if m: ins.append((m.group(1), m.group(2).strip()))
idx = [i for i, (op, _) in enumerate(ins) if op.startswith('CLMAD')]
print("instructions %d, CLMAD %d" % (len(ins), len(idx)))
gaps = collections.Counter()
same = adjacent = 0
def regs(s): return re.findall(r'R\d+', s)
for k in range(1, len(idx)):
    g = idx[k] - idx[k-1]
    gaps[min(g, 20)] += 1
    if g == 1:
        adjacent += 1
        a, b = ins[idx[k-1]], ins[idx[k]]
        if regs(a[1])[1:3] == regs(b[1])[1:3] and a[0] != b[0]: same += 1
print("gap histogram (instructions between consecutive CLMADs, 20 = 20+):")
for g in sorted(gaps): print("  %2d: %d" % (g, gaps[g]))
print("adjacent CLMAD pairs: %d, of which lo/hi of the same operands: %d" % (adjacent, same))
ops = collections.Counter(op for op, _ in ins if op.startswith('CLMAD'))
print("by opcode:", dict(ops))
PY
echo "=== done"
