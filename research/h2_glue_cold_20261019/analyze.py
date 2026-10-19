#!/usr/bin/env python3
"""Reads cold.csv / instances.csv recorded by h2_glue_cold and applies the
frozen rules of PROTOCOL.md (pricing, bootstrap, controls, per-cell verdicts).
Writes analysis.json and analysis.txt. Numbers come from the recorded counters;
this script only prices and aggregates them."""
import csv, json, sys, os, itertools
import numpy as np

run = sys.argv[1]
A1 = len(sys.argv) > 2 and sys.argv[2] == "A1"  # Amendment 1 control rule
rows = list(csv.DictReader(open(os.path.join(run, "cold.csv"))))
insts = list(csv.DictReader(open(os.path.join(run, "instances.csv"))))
SIG = [0.0, 0.5, 1.0]
IOTA = [1.0, 10.0, 100.0, 1000.0]
NU = [0.0, 0.1, 1.0]
NB = 4000
rng = np.random.default_rng(20261019)

build = {(int(r["bits"]), int(r["instance"])): r for r in insts}
data = {}  # (bits, inst, tgt, arm) -> dict
bad = []
for r in rows:
    key = (int(r["bits"]), int(r["instance"]), int(r["target"]), r["arm"])
    d = {k: float(r[k]) for k in ["steps", "legendre", "M", "S", "I", "native", "words", "seconds"]}
    d["p"] = float(r["p"]); d["ell"] = float(r["ell"]); d["ok"] = r["verified"] == "1"
    if not d["ok"]:
        bad.append(key)
    data[key] = d
main_arms = {a for (_, _, _, a) in data if "@" not in a}
tg = sorted({(b, i, t) for (b, i, t, a) in data if a == "R"})
N = len(tg)
inst_ids = sorted({(b, i) for (b, i, t) in tg})

def arm_vec(arm):
    """per-target counters for the arm; arm J gets construction, table build, Legendre."""
    out = []
    for (b, i, t) in tg:
        d = data[(b, i, t, arm)]
        c = dict(M=d["M"], S=d["S"], I=d["I"], N=d["native"], L=0.0, words=d["words"], steps=d["steps"], secs=d["seconds"], p=d["p"], ell=d["ell"])
        if arm == "J":
            bl = build[(b, i)]
            c["M"] += float(bl["build_M"]); c["S"] += float(bl["build_S"]); c["I"] += float(bl["build_I"])
            c["N"] += float(bl["build_native"]) + d["p"]  # sqrt-table build: p native ops
            c["L"] = d["legendre"]
        out.append(c)
    return out

def cost(c, sig, iota, nu):
    gae = 9 + 4 * sig + iota
    return c["M"] + sig * c["S"] + iota * (c["I"] + c["L"]) + nu * gae * c["N"]

def costs(vec, sig, iota, nu):
    return np.array([cost(c, sig, iota, nu) for c in vec])

# two-level bootstrap index sets (resample instances, then targets inside)
idx_of = {k: [j for j, (b, i, t) in enumerate(tg) if (b, i) == k] for k in inst_ids}
boots = []
for _ in range(NB):
    pick = rng.integers(0, len(inst_ids), len(inst_ids))
    sel = []
    for p_ in pick:
        ids = idx_of[inst_ids[p_]]
        sel.extend(rng.choice(ids, len(ids)))
    boots.append(np.array(sel))

def ratio(num, den):
    pt = num.sum() / den.sum()
    bs = np.array([num[s].sum() / den[s].sum() for s in boots])
    return pt, np.percentile(bs, 2.5), np.percentile(bs, 97.5)

V = {a: arm_vec(a) for a in sorted(main_arms)}
out = {"N_targets": N, "instances": len(inst_ids), "invalid_rows": [list(k) for k in bad]}
txt = []
def P(s=""):
    txt.append(s); print(s)

P(f"targets {N}, instances {len(inst_ids)}, unverified rows {len(bad)}")

# ---- controls
exact = all(
    all(abs(data[(b, i, t, "R")][k] - data[(b, i, t, "Rr")][k]) == 0 for k in ["steps", "M", "S", "I", "native", "words"])
    for (b, i, t) in tg
)
rep_ok = True; nrep = 0
for (b, i, t, a) in list(data):
    if a.endswith("@replay"):
        base = data[(b, i, t, a[:-7])]
        nrep += 1
        if any(abs(base[k] - data[(b, i, t, a)][k]) != 0 for k in ["steps", "M", "S", "I", "native", "words", "legendre"]):
            rep_ok = False
P(f"A/A exact replay (Rr == R, all counters, {N} targets): {exact}")
P(f"replay pass: {nrep} rows re-run, identical counters: {rep_ok}")
aa = []; pc = []
for sig, iota, nu in itertools.product(SIG, IOTA, NU):
    r = costs(V["Rp"], sig, iota, nu); b = costs(V["R"], sig, iota, nu); pl = costs(V["Pl"], sig, iota, nu)
    aa.append(ratio(r, b)); pc.append(ratio(pl, b))
aa_strict = all(lo <= 1 <= hi and abs(pt - 1) < 0.03 for pt, lo, hi in aa)
aa_ci = all(lo <= 1 <= hi for pt, lo, hi in aa)
aa_ok = aa_ci if A1 else aa_strict
pc_ok = all(lo > 1 for pt, lo, hi in pc)
P(f"A/A independent (Rp/R) over 36 combos: ratio range [{min(a[0] for a in aa):.4f}, {max(a[0] for a in aa):.4f}], all CIs contain 1: {all(lo<=1<=hi for _,lo,hi in aa)}, pass (strict |r-1|<0.03: {aa_strict}; CI-only: {aa_ci}; rule in force {"A1 CI-only" if A1 else "original strict"}): {aa_ok}")
P(f"positive control (plain/neg rho) ratio range [{min(a[0] for a in pc):.3f}, {max(a[0] for a in pc):.3f}], all CI > 1: {pc_ok}")
cal = {}
for bits in sorted({b for b, _ in inst_ids}):
    s = [c["steps"] / (0.886 * np.sqrt(c["ell"])) for c, (b, i, t) in zip(V["R"], tg) if b == bits]
    cal[bits] = float(np.mean(s))
cal_ok = all(0.85 <= v <= 1.25 for v in cal.values())
P(f"rho calibration mean steps/(0.886 sqrt ell): {cal}, pass: {cal_ok}")
controls_ok = exact and rep_ok and aa_ok and pc_ok and cal_ok and not bad
P(f"ALL CONTROLS PASS: {controls_ok}")
out["controls"] = dict(exact=exact, replay=rep_ok, aa=aa_ok, positive=pc_ok, calibration=cal, calibration_ok=cal_ok, all=controls_ok)

# ---- memory and equal-memory rho
mem_J = {k: np.mean([V["J"][j]["words"] for j in idx_of[k]]) for k in inst_ids}
dgrid = [0, 2, 4, 6, 8, 10, 12]
RM_choice = {}
for k in inst_ids:
    mems = {d: np.mean([V[f"D{d}"][j]["words"] for j in idx_of[k]]) for d in dgrid}
    ok_d = [d for d in dgrid if mems[d] <= mem_J[k]]
    RM_choice[k] = min(ok_d) if ok_d else None
RM = []
for j, (b, i, t) in enumerate(tg):
    d = RM_choice[(b, i)]
    RM.append(V[f"D{d}"][j] if d is not None else V["R"][j])
V["RM"] = RM
out["RM_choice"] = {f"{b}/{i}": v for (b, i), v in RM_choice.items()}
out["mem_words_J_mean"] = {f"{b}/{i}": float(v) for (b, i), v in mem_J.items()}
P("equal-memory rho choice d* per instance: " + str({f"{b}/{i}": v for (b, i), v in RM_choice.items()}))

# ---- per-cell verdicts
cells = []
for sig, iota in itertools.product(SIG, IOTA):
    rec = {"sigma": sig, "iota": iota, "nu": {}}
    ratios_R, ratios_cmp = [], []
    for nu in NU:
        cJ = costs(V["J"], sig, iota, nu); cR = costs(V["R"], sig, iota, nu); cM = costs(V["RM"], sig, iota, nu)
        cmp = cR if cR.sum() <= cM.sum() else cM
        rR = ratio(cJ, cR); rM = ratio(cJ, cM); rC = ratio(cJ, cmp)
        rec["nu"][str(nu)] = dict(J_over_R=rR, J_over_RM=rM, J_over_cmp=rC, cmp="R" if cmp is cR else "RM")
    lo = [rec["nu"][str(n)]["J_over_cmp"][1] for n in NU]
    hi = [rec["nu"][str(n)]["J_over_cmp"][2] for n in NU]
    rec["verdict"] = "rho-wins" if all(x > 1 for x in lo) else ("IC-wins" if all(x < 1 for x in hi) else "unresolved")
    cells.append(rec)
out["cells"] = cells
P("\ncell (S/M, I/M): J/comparator ratio [95% CI] at nu=0 | 0.1 | 1 -> verdict")
for c in cells:
    s = " | ".join("{:.3f} [{:.3f},{:.3f}]".format(*c["nu"][str(n)]["J_over_cmp"]) for n in NU)
    P(f"  ({c['sigma']}, {c['iota']:g}): {s} -> {c['verdict']}")
n_rho = sum(c["verdict"] == "rho-wins" for c in cells)
n_ic = sum(c["verdict"] == "IC-wins" for c in cells)
decision = "CLOSE H2 (rho-neg wins 12/12)" if n_rho == 12 else "SCOPE the <=22M+I genus-2 addition"
if not controls_ok:
    decision = "NO DECISION (a control failed)"
P(f"\ncells: rho-wins {n_rho}, IC-wins {n_ic}, unresolved {12-n_rho-n_ic}; DECISION: {decision}")
out["decision"] = decision; out["tally"] = dict(rho=n_rho, ic=n_ic, unresolved=12 - n_rho - n_ic)

# vs default R only, distance remaining
P("\nJ/R (default negation rho only), nu=0.1:")
for c in cells:
    P("  ({}, {:g}): {:.3f} [{:.3f},{:.3f}]".format(c["sigma"], c["iota"], *c["nu"]["0.1"]["J_over_R"]))

# ---- break-even Jacobian addition cost per cell: price J's Jacobian-addition share
# Treat J's cost as  W_other + steps*c_add ; find c_add (in M-equivalents) with J == comparator.
P("\nbreak-even Jacobian addition cost (M-equivalent, nu=0.1) vs measured backend cost:")
be = []
jm = np.mean([float(r["jac_M"]) for r in insts]); js = np.mean([float(r["jac_S"]) for r in insts]); ji = np.mean([float(r["jac_I"]) for r in insts])
out["backend_per_add"] = dict(M=jm, S=js, I=ji)
em = np.mean([float(r["eadd_M"]) for r in insts]); es = np.mean([float(r["eadd_S"]) for r in insts]); ei = np.mean([float(r["eadd_I"]) for r in insts])
out["E_per_add"] = dict(M=em, S=es, I=ei)
P(f"  backend: Jacobian add {jm:.2f}M + {js:.2f}S + {ji:.3f}I ; E(F_p^2) add {em:.2f}M + {es:.2f}S + {ei:.3f}I")
for c in cells:
    sig, iota = c["sigma"], c["iota"]
    nu = 0.1
    cmp_cost = min(costs(V["R"], sig, iota, nu).sum(), costs(V["RM"], sig, iota, nu).sum())
    J = V["J"]
    # Jacobian additions per target: steps (walk) ; priced at measured backend cost
    adds = sum(c_["steps"] for c_ in J)
    meas = jm + sig * js + iota * ji
    other = costs(J, sig, iota, nu).sum() - adds * meas
    beK = (cmp_cost - other) / adds
    be.append(dict(sigma=sig, iota=iota, breakeven=beK, measured=meas, ratio=beK / meas))
    P(f"  ({sig}, {iota:g}): measured {meas:.1f}M-eq, break-even {beK:.1f}M-eq (= {beK/meas:.2f} x measured), other J cost share {other/costs(J,sig,iota,nu).sum():.2f}")
out["breakeven"] = be


# ---- secondary analyses (Amendment 1 item 3; never used for the decision)
def cost_v(c, sig, iota, nu, leg="inv", addc=None):
    gae = 9 + 4 * sig + iota
    legc = {"inv": iota, "none": 0.0, "30M": 30.0}[leg]
    x = c["M"] + sig * c["S"] + iota * c["I"] + legc * c["L"] + nu * gae * c["N"]
    if addc is not None:  # replace walk Jacobian additions by addc(sig, iota)
        x += c["steps"] * (addc(sig, iota) - (jm + sig * js + iota * ji))
    return x
jm_, js_, ji_ = jm, js, ji
sec = {}
P("\nSECONDARY (not for the decision): tally of rho-wins / IC-wins / unresolved cells, and range of J/cmp at nu=0.1")
for name, leg, addc in [("primary (Legendre = I)", "inv", None), ("Legendre = 0", "none", None), ("Legendre = 30M", "30M", None),
                        ("hyp: add = 22M+I, Legendre = I", "inv", lambda sg, io: 22 + io), ("hyp: add = 22M+I, Legendre = 0", "none", lambda sg, io: 22 + io)]:
    tally = dict(rho=0, ic=0, un=0); r01 = []
    verd = []
    for sig, iota in itertools.product(SIG, IOTA):
        lo_, hi_ = [], []
        for nu in NU:
            cJ = np.array([cost_v(c, sig, iota, nu, leg, addc) for c in V["J"]])
            cR = costs(V["R"], sig, iota, nu); cM = costs(V["RM"], sig, iota, nu)
            cmp = cR if cR.sum() <= cM.sum() else cM
            pt, lo, hi = ratio(cJ, cmp); lo_.append(lo); hi_.append(hi)
            if nu == 0.1: r01.append(pt)
        v = "rho" if all(x > 1 for x in lo_) else ("ic" if all(x < 1 for x in hi_) else "un")
        tally[v] += 1; verd.append(v)
    P(f"  {name}: rho-wins {tally['rho']}, IC-wins {tally['ic']}, unresolved {tally['un']}; J/cmp at nu=0.1 in [{min(r01):.3f}, {max(r01):.3f}]")
    sec[name] = dict(tally=tally, ratio_nu01=[float(x) for x in r01])
out["secondary"] = sec
P("\nbreak-even Jacobian addition cost c* (M-equivalents) at which J == comparator, by nu; negative = even a free addition loses")
be2 = []
for sig, iota in itertools.product(SIG, IOTA):
    row = []
    for nu in NU:
        cmp_cost = min(costs(V["R"], sig, iota, nu).sum(), costs(V["RM"], sig, iota, nu).sum())
        adds = sum(c_["steps"] for c_ in V["J"])
        meas = jm + sig * js + iota * ji
        other = costs(V["J"], sig, iota, nu).sum() - adds * meas
        row.append((cmp_cost - other) / adds)
    be2.append(dict(sigma=sig, iota=iota, breakeven_by_nu=row, measured=jm + sig * js + iota * ji, hyp22=22 + iota))
    P(f"  (S/M {sig}, I/M {iota:g}): c* = {row[0]:.1f} | {row[1]:.1f} | {row[2]:.1f}   measured {jm+sig*js+iota*ji:.1f}, 22M+I = {22+iota:g}")
out["breakeven_by_nu"] = be2

# ---- time x memory table
tm = {}
for bits in sorted({b for b, _ in inst_ids}):
    rowsb = {}
    sel = [j for j, (b, i, t) in enumerate(tg) if b == bits]
    for a in ["J"] + [f"D{d}" for d in dgrid] + ["R", "Pl"]:
        v = V[a]
        rowsb[a] = dict(steps_over_sqrt_ell=float(np.mean([v[j]["steps"] / np.sqrt(v[j]["ell"]) for j in sel])), words=float(np.mean([v[j]["words"] for j in sel])), words_over_p=float(np.mean([v[j]["words"] / v[j]["p"] for j in sel])),
                        cost_sig05_iota10_nu01=float(np.mean([cost(v[j], 0.5, 10.0, 0.1) / v[j]["p"] for j in sel])))
    tm[bits] = rowsb
out["time_memory"] = {str(k): v for k, v in tm.items()}
P("\ntime x memory (mean per target; cost at S/M=0.5, I/M=10, nu=0.1 in units of p):")
for bits, rb in tm.items():
    P(f" bits {bits}:")
    for a, v in rb.items():
        P(f"   {a:>4}: steps/sqrt(l) {v['steps_over_sqrt_ell']:.3f}  words/p {v['words_over_p']:.4f}  cost/p {v['cost_sig05_iota10_nu01']:.2f}")

# wall time (descriptive)
P("\nwall seconds per target (descriptive, shared VM, threaded): " + ", ".join(f"{a}={np.mean([c['secs'] for c in V[a]]):.2f}" for a in ["R", "Rp", "J", "Pl"]))
aa_w = np.sum([c["secs"] for c in V["Rp"]]) / np.sum([c["secs"] for c in V["R"]])
P(f"wall A/A Rp/R = {aa_w:.4f} (no wall ranking is claimed)")
out["wall_aa"] = aa_w
json.dump(out, open(os.path.join(run, "analysis.json"), "w"), indent=1, default=float)
open(os.path.join(run, "analysis.txt"), "w").write("\n".join(txt) + "\n")
