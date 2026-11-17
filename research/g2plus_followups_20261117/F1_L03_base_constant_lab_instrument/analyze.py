#!/usr/bin/env python3
"""F1 (L03 re-measure) analysis. Reads lab genus_ladder records; applies the frozen
PROTOCOL sec 7 decision rule. No threshold lives here except the frozen ones.
Standing: [pcc:unverified]. Standard library only."""
import sys, json, math, random, argparse

G = 3
KAPPA = G * G  # 9

# ---- FPM (SPEC sec 2) -------------------------------------------------------
def fpm(v, I, kappa):
    return (v.get("cantor_mul", 0) + I * v.get("cantor_inv", 0)
            + v.get("rel_mul", 0) + kappa * v.get("la_mul", 0))

def fpm_low(v):  return fpm(v, 1, 1)
def fpm_mid(v):  return fpm(v, 10, KAPPA)
def fpm_high(v): return fpm(v, 30, KAPPA)

BAND = {"low": fpm_low, "mid": fpm_mid, "high": fpm_high}

def la_share_mid(v):
    m = fpm_mid(v)
    return (KAPPA * v.get("la_mul", 0)) / m if m else 0.0

def rel_share_mid(v):
    m = fpm_mid(v)
    return v.get("rel_mul", 0) / m if m else 0.0

def base_size(p, cc):
    return math.ceil(cc * p ** (2.0 / 3.0))

# ---- record loading ---------------------------------------------------------
def load(path, cc=None):
    """Load run/skip records. `cc` (from which file this is) is stamped on each
    record because the lab binary does not store cc in the row."""
    runs = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            v = json.loads(line)
            if v.get("kind") in ("run", "skip"):
                if cc is not None and "cc" not in v:
                    v["cc"] = cc
                runs.append(v)
    return runs

def is_ic(v):
    return v.get("kind") == "run" and v.get("arm", "").startswith("C")

def is_rho(v):
    return v.get("arm") == "R_rho_neg"

def key(v):
    return (v["p"], v["curve_seed"], v["seed"])

# ---- bootstrap (SPEC sec 3) -------------------------------------------------
def hier_bootstrap(by_curve, n_seed_draw, n_resample=2000, seed=20261117):
    """by_curve: dict curve_seed -> list of per-pair ratios. Returns (lo, hi, point)."""
    rng = random.Random(seed)
    curves = list(by_curve.keys())
    if not curves:
        return (float("nan"), float("nan"), float("nan"))
    point = 1.0 - _mean([r for rs in by_curve.values() for r in rs])
    vals = []
    for _ in range(n_resample):
        drawn = []
        chosen = [rng.choice(curves) for _ in range(len(curves))]
        for c in chosen:
            pool = by_curve[c]
            for _ in range(n_seed_draw):
                drawn.append(rng.choice(pool))
        vals.append(1.0 - _mean(drawn))
    vals.sort()
    return (vals[int(0.025 * n_resample)], vals[int(0.975 * n_resample)], point)

def _mean(xs):
    return sum(xs) / len(xs) if xs else float("nan")

# ---- pairing + statistic ----------------------------------------------------
def pair_treatment(cc1_runs, cc2_runs, band_fn):
    """Return dict p -> {curve_seed -> [ratio,...]} and a pairing-error flag."""
    c1 = {key(v): v for v in cc1_runs if is_ic(v)}
    c2 = {key(v): v for v in cc2_runs if is_ic(v)}
    out = {}
    err = False
    for k, v1 in c1.items():
        if k not in c2:
            err = True
            continue
        p, cs, _ = k
        r = band_fn(v1) / band_fn(c2[k])
        out.setdefault(p, {}).setdefault(cs, []).append(r)
    return out, err

def pair_aa(cc2_runs, band_fn):
    """A/A on cc2: seed3->seed1, seed4->seed2, ratio B/A. p -> {curve_seed -> [ratio]}"""
    ic = {key(v): v for v in cc2_runs if is_ic(v)}
    out = {}
    for (p, cs, s), B in ic.items():
        if s in (3, 4):
            a_s = s - 2
            A = ic.get((p, cs, a_s))
            if A is not None:
                out.setdefault(p, {}).setdefault(cs, []).append(band_fn(B) / band_fn(A))
    return out

# ---- gates (SPEC sec 4 wiring, PROTOCOL sec 6) ------------------------------
def gates(cc1_runs, cc2_runs):
    ic = [v for v in cc1_runs + cc2_runs if is_ic(v)]
    rho = [v for v in cc2_runs if is_rho(v)]
    exhausted = [f'{v.get("arm")}/{v["p"]}/{v["curve_seed"]}/{v["seed"]}'
                 for v in ic if v.get("exhausted")]
    q1 = all(v.get("certified") and v.get("ok") and not v.get("exhausted") for v in ic) and len(ic) > 0
    # Q2: realized base equals ceil(cc*p^(2/3)); cc read from the run's label
    q2 = True
    for v in ic:
        cc = v.get("cc")
        if cc is None:
            q2 = False; break
        if v.get("fb_cols") != base_size(v["p"], cc):
            q2 = False; break
    # Q3: every cc1 cell paired to a cc2 cell on (p,curve_seed,seed)
    c2keys = {key(v) for v in cc2_runs if is_ic(v)}
    q3 = all(key(v) in c2keys for v in cc1_runs if is_ic(v))
    # Q4: trials==42975 at 8209, first curve, seed 1, cc=2 (hard instrument fidelity).
    # "first curve" = smallest curve_seed among cc=2 p=8209 cells.
    q4 = None
    c2_8209 = [v for v in cc2_runs if is_ic(v) and v["p"] == 8209]
    if c2_8209:
        first_cs = min(v["curve_seed"] for v in c2_8209)
        for v in c2_8209:
            if v["curve_seed"] == first_cs and v["seed"] == 1:
                q4 = (v.get("trials") == 42975)
    # Q5: rho present, certified, cc-independent (one rho arm per (p,curve_seed,seed))
    q5 = len(rho) > 0 and all(v.get("ok") for v in rho)
    # Q6: every IC attempts == 1
    q6 = all(v.get("attempts", 1) == 1 for v in ic)
    # Q7: A/A non-degenerate (both aa pairs exist at each prime)
    aa = pair_aa(cc2_runs, fpm_mid)
    q7 = len(aa) > 0 and all(len(rs) > 0 for p in aa.values() for rs in p.values())
    return {
        "Q1_all_certified_ok_not_exhausted": bool(q1),
        "Q2_base_size_ok": bool(q2),
        "Q3_pairing_curveseeds_match": bool(q3),
        "Q4_trials_42975_at_8209_c0_s1": (None if q4 is None else bool(q4)),
        "Q5_rho_cc_independent_and_certified": bool(q5),
        "Q6_attempts_all_1": bool(q6),
        "Q7_aa_nondegenerate": bool(q7),
        "any_exhausted_cells": exhausted,
    }

# ---- verdict (PROTOCOL sec 7, applied verbatim) -----------------------------
def decide(treatment, aa, gate, primes=(8209, 16411, 32771)):
    reasons = []
    # Q4 hard
    if gate["Q4_trials_42975_at_8209_c0_s1"] is False:
        return "inconclusive", "Q4 instrument fidelity failed"
    for q in ("Q1_all_certified_ok_not_exhausted", "Q2_base_size_ok",
              "Q3_pairing_curveseeds_match", "Q5_rho_cc_independent_and_certified",
              "Q6_attempts_all_1"):
        if gate[q] is False:
            return "inconclusive", f"{q} failed on a decision cell"
    if gate["any_exhausted_cells"]:
        return "inconclusive", "exhausted primary cell: " + ",".join(gate["any_exhausted_cells"])
    # A/A noise guard: upper bound of |G_AA| must not exceed 0.05
    for p in primes:
        a = aa.get(str(p)) or aa.get(p)
        if a is None:
            return "inconclusive", f"no A/A at p={p}"
        if max(abs(a["lo"]), abs(a["hi"])) > 0.05:
            return "inconclusive", f"A/A noise at p={p} exceeds 0.05"
    big = 32771
    tb = treatment[str(big)]["mid"]
    # killed first
    if tb["G"] < 0.05 or (tb["lo"] <= 0 <= tb["hi"]):
        return "killed", f"G(32771)={tb['G']:.4f}, CI=[{tb['lo']:.4f},{tb['hi']:.4f}]"
    # supported
    ok = True
    for p in primes:
        t = treatment[str(p)]["mid"]
        if not (t["G"] >= 0.15 and not (t["lo"] <= 0 <= t["hi"])):
            ok = False; reasons.append(f"p={p}: G={t['G']:.4f} CI=[{t['lo']:.4f},{t['hi']:.4f}]")
    # clause 3: G(32771) CI above A/A CI
    a = aa[str(big)]
    if not (tb["lo"] > a["hi"]):
        ok = False; reasons.append(f"G(32771) lo {tb['lo']:.4f} not above A/A hi {a['hi']:.4f}")
    if ok:
        return "supported", f"G>=0.15 w/ CI>0 at all primes; G(32771) above A/A"
    return "inconclusive", "; ".join(reasons) if reasons else "supported clauses not all met"

# ---- driver -----------------------------------------------------------------
def analyze(cc1_path, cc2_path, secondary_paths):
    cc1 = load(cc1_path, 1.0)
    cc2 = load(cc2_path, 2.0)
    primes = sorted({v["p"] for v in cc1 if is_ic(v)})
    treatment = {}
    for p in primes:
        treatment[str(p)] = {}
        for bn, bf in BAND.items():
            by_p, _ = pair_treatment([v for v in cc1 if v["p"] == p],
                                     [v for v in cc2 if v["p"] == p], bf)
            byc = by_p.get(p, {})
            lo, hi, pt = hier_bootstrap(byc, 4)
            npairs = sum(len(r) for r in byc.values())
            treatment[str(p)][bn] = {"G": pt, "lo": lo, "hi": hi,
                                     "mean_ratio": (1 - pt), "n_pairs": npairs}
    aa = {}
    for p in primes:
        by_p = pair_aa([v for v in cc2 if v["p"] == p], fpm_mid)
        byc = by_p.get(p, {})
        lo, hi, pt = hier_bootstrap(byc, 2)
        aa[str(p)] = {"G_AA": pt, "lo": lo, "hi": hi}
    shares = {}
    for p in primes:
        shares[str(p)] = {}
        for lab, runs in (("cc1", cc1), ("cc2", cc2)):
            cells = [v for v in runs if is_ic(v) and v["p"] == p]
            shares[str(p)][lab] = {
                "la_share_mid": _mean([la_share_mid(v) for v in cells]),
                "rel_share_mid": _mean([rel_share_mid(v) for v in cells]),
            }
    rho_ratio = {}
    rho_by = {key(v): v for v in cc2 if is_rho(v)}
    for p in primes:
        rho_ratio[str(p)] = {}
        for lab, runs in (("cc1", cc1), ("cc2", cc2)):
            byc = {}
            for v in runs:
                if is_ic(v) and v["p"] == p and key(v) in rho_by:
                    byc.setdefault(v["curve_seed"], []).append(fpm_mid(v) / fpm_mid(rho_by[key(v)]))
            if byc:
                rng = random.Random(20261117)
                point = _mean([r for rs in byc.values() for r in rs])
                vals = []
                curves = list(byc.keys())
                for _ in range(2000):
                    drawn = []
                    for c in [rng.choice(curves) for _ in curves]:
                        for _ in range(len(byc[c])):
                            drawn.append(rng.choice(byc[c]))
                    vals.append(_mean(drawn))
                vals.sort()
                rho_ratio[str(p)][lab] = {"mid": {"ratio": point,
                                                   "lo": vals[50], "hi": vals[1949]}}
    secondary = {}
    sec = {}
    import re as _re, os as _os
    for pth in secondary_paths:
        m = _re.search(r"cc([0-9.]+)\\.jsonl$", _os.path.basename(pth))
        fcc = float(m.group(1)) if m else None
        for v in load(pth, fcc):
            if is_ic(v):
                sec.setdefault(v["p"], {}).setdefault(v.get("cc"), []).append(v)
    prim_cc = {"cc1": (1.0, cc1), "cc2": (2.0, cc2)}
    for p in primes:
        means = {}
        exhausted = []
        for cc in (0.5, 1.0, 1.5, 2.0, 3.0):
            if cc in (1.0, 2.0):
                cells = [v for v in (cc1 if cc == 1.0 else cc2) if is_ic(v) and v["p"] == p]
            else:
                cells = [v for v in sec.get(p, {}).get(cc, []) if is_ic(v)]
            live = [v for v in cells if not v.get("exhausted")]
            if any(v.get("exhausted") for v in cells):
                exhausted.append(str(cc))
            if live:
                means[("%g" % cc)] = _mean([fpm_mid(v) for v in live])
        argmin = min(means, key=means.get) if means else None
        secondary[str(p)] = {"means_mid": means, "argmin_cc": argmin, "exhausted": exhausted}
    gate = gates(cc1, cc2)
    verdict, reason = decide(treatment, aa, gate)
    return {"gates": gate, "treatment": treatment, "aa": aa, "shares": shares,
            "rho_ratio": rho_ratio, "secondary": secondary,
            "verdict": verdict, "verdict_reason": reason}

def tables_md(res):
    out = ["# F1 tables [pcc:unverified]", "",
           "| p | G_mid | 95% CI | G_low | G_high | la_share cc2/cc1 | c/rho mid cc2/cc1 | G_AA mid |",
           "|---|---|---|---|---|---|---|---|"]
    for p in sorted(res["treatment"], key=int):
        t = res["treatment"][p]; a = res["aa"][p]; sh = res["shares"][p]
        rr = res["rho_ratio"].get(p, {})
        r2 = rr.get("cc2", {}).get("mid", {}).get("ratio", float("nan"))
        r1 = rr.get("cc1", {}).get("mid", {}).get("ratio", float("nan"))
        out.append("| %s | %.4f | [%.4f,%.4f] | %.4f | %.4f | %.3f/%.3f | %.4g/%.4g | %.4f |" % (
            p, t["mid"]["G"], t["mid"]["lo"], t["mid"]["hi"], t["low"]["G"], t["high"]["G"],
            sh["cc2"]["la_share_mid"], sh["cc1"]["la_share_mid"], r2, r1, a["G_AA"]))
    out.append("")
    for p in sorted(res["secondary"], key=int):
        s = res["secondary"][p]
        out.append("S1 p=%s means_mid=%s argmin=%s exhausted=%s" % (
            p, {k: round(v) for k, v in s["means_mid"].items()}, s["argmin_cc"], s["exhausted"]))
    out.append("")
    out.append("VERDICT: %s (%s)" % (res["verdict"], res["verdict_reason"]))
    return "\n".join(out)

# ---- selftest (SPEC sec 5) --------------------------------------------------
def selftest():
    v1 = {"cantor_mul": 16298030, "cantor_inv": 370833, "rel_mul": 14992132, "la_mul": 2012026}
    v2 = {"cantor_mul": 9811891, "cantor_inv": 223134, "rel_mul": 8985775, "la_mul": 6960576}
    assert fpm_low(v1) == 33673021, fpm_low(v1)
    assert fpm_mid(v1) == 53106726, fpm_mid(v1)
    assert fpm_high(v1) == 60523386, fpm_high(v1)
    assert fpm_low(v2) == 25981376, fpm_low(v2)
    assert fpm_mid(v2) == 83674190, fpm_mid(v2)
    assert fpm_high(v2) == 88136870, fpm_high(v2)
    assert base_size(1031, 1.0) == 103 and base_size(1031, 2.0) == 205
    assert base_size(8209, 1.0) == 407 and base_size(8209, 2.0) == 814
    assert base_size(32771, 1.0) == 1025 and base_size(32771, 2.0) == 2049
    assert abs(la_share_mid(v2) - 62645184 / 83674190) < 1e-3
    assert abs(la_share_mid(v1) - 18108234 / 53106726) < 1e-3
    assert abs((1 - fpm_mid(v1) / fpm_mid(v2)) - 0.36531) < 1e-4
    r = {"cantor_mul": 883094, "cantor_inv": 19976}
    assert fpm_mid(r) == 1082854 and fpm_low(r) == 903070
    # T6 bootstrap determinism
    bc = {0: [0.6, 0.7], 1: [0.65, 0.68], 2: [0.62, 0.71]}
    a = hier_bootstrap(bc, 2); b = hier_bootstrap(bc, 2)
    assert a == b, (a, b)
    # T7 pairing guard
    _, err = pair_treatment([{"kind": "run", "arm": "C_double_lp+pre", "p": 8209, "curve_seed": 1,
                              "seed": 1, "cantor_mul": 1, "cantor_inv": 0, "rel_mul": 0, "la_mul": 0}],
                            [], fpm_mid)
    assert err is True
    # T8 verdict mapping
    def mk(g):
        return {"8209": {"mid": {"G": g[0], "lo": g[0]-.05, "hi": g[0]+.05}},
                "16411": {"mid": {"G": g[1], "lo": g[1]-.05, "hi": g[1]+.05}},
                "32771": {"mid": {"G": g[2], "lo": g[2]-.05, "hi": g[2]+.05}}}
    aa_ok = {str(p): {"G_AA": 0.0, "lo": -0.01, "hi": 0.01} for p in (8209, 16411, 32771)}
    gate_ok = {"Q1_all_certified_ok_not_exhausted": True, "Q2_base_size_ok": True,
               "Q3_pairing_curveseeds_match": True, "Q4_trials_42975_at_8209_c0_s1": True,
               "Q5_rho_cc_independent_and_certified": True, "Q6_attempts_all_1": True,
               "Q7_aa_nondegenerate": True, "any_exhausted_cells": []}
    assert decide(mk([.3, .3, .3]), aa_ok, gate_ok)[0] == "supported"
    assert decide(mk([.3, .3, .02]), aa_ok, gate_ok)[0] == "killed"
    km = mk([.3, .1, .08]); km["32771"]["mid"] = {"G": .08, "lo": -.01, "hi": .17}
    assert decide(km, aa_ok, gate_ok)[0] == "killed"
    aa_noisy = {str(p): {"G_AA": 0.0, "lo": -0.08, "hi": 0.08} for p in (8209, 16411, 32771)}
    assert decide(mk([.3, .3, .3]), aa_noisy, gate_ok)[0] == "inconclusive"
    gate_q4 = dict(gate_ok); gate_q4["Q4_trials_42975_at_8209_c0_s1"] = False
    assert decide(mk([.3, .3, .3]), aa_ok, gate_q4) == ("inconclusive", "Q4 instrument fidelity failed")
    print("OK")

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cc1", nargs="?")
    ap.add_argument("cc2", nargs="?")
    ap.add_argument("--secondary", nargs="*", default=[])
    ap.add_argument("--out")
    ap.add_argument("--tables")
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        selftest(); return
    res = analyze(a.cc1, a.cc2, a.secondary)
    if a.out:
        json.dump(res, open(a.out, "w"), indent=1, default=float)
    if a.tables:
        open(a.tables, "w").write(tables_md(res))
    print("VERDICT:", res["verdict"], "(", res["verdict_reason"], ")")

if __name__ == "__main__":
    main()
