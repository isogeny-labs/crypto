"""Exploratory (not in the preset): sum over F4 rounds of rows*cols*density (nonzeros) per run."""
import re, glob, os, json, collections, math, sys
ROUND = re.compile(r'^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s+([\d.]+)%')
logdir = sys.argv[1] if len(sys.argv) > 1 else '/dev/shm/bkk/runs/log'
agg = collections.defaultdict(list)
for f in sorted(glob.glob(f'{logdir}/m*_R_*.log')):
    tag = os.path.basename(f)[:-4]
    m, b, i, _, arm = tag.split('_')
    t = open(f).read()
    if 'overall(elapsed)' not in t:
        continue
    nnz = sum(int(g[3]) * int(g[4]) * float(g[5]) / 100 for g in (ROUND.match(l).groups() for l in t.splitlines() if ROUND.match(l)))
    agg[(int(m[1:]), int(b[1:]), arm)].append(nnz)
for k in sorted(agg):
    v = agg[k]
    print(*k, 'n', len(v), 'NNZsum_gm %.3g' % (2 ** (sum(math.log2(x) for x in v) / len(v))))
