#!/usr/bin/env python3
"""analysis/summary.json for research/postgb_linear_stage_20261026: every
PROTOCOL.md measure and prediction, computed from the run JSONs alone
(host B, amendment 2: runs/R2-L*.json corpus, runs/R2-A*.json portable
ablation, runs/R2-D*.json jv_cover DLP; host A: the first runs/L1-L4.json, as
a host-dependence check only). No number is typed in."""
import json
import math
import random
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RUNS = ROOT / "runs"
FAMILIES = ["jv-nagao", "gaudry-s4", "gaudry-d4", "gaudry-s5", "dense"]


def load(names):
    rows, meta = [], {}
    for n in names:
        d = json.loads((RUNS / f"{n}.json").read_text())
        meta[n] = {k: d[k] for k in d if k not in ("rows",)}
        for r in d["rows"]:
            r["card"] = n
            rows.append(r)
    return rows, meta


def arm(r, a):
    for x in r["arms"]:
        if x["arm"] == a:
            return x
    return None


def boot_geomean(logs, seed=20261027, b=4000):
    rng = random.Random(seed)
    n = len(logs)
    m = sum(logs) / n
    bs = sorted(sum(rng.choice(logs) for _ in range(n)) / n for _ in range(b))
    return math.exp(m), [math.exp(bs[int(0.025 * b)]), math.exp(bs[int(0.975 * b) - 1])]


def spearman(xs, ys):
    def rank(v):
        o = sorted(range(len(v)), key=lambda i: v[i])
        r = [0.0] * len(v)
        i = 0
        while i < len(o):
            j = i
            while j + 1 < len(o) and v[o[j + 1]] == v[o[i]]:
                j += 1
            for k in range(i, j + 1):
                r[o[k]] = (i + j) / 2
            i = j + 1
        return r
    rx, ry = rank(xs), rank(ys)
    mx, my = sum(rx) / len(rx), sum(ry) / len(ry)
    num = sum((a - mx) * (b - my) for a, b in zip(rx, ry))
    den = math.sqrt(sum((a - mx) ** 2 for a in rx) * sum((b - my) ** 2 for b in ry))
    return num / den


def family_table(rows, num, den="fast"):
    out = {}
    for fam in FAMILIES:
        rs = [r for r in rows if r["family"] == fam and arm(r, num) and arm(r, den)]
        if not rs:
            continue
        logs = [math.log(arm(r, num)["median_ms"] / arm(r, den)["median_ms"]) for r in rs]
        g, ci = boot_geomean(logs)
        out[fam] = {
            "systems": len(rs),
            "delta": [min(r["delta"] for r in rs), max(r["delta"] for r in rs)],
            "n": sorted({r["n"] for r in rs}),
            "p": sorted({r["p"] for r in rs}),
            "geomean": g,
            "ci95": ci,
            "min": math.exp(min(logs)),
            "max": math.exp(max(logs)),
            "median_ms": {a: sorted(arm(r, a)["median_ms"] for r in rs)[len(rs) // 2]
                          for a in (num, den)},
            "counted_ratio": [arm(r, num)["muls"] / max(1, arm(r, den)["muls"]) for r in rs],
        }
        cr = out[fam]["counted_ratio"]
        out[fam]["counted_ratio"] = {"min": min(cr), "max": max(cr),
                                     "geomean": math.exp(sum(map(math.log, cr)) / len(cr))}
    return out


def main():
    corpus, meta = load([f"R2-L{i}" for i in range(1, 7)])
    abl, meta_a = load(["R2-A1", "R2-A2"])
    host_a, meta_ha = load(["L1", "L2", "L3", "L4"])
    out = {"schema": "postgb_linear_stage.summary/v1", "host": "B (amendment 2)",
           "meta": meta, "meta_ablation": meta_a, "meta_host_a": meta_ha,
           "skipped": {n: m.get("skipped", []) for n, m in {**meta, **meta_a}.items()}}
    gates = {
        "systems": len(corpus),
        "disagree": [f"{r['card']}:{r['family']}:{r['label']}" for r in corpus if not r["agree"]],
        "planted_lost": [f"{r['card']}:{r['label']}" for r in corpus if r["planted_found"] is False],
        "fast_gave_up": [f"{r['card']}:{r['label']}:{arm(r, 'fast')['gave_up']}" for r in corpus
                         if arm(r, "fast")["gave_up"]],
        "sparse_fell_back": [f"{r['card']}:{r['label']}" for r in corpus if arm(r, "sparse")["fallback"]],
        "ablation_disagree": [f"{r['card']}:{r['label']}" for r in abl if not r["agree"]],
    }
    gates["pass"] = not gates["disagree"] and not gates["planted_lost"] and not gates["ablation_disagree"]
    out["gates"] = gates
    out["sparse_over_fast"] = family_table(corpus, "sparse")
    out["dense_over_fast"] = family_table(corpus, "dense")
    out["ablation_sparse_over_portable_fast"] = family_table(abl, "sparse")
    out["host_a_sparse_over_fast"] = family_table(host_a, "sparse")
    out["host_a_dense_over_fast"] = family_table(host_a, "dense")
    # fast arm with vs without the explicit SIMD kernels, same systems (host B)
    simd = {(r["family"], r["label"]): arm(r, "fast")["median_ms"] for r in corpus}
    pairs = [(r["family"], simd[(r["family"], r["label"])], arm(r, "fast")["median_ms"])
             for r in abl if (r["family"], r["label"]) in simd]
    out["portable_over_simd_fast"] = {
        fam: math.exp(sum(math.log(b / a) for f, a, b in pairs if f == fam)
                      / max(1, sum(1 for f, *_ in pairs if f == fam)))
        for fam in FAMILIES if any(f == fam for f, *_ in pairs)}
    dense_rows = [r for r in corpus if r["family"] == "dense"]
    rho = spearman([r["delta"] for r in dense_rows],
                   [arm(r, "sparse")["median_ms"] / arm(r, "fast")["median_ms"] for r in dense_rows])
    # per-system table (for the README and the figures)
    out["systems"] = [{
        "card": r["card"], "family": r["family"], "label": r["label"], "n": r["n"], "p": r["p"],
        "delta": r["delta"], "f4_ms": r["f4_ms"], "sols": arm(r, "fast")["sols"],
        "agree": r["agree"], "planted_found": r["planted_found"],
        "ms": {a["arm"]: a["median_ms"] for a in r["arms"]},
        "muls": {a["arm"]: a["muls"] for a in r["arms"]},
        "fast_phases_ms": {k: arm(r, "fast")["fast"][k] for k in
                           ("nf_ms", "krylov_ms", "bm_ms", "roots_ms", "readout_ms")}
        if arm(r, "fast").get("fast") else None,
    } for r in corpus]
    # jv_cover DLP, end to end
    d1 = json.loads((RUNS / "R2-D1.json").read_text())
    d2 = json.loads((RUNS / "R2-D2.json").read_text())

    def dlp_rows(d):
        rs = d if isinstance(d, list) else d.get("rows", d)
        rs = [r.get("Dlp", r) if isinstance(r, dict) else r for r in rs]
        return {(r["p"], r["seed"]): r for r in rs if isinstance(r, dict) and "rho" in r}
    a, b = dlp_rows(d1), dlp_rows(d2)
    dlp = []
    for k in sorted(set(a) & set(b)):
        x, y = a[k], b[k]
        rho_s = [z["s"] for z in x["rho"]] + [z["s"] for z in y["rho"]]
        dlp.append({
            "p": k[0], "seed": k[1],
            "correct": [x["correct"], y["correct"]],
            "s": [x["s"], y["s"]],
            "s_ratio_sparse_over_fast": x["s"] / y["s"],
            "lin_muls": [x["lin_muls"], y["lin_muls"]],
            "lin_ms": [x.get("lin_ms"), y.get("lin_ms")],
            "f4_ms": [x.get("f4_ms"), y.get("f4_ms")],
            "lin_ms_ratio_sparse_over_fast": (x["lin_ms"] / y["lin_ms"]) if y.get("lin_ms") else None,
            "wall_ms": [x["wall_ms"], y["wall_ms"]],
            "rho_s_mean": [x["rho_s_mean"], y["rho_s_mean"]],
            "s_over_rho": [x["s_over_rho"], y["s_over_rho"]],
            "relations": [x["relations"], y["relations"]],
            "rho_all_correct": all(z["correct"] for z in x["rho"] + y["rho"]),
            "rho_pooled_mean_s": sum(rho_s) / len(rho_s),
        })
    out["dlp"] = dlp
    # predictions
    so = out["sparse_over_fast"]
    P = {}
    P["P1"] = {"held": all(v["geomean"] >= 1.5 and v["ci95"][0] > 1 for v in so.values()),
               "by_family": {k: [v["geomean"], v["ci95"]] for k, v in so.items()}}
    P["P2"] = {"held": rho > 0, "spearman_rho": rho, "systems": len(dense_rows)}
    gu = len(gates["fast_gave_up"]) / max(1, len(corpus))
    P["P3"] = {"held": gu <= 0.05, "gave_up_fraction": gu}
    counted = [s["muls"]["sparse"] / max(1, s["muls"]["fast"]) for s in out["systems"]]
    P["P4"] = {
        "logs_correct": all(all(r["correct"]) for r in dlp),
        "counted_within_25pct_every_system": all(0.75 <= c <= 1.25 for c in counted),
        "counted_ratio_range": [min(counted), max(counted)],
        "s_within_10pct": all(abs(r["s_ratio_sparse_over_fast"] - 1) <= 0.10 for r in dlp),
        "lin_ms_lower_every_run": all(r["lin_ms_ratio_sparse_over_fast"] and r["lin_ms_ratio_sparse_over_fast"] > 1
                                      for r in dlp),
    }
    P["P4"]["held"] = (P["P4"]["logs_correct"] and P["P4"]["counted_within_25pct_every_system"]
                       and P["P4"]["s_within_10pct"] and P["P4"]["lin_ms_lower_every_run"])
    ab = out["ablation_sparse_over_portable_fast"]
    P["P5"] = {"held": all(v["geomean"] >= 1.2 for v in ab.values()),
               "by_family": {k: v["geomean"] for k, v in ab.items()}}
    out["predictions"] = P
    (ROOT / "analysis").mkdir(exist_ok=True)
    (ROOT / "analysis" / "summary.json").write_text(json.dumps(out, indent=1) + "\n")
    for k, v in P.items():
        print(k, v["held"])
    print("gates", gates["pass"], {k: len(v) for k, v in gates.items() if isinstance(v, list)})


if __name__ == "__main__":
    main()
