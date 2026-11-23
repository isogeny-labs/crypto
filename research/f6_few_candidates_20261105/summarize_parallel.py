"""Tabulates reports/parallel-<run>/raw/*.jsonl: verdicts and indices of every
target must agree between the arms and across rounds; then the wall time per
cell (sum over targets of the probe's per-target `ms`, median over rounds) and
the word XORs, d0 / d2.  Writes <run>/table.tsv.  python3 summarize_parallel.py <run>"""
import collections
import glob
import json
import statistics
import sys

run = sys.argv[1]
CELLS = ["K1_17", "K1_23", "K0_23", "K0_31", "K0_19", "K1_19", "K1_17d7", "K1_29", "K0_37"]
dec = collections.defaultdict(set)
wall = collections.defaultdict(list)
xor = collections.defaultdict(list)
hits = {}
for path in sorted(glob.glob(f"{run}/raw/*.jsonl")):
    lines = [json.loads(line) for line in open(path) if line.strip()]
    head, body = lines[0], lines[1:]
    ms = 0.0
    for d in body:
        if "target" in d:
            dec[(head["cell"], d["target"])].add((d["hit"], json.dumps(d["indices"]), d["exhausted"]))
            ms += d["ms"]
        elif "total_ms" in d:
            xor[(head["cell"], head["arm"])].append(d["word_xors"])
            hits[head["cell"]] = d["hits"]
    wall[(head["cell"], head["arm"])].append(ms)
print(f"targets {len(dec)}; verdicts/indices differing across arms or rounds: "
      f"{sum(len(v) != 1 for v in dec.values())}")
with open(f"{run}/table.tsv", "w") as f:
    f.write("cell\thits\twall_d0\twall_d2\twall_d0/d2\twall_lo\twall_hi\txor_d0\txor_d2\txor_d2/d0\n")
    for c in CELLS:
        a, b = wall[(c, "d0")], wall[(c, "d2")]
        r = statistics.median(a) / statistics.median(b)
        lo, hi = min(a) / max(b), max(a) / min(b)
        x0, x2 = statistics.median(xor[(c, "d0")]), statistics.median(xor[(c, "d2")])
        f.write(f"{c}\t{hits[c]}\t{statistics.median(a):.1f}\t{statistics.median(b):.1f}\t{r:.3f}\t{lo:.3f}"
                f"\t{hi:.3f}\t{x0:.0f}\t{x2:.0f}\t{x2 / x0:.3f}\n")
        print(f"{c:8s} hits {hits[c]}  wall d0/d2 {r:5.2f} [{lo:.2f}-{hi:.2f}]  work d2/d0 {x2 / x0:.2f}")
