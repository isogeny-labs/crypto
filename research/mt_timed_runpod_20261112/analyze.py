#!/usr/bin/env python3
"""Analysis of the RunPod timed head-to-head (rules in PROTOCOL.md).  usage: analyze.py results/holdout [summary.json]
The main analysis runs once per build (std, fast); then a cross-build table (fast/std per side).
Time of a run key = median over its repeats of ms_arm (solve only) and of ms (incl. the shared verification pass).
icbest / rhobest / bsgsbest = per configuration, the fastest key of that side (by median time)."""
import json, sys, os, random, statistics as st, collections, math
IC = {"ic5", "ic5ks", "ic5w", "ic5rw"}; RHO = {"rho2xks", "rhodpu"}; BSGS = {"bsgs2"}
def boot(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    m = sorted(st.median(rnd.choices(xs, k=n)) for _ in range(B)); return m[int(.025 * B)], m[int(.975 * B) - 1]
def verdict(lo, hi): return "cheaper" if hi < 1 else ("dearer" if lo > 1 else "unranked")
def main(d, out_json=None, build=None):
    rows = [json.loads(l) for l in open(os.path.join(d, "runs.jsonl"))]
    if build: rows = [r for r in rows if r.get("build") == build]; print(f"\n# build {build}")
    bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
    print(f"executions {len(rows)}; errors or unverified {len(bad)}")
    reps = collections.defaultdict(list)
    for r in rows:
        if r in bad: continue
        reps[(r["pb"], r["T"], r["idx"], r["seed"], r["arm"], r["param"], r.get("doff"))].append(r)
    cfg = collections.defaultdict(dict)
    for k, rs in reps.items():
        rs = sorted(rs, key=lambda r: r["rep"])
        cfg[k[:4]][k[4:]] = dict(arm=k[4], ms_arm=st.median(r["ms_arm"] for r in rs), ms=st.median(r["ms"] for r in rs),
                                 per_rep_arm=[r["ms_arm"] for r in rs], counts=rs[0])
    cells = sorted({c[:2] for c in cfg}); S = {"cells": []}
    def best(c, side, f): return min((v for v in cfg[c].values() if v["arm"] in side), key=lambda v: v[f])
    print("\n| bits | T | ic/rho solve [95%] | ic/rho incl. verify [95%] | ic/bsgs solve | A/A rep0/rep1 (rho side) [95%] | fixed-key ic/rho solve [95%] | winners ic / rho |")
    print("|---|---|---|---|---|---|---|---|")
    for pb, T in cells:
        cs = [c for c in cfg if c[:2] == (pb, T)]
        r_arm = [best(c, IC, "ms_arm")["ms_arm"] / best(c, RHO, "ms_arm")["ms_arm"] for c in cs]
        r_all = [best(c, IC, "ms")["ms"] / best(c, RHO, "ms")["ms"] for c in cs]
        r_bs = [best(c, IC, "ms_arm")["ms_arm"] / best(c, BSGS, "ms_arm")["ms_arm"] for c in cs]
        aa = []
        for c in cs:
            v = best(c, RHO, "ms_arm")["per_rep_arm"]
            if len(v) >= 2: aa.append(v[0] / v[1])
        # selection-bias control: choose each side's key on repeat 0 (pooled over the cell), score on repeats >= 1
        def pick(side):
            tot = collections.defaultdict(list)
            for c in cs:
                for kk, v in cfg[c].items():
                    if v["arm"] in side: tot[kk].append(v["per_rep_arm"][0])
            return min(tot, key=lambda kk: st.median(tot[kk]))
        ki, kr = pick(IC), pick(RHO)
        fixed = [st.median(cfg[c][ki]["per_rep_arm"][1:]) / st.median(cfg[c][kr]["per_rep_arm"][1:]) for c in cs
                 if len(cfg[c][ki]["per_rep_arm"]) > 1]
        win_ic = collections.Counter(best(c, IC, "ms_arm")["arm"] for c in cs)
        win_rho = collections.Counter("%s %s" % (best(c, RHO, "ms_arm")["arm"], best(c, RHO, "ms_arm")["counts"]["param"]) for c in cs)
        row = dict(bits=pb, T=T, n=len(cs))
        for name, xs in (("ic_rho_solve", r_arm), ("ic_rho_all", r_all), ("ic_bsgs_solve", r_bs), ("aa_rho", aa), ("fixed_ic_rho_solve", fixed)):
            if xs:
                lo, hi = boot(xs); row[name] = dict(median=st.median(xs), lo=lo, hi=hi, n=len(xs), wins=sum(x < 1 for x in xs))
        row["verdict_solve"] = verdict(row["ic_rho_solve"]["lo"], row["ic_rho_solve"]["hi"])
        row["verdict_all"] = verdict(row["ic_rho_all"]["lo"], row["ic_rho_all"]["hi"])
        row["fixed_keys"] = dict(ic=list(map(str, ki)), rho=list(map(str, kr)))
        row["winners"] = dict(ic=dict(win_ic), rho=dict(win_rho)); S["cells"].append(row)
        f = lambda x: "%.3f [%.3f, %.3f]" % (x["median"], x["lo"], x["hi"]) if x else "-"
        print(f"| {pb} | {T} | {f(row['ic_rho_solve'])} {row['verdict_solve']} | {f(row['ic_rho_all'])} | {row['ic_bsgs_solve']['median']:.3f} | "
              f"{f(row.get('aa_rho'))} | {f(row.get('fixed_ic_rho_solve'))} | {dict(win_ic)} / {dict(win_rho)} |")
    # A/A noise floor over all keys: |log(rep_i / median)|
    devs = [abs(math.log(x / st.median(v["per_rep_arm"]))) for c in cfg for v in cfg[c].values() for x in v["per_rep_arm"]
            if len(v["per_rep_arm"]) > 1 and v["ms_arm"] > 0]
    if devs:
        S["aa_median_abs_log_dev"] = st.median(devs); S["aa_p95_abs_log_dev"] = sorted(devs)[int(.95 * len(devs))]
        print(f"\nA/A: repeat deviation from the key median, median |log| {S['aa_median_abs_log_dev']:.4f}, 95th pct {S['aa_p95_abs_log_dev']:.4f}")
    # opcost: measured prices in GAE (padd = 1 GAE)
    S["opcost"] = []
    for l in open(os.path.join(d, "opcost.jsonl")):
        o = json.loads(l)
        if "padd_ns" not in o: continue
        g = o["padd_ns"]
        S["opcost"].append(dict(pb=o["pb"], when=o["when"], gae_ns=g, I_over_M=o["inv_ns"] / o["mul_ns"],
                                mul_gae=o["mul_ns"] / g, native_gae=o["canon_hash_ns"] / g, step_index_gae=o["step_index_ns"] / g,
                                std_lookup_gae={t["log2_size"]: t["std_lookup_ns"] / g for t in o["tables"]},
                                fast_lookup_gae={t["log2_size"]: t["fast_lookup_ns"] / g for t in o["tables"]}))
    print("\nopcost (GAE = one affine addition):")
    for o in S["opcost"]:
        print(f"  {o['pb']}b {o['when']}: GAE {o['gae_ns']:.0f} ns, I/M {o['I_over_M']:.1f}, native {o['native_gae']:.4f}, "
              f"std lookup " + ", ".join(f"2^{k}:{v:.3f}" for k, v in o["std_lookup_gae"].items()))
    return S, cfg
def cross(d):
    print("\n# fast / std, solve-phase time of each side's fastest key per configuration (median [95%])")
    print("| bits | T | IC fast/std | rho fast/std |"); print("|---|---|---|---|")
    _, cs = main(d, None, "std"); _, cf = main(d, None, "fast"); out = []
    for pb, T in sorted({c[:2] for c in cs}):
        row = dict(bits=pb, T=T)
        for name, side in (("ic", IC), ("rho", RHO)):
            r = [min(v["ms_arm"] for v in cf[c].values() if v["arm"] in side) / min(v["ms_arm"] for v in cs[c].values() if v["arm"] in side)
                 for c in cs if c[:2] == (pb, T) and c in cf]
            lo, hi = boot(r); row[name] = dict(median=st.median(r), lo=lo, hi=hi, n=len(r))
        out.append(row)
        print(f"| {pb} | {T} | {row['ic']['median']:.3f} [{row['ic']['lo']:.3f}, {row['ic']['hi']:.3f}] | {row['rho']['median']:.3f} [{row['rho']['lo']:.3f}, {row['rho']['hi']:.3f}] |")
    return out
if __name__ == "__main__":
    d = sys.argv[1]; S = {}
    for b in ("std", "fast"):
        S[b], _ = main(d, None, b)
    S["fast_over_std"] = cross(d)
    if len(sys.argv) > 2: json.dump(S, open(sys.argv[2], "w"), indent=1)
