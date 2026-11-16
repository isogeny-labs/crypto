import json, glob, math, statistics as st
from collections import defaultdict
from scipy import stats

rho, ic = {}, []
for f in glob.glob("results_cost/grid_*.jsonl"):
    for line in open(f):
        r = json.loads(line)
        if r["kind"] != "rho": ic.append(r)
RHO = {}
for f in glob.glob("results_cost/rho_b*.jsonl"):
    r = json.loads(open(f).readline()); RHO[r["bits"]] = r
for b, r in RHO.items():
    for m in (2, 3, 4): rho[(b, m)] = r
MINB = 18

def costs(r, strat, D):
    pre = r["full_precomp_ops"] if strat == "FULL" else r["mitm_precomp_ops"]
    desc = r["desc_ops_mean"] if strat == "FULL" else r["mitm_desc_ops_mean"]
    mem = r["table_entries"] if strat == "FULL" else r["mitm_table_entries"] + r["mitm_list"]
    return pre / D + desc, mem

out = []
out.append("## Rho baseline (r-adding walk + Brent, no negation map), 20 scalars per curve\n")
out.append("| bits | q | total ops mean (min-max) | walk steps mean (min-max) | setup ops | sqrt(pi q/2) | recovered |")
out.append("|---|---|---|---|---|---|---|")
for b in sorted(RHO):
    r = RHO[b]; w = r["walk_steps_all"]; o = r["ops_all"]
    out.append(f"| {b} | {r['q']} | {st.mean(o):.0f} ({min(o)}-{max(o)}) | {st.mean(w):.0f} ({min(w)}-{max(w)}) | {r['setup_mean']:.0f} | {r['sqrt_pi_q_over_2']:.0f} | {r['recovered']}/{r['n']} |")

out.append("\n## Index calculus: cost of the FIRST discrete log (precomputation + one descent), in group ops, ratio to rho\n")
out.append("| bits | m | c | coset | \\|FB\\| | yield | FULL ops (×rho) | FULL table entries | MITM ops (×rho) | MITM memory | descent ops FULL / MITM | LA cols | rec |")
out.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for r in sorted(ic, key=lambda r: (r["bits"], r["m"], r["c"], r["coset"])):
    rh = rho[(r["bits"], r["m"])]["ops_mean"]
    f1, fm = costs(r, "FULL", 1); m1, mm = costs(r, "MITM", 1)
    out.append(f"| {r['bits']} | {r['m']} | {r['c']} | {r['coset']} | {r['fb']} | {r['yield_']:.3f} | {f1:.0f} ({f1/rh:.1f}×) | {fm} | {m1:.0f} ({m1/rh:.1f}×) | {mm} | "
               f"{r['desc_ops_mean']:.0f} / {r['mitm_desc_ops_mean']:.0f} | {r['la_cols']} | {r['recovered']}/{r['nscal']} |")

# scaling exponents: log2(cost) vs log2(q), per (m, c, coset, strategy), D in {1, 100}
out.append("\n## Scaling exponents: cost ∝ q^e (least squares over bit sizes 18-24 only, to limit fixed-overhead bias)\n")
out.append("| series | m | c | coset | D (DLPs sharing precomp) | e (95% CI) | bit sizes | cost at largest size ×rho |")
out.append("|---|---|---|---|---|---|---|---|")
bs = [b for b in sorted(RHO) if b >= MINB]
for lab, key in (("rho walk only", "walk_steps_all"), ("rho total", "ops_all")):
    lr = stats.linregress([math.log2(RHO[b]["q"]) for b in bs], [math.log2(st.mean(RHO[b][key])) for b in bs])
    out.append(f"| {lab} | - | - | - | 1 | {lr.slope:.3f} (±{1.96*lr.stderr:.3f}) | {bs[0]}-{bs[-1]} | 1.0 |")
groups = defaultdict(list)
for r in ic: groups[(r["m"], r["c"], r["coset"])].append(r)
for (m, c, cs), rs in sorted(groups.items()):
    rs = sorted([r for r in rs if r["bits"] >= MINB], key=lambda r: r["bits"])
    if len(rs) < 3: continue
    for strat in ("FULL", "MITM"):
        for D in (1, 100):
            xs = [math.log2(r["q"]) for r in rs]; ys = [math.log2(costs(r, strat, D)[0]) for r in rs]
            l = stats.linregress(xs, ys)
            last = rs[-1]; ratio = costs(last, strat, D)[0] / rho[(last["bits"], m)]["ops_mean"]
            out.append(f"| IC {strat} | {m} | {c} | {cs} | {D} | {l.slope:.3f} (±{1.96*l.stderr:.3f}) | {rs[0]['bits']}-{rs[-1]['bits']} | {ratio:.2f}× |")
    xs = [math.log2(r["q"]) for r in rs]
    l = stats.linregress(xs, [math.log2(r["table_entries"]) for r in rs])
    out.append(f"| FULL table memory | {m} | {c} | {cs} | - | {l.slope:.3f} | | |")

open("results_cost/cost_summary.md", "w").write("\n".join(out) + "\n")
print("\n".join(out))
