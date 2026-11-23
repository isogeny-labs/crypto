#!/usr/bin/env python3
"""Step 1: break down la_add / la_mul of the ladder's 2lp-sfb rows at p = 131101
by source, from the existing raw rows only (no new run).

The rows record la_add, la_mul, core_rows (M), core_cols (n), attempts and
relations, not the per-source split.  The split below is a structural model
of ic_control's counted code (build_rows, peel, Dsu, wiedemann on A^T D A,
berlekamp_massey), with every term except the matrix non-zeros fixed by n, M,
attempts and relations; nnz is then solved from la_add, and the number of
non-unit entries from la_mul.  Terms whose size the rows do not record
(graph/DSU, cycle-row building) are given generous upper bounds, so the
Wiedemann SpMV share is a lower bound.  Every number is [pcc:unverified].
"""
import json
import sys

R_BITS = 51  # r ~ 2.25e15 at p = 131101
INV = 2 * R_BITS

paths = sys.argv[1:] or ["../quartic_diem_ladder_20261121/raw/run131101_odd.jsonl",
                          "../quartic_diem_ladder_20261121/raw/run131101_even.jsonl"]
rows = []
for path in paths:
    for line in open(path):
        r = json.loads(line)
        if r.get("kind") == "run" and r.get("arm") == "Q_diem_2lp_sfb":
            rows.append(r)

tot = {}
out = []
for r in rows:
    n, M, a, rel = r["core_cols"], r["core_rows"], r["attempts"], r["relations"]
    length = 2 * n + 2
    L = n                         # BM degree (full-rank core)
    napply = length + L - 1       # m_apply calls per attempt (Krylov + Horner)
    # adds per attempt that do not scale with nnz
    bm_add = 2 * n * n            # discrepancy + update terms, ~n^2 each
    vec_add = L * n               # Horner accumulation
    proj_add = [16 * length] + [n * length] * (a - 1)   # attempt 1 sparse, later dense
    graph_ub = 2 * rel + 10 * rel  # adjacency (2/LP edge) + DSU finds/unions (<= 10/rel, generous)
    rowsbuild_ub = 20 * rel       # combine() along BFS-tree paths (generous)
    peel_tscan_ub = 0             # folded into the nnz terms below (2 passes ~ 2 nnz)
    fixed = a * (bm_add + vec_add) + sum(proj_add) + graph_ub + rowsbuild_ub
    # spmv adds per attempt = nnz * (2*napply + 2)  [mat_vec + mat_t_vec per apply,
    # A^T D c, residual] + peel/tscan ~ 2 nnz
    per_nnz = a * (2 * napply + 2) + 2
    nnz = (r["la_add"] - fixed) / per_nnz
    spmv_add = a * nnz * (2 * napply + 2)
    # muls: D scaling M per apply + A^T D c (M) + 2*generic per apply + BM + Horner + proj
    bm_mul = 2 * n * n + n * INV  # + pivot inversions (<= n of them)
    fixed_mul = a * (M * (napply + 1) + bm_mul + L * n + n + INV) + 16 * length + n * length * (a - 1)
    generic = (r["la_mul"] - fixed_mul) / (a * (2 * napply + 1))
    e = {"curve_seed": r["curve_seed"], "seed": r["seed"], "attempts": a, "M": M, "n": n,
         "la_add": r["la_add"], "la_mul": r["la_mul"],
         "nnz_est": round(nnz), "nnz_per_row_est": round(nnz / M, 1),
         "generic_est": round(generic),
         "share_add": {
             "wiedemann_spmv_AtDA": spmv_add / r["la_add"],
             "berlekamp_massey": a * bm_add / r["la_add"],
             "horner_projection": (a * vec_add + sum(proj_add)) / r["la_add"],
             "graph_dsu_ub": graph_ub / r["la_add"],
             "cycle_rows_build_ub": rowsbuild_ub / r["la_add"],
             "peel_tscan": 2 * nnz / r["la_add"]}}
    out.append(e)
    for k, v in e["share_add"].items():
        tot[k] = tot.get(k, 0) + v * r["la_add"]
S = sum(r["la_add"] for r in rows)
summary = {"n_runs": len(rows), "la_add_total": S,
           "share_of_la_add_pooled": {k: v / S for k, v in tot.items()},
           "native_all_share_of_la_add": sum(r["la_add"] for r in rows) / sum(
               r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"] + r.get("idx_lookups", 0)
               + r.get("idx_inserts", 0) for r in rows),
           "label": "[pcc:unverified] structural model; exact split needs the instrumented replay"}
json.dump({"summary": summary, "runs": out}, sys.stdout, indent=1)
print()
