#!/usr/bin/env python3
"""VALIDATOR robustness analysis (not pre-registered) of icbest vs rhobest under the protocol's decision rule
(bootstrap as analyze.py: random.Random(1).choices, B = 4000), varying:
  native-work convention N (per arm), the reference set (rho2xks kappa above the grid; rho2x without retention;
  a MODELLED distinguished-point rho without the negation map), parameter selection (cross-fit), and the
  hash-probe count of the retention IC arms.
usage: sens_analysis.py <frozen study dir> <replay_all.json or -> <sens_runs.json[,more.json] or -> <out.json>"""
import json, sys, os, random, statistics as st, collections
FZ, RA, SK, OUT = sys.argv[1:5]
D = json.load(open(os.path.join(FZ, "results/holdout/runs.json")))
R = [dict(zip(D["fields"], x)) for x in D["rows"]]
XC = {}
if RA != "-":
    ra = json.load(open(RA))["results"]
    for x in ra: XC[x["row"]] = x["xc"]
for i, r in enumerate(R): r["_row"] = i; r["_xc"] = XC.get(i)
EXTRA = []
for skf in ([] if SK == "-" else SK.split(",")):
    for o in json.load(open(skf))["rows"]:
        assert "error" not in o and o["ok"] == o["targets"]
        EXTRA.append(dict(arm=o["arm"], pb=o["pb"], T=o["T"], idx=o["idx"], seed=o["seed"], param=o["param"], k=None, doff=None,
                          targets=o["targets"], ok=o["ok"], I=o["I"], M=o["M"], S=o["S"], H=o["H"], L=o["L"], N=o["N"], _row=None, _xc=None, _extra=True))
ICS = ["ic5", "ic5ks", "ic5w", "ic5rw"]; RHOS = ["rho2xks", "rhodpu"]; CI = [3, 10, 30]; HP = [0, 0.1, 1]
# MODEL (not a measurement): a distinguished-point rho without the negation map, built from each rhodpu kappa = 0 run:
# setup and verification unchanged, walk-phase field and hash-table work x sqrt(2) (walk on n points, not n/2 classes),
# one native operation per point (one hash of x; no canonicalisation) instead of two.  Upper bound on such a rho's
# strength at h = 1: rhodpu's look-ahead and fruitless-cycle overheads are kept, not removed.
MODEL = []
if os.environ.get("SENS_MODEL", "1") == "1":
    rows_full = [json.loads(l) for l in open(os.path.join(FZ, "results/holdout/rows.jsonl"))]
    for o in rows_full:
        if o["arm"] != "rhodpu" or o["param"] != 0.0: continue
        db, tg, vf = o["phases"]["db"], o["phases"]["targets"], o["phases"]["verify"]
        w = [b - a for a, b in zip(db, tg)]; v = [c - b for b, c in zip(tg, vf)]
        assert db[3] == 0
        q = 2 ** 0.5
        MODEL.append(dict(arm="rhodpn_model", pb=o["pb"], T=o["T"], idx=o["idx"], seed=o["seed"], param=0.0, doff=o["doff"], k=None,
                          targets=o["targets"], ok=o["ok"], I=db[0] + q * w[0] + v[0], M=db[1] + q * w[1] + v[1], S=db[2] + q * w[2] + v[2],
                          H=q * w[3], L=0, N=q * o["N"] / 2, _row=None, _xc=None, _model=True))
def boot(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    m = sorted(st.median(rnd.choices(xs, k=n)) for _ in range(B)); return m[int(.025 * B)], m[int(.975 * B) - 1]
def mkN(conv):
    def N(r):
        a = r["arm"]; n = r["N"]
        if a in ("rho2xks", "rho2x"):
            c = conv.get("rho2xks", 1.0)
            if c == "restarts": return r["_xc"][7] if r["_xc"] else None
            return c * n
        if a == "rhodpu": return conv.get("rhodpu", 1.0) * n
        if a in ICS: return conv.get("ic", 1.0) * n
        return n
    return N
PROBE = [False]
def cost(r, ci, h, Nf, rule="5a"):
    u = r["H"] + r["L"] + (Nf(r) if rule == "5a" else 0)
    if PROBE[0] and r["arm"] in ("ic5ks", "ic5rw"): u += r["_xc"][1]
    return ((ci * r["I"] + r["M"] + r["S"]) / (ci + 3) + h * u) / r["targets"]
def build(rows):
    cfg = collections.OrderedDict()
    for r in rows: cfg.setdefault((r["pb"], r["T"], r["idx"], r["seed"]), []).append(r)
    return cfg
CFG = build(R)
for e in EXTRA:
    CFG[(e["pb"], e["T"], e["idx"], e["seed"])].append(e)
for e in MODEL:
    CFG[(e["pb"], e["T"], e["idx"], e["seed"])].append(e)
cells = sorted({k[:2] for k in CFG})
def rtype(r): return (r["arm"], r["param"], r.get("doff"))
def analyse(conv, rule="5a", use_extra=False, crossfit=False, use_model=False):
    PROBE[0] = bool(conv.get("ic_probe")) and bool(XC)
    if conv.get("ic_probe") and not XC: return dict(skipped="needs replay_all.json")
    Nf = mkN(conv); dec = collections.Counter(); med = {h: [] for h in HP}; hi1 = 0; lo0 = 0; win = collections.Counter()
    per = []
    for ci in CI:
        for pb, T in cells:
            ks = [k for k in CFG if k[:2] == (pb, T)]
            ss = []
            for h in HP:
                def runs(k, side):
                    sd = list(side) + (["rhodpn_model"] if use_model and side is RHOS else []) + (["rho2x"] if use_extra and side is RHOS else [])
                    return [r for r in CFG[k] if r["arm"] in sd and (use_extra or not r.get("_extra"))]
                if not crossfit:
                    rat = [min(cost(r, ci, h, Nf, rule) for r in runs(k, ICS)) / min(cost(r, ci, h, Nf, rule) for r in runs(k, RHOS)) for k in ks]
                    if ci == 10:
                        for k in ks: win[(h, rtype(min(runs(k, RHOS), key=lambda r: cost(r, ci, h, Nf, rule))))] += 1
                else:
                    rat = []
                    for seed in (41, 42):
                        other = [k for k in ks if k[3] != seed]; mine = [k for k in ks if k[3] == seed]
                        def pick(side):
                            types = collections.defaultdict(list)
                            for k in other:
                                for r in runs(k, side): types[rtype(r)].append(cost(r, ci, h, Nf, rule))
                            return min(types, key=lambda t: st.median(types[t]))
                        ti, tr = pick(ICS), pick(RHOS)
                        for k in mine:
                            ci_ = [cost(r, ci, h, Nf, rule) for r in runs(k, ICS) if rtype(r) == ti][0]
                            cr_ = [cost(r, ci, h, Nf, rule) for r in runs(k, RHOS) if rtype(r) == tr][0]
                            rat.append(ci_ / cr_)
                lo, hi = boot(rat); m = st.median(rat)
                ss.append((h, m, lo, hi))
                if ci == 10: med[h].append(m)
                if h == 1 and hi < 1: hi1 += 1
                if h == 0 and lo > 1: lo0 += 1
                per.append(dict(cI=ci, bits=pb, T=T, h=h, median=m, lo=lo, hi=hi))
            d = "cheaper" if all(s[3] < 1 for s in ss) else ("dearer" if all(s[2] > 1 for s in ss) else "unranked")
            dec[d] += 1
    out = dict(decisions=dict(dec), I10M_median_range={str(h): [round(min(v), 4), round(max(v), 4)] for h, v in med.items()},
               h1_cells_hi_below_1_of_27=hi1, h0_cells_lo_above_1_of_27=lo0, per=per)
    if win:
        out["rho_winner_I10M"] = {str(h): {f"{t[0]}:{t[1]}:{t[2]}": c for (hh, t), c in sorted(win.items(), key=str) if hh == h} for h in HP}
    return out
V = collections.OrderedDict()
V["V0_frozen"] = ({}, "5a", False, False)
V["V1_rho2xks_index_free"] = ({"rho2xks": 0.0}, "5a", False, False)
if XC: V["V1b_rho2xks_restarts_only"] = ({"rho2xks": "restarts"}, "5a", False, False)
V["V2_rhodpu_1_per_point"] = ({"rhodpu": 0.5}, "5a", False, False)
V["V3_minimal_rho_native"] = ({"rho2xks": 0.0, "rhodpu": 0.5, "ic": 0.0}, "5a", False, False)
V["V3b_rho2xks0_rhodpu0"] = ({"rho2xks": 0.0, "rhodpu": 0.0, "ic": 0.0}, "5a", False, False)
V["V4_rhodpu_4_per_point"] = ({"rhodpu": 2.0}, "5a", False, False)
V["V5_rho2xks_2_per_step"] = ({"rho2xks": 2.0}, "5a", False, False)
V["V6_v4_rule_N_unpriced"] = ({}, "v4", False, False)
V["V7_crossfit_frozen"] = ({}, "5a", False, True)
V["V10_model_nonnegation_dp_rho_added"] = ({}, "5a", False, False, True)
V["V11_ic5rw_ic5ks_two_probes"] = ({"ic_probe": True}, "5a", False, False)
if EXTRA:
    V["V8_kappa_to_1.0_frozen"] = ({}, "5a", True, False)
    V["V9_kappa_to_1.0_minimal_native"] = ({"rho2xks": 0.0, "rhodpu": 0.5, "ic": 0.0}, "5a", True, False)
    V["V12_kappa_to_1.0_rho2xks_index_free"] = ({"rho2xks": 0.0}, "5a", True, False)
    V["V13_all_extra_and_model_frozen"] = ({}, "5a", True, False, True)
res = {}
for name, spec in V.items():
    conv, rule, ue, cf = spec[:4]; um = spec[4] if len(spec) > 4 else False
    res[name] = analyse(conv, rule, ue, cf, um)
    print(name, json.dumps({k: v for k, v in res[name].items() if k not in ("per", "rho_winner_I10M")}))
# native-work share and per-step accounting at I = 10M (medians over runs)
sh = collections.defaultdict(list)
for r in R:
    f = (10 * r["I"] + r["M"] + r["S"]) / 13
    sh[r["arm"]].append(dict(N_per_target=r["N"] / r["targets"], H_per_target=r["H"] / r["targets"], field_gae_per_target=f / r["targets"],
                             N_over_field=r["N"] / f, H_over_field=r["H"] / f))
res["accounting_medians_I10M"] = {a: {k: st.median([x[k] for x in v]) for k in v[0]} for a, v in sh.items()}
json.dump(res, open(OUT, "w"), indent=1)
