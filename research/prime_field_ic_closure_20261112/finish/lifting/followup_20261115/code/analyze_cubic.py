#!/usr/bin/env python3
"""Tables for the cubic-path xedni toy.  [pcc:unverified]"""
import sys, json, glob, math, collections
import numpy as np

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from bounds_hermite import rho_max, log2_N_hermite, margin_from_log2N

d = sys.argv[1]
recs = []
for f in sorted(glob.glob(d + "/*.jsonl")):
    for l in open(f):
        recs.append(json.loads(l))
X = [r for r in recs if r["arm"] == "X"]
PS = [r for r in recs if r["arm"] == "PS"]
cells = collections.defaultdict(list)
for r in X:
    cells[(r["r"], r["bits"])].append(r)
out = {"cells": [], "fits": {}, "ps": {}}
print("r bits n ok rel_nontriv rel_triv H_med lamG_med rhoG_med rhoG_max rhoG_p90 eps_med xcert_min_log2 sec_med")
for (r, b) in sorted(cells):
    rs = cells[(r, b)]
    ok = [x for x in rs if x["status"] == "ok"]
    hs = [x["heights"] for x in ok if x.get("heights") and x["heights"].get("rho_G") and not x.get("rel")]
    rel = [x for x in ok if x.get("rel")]
    nontriv = [x for x in rel if not x.get("rel_trivial")]
    row = dict(r=r, bits=b, n=len(rs), ok=len(ok), status=dict(collections.Counter(x["status"] for x in rs)),
               rel_nontrivial=len(nontriv), rel_trivial=len(rel) - len(nontriv),
               rel_examples=[x["rel"] for x in nontriv][:5],
               H_med=float(np.median([h["H"] for h in hs])) if hs else None,
               lamG_med=float(np.median([h["lambda_G"] for h in hs])) if hs else None,
               rhoG_med=float(np.median([h["rho_G"] for h in hs])) if hs else None,
               rhoG_p90=float(np.percentile([h["rho_G"] for h in hs], 90)) if hs else None,
               rhoG_max=float(max(h["rho_G"] for h in hs)) if hs else None,
               eps_med=float(np.median([h["eps"] for h in hs])) if hs else None,
               eps_over_lam_max=float(max(h["eps"] / h["lambda_G"] for h in hs)) if hs else None,
               log2DW_med=float(np.median([x["log2_absDW"] for x in ok])) if ok else None,
               xcert_min_log2=float(min(x["xcert_log2"] for x in ok if x.get("xcert_log2") is not None)) if ok else None,
               singular_gram=sum(1 for x in ok if x.get("heights") and x["heights"].get("lambda_vec") == "singular"),
               sec_med=float(np.median([x["sec"] for x in rs])))
    out["cells"].append(row)
    print(r, b, row["n"], row["ok"], row["rel_nontrivial"], row["rel_trivial"],
          *(f"{row[k]:.3f}" if row[k] is not None else "-" for k in ("H_med", "lamG_med", "rhoG_med", "rhoG_max", "rhoG_p90", "eps_med", "xcert_min_log2", "sec_med")),
          row["status"])
# H(b) linear fits per r (nats)
for r in sorted(set(k[0] for k in cells)):
    pts = [(c["bits"], c["H_med"]) for c in out["cells"] if c["r"] == r and c["H_med"]]
    bs, Hs = np.array([p[0] for p in pts]), np.array([p[1] for p in pts])
    c1, c0 = np.polyfit(bs, Hs, 1)
    allrho = [x["heights"]["rho_G"] for x in X if x["r"] == r and x["status"] == "ok" and x.get("heights") and x["heights"].get("rho_G") and not x.get("rel")]
    rho12 = [x["heights"]["rho_G"] for x in X if x["r"] == r and x["bits"] >= 12 and x["status"] == "ok" and x.get("heights") and x["heights"].get("rho_G") and not x.get("rel")]
    rm = rho_max(r, 256)
    out["fits"][r] = dict(H_a=c0, H_c=c1, H256=c0 + c1 * 256, rhoG_med_all=float(np.median(allrho)),
                          rhoG_p90_all=float(np.percentile(allrho, 90)), rhoG_max_all=float(max(allrho)),
                          rho_max_256=rm,
                          margin256_at_med=margin_from_log2N(log2_N_hermite(r, float(np.median(allrho))), 256),
                          margin256_at_max=margin_from_log2N(log2_N_hermite(r, float(max(allrho))), 256),
                          lambda_required_256=(c0 + c1 * 256) / rm if rm else None,
                          rhoG_max_bits_ge12=float(max(rho12)),
                          margin256_at_max_ge12=margin_from_log2N(log2_N_hermite(r, float(max(rho12))), 256),
                          margin256_at_4xmax=margin_from_log2N(log2_N_hermite(r, 4 * float(max(allrho))), 256),
                          n=len(allrho))
print("\nfits / margins at 256 bits (Hermite cutoff), rho from this toy")
for r, f in out["fits"].items():
    print(r, {k: (round(v, 3) if isinstance(v, float) else v) for k, v in f.items()})
for x in PS:
    key = (x["r"], x["bits"])
    s = out["ps"].setdefault(f"{key[0]}_{key[1]}", collections.Counter())
    s[x["status"]] += 1
    if x["status"] == "ok":
        s["planted_realized"] += int(bool(x.get("planted_realized")))
        s["rel_found"] += int(bool(x.get("rel")))
print("\nPS arm (P3 = P1 + P2 in E(F_p)):", {k: dict(v) for k, v in out["ps"].items()})
json.dump(out, open(sys.argv[2], "w"), indent=1, default=str)
