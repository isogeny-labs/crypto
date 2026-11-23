#!/usr/bin/env python3
"""L03 analysis (SPEC.md section 8). Python 3 standard library only."""
import sys, json, math, random, glob

GRID = [2.0 ** (k / 2.0) for k in range(-6, 5)]
PRIMES = [8209, 16411, 32771]
D = [1, 3, 10, 30]
LAM = [1.0, 0.35, 0.255]
BANDS = {"lo": (1, 1), "mid": (10, 9), "hi": (30, 9)}
RHO_LAB_MID = {8209: 1.425e8, 16411: 2.887e8, 32771: 1.0084e9}  # [pcc:unverified]
CAL = {8209: dict(cantor_per_trial=280.36, rel_per_trial=207.86, trials=43964, core_cols=741.0, la_mul=6.976e6),
       16411: dict(cantor_per_trial=278.62, rel_per_trial=228.72, trials=102969, core_cols=1196.8, la_mul=1.798e7),
       32771: dict(cantor_per_trial=278.52, rel_per_trial=231.98, trials=242686, core_cols=1933.2, la_mul=4.643e7)}
FB_EXPECT = {
    8209: [51, 72, 102, 144, 204, 288, 407, 576, 814, 1151, 1628],
    16411: [81, 115, 162, 229, 323, 457, 646, 914, 1292, 1827, 2584],
    32771: [129, 182, 257, 363, 513, 725, 1025, 1449, 2049, 2897, 4097]}
B = 4000


def fpm(x, d, lam, band, reading="primary"):
    I, kappa = BANDS[band] if isinstance(band, str) else band
    f_fixed = x["cm_setup"] + I * x["ci_setup"] + x["cm_verify"] + I * x["ci_verify"] + x["rel_fixed_mul"]
    f_rel = x["cm_loop"] + I * x["ci_loop"] + x["rel_trial_mul"]
    la = x["la_mul"] + (0.1 * x["la_add"] if reading == "S_add" else 0)
    return f_fixed + f_rel / d + lam * kappa * la


def gi(cc):
    for i, g in enumerate(GRID):
        if abs(cc - g) < 1e-9:
            return i
    raise ValueError(cc)


def select(runs_S, d, lam, band, reading):
    """runs_S: dict cc_index -> list of runs. Return cc_star (float), means."""
    means = {}
    for i in range(len(GRID)):
        v = [fpm(x, d, lam, band, reading) for x in runs_S[i]]
        means[i] = sum(v) / len(v)
    best = min(means.values())
    # on exact tie take the larger cc
    cands = [i for i in means if means[i] == best]
    i = max(cands)
    return GRID[i], means


def decide(triples):
    """triples: [(point, lo, hi)] for 8209, 16411, 32771."""
    pts = [t[0] for t in triples]
    lows = [t[1] for t in triples]
    his = [t[2] for t in triples]
    killed = sum(1 for x in pts if x < 0.15) >= 2 or his[2] < 0.15
    supported = sum(1 for x in lows if x >= 0.15) >= 2 and lows[2] >= 0.15
    if killed and not supported:
        return "killed"
    if supported and not killed:
        return "supported"
    return "inconclusive"


def pct_idx():
    return int(0.025 * B), int(0.975 * B) - 1


def selftest():
    ok = True

    def check(name, cond, info=""):
        nonlocal ok
        print(("PASS " if cond else "FAIL ") + name, info)
        ok &= bool(cond)
    # P1/P2
    def synth(d):
        runs = {}
        for i, cc in enumerate(GRID):
            x = dict(cm_setup=0, ci_setup=0, cm_verify=0, ci_verify=0, rel_fixed_mul=0,
                     cm_loop=1.0 / cc, ci_loop=0, rel_trial_mul=0, la_mul=cc * cc / 9.0, la_add=0)
            runs[i] = [x]
        return runs
    for d, ecc, emin, name in [(1, 0.7071067811865476, 1.914213562373095, "P1"),
                               (8, 0.3535533905932738, 0.47855339059327373, "P2")]:
        runs = synth(d)
        cc, means = select(runs, d, 1.0, "mid", "primary")
        mn = min(means.values())
        check(name, abs(cc - ecc) < 1e-12 and abs(mn - emin) < 1e-9, f"cc*={cc!r} min={mn!r}")
    # P3
    x = dict(cm_loop=1000, rel_trial_mul=500, la_mul=100, cm_setup=7, rel_fixed_mul=3, ci_setup=0,
             cm_verify=0, ci_verify=0, ci_loop=0, la_add=0)
    a, b, c = fpm(x, 1, 1.0, "mid"), fpm(x, 10, 1.0, "mid"), fpm(x, 10, 0.255, "mid")
    check("P3", abs(a - 2410) < 1e-9 and abs(b - 1060) < 1e-9 and abs(c - 389.5) < 1e-9, f"{a} {b} {c}")
    # P4
    A = [(0.3, 0.2, 0.4)] * 3
    Bt = [(0.1, 0.05, 0.2), (0.12, 0.05, 0.2), (0.5, 0.4, 0.6)]
    C = [(0.3, 0.2, 0.4), (0.3, 0.2, 0.4), (0.12, 0.1, 0.14)]
    Dd = [(0.3, 0.1, 0.4), (0.3, 0.2, 0.4), (0.3, 0.1, 0.4)]
    r = [decide(t) for t in (A, Bt, C, Dd)]
    check("P4", r == ["supported", "killed", "killed", "inconclusive"], str(r))
    # P5
    check("P5", pct_idx() == (100, 3899), str(pct_idx()))
    print("SELFTEST", "PASS" if ok else "FAIL")
    return 0 if ok else 1


def load(files):
    runs = []
    for fn in files:
        with open(fn) as fh:
            for line in fh:
                line = line.strip()
                if line and '"kind":"run"' in line:
                    runs.append(json.loads(line))
    return runs


def fmt(v):
    if isinstance(v, float):
        return f"{v:.6g}"
    return str(v)


def main(argv):
    if "--selftest" in argv:
        return selftest()
    files, vfiles, out, tables = [], [], "analysis/analysis.json", "analysis/tables.md"
    mode = "runs"
    i = 0
    while i < len(argv):
        a = argv[i]
        if a == "--verify":
            mode = "verify"
        elif a == "--out":
            out = argv[i + 1]; i += 1
        elif a == "--tables":
            tables = argv[i + 1]; i += 1
        else:
            (files if mode == "runs" else vfiles).append(a)
        i += 1
    files = [f for fn in files for f in sorted(glob.glob(fn))] or files
    vfiles = [f for fn in vfiles for f in sorted(glob.glob(fn))] or vfiles
    runs = load(files)
    vruns = load(vfiles)
    key = {}
    for x in runs:
        key[(x["p"], x["curve_seed"], x["seed"], gi(x["cc"]))] = x
    curves = {p: sorted({k[1] for k in key if k[0] == p}) for p in PRIMES}
    gates = []

    def gate(id_, ok, values):
        gates.append({"id": id_, "pass": bool(ok), "values": values})
    gate("Q_count", len(key) == 396 and len(runs) == 396, {"n_runs": len(runs), "n_keys": len(key)})
    bad4 = [k for k, x in key.items() if not (x["certified"] is True and x["exhausted"] is False and x["recovered"] == x["t_planted"])]
    gate("Q4", not bad4, {"failures": len(bad4)})
    bad5 = [k for k, x in key.items() if x["fb_cols"] != FB_EXPECT[k[0]][k[3]]]
    gate("Q5", not bad5, {"failures": len(bad5)})
    gate("Q6", len(vruns) > 0 and all(x["bad_relations"] == 0 and x["certified"] for x in vruns),
         {"n_verify_runs": len(vruns), "bad_relations_total": sum(x["bad_relations"] for x in vruns),
          "verified_relations_total": sum(x["verified_relations"] for x in vruns),
          "n_certified": sum(1 for x in vruns if x["certified"])})

    def sel(p, seeds, ci):
        return [key[(p, c, s, ci)] for c in curves[p] for s in seeds if (p, c, s, ci) in key]
    cal_vals, q7ok = {}, True
    for p in PRIMES:
        rr = sel(p, [1, 2, 3, 4], gi(2.0))
        if not rr:
            q7ok = False; continue
        T = sum(x["trials"] for x in rr)
        v = dict(cantor_per_trial=sum(x["cantor_mul"] + 10 * x["cantor_inv"] for x in rr) / T,
                 rel_per_trial=sum(x["rel_fixed_mul"] + x["rel_trial_mul"] for x in rr) / T,
                 trials=T / len(rr), core_cols=sum(x["core_cols"] for x in rr) / len(rr),
                 la_mul=sum(x["la_mul"] for x in rr) / len(rr))
        tol = dict(cantor_per_trial=.05, rel_per_trial=.10, trials=.15, core_cols=.05, la_mul=.20)
        row = {}
        for k in v:
            rel = v[k] / CAL[p][k] - 1
            row[k] = {"value": v[k], "lab": CAL[p][k], "rel_dev": rel, "tol": tol[k], "pass": abs(rel) <= tol[k]}
            q7ok &= abs(rel) <= tol[k]
        cal_vals[p] = row
    gate("Q7", q7ok, cal_vals)
    aa, q8ok = {}, True
    for p in PRIMES:
        S = [fpm(x, 1, 1.0, "mid") for x in sel(p, [1, 2], gi(2.0))]
        E = [fpm(x, 1, 1.0, "mid") for x in sel(p, [3, 4], gi(2.0))]
        if not S or not E:
            q8ok = False; continue
        aa[p] = (sum(S) / len(S)) / (sum(E) / len(E))
        q8ok &= abs(aa[p] - 1) <= 0.05
    gate("Q8", q8ok, aa)
    q9, q9ok = {}, True
    for p in PRIMES:
        a = sel(p, [1, 2, 3, 4], gi(0.25)); b = sel(p, [1, 2, 3, 4], gi(2.0)); c = sel(p, [1, 2, 3, 4], gi(4.0))
        if not (a and b and c):
            q9ok = False; continue
        tr = (sum(x["trials"] for x in a) / len(a)) / (sum(x["trials"] for x in b) / len(b))
        la = (sum(x["la_mul"] for x in c) / len(c)) / (sum(x["la_mul"] for x in b) / len(b))
        q9[p] = {"trials_ratio_0.25_over_2": tr, "la_mul_ratio_4_over_2": la}
        q9ok &= tr >= 3.0 and la >= 2.0
    gate("Q9", q9ok, q9)

    # selection and evaluation
    selection, evaluation = {}, {}
    rng = random.Random(20261116)
    lo_i, hi_i = pct_idx()
    all_have = len(key) == 396

    def Eruns(p, c, s, ci):
        return key[(p, c, s, ci)]
    for band in ["lo", "mid", "hi"]:
        for reading in ["primary", "S_add"]:
            for p in PRIMES:
                for d in D:
                    for lam in LAM:
                        runsS = {ci: sel(p, [1, 2], ci) for ci in range(11)}
                        ccs, means = select(runsS, d, lam, band, reading)
                        i_star = gi(ccs)
                        Ep = {ci: sel(p, [3, 4], ci) for ci in range(11)}
                        num = sum(fpm(x, d, lam, band, reading) for x in Ep[i_star])
                        den = sum(fpm(x, d, lam, band, reading) for x in Ep[gi(2.0)])
                        Gp = 1 - num / den
                        gs = []
                        for _ in range(B):
                            n_, d_ = 0.0, 0.0
                            for _c in range(3):
                                c = rng.choice(curves[p])
                                for _s in range(2):
                                    s = rng.choice([3, 4])
                                    n_ += fpm(key[(p, c, s, i_star)], d, lam, band, reading)
                                    d_ += fpm(key[(p, c, s, gi(2.0))], d, lam, band, reading)
                            gs.append(1 - n_ / d_)
                        gs.sort()
                        k = f"{band}|{reading}|{p}|{d}|{lam}"
                        selection[k] = {"cc_star": ccs, "edge": ccs in (GRID[0], GRID[-1]),
                                        "nb_star": FB_EXPECT[p][i_star], "meanS": {str(GRID[i]): means[i] for i in means}}
                        mfb = sum(x["beta"] for x in Ep[i_star]) / len(Ep[i_star])
                        selection[k]["beta_star_E"] = mfb
                        evaluation[k] = {"G": Gp, "lo": gs[lo_i], "hi": gs[hi_i],
                                         "sumE_star": num, "sumE_cc2": den,
                                         "ratio_to_lab_rho": (num / len(Ep[i_star])) / RHO_LAB_MID[p] if band == "mid" and reading == "primary" else None}
    # interaction (H2)
    rng2 = random.Random(20261117)
    inter = {}
    for p in PRIMES:
        c1 = selection[f"mid|primary|{p}|1|1.0"]["cc_star"]
        c10 = selection[f"mid|primary|{p}|10|1.0"]["cc_star"]
        i1, i10 = gi(c1), gi(c10)
        num = sum(fpm(key[(p, c, s, i10)], 10, 1.0, "mid") for c in curves[p] for s in [3, 4])
        den = sum(fpm(key[(p, c, s, i1)], 10, 1.0, "mid") for c in curves[p] for s in [3, 4])
        gs = []
        for _ in range(B):
            n_, d_ = 0.0, 0.0
            for _c in range(3):
                c = rng2.choice(curves[p])
                for _s in range(2):
                    s = rng2.choice([3, 4])
                    n_ += fpm(key[(p, c, s, i10)], 10, 1.0, "mid")
                    d_ += fpm(key[(p, c, s, i1)], 10, 1.0, "mid")
            gs.append(1 - n_ / d_)
        gs.sort()
        inter[p] = {"cc_star_1": c1, "cc_star_10": c10, "G_inc": 1 - num / den, "lo": gs[lo_i], "hi": gs[hi_i]}
    h2 = sum(1 for p in PRIMES if inter[p]["cc_star_10"] < inter[p]["cc_star_1"]) >= 2 and inter[32771]["lo"] >= 0.10
    # attribution
    attr = {}
    for p in PRIMES:
        Ep = lambda ci: sel(p, [3, 4], ci)
        F = lambda ci, d, lam: sum(fpm(x, d, lam, "mid") for x in Ep(ci))
        F0 = F(gi(2.0), 1, 1.0)
        for d in D:
            for lam in LAM:
                c_d1 = selection[f"mid|primary|{p}|{d}|1.0"]["cc_star"]
                c_dl = selection[f"mid|primary|{p}|{d}|{lam}"]["cc_star"]
                Ed = F(gi(2.0), d, 1.0) / F0
                Rd = F(gi(c_d1), d, 1.0) / F(gi(2.0), d, 1.0)
                Sdl = F(gi(c_dl), d, lam) / F(gi(c_d1), d, 1.0)
                tot = F(gi(c_dl), d, lam) / F0
                attr[f"{p}|{d}|{lam}"] = {"E_d": Ed, "R_d": Rd, "S_d_lam": Sdl, "total": tot,
                                          "product": Ed * Rd * Sdl, "G_1_retune": (1 - (F(gi(selection[f'mid|primary|{p}|1|1.0']['cc_star']), 1, 1.0) / F0)) }
    # H3
    def ols(xs, ys):
        n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
        return sum((a - mx) * (b - my) for a, b in zip(xs, ys)) / sum((a - mx) ** 2 for a in xs)
    h3, h3ok = {}, True
    for p in PRIMES:
        xs, ys = [], []
        for ci in range(2, 9):
            for x in sel(p, [1, 2, 3, 4], ci):
                xs.append(math.log(x["fb_cols"])); ys.append(math.log(x["trials"]))
        sT = ols(xs, ys)
        xs, ys = [], []
        for ci in range(11):
            for x in sel(p, [1, 2, 3, 4], ci):
                xs.append(math.log(x["core_cols"])); ys.append(math.log(x["la_mul"]))
        sL = ols(xs, ys)
        h3[p] = {"slope_T": sT, "slope_L": sL, "pass": (-1.3 <= sT <= -0.7) and (1.7 <= sL <= 2.4)}
        h3ok &= h3[p]["pass"]
    # rho
    rho = {}
    for p in PRIMES:
        pooled = []
        for c in curves[p]:
            rr = [key[(p, c, s, gi(2.0))] for s in [3, 4]]
            ops = sum(x["group_ops_loop"] for x in rr)
            per = sum(x["cm_loop"] + 10 * x["ci_loop"] for x in rr) / ops
            r_ = None
            pooled.append((c, per))
        rmap = {}
        for line in open("instances.jsonl"):
            j = json.loads(line)
            if j["p"] == p:
                rmap[j["curve_seed"]] = j["r"]
        rho_model = sum(math.sqrt(math.pi * rmap[c] / 4) * per for c, per in pooled) / len(pooled)
        rho[p] = {"rho_model": rho_model, "rho_lab_mid": RHO_LAB_MID[p]}
        for d in D:
            for lam in LAM:
                k = f"mid|primary|{p}|{d}|{lam}"
                i_star = gi(selection[k]["cc_star"])
                m = sum(fpm(x, d, lam, "mid") for x in sel(p, [3, 4], i_star)) / 6
                rho[p][f"{d}|{lam}"] = {"ratio_lab": m / RHO_LAB_MID[p], "ratio_model": m / rho_model}
    # verdict
    trip = [(evaluation[f"mid|primary|{p}|10|1.0"]["G"], evaluation[f"mid|primary|{p}|10|1.0"]["lo"],
             evaluation[f"mid|primary|{p}|10|1.0"]["hi"]) for p in PRIMES]
    verdict = decide(trip)
    if not all(g["pass"] for g in gates):
        verdict = "inconclusive (gate)"
    # per_cc_means
    pcm = {}
    fields = ["trials", "relations", "fb_cols", "beta", "core_rows", "core_cols", "core_nnz", "core_nonunit",
              "core_max_row", "la_mul", "la_add", "cm_loop", "rel_trial_mul", "dup_relations", "attempts"]
    for p in PRIMES:
        for ci in range(11):
            rr = sel(p, [1, 2, 3, 4], ci)
            m = {f: sum(x[f] for x in rr) / len(rr) for f in fields}
            m["rels_by_lp"] = [sum(x["rels_by_lp"][j] for x in rr) / len(rr) for j in range(3)]
            m["FPM_mid_d1"] = sum(fpm(x, 1, 1.0, "mid") for x in rr) / len(rr)
            m["F_rel_mid"] = sum(x["cm_loop"] + 10 * x["ci_loop"] + x["rel_trial_mul"] for x in rr) / len(rr)
            m["F_la_mid"] = sum(9 * x["la_mul"] for x in rr) / len(rr)
            pcm[f"{p}|{GRID[ci]}"] = m
    res = {"gates": gates, "selection": selection, "evaluation": evaluation,
           "interaction": {"per_prime": inter, "H2_held": h2}, "attribution": attr,
           "h3": {"per_prime": h3, "H3_held": h3ok}, "rho": rho, "verdict": verdict, "per_cc_means": pcm}
    with open(out, "w") as fh:
        json.dump(res, fh, indent=1, sort_keys=True)
    # tables
    T = [f"VERDICT: {verdict}", ""]
    T.append("## T1 per-cc means over all 12 runs per prime [pcc:unverified]\n")
    T.append("| p | cc | fb_cols | beta | trials | relations | rels_by_lp | core_rows | core_cols | core_nnz | la_mul | la_add | dup | attempts | FPM_mid(d=1) |")
    T.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for p in PRIMES:
        for ci in range(11):
            m = pcm[f"{p}|{GRID[ci]}"]
            T.append(f"| {p} | {fmt(GRID[ci])} | {fmt(m['fb_cols'])} | {fmt(m['beta'])} | {fmt(m['trials'])} | {fmt(m['relations'])} | {'/'.join(fmt(v) for v in m['rels_by_lp'])} | {fmt(m['core_rows'])} | {fmt(m['core_cols'])} | {fmt(m['core_nnz'])} | {fmt(m['la_mul'])} | {fmt(m['la_add'])} | {fmt(m['dup_relations'])} | {fmt(m['attempts'])} | {fmt(m['FPM_mid_d1'])} |")
    T.append("\n## T2 cc*, beta*, G [95% CI] (band mid, primary, lambda=1; d>1 rows: synthetic discount) [pcc:unverified]\n")
    T.append("| p | d | label | cc* | nb* | beta*(E) | edge | G | CI lo | CI hi |")
    T.append("|---|---|---|---|---|---|---|---|---|---|")
    for p in PRIMES:
        for d in D:
            k = f"mid|primary|{p}|{d}|1.0"
            s, e = selection[k], evaluation[k]
            T.append(f"| {p} | {d} | {'measured' if d == 1 else 'synthetic discount'} | {fmt(s['cc_star'])} | {s['nb_star']} | {fmt(s['beta_star_E'])} | {s['edge']} | {fmt(e['G'])} | {fmt(e['lo'])} | {fmt(e['hi'])} |")
    T.append("\n## T2b G for lambda < 1 (SGE parameter; synthetic discount for d>1) [pcc:unverified]\n")
    T.append("| p | d | lambda | label | cc* | G | CI lo | CI hi |")
    T.append("|---|---|---|---|---|---|---|---|")
    for p in PRIMES:
        for d in D:
            for lam in LAM[1:]:
                k = f"mid|primary|{p}|{d}|{lam}"
                s, e = selection[k], evaluation[k]
                T.append(f"| {p} | {d} | {lam} | SGE parameter{'; synthetic discount' if d > 1 else ''} | {fmt(s['cc_star'])} | {fmt(e['G'])} | {fmt(e['lo'])} | {fmt(e['hi'])} |")
    T.append("\n## T2c G at d=10, lambda=1 by band and reading [pcc:unverified]\n")
    T.append("| band | reading | p | cc* | G | CI lo | CI hi |")
    T.append("|---|---|---|---|---|---|---|")
    for band in ["lo", "mid", "hi"]:
        for reading in ["primary", "S_add"]:
            for p in PRIMES:
                k = f"{band}|{reading}|{p}|10|1.0"
                T.append(f"| {band} | {reading} | {p} | {fmt(selection[k]['cc_star'])} | {fmt(evaluation[k]['G'])} | {fmt(evaluation[k]['lo'])} | {fmt(evaluation[k]['hi'])} |")
    T.append("\n## T3 attribution (E, band mid; d>1 synthetic discount; lambda<1 SGE parameter) [pcc:unverified]\n")
    T.append("| p | d | lambda | E_d | R_d | S_d,lambda | product | total | G_1 (retune at d=1) |")
    T.append("|---|---|---|---|---|---|---|---|---|")
    for p in PRIMES:
        for d in D:
            for lam in LAM:
                a = attr[f"{p}|{d}|{lam}"]
                T.append(f"| {p} | {d}{' (synthetic discount)' if d > 1 else ''} | {lam}{' (SGE parameter)' if lam < 1 else ''} | {fmt(a['E_d'])} | {fmt(a['R_d'])} | {fmt(a['S_d_lam'])} | {fmt(a['product'])} | {fmt(a['total'])} | {fmt(a['G_1_retune'])} |")
    T.append("\n## T4 predicted IC/rho ratio (distance remaining; band mid; no ranking claim) [pcc:unverified]\n")
    T.append("| p | d | lambda | label | ratio vs lab rho | ratio vs model rho |")
    T.append("|---|---|---|---|---|---|")
    for p in PRIMES:
        for d in D:
            for lam in LAM:
                r_ = rho[p][f"{d}|{lam}"]
                lab = ("synthetic discount" if d > 1 else "") + ("; " if d > 1 and lam < 1 else "") + ("SGE parameter" if lam < 1 else "")
                T.append(f"| {p} | {d} | {lam} | {lab or 'measured'} | {fmt(r_['ratio_lab'])} | {fmt(r_['ratio_model'])} |")
    T.append("\n## T5 gates [pcc:unverified]\n")
    T.append("| gate | pass | values |")
    T.append("|---|---|---|")
    for g in gates:
        T.append(f"| {g['id']} | {g['pass']} | `{json.dumps(g['values'], default=str)[:600]}` |")
    T.append(f"\nH2 held: {h2}; H3 held: {h3ok}")
    T.append("\n## H2 interaction [pcc:unverified]\n")
    T.append("| p | cc*_1 | cc*_10 | G_inc | CI lo | CI hi |")
    T.append("|---|---|---|---|---|---|")
    for p in PRIMES:
        v = inter[p]
        T.append(f"| {p} | {fmt(v['cc_star_1'])} | {fmt(v['cc_star_10'])} | {fmt(v['G_inc'])} | {fmt(v['lo'])} | {fmt(v['hi'])} |")
    T.append("\n## H3 slopes [pcc:unverified]\n")
    T.append("| p | slope ln trials on ln fb_cols (cc 0.25..2) | slope ln la_mul on ln core_cols | in range |")
    T.append("|---|---|---|---|")
    for p in PRIMES:
        v = h3[p]
        T.append(f"| {p} | {fmt(v['slope_T'])} | {fmt(v['slope_L'])} | {v['pass']} |")
    with open(tables, "w") as fh:
        fh.write("\n".join(T) + "\n")
    print("verdict:", verdict)
    for g in gates:
        print("gate", g["id"], g["pass"])
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
