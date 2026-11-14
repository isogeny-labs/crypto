#!/usr/bin/env python3
"""Summarise raw.jsonl -> summary.json + summary.md.  [pcc:unverified]"""
import json, math, random, sys, os
from collections import defaultdict
OUT = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(OUT, os.environ.get("RAW","raw.jsonl")))]
PRICINGS = ["fermat", "batch"]
WS = [0.0, 0.1, 1.0]

def gae(r, pricing, w):
    if r["kind"] == "ic":
        c = r["counts"]; tot = {k: sum(v) for k, v in c.items()}
        la = r["la_ops_det"]
    else:
        tot = r["counts"]; la = 0
    cI = r["cI"] if pricing == "fermat" else 3
    meq = tot["M"] + cI*tot["I"] + r["cL"]*tot["L"] + r["cR"]*tot["R"] + la
    unit = cI + 3
    return meq/unit + w*tot["H"]

def phase_gae(r, pricing):
    cI = r["cI"] if pricing == "fermat" else 3; unit = cI + 3
    out = {}
    for ph, idx in (("base", 0), ("table", 1), ("decomp", 2), ("walk", 3)):
        c = {k: v[idx] for k, v in r["counts"].items()}
        out[ph] = (c["M"] + cI*c["I"] + r["cL"]*c["L"] + r["cR"]*c["R"])/unit
    out["la"] = r["la_ops_det"]/unit
    return out

def mean(xs): return sum(xs)/len(xs) if xs else float("nan")

def boot_ratio(A, B, f, nb=2000, seed=1):
    """A,B: dict curve->list of rows.  ratio mean f(A)/mean f(B), stratified bootstrap."""
    rng = random.Random(seed)
    def stat(sa, sb):
        return mean([f(x) for x in sa]) / mean([f(x) for x in sb])
    allA = [x for v in A.values() for x in v]; allB = [x for v in B.values() for x in v]
    pt = stat(allA, allB); bs = []
    for _ in range(nb):
        sa = [x for k, v in A.items() for x in rng.choices(v, k=len(v))]
        sb = [x for k, v in B.items() for x in rng.choices(v, k=len(v))]
        bs.append(stat(sa, sb))
    bs.sort()
    return pt, bs[int(0.025*nb)], bs[int(0.975*nb)-1]

ic = defaultdict(lambda: defaultdict(list))   # (bits,oracle,arm,Fgrid) -> curve -> rows
rho = defaultdict(lambda: defaultdict(list))  # (bits,arm) -> curve -> rows
fails = defaultdict(int); wrong = defaultdict(int)
for r in rows:
    if r["kind"] == "ic":
        F0 = (2*r["n"]) ** (1/3)
        g = min([0.5, 0.71, 1.0, 1.41, 2.0], key=lambda g: abs(max(12, int(round(F0*g/6))*6) - r["Fgrid"]))
        if r["oracle"] == "semaev": g = 1.0
        key = (r["bits"], r["oracle"], r["arm"], g)
        if not r.get("ok"):
            fails[key] += 1
            if "counts" in r: wrong[key] += 1
            continue
        ic[key][r["curve"]].append(r)
    else:
        key = (r["bits"], r["arm"])
        if not r["ok"]: wrong[key] += 1; continue
        rho[key][r["curve"]].append(r)

flat = lambda d: [x for v in d.values() for x in v]
S = {"ic_cells": [], "rho": [], "ratios": [], "fails": {str(k): v for k, v in fails.items()},
     "wrong": {str(k): v for k, v in wrong.items()}}
bits_list = sorted({k[0] for k in ic})
md = []
md.append("## IC cells (mean over curves x seeds)\n")
md.append("| bits | oracle | arm | F | cols | n runs | rels(det) | trials(det) | full-rank rels | table entries | GAE fermat w=0 | GAE batch w=0 | GAE batch w=1 | phase share batch (table/decomp/walk/base/la) |")
md.append("|--:|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|")
for key in sorted(ic):
    bits, orc, arm, F = key; R = flat(ic[key])
    fr = [x["full_rels"] for x in R if x.get("full_rels")]
    ph = {k: mean([phase_gae(x, "batch")[k] for x in R]) for k in ("table", "decomp", "walk", "base", "la")}
    tot = sum(ph.values())
    cell = dict(bits=bits, oracle=orc, arm=arm, Fgrid=F, F=R[0]["F"], ncols=mean([x['ncols'] for x in R]), runs=len(R),
                rels_det=mean([x["rels_det"] for x in R]), trials_det=mean([x["trials_det"] for x in R]),
                full_rels=(mean(fr) if fr else None), full_n=len(fr),
                table_entries=mean([x["table_entries"] for x in R]),
                gae={f"{pr}_w{w}": mean([gae(x, pr, w) for x in R]) for pr in PRICINGS for w in WS},
                phase_batch=ph)
    S["ic_cells"].append(cell)
    md.append(f"| {bits} | {orc} | {arm} | g={F} F~{mean([x['F'] for x in R]):.0f} | {cell['ncols']:.0f} | {len(R)} | {cell['rels_det']:.1f} | {cell['trials_det']:.0f} | "
              f"{(f'{cell['full_rels']:.0f} ({len(fr)})' if fr else '-')} | {cell['table_entries']:.0f} | {cell['gae']['fermat_w0.0']:.0f} | "
              f"{cell['gae']['batch_w0.0']:.0f} | {cell['gae']['batch_w1.0']:.0f} | "
              + "/".join(f"{ph[k]/tot:.2f}" for k in ("table", "decomp", "walk", "base", "la")) + " |")

md.append("\n## Rho arms\n")
md.append("| bits | arm | walks | mean steps | expected sqrt(pi n/(2k)) | fruitless | GAE fermat w=0 | GAE batch w=0 | GAE batch w=1 |")
md.append("|--:|:--|--:|--:|--:|--:|--:|--:|--:|")
for key in sorted(rho):
    bits, arm = key; R = flat(rho[key]); n = mean([x["n"] for x in R])
    k = {"plain": 1, "neg": 2, "aut6": 6}[arm]
    cell = dict(bits=bits, arm=arm, walks=len(R), steps=mean([x["steps"] for x in R]),
                expected=math.sqrt(math.pi*n/(2*k)), fruitless=mean([x["fruitless"] for x in R]),
                gae={f"{pr}_w{w}": mean([gae(x, pr, w) for x in R]) for pr in PRICINGS for w in WS})
    S["rho"].append(cell)
    md.append(f"| {bits} | {arm} | {len(R)} | {cell['steps']:.0f} | {cell['expected']:.0f} | {cell['fruitless']:.2f} | "
              f"{cell['gae']['fermat_w0.0']:.0f} | {cell['gae']['batch_w0.0']:.0f} | {cell['gae']['batch_w1.0']:.0f} |")

def best_F(bits, arm, pr, w):
    cands = [(mean([gae(x, pr, w) for x in flat(ic[k])]), k) for k in ic if k[0] == bits and k[1] == "table" and k[2] == arm]
    return min(cands)[1]

md.append("\n## Ratios (point estimate [95% stratified bootstrap CI])\n")
md.append("| bits | comparison | pricing | w | IC ratio | rho neg/aut6 | net = IC ratio / rho ratio | IC cmorb / rho aut6 (distance) |")
md.append("|--:|:--|:--|--:|:--|:--|:--|:--|")
for bits in bits_list:
    for pr in PRICINGS:
        for w in WS:
            f = lambda x, pr=pr, w=w: gae(x, pr, w)
            rr = boot_ratio(rho[(bits, "neg")], rho[(bits, "aut6")], f)
            comps = []
            comps.append(("table eq-F (F0) fold/cmorb", (bits, "table", "fold", 1.0), (bits, "table", "cmorb", 1.0)))
            comps.append(("table A/A fold/foldAA (F0)", (bits, "table", "fold", 1.0), (bits, "table", "foldAA", 1.0)))
            comps.append(("table foldcm/cmorb (F0)", (bits, "table", "foldcm", 1.0), (bits, "table", "cmorb", 1.0)))
            comps.append(("table each-arm-best-F fold/cmorb", best_F(bits, "fold", pr, w), best_F(bits, "cmorb", pr, w)))
            comps.append(("semaev eq-F fold/cmorb", (bits, "semaev", "fold", 1.0), (bits, "semaev", "cmorb", 1.0)))
            comps.append(("semaev A/A fold/foldAA", (bits, "semaev", "fold", 1.0), (bits, "semaev", "foldAA", 1.0)))
            for name, ka, kb in comps:
                if ka not in ic or kb not in ic: continue
                icr = boot_ratio(ic[ka], ic[kb], f)
                # net with bootstrap: combine independent bootstrap draws
                rng = random.Random(7); nb = 2000
                def draw(D):
                    return mean([f(x) for k2, v in D.items() for x in rng.choices(v, k=len(v))])
                nets = sorted((draw(ic[ka])/draw(ic[kb])) / (draw(rho[(bits, 'neg')])/draw(rho[(bits, 'aut6')])) for _ in range(nb))
                net = (icr[0]/rr[0], nets[50], nets[1949])
                dist = mean([f(x) for x in flat(ic[kb])]) / mean([f(x) for x in flat(rho[(bits, 'aut6')])])
                if "A/A" in name: dist = float('nan')
                S["ratios"].append(dict(bits=bits, comp=name, pricing=pr, w=w, ka=list(map(str, ka)), kb=list(map(str, kb)),
                                        ic=icr, rho=rr, net=net, dist=dist))
                fmt = lambda t: f"{t[0]:.3f} [{t[1]:.3f}, {t[2]:.3f}]"
                md.append(f"| {bits} | {name}{' (g='+str(ka[3])+' vs '+str(kb[3])+')' if 'best' in name else ''} | {pr} | {w} | {fmt(icr)} | {fmt(rr)} | {fmt(net) if 'A/A' not in name else '-'} | {dist:.1f} |")
md.append(f"\nfailures (no determination within cap) {dict(fails)}; wrong answers {dict(wrong)}")
json.dump(S, open(os.path.join(OUT, "summary.json"), "w"), indent=1, default=str)
open(os.path.join(OUT, "summary.md"), "w").write("\n".join(md) + "\n")
print("\n".join(md))
