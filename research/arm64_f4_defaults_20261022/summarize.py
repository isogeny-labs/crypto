"""Tabulates one host's records (<out>/{inh,gf2}/raw/*.jsonl): checks every arm
gives the same decisions, fingerprints and word counts as the default in every
round, then takes medians over rounds and ratios arm / default, and applies the
counts of scripts/DECISION_RULE.md.  Writes <out>/inh.tsv and <out>/gf2.tsv.

    python3 summarize.py <out>
"""
import collections
import glob
import json
import math
import statistics
import sys

out = sys.argv[1]
med = statistics.median
CELLS = ["K1_17", "K1_23", "K0_23", "K0_31", "K0_19", "K1_19", "K1_17d7", "K1_29", "K0_37"]
DECISION = ("hit", "indices", "exhausted", "reductions", "splits", "propagations",
            "infeasible", "geo_refute", "geo_witness", "partial_refute", "geo_adds")


def records(tool):
    for path in sorted(glob.glob(f"{out}/{tool}/raw/*.jsonl")):
        lines = [json.loads(line) for line in open(path) if line.strip()]
        if lines:
            yield lines[0], lines[1:]


# Inherited F4 / F6-IC: decisions per (cell, engine, target) over arms and rounds;
# word counts per (cell, engine, arm) over rounds; CPU per (cell, engine, arm).
dec = collections.defaultdict(set)
units = collections.defaultdict(set)
cpu = collections.defaultdict(list)
for head, body in records("inh"):
    for d in body:
        if "target" in d:
            dec[(head["cell"], d["arm"], d["target"])].add(tuple(json.dumps(d.get(k)) for k in DECISION))
        elif "total_ms" in d:
            units[(head["cell"], d["arm"], head["arm"])].add(d["word_xors"])
            cpu[(head["cell"], d["arm"], head["arm"])].append(d["cpu_ms"])
if dec:
    print(f"inherited: targets {len(dec)}; decisions differing across arms/rounds: "
          f"{sum(len(v) != 1 for v in dec.values())}; arm word counts varying across rounds: "
          f"{sum(len(v) != 1 for v in units.values())}")
    wins = collections.Counter()
    with open(f"{out}/inh.tsv", "w") as f:
        f.write("cell\tengine\tcpu_on\tcpu_noeager\tcpu_nogate\tnoeager/on\tnoeager/on_lo\tnoeager/on_hi"
                "\tnogate/on\tnogate/on_lo\tnogate/on_hi\tunits_on\tunits_noeager\tunits_nogate\n")
        for c in CELLS:
            for e in ("f4", "f6p"):
                if not cpu.get((c, e, "on")):
                    continue
                m = {a: med(cpu[(c, e, a)]) for a in ("on", "noeager", "nogate")}
                row = [c, e] + [f"{m[a]:.1f}" for a in ("on", "noeager", "nogate")]
                for a in ("noeager", "nogate"):
                    r = m[a] / m["on"]
                    lo = min(cpu[(c, e, a)]) / max(cpu[(c, e, "on")])
                    hi = max(cpu[(c, e, a)]) / min(cpu[(c, e, "on")])
                    row += [f"{r:.3f}", f"{lo:.3f}", f"{hi:.3f}"]
                    wins[(e, a)] += r >= 1.0
                row += [str(sorted(units[(c, e, a)])[0]) for a in ("on", "noeager", "nogate")]
                f.write("\t".join(row) + "\n")
                print(f"  {c:8s} {e:4s} CPU on {m['on']:9.1f} ms  noeager/on {m['noeager'] / m['on']:.2f}"
                      f"  nogate/on {m['nogate'] / m['on']:.2f}")
    for e in ("f4", "f6p"):
        print(f"  {e}: noeager/on >= 1 on {wins[(e, 'noeager')]}/9 cells (rule: >= 6); "
              f"nogate/on >= 1 on {wins[(e, 'nogate')]}/9 (rule for f6p: >= 7)")

# GF(2) tables: exactness keys per (tool, threads, case) over arms and rounds.
times = collections.defaultdict(list)
keys = collections.defaultdict(set)
for head, body in records("gf2"):
    for d in body:
        if "case" not in d:
            continue
        k = (head["tool"], head["threads"], d["case"])
        times[k + (head["arm"],)].append(d["wall_ms"])
        if head["tool"] == "f5":
            keys[k].add((d.get("rows_fp"), d.get("row_space_fp"), d.get("output_terms"), d.get("rank")))
        else:
            keys[k].add((d.get("basis_fp"), d.get("word_xors")))
if keys:
    print(f"gf2: exactness keys {len(keys)}; differing across arms/rounds: "
          f"{sum(len(v) != 1 for v in keys.values())}")
    with open(f"{out}/gf2.tsv", "w") as f:
        f.write("tool\tthreads\tcase\tarm\twall_t4\twall_arm\tarm/t4\n")
        groups = collections.defaultdict(list)
        for (tool, th, case, arm), v in sorted(times.items()):
            base = times.get((tool, th, case, "t4"))
            if arm == "t4" or not base:
                continue
            r = med(v) / med(base)
            groups[(tool, th, arm)].append(r)
            f.write(f"{tool}\t{th}\t{case}\t{arm}\t{med(base):.3f}\t{med(v):.3f}\t{r:.3f}\n")
    for (tool, th, arm), rs in sorted(groups.items()):
        gm = math.exp(sum(math.log(r) for r in rs) / len(rs))
        print(f"  {tool} t{th} {arm:5s} arm/t4 geomean {gm:.3f} (speedup {1 / gm:.3f}), "
              f"min {min(rs):.3f}, max {max(rs):.3f}, cases {len(rs)}")
