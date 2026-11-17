"""Tabulates reports/<run>/raw/*.jsonl: checks every target's verdict and
indices agree across arms and rounds, that the f4 control engine charges the
same word XORs in both arms, and that the gate's pair lookups and group
additions are equal in both arms; then takes medians over rounds of the
recorded CPU time and word XORs, arm off / on.  Writes <run>/table.tsv.

    python3 summarize.py <run>
"""
import collections
import glob
import json
import statistics
import sys

run = sys.argv[1]
CELLS = ["K1_17", "K1_23", "K0_23", "K0_31", "K0_19", "K1_19", "K1_17d7", "K1_29", "K0_37"]
dec = collections.defaultdict(set)
geo = collections.defaultdict(set)
tot = collections.defaultdict(list)
nodes = collections.defaultdict(set)
for path in sorted(glob.glob(f"{run}/raw/*.jsonl")):
    lines = [json.loads(line) for line in open(path) if line.strip()]
    head, body = lines[0], lines[1:]
    cell, arm = head["cell"], head["arm"]
    per = collections.defaultdict(lambda: [0, 0, 0, 0])
    for d in body:
        if "target" in d:
            dec[(cell, d["arm"], d["target"])].add((d["hit"], json.dumps(d["indices"]), d["exhausted"]))
            if d["arm"] == "f6p":
                geo[(cell, d["target"])].add((d["pair_lookups"], d["geo_adds"]))
            p = per[d["arm"]]
            p[0] += d["reductions"]
            p[1] += d["splits"]
        elif "total_ms" in d:
            tot[(cell, d["arm"], arm)].append((d["cpu_ms"], d["word_xors"]))
    for e, p in per.items():
        nodes[(cell, e, arm)].add((p[0], p[1]))
print(f"targets x engines {len(dec)}; verdicts/indices differing across arms or rounds: "
      f"{sum(len(v) != 1 for v in dec.values())}")
print(f"f6p targets {len(geo)}; pair lookups / group additions differing between arms: "
      f"{sum(len(v) != 1 for v in geo.values())}")
with open(f"{run}/table.tsv", "w") as f:
    f.write("cell\tengine\tcpu_off\tcpu_on\tcpu_off/on\tcpu_lo\tcpu_hi\txor_off\txor_on\txor_off/on"
            "\treductions_off\treductions_on\tsplits_off\tsplits_on\n")
    for c in CELLS:
        for e in ("f4", "f6p"):
            off, on = tot.get((c, e, "off")), tot.get((c, e, "on"))
            if not off or not on:
                continue
            co = statistics.median(x for x, _ in off)
            cn = statistics.median(x for x, _ in on)
            lo = min(x for x, _ in off) / max(x for x, _ in on)
            hi = max(x for x, _ in off) / min(x for x, _ in on)
            xo = statistics.median(x for _, x in off)
            xn = statistics.median(x for _, x in on)
            (ro, so), = nodes[(c, e, "off")]
            (rn, sn), = nodes[(c, e, "on")]
            f.write(f"{c}\t{e}\t{co:.1f}\t{cn:.1f}\t{co / cn:.3f}\t{lo:.3f}\t{hi:.3f}\t{xo:.0f}\t{xn:.0f}"
                    f"\t{xo / xn:.3f}\t{ro}\t{rn}\t{so}\t{sn}\n")
            print(f"{c:8s} {e:4s} cpu off/on {co / cn:5.2f} [{lo:.2f}-{hi:.2f}]  xor off/on {xo / xn:5.2f}"
                  f"  reductions {ro}->{rn} splits {so}->{sn}")
