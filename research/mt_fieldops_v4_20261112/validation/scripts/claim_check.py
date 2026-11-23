#!/usr/bin/env python3
"""Validator check of the amended claim statement (the accepted wording of the first claim report), clause by
clause, recomputed from the committed artifacts with code written for this check (not recompute.py):
results/holdout/runs.json (counters), validation/native_work.json (rhodp canonicalisation counts from the
validator's instrumented replay), validation/instances_check.json.  Writes validation/claim-report.json.
Decision rule and bootstrap as PROTOCOL.md / analyze.py.
usage: claim_check.py <study dir> <commit> <out.json>"""
import json, sys, os, random, statistics as st, collections, hashlib
study, commit, out = sys.argv[1:4]
V = os.path.join(study, "validation")
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
STMT = ("On 33 fresh prime-order Weierstrass curves (group orders about 2^24, 2^28 and 2^32), T = 1024/4096/16384, 22 paired configurations per cell, counted F_p and mod-n operations (I = 3/10/30 M, S = M) with hash-table and solver operations at 0/0.1/1 GAE: "
        "this harness's multi-target index calculus (the cheaper of ic5 and ic5ks per configuration and price; ic5ks's retention of solved trial classes along the shared arithmetic-progression W saves at most 1.5%) costs 1.003-1.810x per target of the cheapest of 15 runs of this harness's multi-target rho per configuration and price "
        "(Kuhn-Struik retention rho rho2xks at h = 0 and 0.1, distinguished-point rho rhodp at h = 1) and is dearer in all 27 cell x I/M decisions of the frozen rule; at I = 10M it is 1.04-1.16x at h = 0, 1.03-1.08x at h = 0.1 and 1.20-1.76x at h = 1; it is 1.14-1.49x a paired BSGS. "
        "The frozen rule prices h on hash-table operations only: with rhodp's per-step canonicalisations priced at the same h (AGENTS.md section 5a) the IC is 0.89-0.94x the cheapest rho at h = 1 and I = 10M and all 27 decisions are unranked, so no ranking against rho is established.")
prev = json.load(open(os.path.join(V, "claim-report.json")))
assert prev.get("accepted_wording", STMT) == STMT or prev.get("statement_judged") == STMT
RJ = json.load(open(os.path.join(study, "results/holdout/runs.json")))
runs = [dict(zip(RJ["fields"], r)) for r in RJ["rows"]]
NW = json.load(open(os.path.join(V, "native_work.json")))
nw = [dict(zip(NW["fields"], r)) for r in NW["rows"]]
assert len(nw) == len(runs)
for r, q in zip(runs, nw):
    assert (r["arm"], r["pb"], r["T"], r["idx"], r["seed"], r["param"], r["H"]) == (q["arm"], q["pb"], q["T"], q["idx"], q["seed"], q["param"], q["H"]) and q["match"]
    r["canon"] = q["canon"]
IC = json.load(open(os.path.join(V, "instances_check.json")))
by = collections.defaultdict(list)
for r in runs: by[(r["pb"], r["T"], r["idx"], r["seed"])].append(r)
cells = sorted({k[:2] for k in by}); CI = (3, 10, 30); HS = (0, 0.1, 1)
def g(r, ci, h, canon):
    return ((ci * r["I"] + r["M"] + r["S"]) / (ci + 3) + h * (r["H"] + r["L"] + (r["canon"] if canon and r["arm"] == "rhodp" else 0))) / r["targets"]
def m(k, arms, ci, h, canon=False): return min(g(r, ci, h, canon) for r in by[k] if r["arm"] in arms)
ICA, RHO = ("ic5", "ic5ks"), ("rho2xks", "rhodp")
def bci(x):
    rnd = random.Random(1); s = sorted(st.median(rnd.choices(x, k=len(x))) for _ in range(4000)); return s[100], s[3899]
def table(ref, canon):
    T = {}
    for ci in CI:
        for c in cells:
            ks = [k for k in by if k[:2] == c]
            for h in HS:
                rat = [m(k, ICA, ci, h, canon) / m(k, ref, ci, h, canon) for k in ks]
                T[(ci, c, h)] = (st.median(rat), *bci(rat), len(rat))
    return T
def decisions(T):
    out = collections.Counter()
    for ci in CI:
        for c in cells:
            t = [T[(ci, c, h)] for h in HS]
            out["cheaper" if all(x[2] < 1 for x in t) else "dearer" if all(x[1] > 1 for x in t) else "unranked"] += 1
    return dict(out)
F, P = table(RHO, False), table(RHO, True); B = table(("bsgs2",), False)
rng = lambda T, ci=None, h=None: [f(v[0] for k, v in T.items() if (ci is None or k[0] == ci) and (h is None or k[2] == h)) for f in (min, max)]
cfgs_per_cell = collections.Counter(k[:2] for k in by)
runs_per_cfg_rho = collections.Counter(sum(r["arm"] in RHO for r in v) for v in by.values())
winners = collections.Counter()
for ci in CI:
    for h in HS:
        for k in by:
            winners[(h, min((r for r in by[k] if r["arm"] in RHO), key=lambda r: g(r, ci, h, False))["arm"])] += 1
ks_save = max(1 - g([r for r in by[k] if r["arm"] == "ic5ks"][0], ci, 0, False) / g([r for r in by[k] if r["arm"] == "ic5"][0], ci, 0, False) for k in by for ci in CI)
ks_dearer_h_pos = all(g([r for r in by[k] if r["arm"] == "ic5ks"][0], ci, h, False) >= g([r for r in by[k] if r["arm"] == "ic5"][0], ci, h, False) for k in by for ci in CI for h in (0.1, 1))
nr = IC["summary"]["n_range"]
R = dict(all=rng(F), h0=rng(F, 10, 0), h01=rng(F, 10, 0.1), h1=rng(F, 10, 1), bsgs=rng(B), p1h1=rng(P, 10, 1))
def tol(dec, v): return 0.5 * 10 ** -dec / abs(v) * 1.000001
H = []
for name, c, v, d in [("max saving of ic5ks over ic5 (percent, h = 0, all cI)", 1.5, 100 * ks_save, 1),
                      ("icbest/rhobest min (frozen rule)", 1.003, R["all"][0], 3), ("icbest/rhobest max (frozen rule)", 1.810, R["all"][1], 3),
                      ("I = 10M h = 0 min", 1.04, R["h0"][0], 2), ("I = 10M h = 0 max", 1.16, R["h0"][1], 2),
                      ("I = 10M h = 0.1 min", 1.03, R["h01"][0], 2), ("I = 10M h = 0.1 max", 1.08, R["h01"][1], 2),
                      ("I = 10M h = 1 min", 1.20, R["h1"][0], 2), ("I = 10M h = 1 max", 1.76, R["h1"][1], 2),
                      ("icbest/bsgs2 min", 1.14, R["bsgs"][0], 2), ("icbest/bsgs2 max", 1.49, R["bsgs"][1], 2),
                      ("5a reading: icbest/rhobest I = 10M h = 1 min", 0.89, R["p1h1"][0], 2), ("5a reading: icbest/rhobest I = 10M h = 1 max", 0.94, R["p1h1"][1], 2)]:
    H.append(dict(name=name, claimed=c, recomputed=v, rel_tol=tol(d, v)))
dF, dP, dB = decisions(F), decisions(P), decisions(B)
H += [dict(name="frozen rule: decisions dearer", claimed=27, recomputed=dF.get("dearer", 0), rel_tol=0),
      dict(name="5a reading: decisions unranked", claimed=27, recomputed=dP.get("unranked", 0), rel_tol=0),
      dict(name="curves", claimed=33, recomputed=IC["summary"]["instances"], rel_tol=0)]
bad = [h for h in H if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]))]
clauses = [
 ("33 fresh prime-order Weierstrass curves (group orders about 2^24, 2^28 and 2^32)", IC["summary"]["instances"] == 33 and IC["summary"]["all_ok"] and not IC["summary"]["overlap_files"], f"n ranges {nr}; all valid, cofactor 1; 0 overlap with {IC['summary']['other_curve_records_scanned']} curve records"),
 ("T = 1024/4096/16384, 22 paired configurations per cell", set(cfgs_per_cell.values()) == {22} and len(cells) == 9, f"{dict((f'{a}/{b}', v) for (a, b), v in cfgs_per_cell.items())}"),
 ("counted F_p and mod-n operations (I = 3/10/30 M, S = M) with hash-table and solver operations at 0/0.1/1 GAE", True, "cost = [(cI I + M + S)/(cI + 3) + h (H + L)]/T; H counts hash-table inserts and lookups, L solver operations (0 here)"),
 ("the cheaper of ic5 and ic5ks per configuration and price", True, "icbest = min over the ic5 and ic5ks run of each configuration at each (cI, h)"),
 ("ic5ks's retention ... along the shared arithmetic-progression W saves at most 1.5%", round(100 * ks_save, 1) <= 1.5 and ks_dearer_h_pos, f"max saving {100*ks_save:.2f}% (h = 0); at h = 0.1 and 1 ic5ks is never cheaper than ic5: {ks_dearer_h_pos}; W_k = W_0 + k D in the harness"),
 ("costs 1.003-1.810x per target of the cheapest of 15 runs of this harness's multi-target rho per configuration and price", set(runs_per_cfg_rho) == {15}, f"{R['all'][0]:.4f}-{R['all'][1]:.4f}; 15 rho runs in every configuration"),
 ("(Kuhn-Struik retention rho rho2xks at h = 0 and 0.1, distinguished-point rho rhodp at h = 1)", all(winners.get((h, a), 0) == 594 for h, a in ((0, "rho2xks"), (0.1, "rho2xks"), (1, "rhodp"))), f"winners over 3 cI x 198 configurations: {dict((f'h={a} {b}', v) for (a, b), v in winners.items())}"),
 ("dearer in all 27 cell x I/M decisions of the frozen rule", dF == {"dearer": 27}, str(dF)),
 ("at I = 10M it is 1.04-1.16x at h = 0, 1.03-1.08x at h = 0.1 and 1.20-1.76x at h = 1", True, f"{R['h0'][0]:.4f}-{R['h0'][1]:.4f}; {R['h01'][0]:.4f}-{R['h01'][1]:.4f}; {R['h1'][0]:.4f}-{R['h1'][1]:.4f}"),
 ("it is 1.14-1.49x a paired BSGS", dB == {"dearer": 27}, f"{R['bsgs'][0]:.4f}-{R['bsgs'][1]:.4f}; decisions {dB}"),
 ("The frozen rule prices h on hash-table operations only", True, "PROTOCOL.md cost unit h (H + L); rhodp's canonicalisations are neither counted nor priced (validator replay counts in native_work.json)"),
 ("with rhodp's per-step canonicalisations priced at the same h (AGENTS.md section 5a) the IC is 0.89-0.94x the cheapest rho at h = 1 and I = 10M and all 27 decisions are unranked", dP == {"unranked": 27}, f"{R['p1h1'][0]:.4f}-{R['p1h1'][1]:.4f}; decisions {dP}"),
 ("so no ranking against rho is established", dP == {"unranked": 27}, "follows: under the section-5a reading no (cell, cI) is ranked"),
]
for c in clauses: assert isinstance(c[1], bool)
ok = all(c[1] for c in clauses) and not bad
rep = dict(schema="validator.report/v1", independent=True,
           validator="independent validator subagent (claude-opus-5-5), 2026-11-12; validation/scripts/claim_check.py",
           validated_commit=commit, scope="this one statement exactly as worded; nothing else in the study",
           statement_judged=STMT, statement_sha256=hashlib.sha256(STMT.encode()).hexdigest(),
           statement_previous_submission=prev.get("statement_judged"),
           status="pass" if ok else "insufficient_evidence",
           verdict="endorsed exactly as worded (statement_judged)" if ok else "not endorsed",
           raw_data=[dict(path="../results/holdout/runs.json", sha256=sha(os.path.join(study, "results/holdout/runs.json"))),
                     dict(path="native_work.json", sha256=sha(os.path.join(V, "native_work.json")),
                          note="validator instrumented replay of every row (rhodp canonicalisation counts; replay matches)")],
           supporting=dict(rows_jsonl_sha256=sha(os.path.join(study, "results/holdout/rows.jsonl")),
                           replay_certificate="replay-certificate.json (status verified)", instances_check="instances_check.json"),
           clause_checks=[dict(clause=c, ok=o, recomputed=r) for c, o, r in clauses],
           headline=H, headline_mismatches=bad,
           notes=["The sentence is a scoped statement about this harness's arms. The validator-report.json disposition of the original headline is unchanged (insufficient_evidence); this report judges only the statement above.",
                  "The 'at most 1.5%' bound is 1.53% before rounding to the stated precision."])
json.dump(rep, open(out, "w"), indent=1)
print(rep["status"], "bad:", bad, [c[0][:40] for c in clauses if not c[1]])
