#!/usr/bin/env python3
"""Lay out the recorded judgments (judgments/j_part*.py), the Haiku recompute records
(recompute/*.jsonl) and the tracer output into verdicts.jsonl, null_rows.json and summary.json.
No measurement is computed here."""
import json, glob, sys, collections, re
from pathlib import Path
B = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(B / "judgments"))
from j_part6 import J
from null_rows import N

recs = {}
for f in sorted(glob.glob(str(B / "recompute/SWB-RECOMPUTE-*.jsonl")), key=lambda x: (x.endswith("-20.jsonl"), x)):
    for l in open(f):
        if l.strip():
            r = json.loads(l); recs[r["row_no"]] = r
readme = [l for l in open(B.parents[2] / "benchmarks/README.md") if l.startswith("| 20")]
orows = json.load(open(B / "scripts/orphan_rows.json"))
DUP = {22: 8, 47: 38}
# rows whose single claim has a recorded recompute mismatch at the printed precision (see the row notes)
OK_PARTIAL = {45, 50, 57, 60, 80, 81, 115}

def row_meta(n):
    if n < 1000:
        c = [x.strip() for x in readme[n - 1].strip().strip("|").split(" | ")]
        return c[0], re.search(r"`(\d{8}-[^`]+)`", c[1]).group(1), True
    o = orows[n - 1001]
    return o["date"], re.search(r"`(\d{8}-[^`]+)`", o["entry"]).group(1), False

def raw_status(n):
    r = recs.get(n)
    if not r: return "unknown"
    st = collections.Counter(c["status"] for c in r["claims"])
    got = st["match"] + st["mismatch"] + st["partial"]
    if n in (6,): return "aggregates only (per-run records not committed)"
    if got and not st["no_raw_in_repo"]: return "yes"
    if got and st["no_raw_in_repo"]: return "partial"
    if st["no_raw_in_repo"] and not got: return "no"
    return "no/undetermined" if not got else "yes"

def protocol_status(n):
    r = recs.get(n)
    if not r: return "unknown"
    p, f = str(r["protocol_git_date"]).strip(), str(r["first_raw_git_date"]).strip()
    if not p or p.lower().startswith(("no ", "none", "empty", "no_protocol")): return "no PROTOCOL file in the study tree"
    if p == f or p[:12] == f[:12]: return "protocol and first raw record added in the same commit (order unprovable from git)"
    return f"protocol first added {p[:25]}; first raw {f[:25]}"

out = []; nulls = []
for n in sorted(set(J) | set(N), key=int):
    date, slug, in_readme = row_meta(n)
    if n in N and n not in J:
        nulls.append(dict(row_no=n, row_date=date, slug=slug, in_readme_table=in_readme, reason=N[n], raw_data_in_repo=raw_status(n)))
        continue
    for i, c in enumerate(J[n], 1):
        out.append(dict(row_no=n, claim_id=f"{n}.{i}", row_date=date, slug=slug, in_readme_table=in_readme,
            duplicate_of=DUP.get(n), claim=c["c"], recomputed_ok=("partial" if n in OK_PARTIAL else c["ok"]), raw_data_in_repo=raw_status(n),
            aa_control=c["aa"], isolation_tier=c["iso"], protocol_frozen_before_data=protocol_status(n),
            class_claimed=c["cc"], class_actual=c["ca"], verdict=c["v"], notes=c["n"]))
out.sort(key=lambda r: (r["row_date"], r["row_no"]), reverse=True)
with open(B / "verdicts.jsonl", "w") as f:
    for r in out: f.write(json.dumps(r, ensure_ascii=False) + "\n")
json.dump(sorted(nulls, key=lambda r: (r["row_date"], r["row_no"]), reverse=True), open(B / "null_rows.json", "w"), indent=1, ensure_ascii=False)
v = collections.Counter(r["verdict"] for r in out)
summ = dict(claims=len(out), rows_with_verdicts=len({r["row_no"] for r in out}), null_rows=len(nulls),
            readme_rows_total=len(readme), verdicts=dict(v),
            by_readme=dict(collections.Counter((r["in_readme_table"], r["verdict"]) .__str__() for r in out)))
json.dump(summ, open(B / "summary.json", "w"), indent=1)
print(json.dumps(summ, indent=1))
