#!/usr/bin/env python3
"""Validator's own agreement sample (stdlib only): curves outside E25's ten and the unit test's four.

(a) replay/agreement/stage_a_p*s*.json (examples/e31_stage_a on (229,3), (353,3), (113,4)): per curve,
    calls, agreement counts, quotient checks, and the per-call identities validate.py checks on
    E25's curves (clean-call echelon of A = existing, A and B equal charpoly/eigenvector counts).
(b) replay/agreement/e2e_{macaulay,chain,chain-sparsest}_p113s4.json: end to end on one new curve;
    the chain records must equal the macaulay record leaf for leaf except the solve's cost
    (call, solve_muls, total_muls, ratio), macaulay_entries, timings and the solver tags, and
    solver_fallbacks must equal reruns + unsolved of the macaulay record.
Writes replay/agreement_comparison.json.
"""
import glob, hashlib, json, os, re, statistics, sys

HERE = os.path.dirname(os.path.abspath(__file__))
AG = os.path.join(HERE, "agreement")
ALLOWED = {"call", "solve_muls", "total_muls", "ratio", "macaulay_entries", "solve_ms", "wall_ms"}


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def leaves(x, pre=""):
    if isinstance(x, dict):
        for k, v in x.items():
            yield from leaves(v, f"{pre}.{k}" if pre else k)
    elif isinstance(x, list):
        for i, v in enumerate(x):
            yield from leaves(v, f"{pre}[{i}]")
    else:
        yield pre, x


def umed(v):
    s = sorted(v)
    return s[len(s) // 2]


out = {"stage_a": [], "e2e": None}
ok = True
for f in sorted(glob.glob(os.path.join(AG, "stage_a_p*s*.json"))):
    for c in json.load(open(f)):
        ex, ca, cb = c["arms"]
        n = c["calls"]
        clean = [i for i in range(n) if ca["call"][i] <= ex["call"][i]]
        fb = n - len(clean)
        rec = {"file": os.path.relpath(f, os.path.dirname(HERE)), "sha256": sha(f), "p": c["p"], "seed": c["seed"], "calls": n,
               "agree_A": c["agree"][0], "agree_B": c["agree"][1],
               "quotient_agree": c["quotient_agree"], "quotient_checked": c["quotient_checked"],
               "fallbacks": [ca["fallbacks"], cb["fallbacks"]], "fallbacks_derived": fb,
               "C_median": [umed(a["call"]) for a in (ex, ca, cb)],
               "existing_over_A": umed(ex["call"]) / umed(ca["call"]), "existing_over_B": umed(ex["call"]) / umed(cb["call"]),
               "clean_echelon_A_eq_existing": all(ca["echelon"][i] == ex["echelon"][i] for i in clean),
               "charpoly_eig_A_eq_B": all(ca["charpoly"][i] == cb["charpoly"][i] and ca["eigenvectors"][i] == cb["eigenvectors"][i] for i in range(n))}
        rec["agrees"] = (rec["agree_A"] == n and rec["agree_B"] == n and c["quotient_agree"] == c["quotient_checked"]
                         and fb == ca["fallbacks"] == cb["fallbacks"] and rec["clean_echelon_A_eq_existing"] and rec["charpoly_eig_A_eq_B"])
        ok &= rec["agrees"]
        out["stage_a"].append(rec)

files = {s: os.path.join(AG, f"e2e_{s}_p113s4.json") for s in ("macaulay", "chain", "chain-sparsest")}
if all(os.path.exists(f) for f in files.values()):
    base = json.load(open(files["macaulay"]))
    e2e = {"curve": "GLS p = 113 seed 4", "records": {s: {"path": os.path.relpath(f, os.path.dirname(HERE)), "sha256": sha(f)} for s, f in files.items()},
           "arms": []}
    for s in ("chain", "chain-sparsest"):
        recs = json.load(open(files[s]))
        for b, r in zip(base, recs):
            lb, lr = dict(leaves(b)), dict(leaves(r))
            diff = sorted(k for k in set(lb) & set(lr) if lb[k] != lr[k])
            bad = [k for k in diff if re.split(r"[.\[]", k)[0] not in ALLOWED]
            arm = {"solver": s, "arm": r["arm"], "leaves": len(lr), "differing": len(diff), "unexpected": bad,
                   "added": sorted(set(lr) - set(lb)), "missing": sorted(set(lb) - set(lr)),
                   "fallbacks": r.get("solver_fallbacks"), "reruns_plus_unsolved": b["reruns"] + b["unsolved"],
                   "residuals": r["residuals"], "correct": r["folded"]["correct"], "walks_correct": all(w["correct"] for w in r["rho"]),
                   "S_macaulay": b["total_muls"], "S": r["total_muls"], "S_macaulay_over_S": b["total_muls"] / r["total_muls"],
                   "C_median_macaulay": b["call"]["median"], "C_median": r["call"]["median"]}
            arm["agrees"] = (not bad and not arm["missing"] and arm["added"] == ["solver", "solver_fallbacks"]
                             and arm["fallbacks"] == arm["reruns_plus_unsolved"] and arm["correct"] and arm["walks_correct"])
            ok &= arm["agrees"]
            e2e["arms"].append(arm)
    out["e2e"] = e2e
out["all_agree"] = bool(ok)
with open(os.path.join(HERE, "agreement_comparison.json"), "w") as f:
    json.dump(out, f, indent=1)
    f.write("\n")
for r in out["stage_a"]:
    print(f"stage A p={r['p']} s={r['seed']}: {r['calls']} calls, agree A {r['agree_A']} B {r['agree_B']}, quotient {r['quotient_agree']}/{r['quotient_checked']}, "
          f"fallbacks {r['fallbacks']}, existing/A {r['existing_over_A']:.3f}, existing/B {r['existing_over_B']:.3f}, ok {r['agrees']}")
if out["e2e"]:
    for a in out["e2e"]["arms"]:
        print(f"e2e p=113 s=4 {a['solver']:>14} arm {a['arm']}: differing {a['differing']} unexpected {a['unexpected']} fallbacks {a['fallbacks']} = {a['reruns_plus_unsolved']}, "
              f"S ratio {a['S_macaulay_over_S']:.3f}, ok {a['agrees']}")
else:
    print("e2e p=113 s=4: not finished")
sys.exit(0 if ok else 1)
