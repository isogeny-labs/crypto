#!/usr/bin/env python3
"""Independent scan of the field-arithmetic counter class (target B, Validator V).

Re-does, in Python, what `field_multiplication_lookalikes_are_classed_or_excluded`
claims: every unpriced counter name in committed records and every "..._uncharged"
literal in src/ that looks like a base-field multiplication or squaring is in
FIELD_ARITHMETIC_COUNTERS or FIELD_ARITHMETIC_EXCLUSIONS. Also counts the
records.jsonl files the test cannot see because they live only in the artifact
store (.artifacts.json manifests), which is the test's blind spot.
Usage: b_class_scan.py CHECKOUT BAND_RS SRC_DIR OUT
"""
import json, os, re, sys

root, band_rs, src_dir, out = sys.argv[1:5]
s = open(band_rs).read()
def block(name):
    b = s[s.index(name):]
    return b[:b.index("];")]
FIELD = set(re.findall(r'"([^"]+)"', block("pub const FIELD_ARITHMETIC_COUNTERS")))
EXCL = set(re.findall(r'\(\s*"([^"]+)"', block("pub const FIELD_ARITHMETIC_EXCLUSIONS")))
NONOP = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged", "native_table_bytes_uncharged"}

def looks(n):
    return any(w.startswith(("mul", "sqr", "squar")) for w in re.split(r"[^A-Za-z0-9]", n))

def complete_route(n):
    m = re.fullmatch(r"complete_route_f2m(\d+)_(mul|square)_uncharged", n)
    return bool(m)

def settled(n):
    b = n.rsplit(".", 1)[-1]
    return ((b in FIELD or complete_route(b)) and b not in NONOP) or n in EXCL

names, store_only, scanned = {}, [], 0
for d, _, files in os.walk(os.path.join(root, "research")):
    if "records.jsonl" in files:
        scanned += 1
        with open(os.path.join(d, "records.jsonl"), errors="replace") as f:
            for line in f:
                i = line.find('"unpriced":[')
                if i < 0: continue
                rest = line[i + 12:]
                for n in rest[:rest.find("]")].split('"')[1::2]:
                    names.setdefault(n, os.path.join(d, "records.jsonl"))
    if ".artifacts.json" in files:
        try:
            m = json.load(open(os.path.join(d, ".artifacts.json")))
            for fn in m.get("files", {}):
                if fn.endswith("records.jsonl") and not os.path.exists(os.path.join(d, fn)):
                    store_only.append(os.path.relpath(os.path.join(d, fn), root))
        except Exception:
            pass
lits = 0
for d, _, files in os.walk(src_dir):
    for fn in files:
        if fn.endswith(".rs"):
            text = open(os.path.join(d, fn), errors="replace").read()
            for lit in text.split('"')[1::2]:
                if lit.endswith("_uncharged") and not lit.startswith("_") and not re.search(r"\s", lit) and "{}" not in lit:
                    lits += 1
                    names.setdefault(lit, os.path.join(d, fn))
unsettled = sorted(n for n in names if looks(n.rsplit(".", 1)[-1]) and not settled(n))
classed = sorted(n for n in names if (n.rsplit(".", 1)[-1] in FIELD))
res = {"records_scanned": scanned, "names": len(names), "src_literals": lits, "field_class_size": len(FIELD),
       "exclusions": sorted(EXCL), "unsettled_lookalikes": unsettled, "class_names_seen": classed,
       "class_names_never_seen": sorted(FIELD - {n.rsplit('.', 1)[-1] for n in names}),
       "store_only_records_not_scanned": len(store_only), "store_only_examples": store_only[:20]}
json.dump(res, open(out, "w"), indent=1)
print(json.dumps({k: (v if not isinstance(v, list) or len(v) < 12 else f"{len(v)} items") for k, v in res.items()}, indent=1))
