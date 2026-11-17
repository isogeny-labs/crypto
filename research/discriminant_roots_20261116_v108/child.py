import sys, json, time, resource
from pathlib import Path
from .kernel import Field, solve, binary_irreducible
start = time.monotonic()
data = json.loads(Path(sys.argv[1]).read_text())
group, fi, ci, method = (sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5])
spec = data[group][fi]
case = spec['cases'][ci]
f = Field(spec['field']['p'], spec['field']['mod'])
if f.p == 2:
    assert binary_irreducible(f)
r = solve(case['H'], f, method, 1061611)
text = json.dumps({'result': r, 'algorithm_seconds': time.monotonic() - start, 'peak_rss_kib': resource.getrusage(resource.RUSAGE_SELF).ru_maxrss})
print(text)
