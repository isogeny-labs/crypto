#!/usr/bin/env python3
"""EXP-KOBLITZ-REF-002: M1-M4 from sessions/ (ecbench band/compare only) against the audit's results.json.
Writes results.json and tables/."""
import json, os, subprocess, statistics, glob
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.abspath(os.path.join(HERE, "../.."))
EB = os.environ.get("ECBENCH", os.path.join(ROOT, "target/release/ecbench")); W = ["0", "0.1", "1"]
AUDIT = json.load(open(os.path.join(HERE, "../koblitz_rho_reference_20261115/results.json")))


def run(*a):
    p = subprocess.run([EB, *a], capture_output=True, text=True)
    if p.returncode:
        raise SystemExit(f"{a}: {p.stderr[-400:]}")
    return p.stdout


def cmp(d, a, b, w):
    j = json.loads(run("compare", "--dir", d, "--a", a, "--b", b, "--weight", w, "--json"))
    o = j["ops"]
    return dict(ratio=o["ratio_b_over_a"], lo=o["ci95"][0], hi=o["ci95"][1], pairs=o["pairs"], status=o["status"])


R = {"curves": {}}
for sd in sorted(glob.glob(os.path.join(HERE, "sessions/n*")), key=lambda s: int(os.path.basename(s)[1:])):
    n = os.path.basename(sd)[1:]
    if not os.path.exists(os.path.join(sd, "session.json")):
        continue
    os.makedirs(os.path.join(HERE, "tables"), exist_ok=True)
    bpath = os.path.join(HERE, "tables", f"band-n{n}.json")
    run("band", "--dir", sd, "--json", bpath)
    open(os.path.join(HERE, "tables", f"band-n{n}.txt"), "w").write(run("band", "--dir", sd, "--ranks"))
    rows = {r["arm"]: r for r in json.load(open(bpath))["rows"]}
    recs = [json.loads(l) for l in open(os.path.join(sd, "records.jsonl"))]
    recs = [r for r in recs if not r["warmup"]]
    arms = {}
    for a in rows:
        rr = [r for r in recs if r["arm"] == a]
        sel = [sum(p.get("work_native", {}).get("jump_selections", 0) for p in r["phases"]) for r in rr]
        arms[a] = dict(runs=rows[a]["runs"], verified=rows[a]["verified"], mean_S=dict(zip(W, rows[a]["mean_s"])),
                       median_total_gae=statistics.median(r["cost"]["total_gae"] for r in rr),
                       mean_jump_selections=statistics.mean(sel) if sel else 0)
    C = dict(log2_r=None, arms=arms, ratios={}, matched={})
    C["log2_r"] = AUDIT["curves"].get(n, {}).get("log2_r")
    C["sqrt_r"] = AUDIT["curves"].get(n, {}).get("sqrt_r")
    C["audit_v5_mean_S"] = AUDIT["curves"].get(n, {}).get("arms", {}).get("rho-v5", {}).get("mean_S")
    C["audit_v5_hash_counted_over_strong_derived"] = AUDIT["curves"].get(n, {}).get(
        "v5_hash_counted_over_strong_derived")
    pair = lambda a, b: {w: cmp(sd, a, b, w) for w in W}  # noqa: E731
    C["ratios"]["rho-v5/rho-strong"] = pair("rho-strong", "rho-v5")
    C["ratios"]["rho-v5-aa/rho-v5"] = pair("rho-v5", "rho-v5-aa")
    for a in rows:
        if a.startswith(("gen-", "k1-")):
            for ref in ("rho-strong", "rho-v5"):
                C["ratios"][f"{a}/{ref}"] = pair(ref, a)
    for w in W:
        ref = "rho-v5" if arms["rho-v5"]["mean_S"][w] < arms["rho-strong"]["mean_S"][w] else "rho-strong"
        C["matched"][w] = {"reference": ref,
                           **{a: C["ratios"][f"{a}/{ref}"][w] for a in rows if a.startswith(("gen-", "k1-"))}}
    aud = AUDIT["curves"].get(n, {}).get("arms", {})
    C["audit_v5_mean_S_w0"] = aud.get("rho-v5", {}).get("mean_S", {}).get("0")
    C["audit_ratios"] = {k: AUDIT["curves"][n]["ratios"].get(k) for k in
                         ("rho-v5/rho-strong",) + tuple(f"{a}/rho-v5" for a in rows if a.startswith(("gen-", "k1-")))}
    R["curves"][n] = C

v = lambda bad, data: "falsified" if bad else ("held" if data else "no data")  # noqa: E731
cur = R["curves"]
m1_bad = [(n, a) for n, C in cur.items() for a, x in C["arms"].items() if x["verified"] != x["runs"]]
m1_aa = [(n, w, C["ratios"]["rho-v5-aa/rho-v5"][w]) for n, C in cur.items() for w in W
         if not C["ratios"]["rho-v5-aa/rho-v5"][w]["lo"] <= 1 <= C["ratios"]["rho-v5-aa/rho-v5"][w]["hi"]]
audits = {n: json.load(open(os.path.join(HERE, "audits", f"audit-n{n}.json"))) for n in cur
          if os.path.exists(os.path.join(HERE, "audits", f"audit-n{n}.json"))}
m2 = [(n, C["arms"]["rho-v5"]["mean_S"]["0"], C["audit_v5_mean_S_w0"]) for n, C in cur.items()]
m2_bad = [x for x in m2 if x[2] is None or abs(x[1] - x[2]) > 1e-9 * max(1, abs(x[2]))]
m3 = [(n, w, C["ratios"]["rho-v5/rho-strong"][w]) for n, C in cur.items() if n in ("17", "23", "29", "31") for w in W]
m3_bad = [x for x in m3 if not x[2]["hi"] < 1]
# M2b (Amendment 1): v5 mean S(w) - audit mean S(w) == w * mean(jump_selections) / sqrt(r), rel 1e-6.
m2b = []
for n, C in cur.items():
    for w in ("0.1", "1"):
        got = C["arms"]["rho-v5"]["mean_S"][w] - C["audit_v5_mean_S"][w]
        want = float(w) * C["arms"]["rho-v5"]["mean_jump_selections"] / C["sqrt_r"]
        m2b.append((n, w, got, want, abs(got - want) <= 1e-6 * max(abs(want), 1e-12)))
R["verdicts"] = {
    "M1": {"unverified": m1_bad, "aa_interval_excludes_1": m1_aa, "audits": {n: a.get("problems", a.get("summary"))
                                                                              for n, a in audits.items()},
           "verdict": v(m1_bad or m1_aa, cur)},
    "M2": {"cells": m2, "verdict": v(m2_bad, m2)},
    "M2b": {"cells": m2b, "verdict": v([x for x in m2b if not x[4]], m2b)},
    "M3": {"cells": [(n, w, x["ratio"], x["lo"], x["hi"]) for n, w, x in m3], "verdict": v(m3_bad, m3)},
    "M4": {"matched": {n: C["matched"] for n, C in cur.items()},
           "expectation_n29_genc1_w0.1_in_[1.4,1.9]": (lambda x: {**x, "verdict": "held" if 1.4 <= x["ratio"] <= 1.9
                                                                 else "falsified"})(
               cur["29"]["matched"]["0.1"]["gen-c1"]) if "29" in cur else None},
}
R["note"] = ("Moving the reference changes the distance remaining, not any IC cost. L0; counted GAE deterministic. "
             "No claim about cryptographic sizes [pcc:unverified].")
json.dump(R, open(os.path.join(HERE, "results.json"), "w"), indent=1)
for k in ("M1", "M2", "M2b", "M3"):
    print(k, R["verdicts"][k]["verdict"])
print("M4 expectation", (R["verdicts"]["M4"]["expectation_n29_genc1_w0.1_in_[1.4,1.9]"] or {}).get("verdict"))
for n, C in cur.items():
    print(f"n{n}: v5/strong " + ", ".join(f"w={w} {C['ratios']['rho-v5/rho-strong'][w]['ratio']:.3f}" for w in W)
          + " | audit " + ", ".join(f"{C['audit_ratios']['rho-v5/rho-strong'][w]['ratio']:.3f}" for w in W
                                    if C['audit_ratios'].get('rho-v5/rho-strong')))
    for w in W:
        m = C["matched"][w]
        print(f"   w={w} matched={m['reference']}: " + ", ".join(f"{a} {x['ratio']:.3f} [{x['lo']:.3f},{x['hi']:.3f}]"
                                                                for a, x in m.items() if a != "reference"))
