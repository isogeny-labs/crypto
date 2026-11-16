#!/usr/bin/env python3
"""VALIDATOR recompute of research/mt_timed_20261112 from the raw rows (validator's own code, not analyze.py).

Inputs: the frozen study directory (git archive of the validated commit).  Reads results/holdout/runs.jsonl
(full rows) and runs.json (compact table), checks they agree, checks the design (36 configurations x 17 keys x
3 repeats, the seeded interleaving of run.py re-derived independently), correctness (ok == targets), counter
determinism across repeats, and recomputes every number of the RESULTS.md table and the opcost ranges:

  key time        = median over repeats (ms_arm primary, ms secondary)
  icbest/rhobest  = per configuration, fastest IC key / fastest rho key (4 IC keys, 10 rho keys, 3 bsgs2 keys)
  cell            = (bits, T), 4 configurations; median of the 4 ratios
  interval        = (a) the protocol bootstrap (random.Random(1).choices, B = 4000, order stats 100 and 3899),
                    (b) the EXACT bootstrap distribution of the median of 4 (all 4^4 resamples, equally likely),
                    (c) a numpy bootstrap, B = 20000, seeded from the validator nonce
  decision        = faster if hi < 1, slower if lo > 1, else unranked (protocol rule)
  fixed-key       = per side the key with the lowest median rep-0 time pooled over the cell's 4 configurations,
                    scored per configuration on median(rep 1, rep 2)
  A/A             = per configuration, rho-best key rep0/rep1; global median and 95th pct of |log(rep/median)|
plus validator-only diagnostics (not pre-registered): leave-one-repeat-out ratios, per-repeat ratios,
per-size sign counts over 12 configurations, ratio drift over the session, load average.

usage: recompute.py <frozen study dir> <out.json> <nonce hex>"""
import json, sys, os, math, random, itertools, collections, statistics as st
import numpy as np

FZ, OUT, NONCE = sys.argv[1], sys.argv[2], sys.argv[3]
H = os.path.join(FZ, "results/holdout")
IC = ("ic5", "ic5ks", "ic5w", "ic5rw"); RHO = ("rho2xks", "rhodpu"); BS = ("bsgs2",)


def med(xs):
    xs = sorted(xs); n = len(xs)
    return xs[n // 2] if n % 2 else 0.5 * (xs[n // 2 - 1] + xs[n // 2])


# ---------------------------------------------------------------- rows
full = [json.loads(l) for l in open(os.path.join(H, "runs.jsonl"))]
comp = json.load(open(os.path.join(H, "runs.json")))
F = comp["fields"]
crow = [dict(zip(F, r)) for r in comp["rows"]]
chk = collections.OrderedDict()
chk["rows_jsonl"] = len(full)
chk["rows_json"] = len(crow)
mism = 0
for a, b in zip(full, crow):
    for f in F:
        if a.get(f) != b.get(f):
            mism += 1
chk["runs_json_vs_runs_jsonl_field_mismatches"] = mism
import hashlib
chk["runs_json_source_sha256_matches"] = comp["source_sha256"] == hashlib.sha256(open(os.path.join(H, "runs.jsonl"), "rb").read()).hexdigest()

# ---------------------------------------------------------------- design, re-derived independently
def design():
    out = []
    for pb in (24, 28, 32):
        for T in (1024, 4096, 16384):
            for idx in range(4):
                for a in IC: out.append((pb, T, idx, 43, a, 1.0, None))
                for kp in (0.05, 0.1, 0.25, 0.5): out.append((pb, T, idx, 43, "rho2xks", kp, None))
                for kp in (0.0, 0.25):
                    for off in (-1, 0, 1): out.append((pb, T, idx, 43, "rhodpu", kp, off))
                for mf in (0.7, 1.0, 1.4): out.append((pb, T, idx, 43, "bsgs2", mf, None))
    return out
D = design()
chk["design_keys"] = len(D)
rnd = random.Random(20261112); order = []
for rep in range(3):
    o = list(range(len(D))); rnd.shuffle(o); order += [(rep, i) for i in o]
bad_order = 0
for r in full:
    rep, i = order[r["seq"]]
    k = D[i]
    if not (r["rep"] == rep and r["key"] == i and (r["pb"], r["T"], r["idx"], r["seed"], r["arm"], r["param"], r.get("doff")) == k):
        bad_order += 1
chk["seq_order_matches_seeded_shuffle"] = bad_order == 0 and sorted(r["seq"] for r in full) == list(range(len(full)))
chk["rows_out_of_design_order"] = bad_order
# interleaving: positions of each key's repeats
gaps = []
pos = collections.defaultdict(dict)
for r in full: pos[r["key"]][r["rep"]] = r["seq"]
for k, v in pos.items(): gaps.append(v[2] - v[0])
chk["repeat_span_seq_min_median_max"] = [min(gaps), med(gaps), max(gaps)]
chk["rep_blocks_contiguous"] = all(full[s]["rep"] == s // len(D) for s in range(len(full))) if all(r["seq"] == j for j, r in enumerate(full)) else None

# ---------------------------------------------------------------- correctness and determinism
chk["errors"] = sum("error" in r for r in full)
chk["ok_ne_targets"] = sum(r.get("ok") != r.get("targets") for r in full)
chk["targets_total"] = sum(r["targets"] for r in full)
chk["targets_verified_total"] = sum(r["ok"] for r in full)
cnt = collections.defaultdict(set)
for r in full:
    cnt[r["key"]].add(tuple(r[c] for c in ("I", "M", "S", "H", "L", "N", "ok", "targets")) + (json.dumps(r["phases"], sort_keys=True),))
chk["keys_with_counters_differing_across_repeats"] = sum(len(v) != 1 for v in cnt.values())
chk["ms_arm_le_ms_all"] = all(r["ms_arm"] <= r["ms"] for r in full)
chk["ms_le_wall_all"] = all(r["ms"] <= 1000 * r["wall_s"] for r in full)
la = [r["loadavg"] for r in full]
chk["loadavg_min_median_max"] = [min(la), med(la), max(la)]
chk["loadavg_gt_1p5"] = sum(x > 1.5 for x in la)
chk["sum_wall_s"] = sum(r["wall_s"] for r in full)

# ---------------------------------------------------------------- per configuration
cfg = collections.defaultdict(lambda: collections.defaultdict(dict))
for r in full:
    c = (r["pb"], r["T"], r["idx"], r["seed"]); k = (r["arm"], r["param"], r.get("doff"))
    cfg[c][k][r["rep"]] = r
def ktime(reps, f): return med([reps[i][f] for i in sorted(reps)])
def best(c, side, f):
    ks = [k for k in cfg[c] if k[0] in side]
    return min(ks, key=lambda k: ktime(cfg[c][k], f))
per_cfg = []
for c in sorted(cfg):
    e = dict(bits=c[0], T=c[1], idx=c[2], seed=c[3])
    for f in ("ms_arm", "ms"):
        ki, kr, kb = best(c, IC, f), best(c, RHO, f), best(c, BS, f)
        ti, tr, tb = ktime(cfg[c][ki], f), ktime(cfg[c][kr], f), ktime(cfg[c][kb], f)
        e[f] = dict(ic_key=list(ki), rho_key=list(kr), bsgs_key=list(kb), ic=ti, rho=tr, bsgs=tb, ic_rho=ti / tr, ic_bsgs=ti / tb)
    kr = best(c, RHO, "ms_arm"); v = cfg[c][kr]
    e["aa_rho_rep0_rep1"] = v[0]["ms_arm"] / v[1]["ms_arm"]
    ki = best(c, IC, "ms_arm"); vi = cfg[c][ki]
    e["per_rep_ic_rho"] = [vi[j]["ms_arm"] / v[j]["ms_arm"] for j in range(3)]
    # leave-one-repeat-out: choose each side's key on the mean of the other two repeats, score on the held-out one
    loo = []
    for h in range(3):
        oth = [j for j in range(3) if j != h]
        sel = lambda side: min((k for k in cfg[c] if k[0] in side), key=lambda k: sum(cfg[c][k][j]["ms_arm"] for j in oth))
        a, b = sel(IC), sel(RHO)
        loo.append(cfg[c][a][h]["ms_arm"] / cfg[c][b][h]["ms_arm"])
    e["loo_ic_rho"] = loo
    per_cfg.append(e)

# ---------------------------------------------------------------- bootstraps
def boot_protocol(xs, B=4000, seed=1):
    rr = random.Random(seed); n = len(xs)
    m = sorted(st.median(rr.choices(xs, k=n)) for _ in range(B))
    return m[int(.025 * B)], m[int(.975 * B) - 1]
def boot_exact(xs):
    n = len(xs); dist = collections.Counter(med([xs[i] for i in t]) for t in itertools.product(range(n), repeat=n))
    tot = n ** n; vals = sorted(dist); cum = 0; lo = hi = None
    for v in vals:
        cum += dist[v]
        if lo is None and cum / tot >= 0.025: lo = v
        if hi is None and cum / tot >= 0.975: hi = v
    return lo, hi, dict(p_median_eq_max=dist[max(xs)] / tot if max(xs) in dist else 0.0)
seed_np = int(NONCE[:16], 16)
def boot_np(xs, B=20000):
    g = np.random.default_rng(seed_np); a = np.array(xs)
    m = np.median(a[g.integers(0, len(a), size=(B, len(a)))], axis=1)
    return float(np.quantile(m, 0.025)), float(np.quantile(m, 0.975))
def verdict(lo, hi): return "faster" if hi < 1 else ("slower" if lo > 1 else "unranked")

cells = []
for pb in (24, 28, 32):
    for T in (1024, 4096, 16384):
        es = [e for e in per_cfg if (e["bits"], e["T"]) == (pb, T)]
        cs = [c for c in cfg if c[:2] == (pb, T)]
        row = dict(bits=pb, T=T, n=len(es))
        series = dict(ic_rho_solve=[e["ms_arm"]["ic_rho"] for e in es], ic_rho_all=[e["ms"]["ic_rho"] for e in es],
                      ic_bsgs_solve=[e["ms_arm"]["ic_bsgs"] for e in es], aa_rho=[e["aa_rho_rep0_rep1"] for e in es])
        # fixed-key control
        def pick(side):
            pool = collections.defaultdict(list)
            for c in cs:
                for k, v in cfg[c].items():
                    if k[0] in side: pool[k].append(v[0]["ms_arm"])
            return min(pool, key=lambda k: med(pool[k]))
        fi, fr = pick(IC), pick(RHO)
        series["fixed_ic_rho_solve"] = [med([cfg[c][fi][1]["ms_arm"], cfg[c][fi][2]["ms_arm"]]) /
                                        med([cfg[c][fr][1]["ms_arm"], cfg[c][fr][2]["ms_arm"]]) for c in sorted(cs)]
        series["loo_ic_rho_solve"] = [x for e in es for x in e["loo_ic_rho"]]
        for name, xs in series.items():
            lo, hi = boot_protocol(xs) if name != "loo_ic_rho_solve" else (None, None)
            elo, ehi, ex = boot_exact(xs) if len(xs) <= 6 else (None, None, {})
            nlo, nhi = boot_np(xs)
            row[name] = dict(values=xs, median=med(xs), min=min(xs), max=max(xs), lo=lo, hi=hi, exact_lo=elo, exact_hi=ehi,
                             np_lo=nlo, np_hi=nhi, below1=sum(x < 1 for x in xs), above1=sum(x > 1 for x in xs), **ex)
        row["verdict_solve"] = verdict(row["ic_rho_solve"]["lo"], row["ic_rho_solve"]["hi"])
        row["verdict_solve_exact"] = verdict(row["ic_rho_solve"]["exact_lo"], row["ic_rho_solve"]["exact_hi"])
        row["verdict_solve_np"] = verdict(row["ic_rho_solve"]["np_lo"], row["ic_rho_solve"]["np_hi"])
        row["verdict_all"] = verdict(row["ic_rho_all"]["lo"], row["ic_rho_all"]["hi"])
        row["verdict_fixed"] = verdict(row["fixed_ic_rho_solve"]["lo"], row["fixed_ic_rho_solve"]["hi"])
        row["verdict_aa_contains_1"] = row["aa_rho"]["lo"] <= 1 <= row["aa_rho"]["hi"]
        row["fixed_keys"] = dict(ic=list(fi), rho=list(fr))
        row["winners_ic"] = dict(collections.Counter(e["ms_arm"]["ic_key"][0] for e in es))
        row["winners_rho"] = dict(collections.Counter("%s %s" % (e["ms_arm"]["rho_key"][0], e["ms_arm"]["rho_key"][1]) for e in es))
        cells.append(row)

# ---------------------------------------------------------------- global A/A, winners, size-level sign counts
devs = []
for c in cfg:
    for k, v in cfg[c].items():
        xs = [v[j]["ms_arm"] for j in sorted(v)]; m = med(xs)
        if m > 0: devs += [abs(math.log(x / m)) for x in xs]
devs_s = sorted(devs)
glob = dict(aa_median_abs_log_dev=med(devs), aa_p95_abs_log_dev=devs_s[int(.95 * len(devs_s))], n=len(devs))
win_rho = collections.Counter(e["ms_arm"]["rho_key"][0] + (" k=%s" % e["ms_arm"]["rho_key"][1]) for e in per_cfg)
win_ic = collections.Counter(e["ms_arm"]["ic_key"][0] for e in per_cfg)
size = {}
for pb in (24, 28, 32):
    es = [e for e in per_cfg if e["bits"] == pb]
    xs = [e["ms_arm"]["ic_rho"] for e in es]
    k = sum(x < 1 for x in xs); n = len(xs)
    p_two = min(1.0, 2 * sum(math.comb(n, j) for j in range(0, min(k, n - k) + 1)) / 2 ** n)
    size[pb] = dict(n=n, below1=k, above1=n - k, min=min(xs), max=max(xs), median=med(xs), sign_test_two_sided_p=p_two,
                    per_rep_below1=sum(x < 1 for e in es for x in e["per_rep_ic_rho"]),
                    per_rep_n=sum(len(e["per_rep_ic_rho"]) for e in es),
                    loo_below1=sum(x < 1 for e in es for x in e["loo_ic_rho"]), loo_n=3 * n,
                    loo_min=min(x for e in es for x in e["loo_ic_rho"]), loo_max=max(x for e in es for x in e["loo_ic_rho"]))
# drift: per-repeat medians of the ic/rho ratio by size (rep index is time order)
drift = {pb: [med([e["per_rep_ic_rho"][j] for e in per_cfg if e["bits"] == pb]) for j in range(3)] for pb in (24, 28, 32)}

# ---------------------------------------------------------------- opcost
op = [json.loads(l) for l in open(os.path.join(H, "opcost.jsonl"))]
ops = []
for o in op:
    g = o["padd_ns"]
    ops.append(dict(pb=o["pb"], when=o["when"], gae_ns=g, I_over_M=o["inv_ns"] / o["mul_ns"], native_gae=o["canon_hash_ns"] / g,
                    step_index_gae=o["step_index_ns"] / g, loop_baseline_gae=o["loop_baseline_ns"] / g,
                    native_minus_baseline_gae=(o["canon_hash_ns"] - o["loop_baseline_ns"]) / g,
                    padd_vs_I_2M_S=g / (o["inv_ns"] + 2 * o["mul_ns"] + o["sqr_ns"]),
                    std_lookup_gae={t["log2_size"]: t["std_lookup_ns"] / g for t in o["tables"]},
                    fast_lookup_gae={t["log2_size"]: t["fast_lookup_ns"] / g for t in o["tables"]},
                    std_over_fast={t["log2_size"]: t["std_lookup_ns"] / t["fast_lookup_ns"] for t in o["tables"]},
                    counters=dict(I=o["I"], M=o["M"], S=o["S"], H=o["H"], L=o["L"], N=o["N"]), param=o["param"]))
def rng(f): v = [f(o) for o in ops]; return [min(v), max(v)]
def lk(sizes, d="std_lookup_gae"): v = [o[d][s] for o in ops for s in sizes]; return [min(v), max(v)]
opr = dict(gae_ns=rng(lambda o: o["gae_ns"]), I_over_M=rng(lambda o: o["I_over_M"]), native_gae=rng(lambda o: o["native_gae"]),
           step_index_gae=rng(lambda o: o["step_index_gae"]), native_minus_baseline_gae=rng(lambda o: o["native_minus_baseline_gae"]),
           padd_vs_I_2M_S=rng(lambda o: o["padd_vs_I_2M_S"]),
           std_lookup_2p10_2p16=lk((10, 12, 14, 16)), std_lookup_2p18_2p20=lk((18, 20)), std_lookup_2p22_2p24=lk((22, 24)),
           std_lookup_all=lk((10, 12, 14, 16, 18, 20, 22, 24)),
           std_over_fast=[min(v for o in ops for v in o["std_over_fast"].values()), max(v for o in ops for v in o["std_over_fast"].values())])
# expected opcost counters for param 0.25 (n1 = 5e6 mul and sqr, n2 = 250000 inv, padd chain, xpair+padd loop, smul(g,7), smul(g,3))
n1, n2 = 5_000_000, 250_000
exp = dict(I=n2 + n2 + 2 * n2 + 4 + 2, M=n1 + 2 * n2 + 4 * n2 + 8 + 4, S=n1 + n2 + 3 * n2 + 6 + 3, H=0, L=0, N=0)
opr["counters_expected_param_0.25"] = exp
opr["counters_match_expected_all"] = all(o["counters"] == exp for o in ops)

json.dump(dict(checks=chk, cells=cells, per_configuration=per_cfg, global_aa=glob, winners=dict(ic=dict(win_ic), rho=dict(win_rho)),
               size_level=size, drift_per_rep_median_ratio=drift, opcost=ops, opcost_ranges=opr), open(OUT, "w"), indent=1)
print(json.dumps(chk, indent=1))
for c in cells:
    f = lambda d: "%.3f [%.3f, %.3f]" % (d["median"], d["lo"], d["hi"])
    print(c["bits"], c["T"], f(c["ic_rho_solve"]), c["verdict_solve"], c["verdict_solve_exact"], c["verdict_solve_np"], "| all", f(c["ic_rho_all"]),
          "| bsgs %.3f" % c["ic_bsgs_solve"]["median"], "| aa", f(c["aa_rho"]), c["verdict_aa_contains_1"], "| fixed", f(c["fixed_ic_rho_solve"]),
          c["verdict_fixed"], "| loo med %.3f [%d/%d<1]" % (c["loo_ic_rho_solve"]["median"], c["loo_ic_rho_solve"]["below1"], len(c["loo_ic_rho_solve"]["values"])),
          c["winners_ic"], c["winners_rho"])
print(glob); print(dict(win_ic), dict(win_rho)); print(json.dumps(size)); print(drift)
print(json.dumps(opr, indent=0))
