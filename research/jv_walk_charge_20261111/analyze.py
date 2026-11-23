#!/usr/bin/env python3
"""Charge the walk into the weak class (PROTOCOL.md). Reads runs/p*_s*_m*.json, the validated arm-U records and
the validated per-prime rho records of research/jv_sieve_line_vs_rho_20261103, and writes analysis.json."""
import glob, json, math, random, sys
V = "../jv_sieve_line_vs_rho_20261103"
C = 331.0
NAT6 = ("canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp")
P = (1009, 1511, 1823); SEEDS = range(17, 25); MS = (8, 32, 128, 512)
rho = {p: [] for p in P}
for f in glob.glob(f"{V}/results/rho/rhostrong_p*_s*_run*.json"):
    for r in json.load(open(f)):
        sl = math.sqrt(r["l"])
        rho[r["p"]].append({"U2": r["s_tuned"], "natA": (sum(r["ops"][k] for k in NAT6) + r["ops"]["fp_adds"]) / sl})
U = {}
for p in P:
    for r in json.load(open(f"{V}/results/cover/U_{p}.json")):
        sl = math.sqrt(r["l"]); fb = r.get("fb_line") or {}
        natA = (r["sieve_adds"] + r["sieve_lookups"] + r["enum_adds"] + fb.get("adds", 0) + fb.get("lookups", 0)
                + (r["total_muls"] - r["sieve_muls"] - r["enum_muls"])) / sl
        U[(p, r["seed"])] = {"S": r["total_muls"] / (C * sl), "natA": natA, "l": r["l"]}
runs = {}
for p in P:
    for s in SEEDS:
        for m in MS:
            d = json.load(open(f"runs/p{p}_s{s}_m{m}.json")); d = d[0] if isinstance(d, list) else d
            assert d["l"] == U[(p, s)]["l"], (p, s)
            reach = d["walk_muls"] + d["transport_muls"] + d["model_muls"]
            ok = d["stage"] == "ok" and d["walk_found"] and d["route_solved"] and d["route_correct"] and d["verified_on_challenge"] and d["challenge_non_weak"]
            runs[(p, s, m)] = {"ok": ok, "reach_S": reach / (C * math.sqrt(d["l"])), "reach_muls": reach,
                               "walk_muls": d["walk_muls"], "path": [d["path_two"], d["path_odd"]], "stage": d["stage"]}
mean = lambda xs: sum(xs) / len(xs)
gm = lambda xs: math.exp(mean([math.log(x) for x in xs]))
def ratio(ps, ss, refs, m, w):
    return gm([(U[(p, s)]["S"] + runs[(p, s, m)]["reach_S"] + w * U[(p, s)]["natA"]) / (refs[p]["U2"] + w * refs[p]["natA"])
               for p in ps for s in ss[p]])
ref0 = {p: {k: mean([x[k] for x in rho[p]]) for k in ("U2", "natA")} for p in P}
rng = random.Random(20261111); B = 5000
out = {"failures": [f"p{p}_s{s}_m{m}:{r['stage']}" for (p, s, m), r in runs.items() if not r["ok"]], "by_M": {}}
for m in MS:
    row = {"reach_share_of_S_U": {}, "path_len_max": max(sum(runs[(p, s, m)]["path"]) for p in P for s in SEEDS)}
    for p in P:
        sh = sorted(runs[(p, s, m)]["reach_S"] / U[(p, s)]["S"] for s in SEEDS)
        row["reach_share_of_S_U"][str(p)] = {"median": (sh[3] + sh[4]) / 2, "max": sh[-1]}
    for scope in ("pooled",) + P:
        ps = P if scope == "pooled" else (scope,)
        allss = {p: list(SEEDS) for p in P}
        cells = {}
        for w in (0, 0.1, 1):
            pt = ratio(ps, allss, ref0, m, w)
            uncharged = gm([(U[(p, s)]["S"] + w * U[(p, s)]["natA"]) / (ref0[p]["U2"] + w * ref0[p]["natA"]) for p in ps for s in SEEDS])
            bs = []
            r2 = random.Random(1000 + m + (0 if scope == "pooled" else scope) + int(w * 10))
            for _ in range(B):
                ss = {p: [r2.choice(list(SEEDS)) for _ in SEEDS] for p in P}
                refs = {}
                for p in P:
                    rr = [r2.choice(rho[p]) for _ in rho[p]]
                    refs[p] = {k: mean([x[k] for x in rr]) for k in ("U2", "natA")}
                bs.append(ratio(ps, ss, refs, m, w))
            bs.sort()
            cells[str(w)] = {"point": pt, "lo": bs[int(0.025 * B)], "hi": bs[int(0.975 * B) - 1], "uncharged_point": uncharged}
        row[str(scope)] = cells
    out["by_M"][str(m)] = row
pool = out["by_M"]["512"]["pooled"]
hi_all = max(c["hi"] for c in pool.values()); lo_any = max(c["lo"] for c in pool.values())
out["verdict"] = ("walk charged, still below rho" if hi_all < 1 else "walk erases the gap" if lo_any > 1 else "not resolved") \
                 + (f" ({len([f for f in out['failures'] if '_m512' in f])} failures at M = 512)" if any('_m512' in f for f in out["failures"]) else "")
json.dump(out, open("analysis.json", "w"), indent=1)
print(out["verdict"]); print("failures:", out["failures"])
for m in MS:
    r = out["by_M"][str(m)]; c = r["pooled"]
    print(f"M={m}: reach share median by p", {p: round(v['median'], 4) for p, v in r['reach_share_of_S_U'].items()},
          "max", {p: round(v['max'], 3) for p, v in r['reach_share_of_S_U'].items()}, "| pooled charged/rho w=0,0.1,1:",
          [f"{c[w]['point']:.4f} [{c[w]['lo']:.4f},{c[w]['hi']:.4f}] (uncharged {c[w]['uncharged_point']:.4f})" for w in ('0', '0.1', '1')])
