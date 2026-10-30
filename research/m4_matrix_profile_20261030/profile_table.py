"""Per-degree-block profile table from data/profile_*.jsonl (m4_matrix_profile output).

Per run (cell, draw, degree) and per degree block d of the columns:
  cols        columns of monomial degree d
  sub%        share of those columns supported only on the m*l subspace coordinates
  rank        pivots leading in block d of the saturated closure basis (rank profile)
  dens        nnz / (pivot rows * columns at or after the block), closure basis, per block d
  in_rho      mean row weight of the unreduced Macaulay input rows, by leading degree
"""
import glob, json, sys

rows = []
for f in sorted(glob.glob("data/profile_*.jsonl")):
    for line in open(f):
        if line.strip():
            r = json.loads(line)
            r["file"] = f
            rows.append(r)

out = []
for r in rows:
    inp, pre, post = r.get("input"), r.get("pre"), r["post"]
    D = r["degree"]
    rec = dict(cell=r["cell"], m=r["m"], draw=r["draw"], sat=r["sat"], degree=D,
               columns=r["columns"], rank=r["rank"], refuted=r["closure_refuted"],
               base_ops=r["base_word_ops"], total_ops=r["total_word_ops"],
               rounds=len(r["rounds"]), products=r["closure_products"], blocks={})
    for d in range(D + 1):
        c = post["cols"][d]
        b = dict(cols=c, sub_cols=post["sub_cols"][d],
                 closure_pivots=post["lead_rows"][d], closure_sub_pivots=post["sub_lead_rows"][d],
                 closure_nnz=post["nnz"][d], closure_sub_nnz=post["sub_nnz"][d])
        if pre:
            b.update(xl_pivots=pre["lead_rows"][d], xl_nnz=pre["nnz"][d])
        if inp:
            b.update(in_rows=inp["lead_rows"][d], in_nnz=inp["nnz"][d],
                     in_weight_sum=inp["lead_weight_sum"][d], in_weight_max=inp["lead_weight_max"][d])
        rec["blocks"][d] = b
    if inp:
        rec["input_rows"] = inp["rows"]
        rec["input_nnz"] = sum(inp["nnz"])
        rec["input_mean_weight"] = rec["input_nnz"] / max(inp["rows"], 1)
    if pre:
        rec["xl_rank"] = pre["rows"]
    out.append(rec)

json.dump(out, open("data/profile_table.json", "w"), indent=1)

def pct(a, b):
    return f"{100 * a / b:5.1f}" if b else "  n/a"

print("| cell | draw | sat | D | cols | rank/cols | XL rank/cols | rho_in | block d: cols (sub%) / closure pivots / closure dens% |")
print("|---|--:|:-:|--:|--:|--:|--:|--:|---|")
for r in out:
    segs = []
    for d, b in r["blocks"].items():
        if b["cols"] == 0:
            continue
        below = sum(x["cols"] for dd, x in r["blocks"].items() if dd <= d)  # columns at or after block (smaller monomials)
        dens = b["closure_nnz"] / (max(r["rank"], 1) * r["columns"])
        segs.append(f"d{d}: {b['cols']} ({pct(b['sub_cols'], b['cols'])}%) / {b['closure_pivots']} / {100 * dens:.2f}")
    print(f"| {r['cell']} | {r['draw']} | {'S' if r['sat'] else 'U'} | {r['degree']} | {r['columns']} | "
          f"{r['rank'] / r['columns']:.3f} | {r.get('xl_rank', 0) / r['columns']:.3f} | "
          f"{r.get('input_mean_weight', float('nan')):.1f} | " + "; ".join(segs) + " |")
