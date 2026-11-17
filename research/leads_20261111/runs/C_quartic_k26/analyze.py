#!/usr/bin/env python3
"""LEAD-C-quartic-n4v2-k26 analysis (local part). Reads raw/ only; writes results.json and prints
Markdown tables used in RESULT.md.

1. committed-record check: per-size means of every count statistic and of the per-phase M-eq
   (recomputed with the committed r_inv / r_lmul) for D, DL1, DL at k = 17..22 against
   research/quartic_n4v2_20261020/results/summary.json (the committed per-unit records are in an
   S3 artifact this container cannot read; the per-size means are what is in Git).
2. M-eq per unit with this host's frozen calibration (median of three `calib` records per size),
   the section-5a band w = 0 / 0.1 / 1 GAE, CPU seconds, and peak RSS.
3. Exponent fits ln cost = a + e ln q, 95% bootstrap over units (resampled within each size),
   on k = 17..24 and k = 22..24, with sensitivities (r_lmul held at its k = 22 value; counts only).
4. DL / rho-neg at k = 22..24 against the calibrated rho-neg model (pooled steps / sqrt(ell) from the
   committed 1225 runs; per-step cost scaled to this host's GAE), band on the calibration
   constant [0.75, 1.15] (the frozen admissibility range), and the k = 25, 26 projections.
"""
import glob
import json
import math
import os
import random
import statistics as stt

D = os.path.dirname(os.path.abspath(__file__))
SUMMARY = os.path.join(D, "../../../quartic_n4v2_20261020/results/summary.json")
PHASES = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
ARMS = ["D", "DL1", "DL"]
KOF = {131071: 17, 262111: 18, 524287: 19, 1048507: 20, 2097091: 21, 4194271: 22, 8388571: 23, 16777183: 24, 33554383: 25, 67108819: 26}
QOF = {v: k for k, v in KOF.items()}


def jl(p):
    with open(p) as f:
        return [json.loads(l) for l in f if l.strip().startswith("{")]


def load_units():
    units = {}  # (arm, q, seed) -> record (last attempt) + attempts
    ratios = {}
    res = {}
    # raw/wide: bin/v2w (k = 17..24); raw/v2w2/k25: bin/v2w2 (packed relations, k = 25)
    for p in sorted(glob.glob(os.path.join(D, "raw/wide/c_ic_*_D.jsonl")) + glob.glob(os.path.join(D, "raw/v2w2/k25/c_ic_*_D.jsonl"))):
        rr = jl(p)
        rj = p.replace(".jsonl", ".res.json")
        if not os.path.exists(rj):
            continue
        r0 = json.load(open(rj))
        for r in rr:
            if r["rec"] == "ratios":
                ratios[(r["q"], r["seed"])] = r
                res[(r["q"], r["seed"])] = r0
            elif r["rec"] == "ic":
                key = (r["arm"], r["q"], r["seed"])
                prev = units.get(key)
                att = (prev["_attempts"] if prev else 0) + 1
                # all attempts are charged (as in summ2): accumulate counts over attempts
                if prev:
                    for k, v in r.items():
                        if any(k.startswith(ph + ".") for ph in PHASES) and isinstance(v, (int, float)):
                            r[k] = v + prev.get(k, 0)
                r["_attempts"] = att
                units[key] = r
    return units, ratios, res


def calib():
    cal = {}
    for p in glob.glob(os.path.join(D, "raw/wide/calib_*.jsonl")) + glob.glob(os.path.join(D, "raw/v2w2/k25/calib_*.jsonl")):
        for r in jl(p):
            if r["rec"] == "calib":
                cal.setdefault(r["q"], []).append(r)
    return {q: (stt.median([r["r_inv"] for r in v]), stt.median([r["r_lmul"] for r in v]), len(v)) for q, v in cal.items()}


def meq_unit(u, r_inv, r_l, gae, r_l_override=None):
    rl = r_l if r_l_override is None else r_l_override
    m = 0.0
    unp = 0.0
    ph = []
    cpu = 0.0
    for p in PHASES:
        x = u.get(p + ".mul", 0) + r_inv * u.get(p + ".inv", 0) + rl * u.get(p + ".lmul", 0)
        ph.append(x)
        m += x
        unp += u.get(p + ".hash", 0) + u.get(p + ".cmp", 0)
        cpu += u.get(p + ".cpu", 0.0)
    return {"w0": m, "w01": m + 0.1 * gae * unp, "w1": m + gae * unp, "cpu": cpu, "ph": ph, "unp": unp,
            "mul": sum(u.get(p + ".mul", 0) for p in PHASES), "lmul": sum(u.get(p + ".lmul", 0) for p in PHASES),
            "inv": sum(u.get(p + ".inv", 0) for p in PHASES)}


def fit(xs, ys):
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    b = sxy / sxx
    return b, my - b * mx


def boot_fit(per_size, B=4000, seed=7):
    """per_size: {q: [cost per unit]}; OLS of ln mean cost on ln q, bootstrap over units within size."""
    qs = sorted(per_size)
    xs = [math.log(q) for q in qs]
    e, a = fit(xs, [math.log(stt.mean(per_size[q])) for q in qs])
    rng = random.Random(seed)
    es = []
    for _ in range(B):
        ys = []
        for q in qs:
            v = per_size[q]
            ys.append(math.log(stt.mean([rng.choice(v) for _ in v])))
        es.append(fit(xs, ys)[0])
    es.sort()
    return e, a, es[int(0.025 * B)], es[int(0.975 * B) - 1]


def main():
    out = {"lead": "LEAD-C-quartic-n4v2-k26"}
    units, ratios, res = load_units()
    cal = calib()
    summ = [json.loads(l) for l in open(SUMMARY) if l.strip()]
    summ_ic = {(r["arm"], r["q"]): r for r in summ if r.get("kind") == "ic_size"}

    # ---------------------------------------------------------------- 1. committed means
    print("## 1. Committed per-size means reproduced (D, DL1, DL; k = 17..22; seeds 1-3)\n")
    print("| arm | q | k | stats compared | max rel. diff (stats) | max rel. diff (phase M-eq, committed r_inv/r_lmul) | verdict |")
    print("|:--|--:|--:|--:|--:|--:|:--|")
    chk = []
    for q in [131071, 262111, 524287, 1048507, 2097091, 4194271]:
        for arm in ARMS:
            sr = summ_ic.get((arm, q))
            us = [units.get((arm, q, s)) for s in (1, 2, 3)]
            if sr is None or any(u is None for u in us):
                continue
            nst = 0
            md = 0.0
            for k, v in sr["stats"].items():
                if k in ("controls_cpu", "controls_wall"):
                    continue
                vals = [u.get("s." + k) for u in us]
                if any(x is None for x in vals):
                    # a stat present in some units only: the summarizer averages the units that have it
                    vals = [x for x in vals if x is not None]
                    if not vals:
                        continue
                m = stt.mean(vals)
                nst += 1
                d = abs(m - v) / max(abs(v), 1e-12) if not (m == 0 and v == 0) else 0.0
                # the committed means are printed to 4 decimals
                if abs(m - v) <= 5e-5:
                    d = 0.0
                md = max(md, d)
            pm = [stt.mean([meq_unit(u, sr["r_inv"], sr["r_lmul"], sr["gae"])["ph"][i] for u in us]) for i in range(7)]
            mp = max(abs(a - b) / max(abs(b), 1e-9) if b != 0 else abs(a) for a, b in zip(pm, sr["phase_meq"]))
            ok = md < 1e-6 and mp < 1e-5
            chk.append({"arm": arm, "q": q, "stats": nst, "max_rel_stats": md, "max_rel_phase_meq": mp, "ok": ok})
            print(f"| {arm} | {q} | {KOF[q]} | {nst} | {md:.1e} | {mp:.1e} | {'match' if ok else 'MISMATCH'} |")
    out["committed_check"] = chk
    out["committed_check_all_match"] = bool(chk) and all(c["ok"] for c in chk)
    print(f"\nall match: {out['committed_check_all_match']} ({len(chk)} arm-size cells)\n")

    # ---------------------------------------------------------------- 2. per size on this host
    sizes = sorted({q for (_, q, _) in units})
    gae = {}
    for q in sizes:
        rs = [r for (qq, s), r in ratios.items() if qq == q]
        if q in cal:
            r_inv = cal[q][0]
            gae[q] = stt.median([r["gae_mul"] for r in rs]) + r_inv * stt.median([r["gae_inv"] for r in rs])
    print("## 2. D family per size on this host (memory mode; M-eq with this host's calib medians)\n")
    print("| arm | q | k | units ok/total | r_inv | r_lmul | M-eq w=0 | w=0.1 | w=1 | CPU s (arm unit, mean) | peak RSS GiB (D-family process, max) | lmul count | log2 ell |")
    print("|:--|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
    per = {}
    for arm in ARMS:
        for q in sizes:
            if q not in cal:
                continue
            us = [(s, units[(arm, q, s)]) for s in (1, 2, 3) if (arm, q, s) in units]
            r_inv, r_l, _ = cal[q]
            vals = [(s, meq_unit(u, r_inv, r_l, gae[q]), u) for s, u in us]
            ok = [v for v in vals if v[2]["ok"] and v[2]["k_found"] == v[2]["k"]]
            rss = max(res[(q, s)]["maxrss_kib"] for s, _ in us) / 2**20
            ell = max(u["ell"] for _, u in us)
            per[(arm, q)] = {"units": len(us), "ok": len(ok), "w0": [v[1]["w0"] for v in ok], "w01": [v[1]["w01"] for v in ok],
                             "w1": [v[1]["w1"] for v in ok], "cpu": [v[1]["cpu"] for v in ok], "rss_gib": rss,
                             "lmul": [v[1]["lmul"] for v in ok], "mul": [v[1]["mul"] for v in ok], "inv": [v[1]["inv"] for v in ok],
                             "r_inv": r_inv, "r_lmul": r_l, "gae": gae[q], "log2_ell": math.log2(ell),
                             "attempts": [v[2]["_attempts"] for v in vals]}
            p = per[(arm, q)]
            print(f"| {arm} | {q} | {KOF[q]} | {p['ok']}/{p['units']} | {r_inv:.2f} | {r_l:.3f} | {stt.mean(p['w0']):.4e} | {stt.mean(p['w01']):.4e} | {stt.mean(p['w1']):.4e} | {stt.mean(p['cpu']):.1f} | {rss:.2f} | {stt.mean(p['lmul']):.3e} | {p['log2_ell']:.1f} |")
    out["per_size"] = {f"{a}:{q}": v for (a, q), v in per.items()}

    # ---------------------------------------------------------------- 3. exponents
    print("\n## 3. Exponent fits (95% bootstrap over units)\n")
    print("| arm | range | sizes | e (M-eq w=0) | e (w=1) | e (CPU s) | e (w=0, r_lmul fixed at k=22) | e (mul+lmul counts) |")
    print("|:--|:--|--:|:--|:--|:--|:--|:--|")
    fits = []
    q22 = 4194271
    for arm in ARMS:
        for name, k0, k1 in [("k17-25", 17, 25), ("k22-25", 22, 25), ("k17-24", 17, 24), ("k22-24", 22, 24), ("k17-22", 17, 22)]:
            qs = [q for q in sizes if (arm, q) in per and k0 <= KOF[q] <= k1 and per[(arm, q)]["ok"] > 0]
            if len(qs) < 2:
                continue
            row = {"arm": arm, "range": name, "sizes": len(qs)}
            cells = []
            for lab, getter in [("w0", lambda q: per[(arm, q)]["w0"]), ("w1", lambda q: per[(arm, q)]["w1"]), ("cpu", lambda q: per[(arm, q)]["cpu"])]:
                e, a, lo, hi = boot_fit({q: getter(q) for q in qs})
                row[lab] = [e, lo, hi]
                cells.append(f"{e:.3f} [{lo:.3f}, {hi:.3f}]")
            # sensitivity: r_lmul fixed at k = 22 value for all sizes (removes the u64 -> Barrett step)
            if (arm, q22) in per:
                rl22 = per[(arm, q22)]["r_lmul"]
                fx = {}
                for q in qs:
                    us = [units[(arm, q, s)] for s in (1, 2, 3) if (arm, q, s) in units]
                    fx[q] = [meq_unit(u, cal[q][0], cal[q][1], gae[q], r_l_override=rl22)["w0"] for u in us if u["ok"]]
                e, a, lo, hi = boot_fit(fx)
                row["w0_rl22"] = [e, lo, hi]
                cells.append(f"{e:.3f} [{lo:.3f}, {hi:.3f}]")
            else:
                cells.append("n/a")
            e, a, lo, hi = boot_fit({q: [m + l for m, l in zip(per[(arm, q)]["mul"], per[(arm, q)]["lmul"])] for q in qs})
            row["counts"] = [e, lo, hi]
            cells.append(f"{e:.3f} [{lo:.3f}, {hi:.3f}]")
            fits.append(row)
            print(f"| {arm} | {name} | {len(qs)} | " + " | ".join(cells) + " |")
    out["fits"] = fits

    # ---------------------------------------------------------------- 4. rho-neg model
    # committed pooled calibration and per-step structure (summary_supp.json, k = 22 neg row)
    supp = [json.loads(l) for l in open(SUMMARY.replace("summary.json", "summary_supp.json")) if l.strip()]
    cpool = [r for r in supp if r.get("kind") == "rho_calibration" and r["arm"] == "neg"][0]["pooled_steps_over_sqrt_ell"]
    r22 = [r for r in supp if r.get("kind") == "rho_size" and r["arm"] == "neg" and r["q"] == q22][0]
    steps22 = r22["meq"][0] / r22["meq_per_step"]
    step_over_gae = r22["meq_per_step"] / r22["gae"]  # priced work per step in units of one E0 addition
    unp_per_step = (r22["meq"][2] - r22["meq"][0]) / (r22["gae"] * steps22)
    print(f"\n## 4. DL / rho-neg (calibrated rho-neg model; c_pool = {cpool}, priced work/step = {step_over_gae:.4f} GAE, unpriced ops/step = {unp_per_step:.3f})\n")
    print("| q | k | rho-neg model M-eq w=0 | DL M-eq w=0 | DL/rho w=0 [band: c in 0.75..1.15 x seed range] | w=0.1 | w=1 | DL CPU s |")
    print("|--:|--:|--:|--:|:--|:--|:--|--:|")
    rows = []
    # rho-neg wall per step on this host is not measured at these sizes (each k = 23 run is ~40 min);
    # CPU model: per-step CPU from the committed k = 22 supplement (616 CPU-s per run on the M4 grid)
    for q in sizes:
        if ("DL", q) not in per or KOF[q] < 22:
            continue
        p = per[("DL", q)]
        sq = 2 ** (p["log2_ell"] / 2)
        steps = cpool * sq
        rho = {}
        for w, key in [(0.0, "w0"), (0.1, "w01"), (1.0, "w1")]:
            rho[key] = steps * (step_over_gae * p["gae"] + w * p["gae"] * unp_per_step)
        cells = []
        rr = {}
        for key in ["w0", "w01", "w1"]:
            m = stt.mean(p[key])
            lo = min(p[key]) / (rho[key] / cpool * 1.15)
            hi = max(p[key]) / (rho[key] / cpool * 0.75)
            rr[key] = [m / rho[key], lo, hi]
            cells.append(f"{m / rho[key]:.5f} [{lo:.5f}, {hi:.5f}]")
        rows.append({"q": q, "k": KOF[q], "rho_model": rho, "ratio": rr})
        print(f"| {q} | {KOF[q]} | {rho['w0']:.4e} | {stt.mean(p['w0']):.4e} | " + " | ".join(cells) + f" | {stt.mean(p['cpu']):.1f} |")
    out["rho_ratio"] = rows

    # projections to k = 25, 26 from the k = 22..24 DL fit (w = 0) and the k = 17..24 fit
    pj = []
    for name in ["k22-25", "k17-25", "k22-24", "k17-24"]:
        f = [x for x in fits if x["arm"] == "DL" and x["range"] == name]
        if not f:
            continue
        e, lo, hi = f[0]["w0"]
        ecpu = f[0]["cpu"][0]
        qtop = max(q for (a, q) in per if a == "DL")
        p24 = per.get(("DL", qtop))
        if not p24 or KOF[qtop] < int(name[-2:]):
            continue
        for q in [q2 for q2 in (33554383, 67108819) if q2 > qtop]:
            fac = q / qtop
            m = stt.mean(p24["w0"]) * fac ** e
            ell_bits = 3 * math.log2(q) - 2
            rho = cpool * 2 ** (ell_bits / 2) * step_over_gae * p24["gae"]
            cpu = stt.mean(p24["cpu"]) * fac ** ecpu
            rss = p24["rss_gib"] * fac  # memory grows about linearly in q (vertices, edges ~ q)
            pj.append({"from": name, "q": q, "k": KOF[q], "e": e, "dl_meq_w0": m, "dl_meq_lo": stt.mean(p24["w0"]) * fac ** lo,
                       "dl_meq_hi": stt.mean(p24["w0"]) * fac ** hi, "rho_model_w0": rho, "ratio_w0": m / rho, "cpu_s": cpu, "rss_gib": rss})
    print("\n## 5. Projections (from the largest measured DL size)\n")
    print("| fit | q | k | e used | DL M-eq w=0 [e band] | rho-neg model | DL/rho w=0 | DL unit CPU s | peak RSS GiB of the family unit (linear in q) |")
    print("|:--|--:|--:|--:|:--|--:|--:|--:|--:|")
    for x in pj:
        print(f"| {x['from']} | {x['q']} | {x['k']} | {x['e']:.3f} | {x['dl_meq_w0']:.3e} [{x['dl_meq_lo']:.3e}, {x['dl_meq_hi']:.3e}] | {x['rho_model_w0']:.3e} | {x['ratio_w0']:.5f} | {x['cpu_s']:.0f} | {x['rss_gib']:.1f} |")
    out["projections"] = pj
    with open(os.path.join(D, "results.json"), "w") as f:
        json.dump(out, f, indent=1)


if __name__ == "__main__":
    main()
