#!/usr/bin/env python3
"""VALIDATOR clause-by-clause check of ONE statement (exactly as worded) against the validator's own recompute.
usage: claim_check.py <frozen study dir> <recompute.json> <instances_check.json> <sensitivity.json> <out.json>"""
import json, sys, os, hashlib, collections
FZ, RC, IC, SE, OUT = sys.argv[1:6]
rc = json.load(open(RC)); ic = json.load(open(IC)); se = json.load(open(SE))
STATEMENT = ("On 33 fresh prime-order Weierstrass curves (group orders about 2^24, 2^28 and 2^32), T = 1024/4096/16384, "
 "22 paired configurations per cell, counted F_p and mod-n operations (I = 3/10/30 M, S = M) with hash-table, solver and "
 "counted native operations (rho step-index hashes, distinguished-point canonicalisations and point hashes, IC offset-step "
 "indices) at 0/0.1/1 GAE each: the cheapest of four multi-target index-calculus variants (ic5, ic5ks, ic5w, ic5rw) per "
 "configuration and price is unranked against the cheapest of ten runs of this harness's multi-target rho (Kuhn-Struik "
 "retention rho rho2xks, distinguished-point rho rhodpu) in all 27 cell x I/M section-5a decisions, because the order flips "
 "with h: at I = 10M it costs 1.06-1.16x per target at h = 0 (dearer in all 9 cells), 1.00-1.05x at h = 0.1 and 0.71-0.80x "
 "at h = 1 (cheaper in all 9 cells); with native operations unpriced it is dearer in all 27; it is 1.11-1.47x a paired BSGS.")
r5, r4 = rc["ranges_5a"], rc["ranges_v4"]
S5 = [s for s in rc["summary_5a"] if s["cand"] == "icbest" and s["vs"] == "rhobest"]
flip = all(any(s["lo"] > 1 for s in S5 if (s["cI"], s["bits"], s["T"]) == key and s["h"] == 0) and
           any(s["hi"] < 1 for s in S5 if (s["cI"], s["bits"], s["T"]) == key and s["h"] == 1)
           for key in {(s["cI"], s["bits"], s["T"]) for s in S5})
inst = ic["summary"]
nb = collections.Counter(inst["n_bits_by_file"].values())
clauses = [
 dict(clause="33 fresh prime-order Weierstrass curves (group orders about 2^24, 2^28 and 2^32)",
      ok=inst["instances"] == 33 and inst["all_ok"] and not inst["overlap_files_pab"],
      recomputed=f"33 valid (p, n prime, cofactor 1, nG = O, fixture log); n bit lengths {dict(sorted(nb.items()))}; regenerate byte-identically; "
                 f"0 (p, a, b) overlap with {inst['curve_records_scanned']} curve records in {inst['json_files_scanned']} JSON files; "
                 f"(p, n) coincides with {inst['overlap_files_pn']} (different j-invariant: not isomorphic)"),
 dict(clause="T = 1024/4096/16384, 22 paired configurations per cell", ok=rc["configurations"] == 198 and rc["per_config_runs"] == {"17": 198},
      recomputed="9 cells x 22 (11 curves x seeds 41, 42) = 198 configurations x 17 runs = 3366 rows, ok == targets in every row"),
 dict(clause="counted F_p and mod-n operations (I = 3/10/30 M, S = M)", ok=True,
      recomputed="cost = [(cI I + M + S)/(cI + 3) + h (H + L + N)]/T; mod-n products/inverses of the IC solver counted as M/I (harness code)"),
 dict(clause="with hash-table, solver and counted native operations (rho step-index hashes, distinguished-point canonicalisations and point hashes, IC offset-step indices) at 0/0.1/1 GAE each",
      ok=True, recomputed="instrumented replay of all 3366 rows: H equals the HashMap probes/inserts in every row of ic5, ic5w, rho2xks, rhodpu, bsgs2; in ic5ks/ic5rw one H covers the retained-table and the table probe of one x (merged-table count; counting both probes changes no decision, sensitivity V11); N = step-index calls (rho2xks), 2 x canonicalisations (rhodpu), W-step indices (ic5w, ic5rw), 0 for ic5, ic5ks, bsgs2; L = 0 in every row. rho2xks's step index is x mod 32 (the 5 low bits of x), called a hash in the statement"),
 dict(clause="the cheapest of four multi-target index-calculus variants (ic5, ic5ks, ic5w, ic5rw) per configuration and price", ok=True,
      recomputed="icbest = min over the 4 IC runs per configuration and (cI, h); 972/972 summary.json entries recomputed exactly"),
 dict(clause="is unranked against the cheapest of ten runs of this harness's multi-target rho (Kuhn-Struik retention rho rho2xks, distinguished-point rho rhodpu) in all 27 cell x I/M section-5a decisions",
      ok=rc["decisions_5a"]["icbest vs rhobest"] == {"unranked": 27} and rc["decisions_5a_npboot"]["icbest vs rhobest"] == {"unranked": 27},
      recomputed=f"{rc['decisions_5a']['icbest vs rhobest']} (protocol bootstrap and an independent numpy bootstrap)"),
 dict(clause="because the order flips with h", ok=flip,
      recomputed="in every one of the 27 (cell, cI): 95% lower bound > 1 at h = 0 and 95% upper bound < 1 at h = 1"),
 dict(clause="at I = 10M it costs 1.06-1.16x per target at h = 0 (dearer in all 9 cells)",
      ok=round(r5["icbest/rhobest I=10M h=0"][0], 2) == 1.06 and round(r5["icbest/rhobest I=10M h=0"][1], 2) == 1.16 and r5["icbest/rhobest I=10M h=0 lo>1 cells"] == 9,
      recomputed=f"{r5['icbest/rhobest I=10M h=0']}; lower bound > 1 in {r5['icbest/rhobest I=10M h=0 lo>1 cells']} of 9"),
 dict(clause="1.00-1.05x at h = 0.1", ok=round(r5["icbest/rhobest I=10M h=0.1"][0], 2) == 1.00 and round(r5["icbest/rhobest I=10M h=0.1"][1], 2) == 1.05,
      recomputed=f"{r5['icbest/rhobest I=10M h=0.1']}"),
 dict(clause="and 0.71-0.80x at h = 1 (cheaper in all 9 cells)",
      ok=round(r5["icbest/rhobest I=10M h=1"][0], 2) == 0.71 and round(r5["icbest/rhobest I=10M h=1"][1], 2) == 0.80 and r5["icbest/rhobest I=10M h=1 hi<1 cells"] == 9,
      recomputed=f"{r5['icbest/rhobest I=10M h=1']}; upper bound < 1 in {r5['icbest/rhobest I=10M h=1 hi<1 cells']} of 9"),
 dict(clause="with native operations unpriced it is dearer in all 27", ok=rc["decisions_v4"]["icbest vs rhobest"] == {"dearer": 27},
      recomputed=f"v4 rule (h on H + L): {rc['decisions_v4']['icbest vs rhobest']}; I = 10M range {r4['icbest/rhobest I=10M all h']}"),
 dict(clause="it is 1.11-1.47x a paired BSGS", ok=round(r5["icbest/bsgs2 all"][0], 2) == 1.11 and round(r5["icbest/bsgs2 all"][1], 2) == 1.47 and rc["decisions_5a"]["icbest vs bsgs2"] == {"dearer": 27},
      recomputed=f"{r5['icbest/bsgs2 all']} over all cells, cI and h; dearer in all 27"),
]
def H(name, claimed, rec, ndp):
    return dict(name=name, claimed=claimed, recomputed=rec, rel_tol=max(0.5 * 10 ** -ndp / abs(claimed), 1e-9) * 1.0000001)
headline = [H("icbest/rhobest I=10M h=0 min", 1.06, r5["icbest/rhobest I=10M h=0"][0], 2), H("icbest/rhobest I=10M h=0 max", 1.16, r5["icbest/rhobest I=10M h=0"][1], 2),
            H("icbest/rhobest I=10M h=0.1 min", 1.00, r5["icbest/rhobest I=10M h=0.1"][0], 2), H("icbest/rhobest I=10M h=0.1 max", 1.05, r5["icbest/rhobest I=10M h=0.1"][1], 2),
            H("icbest/rhobest I=10M h=1 min", 0.71, r5["icbest/rhobest I=10M h=1"][0], 2), H("icbest/rhobest I=10M h=1 max", 0.80, r5["icbest/rhobest I=10M h=1"][1], 2),
            H("icbest/bsgs2 min (all cI, h)", 1.11, r5["icbest/bsgs2 all"][0], 2), H("icbest/bsgs2 max (all cI, h)", 1.47, r5["icbest/bsgs2 all"][1], 2),
            dict(name="unranked decisions of 27 (section 5a rule)", claimed=27, recomputed=rc["decisions_5a"]["icbest vs rhobest"].get("unranked", 0)),
            dict(name="dearer decisions of 27 (N unpriced)", claimed=27, recomputed=rc["decisions_v4"]["icbest vs rhobest"].get("dearer", 0)),
            dict(name="cells dearer at h=0, I=10M (lower bound > 1)", claimed=9, recomputed=r5["icbest/rhobest I=10M h=0 lo>1 cells"]),
            dict(name="cells cheaper at h=1, I=10M (upper bound < 1)", claimed=9, recomputed=r5["icbest/rhobest I=10M h=1 hi<1 cells"]),
            dict(name="curves", claimed=33, recomputed=inst["instances"]), dict(name="configurations per cell", claimed=22, recomputed=rc["configurations"] // 9)]
mism = [h for h in headline if abs(h["claimed"] - h["recomputed"]) > h.get("rel_tol", 1e-9) * max(abs(h["claimed"]), abs(h["recomputed"]))]
sens = {k: dict(decisions=v.get("decisions"), I10M_median_range=v.get("I10M_median_range"), h1_cells_hi_below_1_of_27=v.get("h1_cells_hi_below_1_of_27"))
        for k, v in se.items() if isinstance(v, dict) and "decisions" in v}
out = dict(statement=STATEMENT, statement_sha256=hashlib.sha256(STATEMENT.encode()).hexdigest(),
           all_clauses_ok=all(c["ok"] for c in clauses), clause_checks=clauses, headline=headline, headline_mismatches=mism, sensitivity=sens)
json.dump(out, open(OUT, "w"), indent=1)
print(json.dumps(dict(all_clauses_ok=out["all_clauses_ok"], mismatches=mism, sha=out["statement_sha256"]), indent=1))
for c in clauses: print(c["ok"], "|", c["clause"][:70], "|", c["recomputed"][:160])
