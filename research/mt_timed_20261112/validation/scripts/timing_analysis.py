#!/usr/bin/env python3
"""VALIDATOR analysis of the timing reproduction (timing.py output) against the producer's runs.
  1. producer's keys, validator's timings, all 36 configurations: per-configuration icbest/rhobest and ic/bsgs with the
     keys the producer's medians selected (no selection on the validator's data), per-cell median, the protocol's
     bootstrap and the exact bootstrap, the protocol decision, and the per-configuration agreement with the producer;
  2. independent best-of on the 9 nonce-sampled configurations (all 17 keys);
  3. absolute time reproduction per key (validator median / producer median);
  4. interference: steal on the pinned CPU, busy time on the other CPUs, load average;
  5. opcost reproduction and the loop-scaling (dead-code) check;
  6. (if present) part C: validator's source build vs the producer's binary.
usage: timing_analysis.py <timing.jsonl> <frozen study dir> <recompute.json> <out.json>"""
import json, sys, os, math, random, itertools, collections, statistics as st

TJ, FZ, RC, OUT = sys.argv[1:5]
L = [json.loads(l) for l in open(TJ)]
hdr = [x for x in L if x.get("header")][0]; ftr = [x for x in L if x.get("footer")]
runs = [x for x in L if x.get("kind") == "run"]; ops = [x for x in L if x.get("kind") == "opcost"]
prod = [json.loads(l) for l in open(os.path.join(FZ, "results/holdout/runs.jsonl"))]
rc = json.load(open(RC))
IC = ("ic5", "ic5ks", "ic5w", "ic5rw"); RHO = ("rho2xks", "rhodpu"); BS = ("bsgs2",)


def med(xs):
    xs = sorted(xs); n = len(xs)
    return xs[n // 2] if n % 2 else 0.5 * (xs[n // 2 - 1] + xs[n // 2])


def boot_protocol(xs, B=4000, seed=1):
    rr = random.Random(seed); n = len(xs)
    m = sorted(st.median(rr.choices(xs, k=n)) for _ in range(B))
    return m[int(.025 * B)], m[int(.975 * B) - 1]


def boot_exact(xs):
    n = len(xs); dist = collections.Counter(med([xs[i] for i in t]) for t in itertools.product(range(n), repeat=n))
    tot = n ** n; cum = 0; lo = hi = None
    for v in sorted(dist):
        cum += dist[v]
        if lo is None and cum / tot >= 0.025: lo = v
        if hi is None and cum / tot >= 0.975: hi = v
    return lo, hi


def verdict(lo, hi): return "faster" if hi < 1 else ("slower" if lo > 1 else "unranked")


ok_all = all(r.get("ok") == r.get("targets") and "error" not in r for r in runs)
mine = collections.defaultdict(list)
for r in runs:
    if r["binary"] != "producer": continue
    mine[(r["pb"], r["T"], r["idx"], r["seed"], r["key"][0], r["key"][1], r["key"][2])].append(r["ms_arm"])
pk = collections.defaultdict(list)
for r in prod:
    pk[(r["pb"], r["T"], r["idx"], r["seed"], r["arm"], r["param"], r.get("doff"))].append(r["ms_arm"])
# counters of the validator's executions equal the producer's (deterministic)
cnt_mism = 0
pc = {}
for r in prod:
    pc[(r["pb"], r["T"], r["idx"], r["seed"], r["arm"], r["param"], r.get("doff"))] = tuple(r[c] for c in "IMSHLN")
for r in runs:
    k = (r["pb"], r["T"], r["idx"], r["seed"], r["key"][0], r["key"][1], r["key"][2])
    if tuple(r.get(c) for c in "IMSHLN") != pc[k]: cnt_mism += 1

# 1. producer's keys, validator timings
per_cfg = []; cells = []
for e in rc["per_configuration"]:
    c = (e["bits"], e["T"], e["idx"], e["seed"])
    ki, kr, kb = (tuple(e["ms_arm"][s]) for s in ("ic_key", "rho_key", "bsgs_key"))
    ti, tr, tb = med(mine[c + ki]), med(mine[c + kr]), med(mine[c + kb])
    per_cfg.append(dict(bits=c[0], T=c[1], idx=c[2], ic_key=list(ki), rho_key=list(kr), mine_ic_rho=ti / tr, prod_ic_rho=e["ms_arm"]["ic_rho"],
                        mine_ic_bsgs=ti / tb, prod_ic_bsgs=e["ms_arm"]["ic_bsgs"], n_ic=len(mine[c + ki]), n_rho=len(mine[c + kr]),
                        mine_per_rep=[a / b for a, b in zip(mine[c + ki], mine[c + kr])],
                        log_mine_over_prod=math.log((ti / tr) / e["ms_arm"]["ic_rho"])))
for pb in (24, 28, 32):
    for T in (1024, 4096, 16384):
        v = [x for x in per_cfg if (x["bits"], x["T"]) == (pb, T)]
        xs = [x["mine_ic_rho"] for x in v]; lo, hi = boot_protocol(xs); elo, ehi = boot_exact(xs)
        pcell = [c for c in rc["cells"] if (c["bits"], c["T"]) == (pb, T)][0]
        bs = [x["mine_ic_bsgs"] for x in v]
        cells.append(dict(bits=pb, T=T, mine_median=med(xs), mine_lo=lo, mine_hi=hi, mine_exact_lo=elo, mine_exact_hi=ehi,
                          mine_verdict=verdict(lo, hi), prod_median=pcell["ic_rho_solve"]["median"], prod_lo=pcell["ic_rho_solve"]["lo"],
                          prod_hi=pcell["ic_rho_solve"]["hi"], prod_verdict=pcell["verdict_solve"],
                          same_side_of_1_configs=sum((a["mine_ic_rho"] < 1) == (a["prod_ic_rho"] < 1) for a in v),
                          mine_below1=sum(x < 1 for x in xs), mine_ic_bsgs_median=med(bs), prod_ic_bsgs_median=pcell["ic_bsgs_solve"]["median"],
                          mine_values=xs, prod_values=[x["prod_ic_rho"] for x in v],
                          per_rep_below1=sum(y < 1 for x in v for y in x["mine_per_rep"]), per_rep_n=sum(len(x["mine_per_rep"]) for x in v)))
size = {}
for pb in (24, 28, 32):
    v = [x for x in per_cfg if x["bits"] == pb]
    size[pb] = dict(below1=sum(x["mine_ic_rho"] < 1 for x in v), n=len(v), min=min(x["mine_ic_rho"] for x in v), max=max(x["mine_ic_rho"] for x in v),
                    median=med([x["mine_ic_rho"] for x in v]))
devs = [abs(x["log_mine_over_prod"]) for x in per_cfg]

# 2. independent best-of on part-B configurations
partB = []
for c in sorted({(r["pb"], r["T"], r["idx"], r["seed"]) for r in runs if r["part"] == "B"}):
    ks = [k for k in mine if k[:4] == c]
    def best(side): return min((k for k in ks if k[4] in side), key=lambda k: med(mine[k]))
    bi, br = best(IC), best(RHO)
    e = [x for x in rc["per_configuration"] if (x["bits"], x["T"], x["idx"], x["seed"]) == c][0]
    partB.append(dict(cfg=list(c), mine_ic_key=list(bi[4:]), mine_rho_key=list(br[4:]), mine_bestof_ratio=med(mine[bi]) / med(mine[br]),
                      prod_ic_key=e["ms_arm"]["ic_key"], prod_rho_key=e["ms_arm"]["rho_key"], prod_ratio=e["ms_arm"]["ic_rho"]))

# 3. absolute reproduction per key
absr = collections.defaultdict(list)
for k, v in mine.items():
    absr[(k[4], k[0])].append(med(v) / med(pk[k]))
abs_summary = {f"{a}/{pb}": dict(median=med(v), min=min(v), max=max(v), n=len(v)) for (a, pb), v in sorted(absr.items())}
allabs = [x for v in absr.values() for x in v]

# 4. interference
steal = [r["steal_jiffies_pin"] for r in runs + ops]; other = [r["other_cpus_busy_jiffies"] for r in runs + ops]
la = [r["loadavg"] for r in runs]
wall = sum(r["wall_s"] for r in runs + ops)
interf = dict(executions=len(runs), opcost_runs=len(ops), total_wall_s=wall, steal_jiffies_pin_total=sum(steal),
              steal_share_of_wall=sum(steal) / 100 / wall if wall else None, executions_with_steal=sum(s > 0 for s in steal),
              other_cpus_busy_jiffies_total=sum(other), other_cpus_busy_share=(sum(other) / 100) / (3 * wall) if wall else None,
              loadavg_min_median_max=[min(la), med(la), max(la)], proc_stat_delta=ftr[0]["proc_stat_delta"] if ftr else None)

# 5. opcost
po = [json.loads(l) for l in open(os.path.join(FZ, "results/holdout/opcost.jsonl"))]
def prices(o):
    g = o["padd_ns"]
    return dict(pb=o["pb"], when=o["when"], gae_ns=g, I_over_M=o["inv_ns"] / o["mul_ns"], native_gae=o["canon_hash_ns"] / g,
                step_index_gae=o["step_index_ns"] / g, loop_baseline_ns=o["loop_baseline_ns"], canon_hash_ns=o["canon_hash_ns"],
                std_lookup_gae={t["log2_size"]: t["std_lookup_ns"] / g for t in o["tables"]},
                std_lookup_ns={t["log2_size"]: t["std_lookup_ns"] for t in o["tables"]},
                per_op_ns=dict(mul=o["mul_ns"], sqr=o["sqr_ns"], inv=o["inv_ns"], padd=o["padd_ns"], canon_hash=o["canon_hash_ns"],
                               step_index=o["step_index_ns"], loop_baseline=o["loop_baseline_ns"]),
                counters=[o["I"], o["M"], o["S"], o["H"], o["L"], o["N"]], param=o.get("param"))
mo = [prices(o) for o in ops if o["when"] in ("before", "after")]
sc = [prices(o) for o in ops if o["when"] == "scale1.0"]
def rngs(lst):
    f = lambda key: [min(x[key] for x in lst), max(x[key] for x in lst)]
    lk = lambda sz: [min(x["std_lookup_gae"][sz] for x in lst for sz_ in [sz]), max(x["std_lookup_gae"][sz] for x in lst)]
    return dict(gae_ns=f("gae_ns"), I_over_M=f("I_over_M"), native_gae=f("native_gae"), step_index_gae=f("step_index_gae"),
                std_lookup_2p10_2p16=[min(x["std_lookup_gae"][s] for x in lst for s in (10, 12, 14, 16)), max(x["std_lookup_gae"][s] for x in lst for s in (10, 12, 14, 16))],
                std_lookup_2p22_2p24=[min(x["std_lookup_gae"][s] for x in lst for s in (22, 24)), max(x["std_lookup_gae"][s] for x in lst for s in (22, 24))])
scale = None
if sc:
    b32 = [x for x in mo if x["pb"] == 32]
    scale = dict(param1_per_op_ns=sc[0]["per_op_ns"], param025_per_op_ns_median={k: med([x["per_op_ns"][k] for x in b32]) for k in sc[0]["per_op_ns"]},
                 param1_std_lookup_ns=sc[0]["std_lookup_ns"], param025_std_lookup_ns_median={s: med([x["std_lookup_ns"][s] for x in b32]) for s in sc[0]["std_lookup_ns"]},
                 param1_counters=sc[0]["counters"])
opc = dict(validator=rngs(mo), producer=rngs([prices(o) for o in po]), scale_check=scale, validator_runs=mo)

# 6. part C
pcmp = []
for r in runs:
    if r["binary"] != "pristine": continue
    k = (r["pb"], r["T"], r["idx"], r["seed"], r["key"][0], r["key"][1], r["key"][2])
    pcmp.append(r["ms_arm"] / med(mine[k]))
partC = dict(n=len(pcmp), median_pristine_over_producer=med(pcmp) if pcmp else None, min=min(pcmp) if pcmp else None, max=max(pcmp) if pcmp else None)

res = dict(header=hdr, all_verified=ok_all, counter_mismatches_vs_producer=cnt_mism, cells=cells, size_level=size,
           per_configuration=per_cfg, abs_log_mine_over_prod_ratio=dict(median=med(devs), max=max(devs), p90=sorted(devs)[int(.9 * len(devs))]),
           partB=partB, absolute_time=dict(per_arm_size=abs_summary, overall_median=med(allabs), min=min(allabs), max=max(allabs)),
           interference=interf, opcost=opc, partC=partC)
json.dump(res, open(OUT, "w"), indent=1)
print("verified", ok_all, "counter mismatches", cnt_mism)
for c in cells:
    print("%d %5d mine %.3f [%.3f, %.3f] %-8s | prod %.3f [%.3f, %.3f] %-8s | same side %d/4 | per-rep<1 %d/%d | ic/bsgs mine %.3f prod %.3f" % (
        c["bits"], c["T"], c["mine_median"], c["mine_lo"], c["mine_hi"], c["mine_verdict"], c["prod_median"], c["prod_lo"], c["prod_hi"],
        c["prod_verdict"], c["same_side_of_1_configs"], c["per_rep_below1"], c["per_rep_n"], c["mine_ic_bsgs_median"], c["prod_ic_bsgs_median"]))
print("size", json.dumps(size)); print("|log mine/prod| per config", res["abs_log_mine_over_prod_ratio"])
for b in partB: print("B", b)
print("abs", res["absolute_time"]["overall_median"], res["absolute_time"]["min"], res["absolute_time"]["max"])
print("interference", json.dumps({k: v for k, v in interf.items() if k != "proc_stat_delta"}))
print("opcost validator", json.dumps(opc["validator"])); print("opcost producer", json.dumps(opc["producer"])); print("scale", json.dumps(scale))
print("partC", partC)
