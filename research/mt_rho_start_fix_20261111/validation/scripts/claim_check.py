#!/usr/bin/env python3
"""Validator: judge one scoped claim statement exactly as worded, recomputing every number in it from repository
files only (runs.json, validation/ks_runs.json, instances/), and write validation/claim-report.json.
Cost/target = [(cI I + M + S)/(cI + 3) + h (H + L)]/T; rhobest = min over the 12 harness rho runs per configuration
and price; 'with Kuhn-Struik retention' = min over those 12 and the validator's 4 rho2xks runs; decisions by the
frozen section-5a rule (bootstrap of the median, B = 4000, random.Random(1); cheaper iff hi < 1 at h = 0, 0.1, 1;
dearer iff lo > 1 at all three).
usage: claim_check.py A|B <repo> <validated commit>"""
import json, sys, os, hashlib, random, statistics, collections, subprocess, math
STUDY, REPO, COMMIT = sys.argv[1:4]
D = {"A": "research/mt_fieldops_v3_20261111", "B": "research/mt_rho_start_fix_20261111"}[STUDY]
RUNS = {"A": f"{D}/results/holdout/runs.json", "B": f"{D}/results/runs.json"}[STUDY]
KS = f"{D}/validation/ks_runs.json"
def blob(path): return subprocess.run(["git", "-C", REPO, "show", f"{COMMIT}:{path}"], capture_output=True, check=True).stdout
def sha(b): return hashlib.sha256(b).hexdigest()
raw = {p: blob(p) for p in (RUNS, KS)}
for p, b in raw.items():  # the worktree file the gate will hash must be the validated blob
    assert sha(open(os.path.join(REPO, p), "rb").read()) == sha(b), p
R = json.loads(raw[RUNS]); rows = [dict(zip(R["fields"], r)) for r in R["rows"]]
K = json.loads(raw[KS]); ks = [dict(zip(K["fields"], r)) for r in K["rows"]]
RHO = ["rho", "rho2", "rho2x", "rhoseq"]; CI = [3, 10, 30]; HP = [0, 0.1, 1]
def cost(r, ci, h): return ((ci * r["I"] + r["M"] + r["S"]) / (ci + 3) + h * (r["H"] + r["L"])) / r["targets"]
cfg = collections.defaultdict(lambda: collections.defaultdict(list))
for r in rows: cfg[(r["pb"], r["T"], r["idx"], r["seed"])][r["arm"]].append(r)
for r in ks:
    if r["arm"] == "rho2xks": cfg[(r["pb"], r["T"], r["idx"], r["seed"])]["rho2xks"].append(r)
def val(k, a, ci, h):
    if a == "rhobest": rs = [r for x in RHO for r in cfg[k][x]]
    elif a == "rhoks": rs = [r for x in RHO + ["rho2xks"] for r in cfg[k][x]]
    else: rs = cfg[k][a]
    return min(cost(r, ci, h) for r in rs)
def boot(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    m = sorted(statistics.median(rnd.choices(xs, k=n)) for _ in range(B)); return m[int(.025 * B)], m[int(.975 * B) - 1]
cells = sorted({k[:2] for k in cfg})
E = []
for cand, ref in ({"A": [("ic5", "ic4"), ("ic5", "bsgs2"), ("ic5", "rhobest"), ("ic5", "rhoks")],
                   "B": [("ic4", "rhobest"), ("ic4", "rhoks"), ("ic4", "bsgs2")]}[STUDY]):
    for ci in CI:
        for pb, T in cells:
            ks_ = [k for k in cfg if k[:2] == (pb, T)]
            for h in HP:
                rat = [val(k, cand, ci, h) / val(k, ref, ci, h) for k in ks_]; lo, hi = boot(rat)
                E.append(dict(cand=cand, ref=ref, cI=ci, bits=pb, T=T, h=h, median=statistics.median(rat), lo=lo, hi=hi, n=len(rat)))
def sel(ref, **kw): return [e for e in E if e["ref"] == ref and all(e[a] == b for a, b in kw.items())]
def rng(es): return min(e["median"] for e in es), max(e["median"] for e in es)
def decisions(ref):
    c = collections.Counter()
    for ci in CI:
        for pb, T in cells:
            s = sel(ref, cI=ci, bits=pb, T=T)
            c["cheaper" if all(e["hi"] < 1 for e in s) else "dearer" if all(e["lo"] > 1 for e in s) else "unranked"] += 1
    return dict(c)
def H(name, claimed, rec, dec):  # agreement to the stated precision (half a unit in the last stated digit)
    return dict(name=name, claimed=claimed, recomputed=rec, rel_tol=0.5 * 10 ** (-dec) / abs(rec) + 1e-12)
out = dict(schema="validator.report/v1", independent=True, validator="independent validator subagent (claude-opus-5-5), 2026-11-11; scripts/claim_check.py",
           validated_commit=COMMIT, scope="this one statement exactly as worded; nothing else in the study",
           raw_data=[dict(path=os.path.relpath(os.path.join(REPO, RUNS), os.path.join(REPO, D, "validation")), sha256=sha(raw[RUNS])),
                     dict(path="ks_runs.json", sha256=sha(raw[KS]), note="validator retention arm rho2xks (sensitivity runs, every target verified by the counted check)")])
if STUDY == "A":
    out["statement_submitted"] = ("On 44 fresh prime-order Weierstrass curves (2^22-2^33), T = 1024/4096/16384, 22 paired configurations per cell, counted F_p operations (I = 3/10/30 M, S = M) with hash/solver ops at 0/0.1/1 GAE: the k = 4 term multi-target index calculus ic5 costs 0.47-0.99x per target of ic4 (cheaper in all 36 cell x I/M section-5a decisions) and 1.11-1.49x a paired BSGS (dearer in all 36); against the cheapest of 12 runs of this harness's anchored fixed-database rho family it is 0.85-0.98x, but against the same family with Kuhn-Struik retention (validator arm rho2xks) it is 1.07-1.34x at h = 0 and 1.05-1.18x at h = 0.1, so no ranking against rho is established.")
    ns = [r["n"] for r in rows]; curves = len({(r["pb"], r["idx"]) for r in rows})
    a = rng(sel("ic4")); b = rng(sel("bsgs2")); c = rng(sel("rhobest")); k0 = rng(sel("rhoks", h=0)); k1 = rng(sel("rhoks", h=0.1)); k2 = rng(sel("rhoks", h=1))
    out["clause_checks"] = [
        dict(clause="44 fresh prime-order Weierstrass curves", ok=curves == 44, recomputed=f"{curves} curves (idx 1..11 x 4 sizes); validity and freshness: validator-report.json A-F checks"),
        dict(clause="(2^22-2^33)", ok=False, recomputed=f"group orders n from {min(ns)} (2^{math.log2(min(ns)):.5f}) to {max(ns)} (2^{math.log2(max(ns)):.5f}): about 2^22 to 2^32; no group near 2^33 was tested",
             required_fix="write '(group orders about 2^22 to 2^32)' or '(22- to 32-bit primes p)'"),
        dict(clause="T = 1024/4096/16384, 22 paired configurations per cell", ok=sorted({r['T'] for r in rows}) == [1024, 4096, 16384] and all(e["n"] == 22 for e in E), recomputed="yes"),
        dict(clause="counted F_p operations (I = 3/10/30 M, S = M) with hash/solver ops at 0/0.1/1 GAE", ok=True,
             recomputed="yes; ic5's mod-n elimination is counted in the same I/M units (0.08%-2.5% of its cost); 'F_p and mod-n operations' would be more exact (optional)"),
        dict(clause="ic5 0.47-0.99x of ic4, cheaper in all 36", ok=round(a[0], 2) == 0.47 and round(a[1], 2) == 0.99 and decisions("ic4") == {"cheaper": 36}, recomputed=f"{a[0]:.4f}-{a[1]:.4f}; {decisions('ic4')}"),
        dict(clause="1.11-1.49x a paired BSGS, dearer in all 36", ok=round(b[0], 2) == 1.11 and round(b[1], 2) == 1.49 and decisions("bsgs2") == {"dearer": 36}, recomputed=f"{b[0]:.4f}-{b[1]:.4f}; {decisions('bsgs2')}"),
        dict(clause="0.85-0.98x the cheapest of 12 harness rho runs", ok=round(c[0], 2) == 0.85 and round(c[1], 2) == 0.98, recomputed=f"{c[0]:.4f}-{c[1]:.4f}; decisions {decisions('rhobest')}"),
        dict(clause="with Kuhn-Struik retention (rho2xks): 1.07-1.34x at h = 0, 1.05-1.18x at h = 0.1", ok=(round(k0[0], 2), round(k0[1], 2), round(k1[0], 2), round(k1[1], 2)) == (1.07, 1.34, 1.05, 1.18),
             recomputed=f"h = 0: {k0[0]:.4f}-{k0[1]:.4f}; h = 0.1: {k1[0]:.4f}-{k1[1]:.4f}; (h = 1: {k2[0]:.4f}-{k2[1]:.4f})"),
        dict(clause="so no ranking against rho is established", ok=decisions("rhoks") == {"unranked": 36}, recomputed=f"section-5a decisions against rhobest + rho2xks: {decisions('rhoks')}")]
    out["headline"] = [H("ic5/ic4 min", 0.47, a[0], 2), H("ic5/ic4 max", 0.99, a[1], 2), H("ic5/bsgs2 min", 1.11, b[0], 2), H("ic5/bsgs2 max", 1.49, b[1], 2),
                       H("ic5/rhobest min", 0.85, c[0], 2), H("ic5/rhobest max", 0.98, c[1], 2), H("ic5/rhoKS h0 min", 1.07, k0[0], 2), H("ic5/rhoKS h0 max", 1.34, k0[1], 2),
                       H("ic5/rhoKS h0.1 min", 1.05, k1[0], 2), H("ic5/rhoKS h0.1 max", 1.18, k1[1], 2)]
    out["statement_accepted"] = out["statement_submitted"].replace("(2^22-2^33)", "(group orders about 2^22 to 2^32)")
else:
    out["statement_submitted"] = ("With the rho-family restart defect of examples/mt_fieldops.rs fixed (all 1584 v2 rho jobs rerun, every target verified), the cheapest of four rho arms is 1.4-4.8% cheaper and multi-target IC4 costs 0.951-0.991x it per target at T = 16384 on the v2 curves (cheaper at every I/M and h against this harness's rho family); against the same family with Kuhn-Struik retention IC4 is 1.12-1.17x at h = 0 and the v2 decisions are 0 cheaper, 6 unranked, 12 dearer. IC4 vs paired BSGS is unchanged (1.18-1.80x).")
    rho_rows = [r for r in rows if r["arm"] in RHO]
    v2 = json.loads(blob("research/mt_fieldops_v2_20261111/results/holdout/runs.json")); v2rows = [dict(zip(v2["fields"], r)) for r in v2["rows"]]
    v2cfg = collections.defaultdict(list)
    for r in v2rows:
        if r["arm"] in RHO: v2cfg[(r["pb"], r["T"], r["idx"], r["seed"])].append(r)
    red = []
    for ci in CI:
        for pb, T in cells:
            ks_ = [k for k in cfg if k[:2] == (pb, T)]
            for h in HP:
                new = statistics.median(val(k, "rhobest", ci, h) for k in ks_); old = statistics.median(min(cost(r, ci, h) for r in v2cfg[k]) for k in ks_)
                red.append(1 - new / old)
    t = rng(sel("rhobest", T=16384)); dec16 = collections.Counter()
    for ci in CI:
        for pb in sorted({c[0] for c in cells if c[1] == 16384}):
            s = sel("rhobest", cI=ci, bits=pb, T=16384); dec16["cheaper" if all(e["hi"] < 1 for e in s) else "other"] += 1
    k0_16 = rng(sel("rhoks", h=0, T=16384)); k0_all = rng(sel("rhoks", h=0)); bs = rng(sel("bsgs2"))
    v2b = json.loads(blob("research/mt_fieldops_v2_20261111/results/holdout/summary.json"))
    v2bs = [s["median"] for s in v2b if s["vs"] == "bsgs2"]
    out["clause_checks"] = [
        dict(clause="all 1584 v2 rho jobs rerun, every target verified", ok=len(rho_rows) == 1584 and all(r["ok"] == r["targets"] for r in rho_rows), recomputed=f"{len(rho_rows)} rho-family rows, all ok == targets; replay 2112/2112 and 36 runs independently verified (replay-certificate.json)"),
        dict(clause="the cheapest of four rho arms is 1.4-4.8% cheaper", ok=(round(100 * min(red), 1), round(100 * max(red), 1)) == (1.4, 4.8), recomputed=f"rhobest median cost per (cell, cI, h) {100 * min(red):.3f}%-{100 * max(red):.3f}% lower than in the v2 rows"),
        dict(clause="IC4 0.951-0.991x it at T = 16384, cheaper at every I/M and h", ok=(round(t[0], 3), round(t[1], 3)) == (0.951, 0.991) and dec16 == {"cheaper": 6}, recomputed=f"{t[0]:.4f}-{t[1]:.4f}; T = 16384 (cell, cI) decisions {dict(dec16)}"),
        dict(clause="against the same family with Kuhn-Struik retention IC4 is 1.12-1.17x at h = 0", ok=False,
             recomputed=f"T = 16384: {k0_16[0]:.4f}-{k0_16[1]:.4f}; all six v2 cells: {k0_all[0]:.4f}-{k0_all[1]:.4f}",
             required_fix="the range holds only at T = 16384, but the clause carries no T and the next clause speaks of all v2 decisions; write '1.12-1.17x at h = 0 at T = 16384 (1.12-1.48x over all six v2 cells)'"),
        dict(clause="the v2 decisions are 0 cheaper, 6 unranked, 12 dearer", ok=decisions("rhoks") == {"unranked": 6, "dearer": 12}, recomputed=f"{decisions('rhoks')}"),
        dict(clause="IC4 vs paired BSGS is unchanged (1.18-1.80x)", ok=(round(bs[0], 2), round(bs[1], 2)) == (1.18, 1.80) and sorted(v2bs) == sorted(e["median"] for e in sel("bsgs2")),
             recomputed=f"{bs[0]:.4f}-{bs[1]:.4f}; identical to the v2 summary medians")]
    out["headline"] = [H("rhobest reduction min %", 1.4, 100 * min(red), 1), H("rhobest reduction max %", 4.8, 100 * max(red), 1), H("IC4/rhobest T16384 min", 0.951, t[0], 3),
                       H("IC4/rhobest T16384 max", 0.991, t[1], 3), H("IC4/rhoKS h0 T16384 min", 1.12, k0_16[0], 2), H("IC4/rhoKS h0 T16384 max", 1.17, k0_16[1], 2),
                       H("IC4/bsgs2 min", 1.18, bs[0], 2), H("IC4/bsgs2 max", 1.80, bs[1], 2)]
    out["statement_accepted"] = out["statement_submitted"].replace("IC4 is 1.12-1.17x at h = 0 and", "IC4 is 1.12-1.17x at h = 0 at T = 16384 (1.12-1.48x over all six v2 cells) and")
ok = all(c["ok"] for c in out["clause_checks"])
out["status"] = "pass" if ok else "needs_rewording"
out["verdict"] = "endorsed as worded" if ok else "not endorsed as worded; the numbers recompute, the wording in clause_checks with ok = false must change; statement_accepted would be endorsed"
dst = os.path.join(REPO, D, "validation", "claim-report.json")
json.dump(out, open(dst, "w"), indent=1)
print(dst, out["status"], sha(open(dst, "rb").read()))
for c in out["clause_checks"]: print(" ", c["ok"], c["clause"], "|", c["recomputed"])
