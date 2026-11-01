#!/usr/bin/env python3
"""Build the L09 table. Usage: run_table.py OUT_PREFIX   (writes OUT_PREFIX.json / .md)"""
import sys, os, json, math, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from btn_census import census, d1
HERE = os.path.dirname(os.path.abspath(__file__))
TRI = "/Volumes/SSD990-2/crypto/.scratch/icperf-checker-v2-20261019/inputs/TRIMOSKA-ECICB-2024/upstream/benchmarks"
LAB = "/Volumes/SSD990-2/crypto-worktrees/xenodochial-jang-dc5daf/docs/ic/corpus"
rows = []
def add(group, label, path, b, m, **kw):
    for mode in ("split", "bb"):
        R = census(path, mode=mode, b=b, m=m, label=label, **kw)
        R["group"] = group; R["path"] = path; rows.append(R)
# 1. descended ANF (Trimoska corpus, WDSat ANF), m=3, b=n
for n, l in ((15, 5), (17, 6), (19, 6)):
    add("descended ANF (WDSat .anf)", f"Xn{n}l{l}-1-S.anf", f"{TRI}/Xn{n}l{l}-1-S.anf", n, 3)
# 1b. cross-check: lab-format ANF and CNF-form of the same problems
for n, l, d in ((15, 5, "n15l5"), (19, 6, "n19l6")):
    add("descended ANF (lab p anf)", f"{d}-1-S.anf (lab)", f"{LAB}/{d}/{d}-1-S.anf", n, 3, fmt="labanf")
add("descended CNF (Trimoska)", "Xn15l5-1-S.dimacs", f"{TRI}/Xn15l5-1-S.dimacs", 15, 3, iface_first=15)
# 2. prime PDP CNFs
for b in (16, 24, 32):
    for enc, name in (("bits-s3", "chained-S3"), ("bits-grouplaw", "quotient/group-law")):
        m = 3 if name == "chained-S3" else 2
        for x in (0, 1):
            p = f"{HERE}/inst/pdp_b{b}_m{m}_{enc}_xor{x}.cnf.gz"
            add(f"prime PDP {name} m={m}" + (" +native XOR" if x else " plain CNF"), os.path.basename(p), p, b, m)
    # m=2 S3 (single Semaev) and m=3 grouplaw as extra rows
    for enc, m in (("bits-s3", 2), ("bits-grouplaw", 3)):
        p = f"{HERE}/inst/pdp_b{b}_m{m}_{enc}_xor0.cnf.gz"
        add(f"prime PDP {enc} m={m} plain CNF", os.path.basename(p), p, b, m)
# 3. Bochum hashed network m=2 (crate), measured here
for f in sorted(glob.glob(f"{HERE}/graphs/bochum_m2_*.json")):
    if f.endswith(".params.json"): continue
    pr = json.load(open(f.replace(".json", ".params.json")))
    add("hashed Bochum network m=2 (crate)", os.path.basename(f), f, pr["bits"], 2)
# 4. published 131-bit rows (not rebuilt here)
def published(label, s, V, delta, b, src):
    R = dict(label=label, group="published 131-bit (not rebuilt)", mode="both", format="published", path=src,
             s=s, V=V, delta=delta, nl_ports=None, nl_verts=None, k=None, k_src="n/a", b=b, m=2)
    R["s_over_sqrtlog"] = s / math.sqrt(math.log2(s)); R["d1"] = d1(s)
    R["enum_exp"] = b / 2; R["ratio_vs_enum"] = R["s_over_sqrtlog"] / R["enum_exp"]
    rows.append(R)
published("ECCp-131 target graph (2 slots, seed 1)", 1564506, 71723, 1032, 131,
          "/Volumes/SSD990-2/prime-sat-tensor-review-20261031/hashed_tensor_compile_stats.json (s,V=constraints,Delta); cryptanalysis/docs/hashed-ecdl.md")
published("ECC2K-130 recorded graph", 2694680, None, 1032, 131, "/Volumes/SSD990-2/cryptanalysis/docs/hashed-ecdl.md line 26")
json.dump(rows, open(sys.argv[1] + ".json", "w"), indent=1, default=str)
def f(x, n=1): return "-" if x is None else (f"{x:,}" if isinstance(x, int) else f"{x:,.{n}f}")
out = ["| group | instance | mode | b | m | s (edges) | V | Delta | nonlin ports | s/sqrt(log2 s) | log2 s | u0=lam/2^17 vs lam^0.8 | D.1 admitted | enum (m-1)b/m | ratio | k | k vs b |", "|" + "---|" * 17]
for R in rows:
    k = R.get("k"); b = R.get("b")
    kv = "-" if k is None else (f"{k} >= {b}" if b and k >= b else f"{k} < {b}")
    out.append("| " + " | ".join([R["group"], R["label"], R["mode"], str(b), str(R.get("m")), f(R["s"]), f(R["V"]), f(R["delta"]),
        f(R["nl_ports"]), f(R["s_over_sqrtlog"]), f(R["d1"]["lambda_"], 2),
        f"{R['d1']['u0']:.2e} vs {R['d1']['thr']:.2f}", "yes" if R["d1"]["admitted"] else "no",
        f(R.get("enum_exp"), 2), f(R.get("ratio_vs_enum"), 0), f(k), kv]) + " |")
open(sys.argv[1] + ".md", "w").write("\n".join(out) + "\n")
print("\n".join(out))
