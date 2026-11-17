#!/usr/bin/env python3
"""Frozen analysis of the dedicated-pod run (PROTOCOL.md). Reads runpod/raw/out (and raw/rho), the
local records for the identity control, and prints Markdown; writes runpod/results_pod.json.
Reuses meq_unit / fit from ../analyze.py (unchanged)."""
import glob
import json
import math
import os
import random
import statistics as stt
import sys

D = os.path.dirname(os.path.abspath(__file__))
P = os.path.dirname(D)
sys.path.insert(0, P)
import analyze as A  # noqa: E402

OUT = os.path.join(D, "raw/out")
KOF = A.KOF
MAIN = [4194271, 8388571, 16777183, 33554383, 67108819]
LOW = [131071, 262111, 524287, 1048507, 2097091]
PH = A.PHASES


def jl(p):
    return A.jl(p)


def load_units():
    units, ratios, res = {}, {}, {}
    for p in sorted(glob.glob(os.path.join(OUT, "ic_*_D.jsonl"))):
        rj = p.replace(".jsonl", ".res.json")
        if not os.path.exists(rj):
            continue
        r0 = json.load(open(rj))
        for r in jl(p):
            if r["rec"] == "ratios":
                ratios[(r["q"], r["seed"])] = r
                res[(r["q"], r["seed"])] = r0
            elif r["rec"] == "ic":
                key = (r["arm"], r["q"], r["seed"])
                prev = units.get(key)
                if prev:
                    for k, v in r.items():
                        if any(k.startswith(ph + ".") for ph in PH) and isinstance(v, (int, float)):
                            r[k] = v + prev.get(k, 0)
                r["_attempts"] = (prev["_attempts"] if prev else 0) + 1
                units[key] = r
    return units, ratios, res


def load_cal():
    """{q: {round: [records]}}"""
    cal = {}
    for p in glob.glob(os.path.join(OUT, "cal*_*_*.jsonl")):
        rnd = int(os.path.basename(p)[3:].split("_")[0])
        for r in jl(p):
            if r["rec"] == "calib":
                cal.setdefault(r["q"], {}).setdefault(rnd, []).append(r)
    return cal


def consts(recs):
    return stt.median([r["r_inv"] for r in recs]), stt.median([r["r_lmul"] for r in recs])


def main():
    units, ratios, res = load_units()
    cal = load_cal()
    out = {}
    sizes = sorted({q for (_, q, _) in units if q in cal})
    gae_mul = {q: stt.median([r["gae_mul"] for (qq, s), r in ratios.items() if qq == q]) for q in sizes}
    gae_inv = {q: stt.median([r["gae_inv"] for (qq, s), r in ratios.items() if qq == q]) for q in sizes}

    def gae(q, r_inv):
        return gae_mul[q] + r_inv * gae_inv[q]

    pooled = {q: consts([r for rr in cal[q].values() for r in rr]) for q in sizes}
    rounds = sorted({r for q in sizes for r in cal[q]})

    # ---- calibration table
    print("## Calibration rounds (median of 3 records per size and round)\n")
    print("| k | q | " + " | ".join(f"round {r} r_inv / r_lmul" for r in rounds) + " | pooled (all records) | n |")
    print("|--:|--:|" + "--|" * len(rounds) + "--|--:|")
    caltab = {}
    for q in sizes:
        cells = []
        for r in rounds:
            if r in cal[q]:
                a, b = consts(cal[q][r])
                cells.append(f"{a:.2f} / {b:.3f}")
                caltab.setdefault(q, {})[r] = [a, b]
            else:
                cells.append("-")
        n = sum(len(v) for v in cal[q].values())
        print(f"| {KOF[q]} | {q} | " + " | ".join(cells) + f" | {pooled[q][0]:.2f} / {pooled[q][1]:.3f} | {n} |")
    out["calibration"] = {str(q): {"rounds": caltab.get(q), "pooled": pooled[q]} for q in sizes}

    # ---- per size
    def costs(arm, q, r_inv, r_l, key="w0"):
        g = gae(q, r_inv)
        return [A.meq_unit(units[(arm, q, s)], r_inv, r_l, g)[key] for s in (1, 2, 3)
                if (arm, q, s) in units and units[(arm, q, s)]["ok"] and units[(arm, q, s)]["k_found"] == units[(arm, q, s)]["k"]]

    print("\n## Per size (pooled calibration)\n")
    print("| arm | k | ok/units | attempts | M-eq w=0 (mean) | w=0.1 | w=1 | arm CPU s (mean) | unit CPU s (per seed) | unit wall s | peak RSS GiB | log2 ell |")
    print("|:--|--:|:--|:--|--:|--:|--:|--:|:--|:--|--:|--:|")
    per = {}
    for arm in A.ARMS:
        for q in sizes:
            us = [(s, units[(arm, q, s)]) for s in (1, 2, 3) if (arm, q, s) in units]
            if not us:
                continue
            ok = [u for _, u in us if u["ok"] and u["k_found"] == u["k"]]
            ri, rl = pooled[q]
            g = gae(q, ri)
            m = [A.meq_unit(u, ri, rl, g) for u in ok]
            ucpu = [res[(q, s)]["user_s"] + res[(q, s)]["sys_s"] for s, _ in us]
            uwall = [res[(q, s)]["wall_s"] for s, _ in us]
            rss = max(res[(q, s)]["maxrss_kib"] for s, _ in us) / 2**20
            ell = max(u["ell"] for _, u in us)
            per[(arm, q)] = {"ok": len(ok), "units": len(us), "w0": [x["w0"] for x in m], "w01": [x["w01"] for x in m],
                             "w1": [x["w1"] for x in m], "cpu": [x["cpu"] for x in m], "unit_cpu": ucpu, "unit_wall": uwall,
                             "rss_gib": rss, "log2_ell": math.log2(ell), "attempts": [u["_attempts"] for _, u in us]}
            p = per[(arm, q)]
            if not m:
                print(f"| {arm} | {KOF[q]} | 0/{len(us)} | | | | | | | | | |")
                continue
            print(f"| {arm} | {KOF[q]} | {len(ok)}/{len(us)} | {p['attempts']} | {stt.mean(p['w0']):.4e} | {stt.mean(p['w01']):.4e} | {stt.mean(p['w1']):.4e} | "
                  f"{stt.mean(p['cpu']):.1f} | {', '.join(f'{x:.0f}' for x in ucpu)} | {', '.join(f'{x:.0f}' for x in uwall)} | {rss:.2f} | {p['log2_ell']:.1f} |")
    out["per_size"] = {f"{a}:{q}": v for (a, q), v in per.items()}

    # ---- exponents
    def exp_fit(arm, qs, key="w0", B=4000, seed=7):
        qs = [q for q in qs if q in sizes and per.get((arm, q), {}).get("ok")]
        if len(qs) < 2:
            return None
        xs = [math.log(q) for q in qs]
        e_pt = A.fit(xs, [math.log(stt.mean(costs(arm, q, *pooled[q], key))) for q in qs])[0]
        e_r = []
        for r in rounds:
            if all(r in caltab.get(q, {}) for q in qs):
                e_r.append(A.fit(xs, [math.log(stt.mean(costs(arm, q, *caltab[q][r], key))) for q in qs])[0])
        rng = random.Random(seed)
        # precompute unit costs under every round
        cr = {q: {r: costs(arm, q, *caltab[q][r], key) for r in caltab[q]} for q in qs}
        es, es_u = [], []
        for _ in range(B):
            ys = []
            for q in qs:
                rr = rng.choice(sorted(cr[q]))
                v = cr[q][rr]
                idx = [rng.randrange(len(v)) for _ in v]
                ys.append(math.log(stt.mean([v[i] for i in idx])))
            es.append(A.fit(xs, ys)[0])
        es.sort()
        # unit-only bootstrap with pooled calibration (for comparison)
        rng2 = random.Random(seed)
        cp = {q: costs(arm, q, *pooled[q], key) for q in qs}
        for _ in range(B):
            yu = [math.log(stt.mean([rng2.choice(cp[q]) for _ in cp[q]])) for q in qs]
            es_u.append(A.fit(xs, yu)[0])
        es_u.sort()
        return {"sizes": len(qs), "e": e_pt, "lo": es[int(0.025 * B)], "hi": es[int(0.975 * B) - 1],
                "lo_units_only": es_u[int(0.025 * B)], "hi_units_only": es_u[int(0.975 * B) - 1],
                "e_rounds": e_r}

    print("\n## Exponents (ln mean cost on ln q). Primary interval: units x per-size calibration round, B = 4000\n")
    print("| arm | range | measure | sizes | e (pooled cal) | 95% (units + calibration) | 95% (units only) | e per calibration round |")
    print("|:--|:--|:--|--:|--:|:--|:--|:--|")
    fits = []
    for arm in A.ARMS:
        for name, qs in [("k22-26", MAIN), ("k22-25", MAIN[:4]), ("k17-26", LOW + MAIN), ("k17-22", LOW + MAIN[:1])]:
            for key in (["w0", "w1"] if arm == "DL" else ["w0"]):
                f = exp_fit(arm, qs, key)
                if not f:
                    continue
                f.update({"arm": arm, "range": name, "measure": key})
                fits.append(f)
                print(f"| {arm} | {name} | M-eq {key} | {f['sizes']} | {f['e']:.4f} | [{f['lo']:.4f}, {f['hi']:.4f}] | [{f['lo_units_only']:.4f}, {f['hi_units_only']:.4f}] | "
                      + ", ".join(f"{x:.4f}" for x in f["e_rounds"]) + " |")
    # CPU exponent (DL arm CPU)
    for name, qs in [("k22-26", MAIN), ("k17-26", LOW + MAIN)]:
        qs = [q for q in qs if per.get(("DL", q), {}).get("ok")]
        if len(qs) >= 2:
            e, a, lo, hi = A.boot_fit({q: per[("DL", q)]["cpu"] for q in qs})
            fits.append({"arm": "DL", "range": name, "measure": "cpu", "e": e, "lo": lo, "hi": hi, "sizes": len(qs)})
            print(f"| DL | {name} | CPU s | {len(qs)} | {e:.4f} | (units only) [{lo:.4f}, {hi:.4f}] | | |")
    out["fits"] = fits

    # ---- rho-neg model
    supp = [json.loads(l) for l in open(A.SUMMARY.replace("summary.json", "summary_supp.json")) if l.strip()]
    cpool = [r for r in supp if r.get("kind") == "rho_calibration" and r["arm"] == "neg"][0]["pooled_steps_over_sqrt_ell"]
    r22 = [r for r in supp if r.get("kind") == "rho_size" and r["arm"] == "neg" and r["q"] == 4194271][0]
    steps22 = r22["meq"][0] / r22["meq_per_step"]
    sog = r22["meq_per_step"] / r22["gae"]
    ups = (r22["meq"][2] - r22["meq"][0]) / (r22["gae"] * steps22)
    print(f"\n## DL / rho-neg model (c_pool = {cpool}, priced {sog:.4f} GAE/step, unpriced {ups:.3f} ops/step)\n")
    print("| k | DL/rho w=0 [band] | w=0.1 [band] | w=1 [band] |")
    print("|--:|:--|:--|:--|")
    rows = []
    for q in sizes:
        if KOF[q] < 22 or not per.get(("DL", q), {}).get("ok"):
            continue
        sq = 2 ** (per[("DL", q)]["log2_ell"] / 2)
        row = {"q": q, "k": KOF[q]}
        cells = []
        for w, key in [(0.0, "w0"), (0.1, "w01"), (1.0, "w1")]:
            def rho(ri):
                g = gae(q, ri)
                return sq * (sog * g + w * g * ups)  # per unit of c
            ri, rl = pooled[q]
            mid = stt.mean(costs("DL", q, ri, rl, key)) / (cpool * rho(ri))
            los, his = [], []
            for r in caltab[q]:
                ri_r, rl_r = caltab[q][r]
                v = costs("DL", q, ri_r, rl_r, key)
                los.append(min(v) / (1.15 * rho(ri_r)))
                his.append(max(v) / (0.75 * rho(ri_r)))
            row[key] = [mid, min(los), max(his)]
            cells.append(f"{mid:.5f} [{min(los):.5f}, {max(his):.5f}]")
        rows.append(row)
        print(f"| {KOF[q]} | " + " | ".join(cells) + " |")
    out["rho_model"] = {"c_pool": cpool, "step_over_gae": sog, "unpriced_per_step": ups, "rows": rows}

    # ---- measured rho-neg (Addendum A), cross-check of c only
    rr = []
    for p in glob.glob(os.path.join(D, "raw/rho/rho_*.jsonl")):
        for r in jl(p):
            if r.get("rec") == "rho":
                rr.append(r)
    if rr:
        print("\n## Measured negation rho (Addendum A; cross-check of the model constant only)\n")
        print("| k | runs ok/total | steps/sqrt(ell) per run | mean [min, max] | CPU s per run (mean) |")
        print("|--:|:--|:--|:--|--:|")
        mr = {}
        for q in sorted({r["q"] for r in rr}):
            v = [r for r in rr if r["q"] == q]
            c = [r["steps"] / math.sqrt(r["ell"]) for r in v if r["ok"]]
            mr[q] = {"runs": len(v), "ok": len(c), "c": c, "cpu": [r["cpu"] for r in v]}
            print(f"| {KOF[q]} | {len(c)}/{len(v)} | {', '.join(f'{x:.3f}' for x in c)} | {stt.mean(c):.3f} [{min(c):.3f}, {max(c):.3f}] | {stt.mean(mr[q]['cpu']):.0f} |")
        allc = [x for v in mr.values() for x in v["c"]]
        print(f"\npooled measured c = {stt.mean(allc):.3f} over {len(allc)} runs (model 0.9175; admissibility [0.75, 1.15])")
        out["rho_measured"] = {str(q): v for q, v in mr.items()}
        out["rho_measured_pooled_c"] = stt.mean(allc)

    # ---- identity control against local records (non-timing fields)
    print("\n## Identity control: pod records vs local records (non-timing fields)\n")
    TK = {"r_inv", "r_lmul", "load1", "search_s", "s.controls_wall", "s.controls_cpu", "attempt_cpu"}

    def strip(rec):
        return {k: v for k, v in rec.items() if k not in TK and not k.endswith((".wall", ".cpu"))}
    idn = []
    for q in sizes:
        for s in (1, 2, 3):
            pod = os.path.join(OUT, f"ic_{q}_{s}_D.jsonl")
            loc = [os.path.join(P, f"raw/wide/c_ic_{q}_{s}_D.jsonl"), os.path.join(P, f"raw/v2w2/k25/c_ic_{q}_{s}_D.jsonl")]
            loc = [x for x in loc if os.path.exists(x)]
            if not os.path.exists(pod) or not loc:
                continue
            a = [strip(r) for r in jl(pod)]
            b = [strip(r) for r in jl(loc[0])]
            nf = sum(len(r) for r in a)
            same = a == b
            idn.append({"q": q, "seed": s, "fields": nf, "identical": same})
    nid = sum(1 for x in idn if x["identical"])
    print(f"{nid}/{len(idn)} units identical; {sum(x['fields'] for x in idn)} non-timing fields compared")
    for x in idn:
        if not x["identical"]:
            print(f"  DIFFER: q={x['q']} seed={x['seed']}")
    out["identity"] = idn
    with open(os.path.join(D, "results_pod.json"), "w") as f:
        json.dump(out, f, indent=1)


if __name__ == "__main__":
    main()
