import json, os, statistics, glob, re
ROOT = "/home/user/crypto/research/f4_generic_field_20261015/runs"
def rows(p):
    return [x for x in (json.loads(l) for l in open(p) if l.strip()) if "N" in x]
# A. speed claims: pair rep i of baseline with rep i of candidate
def arm_ms(cell, arm):
    out = {}
    for p in glob.glob(os.path.join(ROOT, "speed", f"{cell}.{arm}.rep*.jsonl")):
        rep = int(re.search(r"rep(\d+)", p).group(1))
        rs = rows(p)
        out[rep] = [r["ms"] for r in rs]
    return out
claims = [("S1a", "C1", "u32", "fp32"), ("S1b", "C2", "u32", "fp32"), ("S2a", "C3", "u32", "fp32"),
          ("S2b", "C4", "u32", "fp32"), ("S3", "C5", "fp64", "fp52")]
for cid, cell, base, cand in claims:
    b = arm_ms(cell, base); c = arm_ms(cell, cand)
    reps = sorted(set(b) & set(c))
    bm = [statistics.median(b[r]) for r in reps]; cm = [statistics.median(c[r]) for r in reps]
    ratios = [x / y for x, y in zip(bm, cm)]
    print(cid, cell, base, cand, "reps", len(reps), "rows per rep", set(len(v) for v in b.values()), set(len(v) for v in c.values()))
    print(cid, "median base", round(statistics.median(bm), 3), "median cand", round(statistics.median(cm), 3))
    print(cid, "paired ratio median", round(statistics.median(ratios), 4), "min", round(min(ratios), 4), "max", round(max(ratios), 4))
# B. ns per muladd from speed reps: median ms * 1e6 / muladds
for cell, arm in [("C1","fp32"),("C5","fp52"),("C6","fp64"),("C7","fp224"),("C8","p192")]:
    rs = [r for p in glob.glob(os.path.join(ROOT, "speed", f"{cell}.{arm}.rep*.jsonl")) for r in rows(p)]
    if not rs: print("no speed rows", cell, arm); continue
    med_ms = statistics.median([r["ms"] for r in rs]); mu = rs[0]["muladds"]
    print("ns/muladd", cell, arm, round(med_ms * 1e6 / mu, 4), "muladds", mu, "backend", rs[0]["backend"], "p", rs[0]["p"])
# C. equality pairs: fields excluding label/clock fields
IGN = {"engine", "backend", "builder", "ms", "wall_s"}
eq = os.path.join(ROOT, "eq")
pairs = [("dickson-p1.u32.jsonl", "dickson-p1.fp32.jsonl"), ("kummer-p2.u32.jsonl", "kummer-p2.fp32.jsonl"),
         ("torus-T24.u32.jsonl", "torus-T24.fp32.jsonl"), ("torus-T31.u32.jsonl", "torus-T31.fp32.jsonl"),
         ("kummer-p2.fp64.jsonl", "kummer-p2.fp32.jsonl"), ("kummer-p2.fp52.jsonl", "kummer-p2.fp32.jsonl"),
         ("kummer-p2.wide4.jsonl", "kummer-p2.fp32.jsonl")]
tot = {"n": 0, "eq": 0, "diff": 0, "unmatched": 0}
def key(x):
    return json.dumps([x.get(k) for k in ("curve", "p", "m", "N", "t", "target_index", "control", "kind")], sort_keys=True)
for a, b in pairs:
    ra, rb = rows(os.path.join(eq, a)), rows(os.path.join(eq, b))
    if len(ra) == len(rb):
        matched = list(zip(ra, rb)); mode = "position"
    else:
        idx = {}
        for y in rb: idx.setdefault(key(y), []).append(y)
        matched = []
        for x in ra:
            cand = idx.get(key(x))
            if cand: matched.append((x, cand.pop(0)))
        mode = "by-key"
    e = 0; only_a = set(); only_b = set()
    for x, y in matched:
        common = [k for k in x if k in y and k not in IGN]
        if all(x[k] == y[k] for k in common): e += 1
    n = len(matched)
    print("eq", a, "vs", b, "mode", mode, "rows", len(ra), len(rb), "compared", n, "equal", e, "differ", n - e, "unmatched_u32", len(ra) - n)
    tot["n"] += n; tot["eq"] += e; tot["diff"] += n - e; tot["unmatched"] += len(ra) - n
print("eq totals", tot)
refs = glob.glob(os.path.join(ROOT, "..", "ref", "*.jsonl"))
print("ref rows total", sum(len(rows(p)) for p in refs), "files", len(refs))
print("key fields present in u32 only (torus-T24):", sorted(set(rows(os.path.join(eq, "torus-T24.u32.jsonl"))[0]) - set(rows(os.path.join(eq, "torus-T24.fp32.jsonl"))[0])))
