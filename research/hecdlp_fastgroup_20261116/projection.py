#!/usr/bin/env python3
"""PROJECTION (arithmetic only, not a measurement of any solver).

Re-prices the frozen genus-4 rho-reference study
(research/genus4_rho_refs_20261116/run.jsonl, read-only) as if group
operations cost what src/cryptanalysis/genus_ladder_fastgroup.rs measured in
its smoke run (smoke.json, g = 4, p = 8209).  Nothing is re-run: every
operation count (group ops, native multiplications, additions, compares) is
taken unchanged from the records; only the price of a group operation moves.
Labelled smoke: the fast-law costs come from a smoke run on a shared host.

usage: projection.py SMOKE_JSON OUT_JSON
"""
import json
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RUN = os.path.join(HERE, "..", "genus4_rho_refs_20261116", "run.jsonl")
smoke_path, out_path = sys.argv[1], sys.argv[2]
B = 2000

rows = [json.loads(l) for l in open(RUN)]
smoke = [r for r in json.load(open(smoke_path)) if r["g"] == 4 and r["p"] == 8209][0]


def key(r):
    return (r["curve_seed"], r.get("target_seed", r["curve_seed"]), r["seed"])


def arm(name):
    return {key(r): r for r in rows if r.get("kind") == "run" and r["arm"] == name}


W = arm("C_double_lp+pre+coord+unitT")
R = arm("R_rho_neg")
P = arm("R_rho_plain")
cal = [r for r in rows if r.get("kind") == "calibration"]
mean = lambda v: sum(v) / len(v)
wm_p = mean([c["w_cal_mul_p"] for c in cal])
wa_p = mean([c["w_cal_add_p"] for c in cal])
wm_r = mean([c["w_cal_mul_r"] for c in cal])
ns_cantor_cal = mean([c["ns_cantor_add"] for c in cal])
ns_add_p = mean([c["ns_addmod_p"] for c in cal])
ns_mul_p = mean([c["ns_mulmod_p"] for c in cal])
ns_mul_r = mean([c["ns_mulmod_r"] for c in cal])

# ---- fast-law costs from the smoke run -------------------------------------
fast = {
    "fast1": (smoke["fast_single"]["mul"], smoke["fast_single"]["inv"], smoke["fast_single"]["ns"]),
}
for b in smoke["batched"]:
    fast[f"fast{b['W']}"] = (b["mul"], b["inv"], b["ns"])
t_cantor_ns = smoke["cantor"]["ns"]
kappa_meas = smoke["ns_inv_euclid"] / smoke["ns_mulmod"]  # inversion / mult, same code as the law
KAPPAS = [("0", 0.0), (f"{kappa_meas:.1f} (measured)", kappa_meas), ("30", 30.0)]


def law_cost(r, law, kappa):
    """Group-law cost of one record in F_p multiplications."""
    if law == "cantor":
        return r["cantor_mul"] + kappa * r["cantor_inv"]
    m, i, _ = fast[law]
    return r["group_ops"] * (m + kappa * i)


def native_ic(r, alpha, rho_r=1.0):
    return r["rel_mul"] + rho_r * r["la_mul"] + alpha * (r["rel_add"] + r["la_add"])


def native_rho(r, reading, alpha):
    C, K = r["native"], r["compares_uncharged"]
    if reading == "F1":
        return C
    return C + alpha * K  # F3: every emitted native op, adds/compares at alpha


def cost_F(r, who, law, kappa, reading, alpha):
    g = law_cost(r, law, kappa)
    return g + (native_ic(r, alpha if reading == "F3" else 0.0) if who == "ic" else native_rho(r, reading, alpha))


def boot(stat, keys, seed=20261116):
    rnd = random.Random(seed)
    targets = sorted({(k[0], k[1]) for k in keys})
    by_t = {t: [k for k in keys if (k[0], k[1]) == t] for t in targets}
    est = stat(keys)
    bs = []
    for _ in range(B):
        ks = []
        for t in [rnd.choice(targets) for _ in targets]:
            ks += [rnd.choice(by_t[t]) for _ in by_t[t]]
        bs.append(stat(ks))
    bs.sort()
    return {"ratio": est, "lo": bs[int(0.025 * B)], "hi": bs[int(0.975 * B) - 1]}


keys = sorted(W)
out = {
    "label": "PROJECTION (arithmetic on frozen counts); fast-law costs from a SMOKE run; [pcc:unverified]",
    "smoke_source": os.path.basename(smoke_path),
    "fast_law_per_add": {k: {"mul": v[0], "inv": v[1], "ns": v[2]} for k, v in fast.items()},
    "cantor_ns_now": t_cantor_ns,
    "kappa_inv_over_mul_measured": kappa_meas,
    "readings": {},
}

# laws: (IC law, rho law)
SCEN = [
    ("cantor/cantor (frozen)", "cantor", "cantor"),
    ("IC cantor, rho fast W=1 (rho-only, as the task words it)", "cantor", "fast1"),
    ("IC fast W=1, rho fast W=1 (law in both arms)", "fast1", "fast1"),
    ("IC fast W=1, rho fast W=16", "fast1", "fast16"),
    ("IC fast W=1, rho fast W=64", "fast1", "fast64"),
]

# ---- field-multiplication reading ------------------------------------------
fm = {}
for kname, kappa in KAPPAS:
    for reading, alpha in [("F1", 0.0), ("F3 alpha=1", 1.0), ("F3 alpha=0.35", ns_add_p / ns_mul_p)]:
        rd = reading.split()[0]
        for sname, lic, lrho in SCEN:
            def stat(ks, lic=lic, lrho=lrho, rd=rd, alpha=alpha, kappa=kappa):
                ic = mean([cost_F(W[k], "ic", lic, kappa, rd, alpha) for k in ks])
                rr = mean([cost_F(R[k], "rho", lrho, kappa, rd, alpha) for k in ks])
                pp = mean([cost_F(P[k], "rho", lrho, kappa, rd, alpha) for k in ks])
                return ic / min(rr, pp)

            fm[f"kappa={kname} | {reading} | {sname}"] = boot(stat, keys)
out["readings"]["field_mult"] = fm

# ---- calibrated reading (Qcal), old-GAE units -------------------------------
# frozen: IC = G + w_mul_p*rel_mul + w_mul_r*la_mul + w_add_p*(adds); rho = G
# fast law: a group op costs c = t_fast / t_cantor (same host, same bench) old GAEs.
qc = {}
variants = {
    "timed ratio": lambda law: 1.0 if law == "cantor" else fast[law][2] / t_cantor_ns,
    "counted ratio (mul + kappa_meas*inv)": lambda law: 1.0
    if law == "cantor"
    else (fast[law][0] + kappa_meas * fast[law][1]) / (393.138 + kappa_meas * 7.0),
}
for vname, cfun in variants.items():
    for sname, lic, lrho in SCEN:
        for rho_native in (False, True):
            def stat(ks, lic=lic, lrho=lrho, cfun=cfun, rho_native=rho_native):
                ic = mean(
                    [
                        W[k]["group_ops"] * cfun(lic) + wm_p * W[k]["rel_mul"] + wm_r * W[k]["la_mul"]
                        + wa_p * (W[k]["rel_add"] + W[k]["la_add"])
                        for k in ks
                    ]
                )

                def rho_c(r):
                    c = r["group_ops"] * cfun(lrho)
                    if rho_native:
                        nat = r["native"] + r["compares_uncharged"] + r["hash_uncharged"] + r["coef_adds_uncharged"]
                        c += nat * ns_add_p / ns_cantor_cal  # each at one modular add (sensitivity)
                    return c

                rr = mean([rho_c(R[k]) for k in ks])
                pp = mean([rho_c(P[k]) for k in ks])
                return ic / min(rr, pp)

            qc[f"{vname} | {sname} | rho native {'priced at 1 add each' if rho_native else 'priced 0 (as frozen)'}"] = boot(stat, keys)
out["readings"]["Qcal"] = qc
out["calibration"] = {
    "ns_cantor_add_calibration_mean": ns_cantor_cal,
    "w_cal_mul_p": wm_p,
    "w_cal_add_p": wa_p,
    "w_cal_mul_r": wm_r,
}
json.dump(out, open(out_path, "w"), indent=1)

# ---- markdown ---------------------------------------------------------------
f = lambda v: f"{v['ratio']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}]"
L = ["# Projection tables (generated by projection.py; arithmetic on frozen counts, fast-law costs from a smoke run)", ""]
L.append("Fast law per addition (g = 4, p = 8209): " + "; ".join(f"{k}: {v[0]:.1f} mul + {v[1]:.4f} inv, {v[2]:.0f} ns" for k, v in fast.items()) + f"; Cantor now {t_cantor_ns:.0f} ns; inv/mul time ratio {kappa_meas:.1f}")
L.append("")
L.append("## Qcal (old-GAE units; ratio arm w / cheapest rho, 95% bootstrap)")
L.append("")
L.append("| variant | ratio [95%] |")
L.append("|---|---|")
for k, v in qc.items():
    L.append(f"| {k} | {f(v)} |")
L.append("")
L.append("## Field-multiplication reading (ratio arm w / cheapest rho)")
L.append("")
L.append("| kappa / reading / law | ratio [95%] |")
L.append("|---|---|")
for k, v in fm.items():
    L.append(f"| {k} | {f(v)} |")
open(out_path.replace(".json", ".md"), "w").write("\n".join(L) + "\n")
print("\n".join(L[:40]))
