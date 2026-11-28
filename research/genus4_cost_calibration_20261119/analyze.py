#!/usr/bin/env python3
"""Readings of PROTOCOL.md (genus4_cost_calibration_20261119). Written before any data. [pcc:unverified]"""
import glob, gzip, json, math, os, random, re

HERE = os.path.dirname(os.path.abspath(__file__))
PO = os.path.join(HERE, "..", "genus4_prime_order_20261118")
NB, SEED = 2000, 20261125
F = ["group_ops", "rel_mul", "rel_add", "la_mul", "la_add", "trials", "relations"]


def rows(path):
    op = gzip.open if path.endswith(".gz") else open
    return [json.loads(l) for l in op(path, "rt") if l.strip()]


def uniq(pattern):
    fs = sorted(glob.glob(pattern))
    return [f for f in fs if not (f.endswith(".jsonl") and f + ".gz" in fs)]


ic, inst = [], {}
for f in uniq(os.path.join(HERE, "ic_pass*_p*_cc*.jsonl*")):
    m = re.search(r"ic_(pass\d)_p(\d+)_cc([\d.]+)\.jsonl", f)
    for r in rows(f):
        if r.get("kind") == "instance":
            inst[(r["p"], r["curve_seed"])] = r
        if r.get("kind") == "run":
            sge = re.search(r"sge\((\d+),([\d.]+)\)", r["arm"])
            r["_pass"], r["_cc"], r["_sge"] = m.group(1), m.group(3), f"{sge.group(1)},{sge.group(2)}"
            ic.append(r)
rho = []
for f in uniq(os.path.join(HERE, "rho_p*.jsonl*")):
    rho += [r for r in rows(f) if r.get("kind") == "run"]

gates = {"G1": [], "G2": [], "G3": []}
for r in ic:
    if not (r.get("certified") and r.get("ok")):
        gates["G1"].append(f"ic {r['p']} {r['curve_seed']} {r['seed']} {r['_cc']}")
for r in rho:
    if not r.get("certified"):
        gates["G1"].append(f"rho {r['p']} {r['seed']}")
key = lambda r: (r["p"], r["curve_seed"], r["target_seed"], r["seed"])
po = {}
for name, cc in (("x", "2.0"), ("xstar", "1.4")):
    for p in (16411, 32771):
        for f in uniq(os.path.join(PO, f"b_p{p}_{name}.jsonl*")):
            for r in rows(f):
                if r.get("kind") == "run":
                    po[(cc,) + key(r)] = r
g2n = 0
for r in ic:
    if r["_sge"] == "4,1.5" and (r["_cc"],) + key(r) in po:
        g2n += 1
        o = po[(r["_cc"],) + key(r)]
        if any(r[x] != o[x] for x in F):
            gates["G2"].append(f"{r['_cc']} {key(r)}")
p1 = {(r["_cc"], r["_sge"]) + key(r): r for r in ic if r["_pass"] == "pass1"}
p2 = {(r["_cc"], r["_sge"]) + key(r): r for r in ic if r["_pass"] == "pass2"}
for k, r in p2.items():
    if k not in p1 or any(r[x] != p1[k][x] for x in F):
        gates["G3"].append(str(k))
if g2n == 0:
    gates["G2"].append("no overlapping rows")
gate_ok = {g: not v for g, v in gates.items()}

out = {"gates": gate_ok, "gate_failures": {g: v[:20] for g, v in gates.items()}, "g2_rows": g2n, "primes": {},
       "claims": "[pcc:unverified]"}


def ols(X, y):
    # normal equations, small dense
    n = len(X[0])
    A = [[sum(X[i][a] * X[i][b] for i in range(len(X))) for b in range(n)] for a in range(n)]
    v = [sum(X[i][a] * y[i] for i in range(len(X))) for a in range(n)]
    for c in range(n):
        piv = max(range(c, n), key=lambda r: abs(A[r][c]))
        A[c], A[piv], v[c], v[piv] = A[piv], A[c], v[piv], v[c]
        for r in range(n):
            if r != c and A[c][c]:
                f = A[r][c] / A[c][c]
                A[r] = [A[r][j] - f * A[c][j] for j in range(n)]
                v[r] -= f * v[c]
    return [v[i] / A[i][i] for i in range(n)]


for p in (16411, 32771):
    R = [r for r in rho if r["p"] == p]
    if not R:
        continue
    tG = sum(r["wall_ns"] for r in R) / sum(r["group_ops"] for r in R)
    I = [r for r in ic if r["p"] == p]
    clusters = sorted({(r["curve_seed"], r["seed"], r["_cc"], r["_sge"]) for r in I})
    by = {c: [r for r in I if (r["curve_seed"], r["seed"], r["_cc"], r["_sge"]) == c] for c in clusters}

    def fit(rs):
        X = [[1.0, r["rel_mul"] + r["rel_add"], r["la_mul"] + r["la_add"]] for r in rs]
        y = [r["wall_ns"] - tG * r["group_ops"] for r in rs]
        b = ols(X, y)
        return b[1], b[2], b[0]
    t_rel, t_la, t0 = fit(I)
    rnd = random.Random(SEED)
    bs = []
    for _ in range(NB):
        rs = []
        for c in [rnd.choice(clusters) for _ in clusters]:
            rs += by[c]
        bs.append(fit(rs))
    q = lambda v: (sorted(v)[int(0.025 * NB)], sorted(v)[int(0.975 * NB) - 1])
    curves = sorted({r["curve_seed"] for r in I})
    neg = sum(math.sqrt(math.pi * inst[(p, c)]["r"] / 4) for c in curves) / len(curves)
    cell = {"rho_runs": len(R), "t_G_ns": tG, "t_G_per_run_ns": [r["wall_ns"] / r["group_ops"] for r in R],
            "t_rel_ns": t_rel, "t_la_ns": t_la, "t_const_ns": t0,
            "w_rel": {"value": t_rel / tG, "ci": [x / tG for x in q([b[0] for b in bs])]},
            "w_la": {"value": t_la / tG, "ci": [x / tG for x in q([b[1] for b in bs])]},
            "ideal_negation_rho_s": tG * neg / 1e9, "arms": {}}
    for arm, cc in (("x", "2.0"), ("x*", "1.4")):
        A = [r for r in I if r["_cc"] == cc and r["_sge"] == "4,1.5"]
        cl = sorted({(r["curve_seed"], r["seed"]) for r in A})
        ab = {c: [r for r in A if (r["curve_seed"], r["seed"]) == c] for c in cl}
        stat = lambda rs: (sum(r["wall_ns"] for r in rs) / len(rs)) / (tG * neg)
        rnd = random.Random(SEED + 1)
        bb = []
        for _ in range(NB):
            rs = []
            for c in [rnd.choice(cl) for _ in cl]:
                rs += ab[c]
            bb.append(stat(rs))
        cell["arms"][arm] = {"runs": len(A), "mean_wall_s": sum(r["wall_ns"] for r in A) / len(A) / 1e9,
                             "ratio_wall_vs_ideal_negation": stat(A), "ci": list(q(bb))}
    out["primes"][str(p)] = cell
aa = [abs(math.log(p2[k]["wall_ns"] / p1[k]["wall_ns"])) for k in p2 if k in p1]
out["aa"] = {"rows": len(aa), "median_abs_log_ratio": sorted(aa)[len(aa) // 2] if aa else None, "max": max(aa) if aa else None}
if "16411" in out["primes"] and "32771" in out["primes"]:
    out["tG_ratio_32771_over_16411"] = out["primes"]["32771"]["t_G_ns"] / out["primes"]["16411"]["t_G_ns"]
ups = [c[k]["ci"][1] for c in out["primes"].values() for k in ("w_rel", "w_la")]
lo16 = out["primes"].get("16411", {}).get("w_la", {}).get("ci", [0])[0]
out["decision"] = ("conservative" if ups and max(ups) < 1 else "fragile" if lo16 > 1.25 else "unresolved")
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

L = ["# Tables (generated by analyze.py; every number [pcc:unverified])", "",
     "Gates: " + ", ".join(f"{g} {'pass' if ok else 'FAIL'}" for g, ok in gate_ok.items()) + f" (G2 rows {g2n})",
     f"A/A: {out['aa']}", f"Decision: **{out['decision']}**", ""]
for p, c in out["primes"].items():
    L += [f"## p = {p}", "",
          f"- t_G = {c['t_G_ns']:.1f} ns per rho group operation ({c['rho_runs']} runs: "
          + ", ".join(f"{x:.1f}" for x in c["t_G_per_run_ns"]) + ")",
          f"- t_rel = {c['t_rel_ns']:.2f} ns, t_la = {c['t_la_ns']:.2f} ns per counted operation",
          f"- **w_rel = {c['w_rel']['value']:.4f}** [{c['w_rel']['ci'][0]:.4f}, {c['w_rel']['ci'][1]:.4f}] GAE; "
          f"**w_la = {c['w_la']['value']:.4f}** [{c['w_la']['ci'][0]:.4f}, {c['w_la']['ci'][1]:.4f}] GAE",
          f"- ideal negation rho at this t_G: {c['ideal_negation_rho_s']:.0f} s", ""]
    for a, v in c["arms"].items():
        L.append(f"- arm {a}: mean wall {v['mean_wall_s']:.2f} s; wall / ideal negation rho = "
                 f"**{v['ratio_wall_vs_ideal_negation']:.5f}** [{v['ci'][0]:.5f}, {v['ci'][1]:.5f}]")
    L.append("")
if "tG_ratio_32771_over_16411" in out:
    L.append(f"t_G(32771) / t_G(16411) = {out['tG_ratio_32771_over_16411']:.3f}")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L))
