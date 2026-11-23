# Analysis for results_cost2/ (cost_vs_rho2.py). Every cost is measured group additions.
import json, glob, math, random, statistics as st
from collections import defaultdict

IC, RHO = [], {}
for f in sorted(glob.glob("results_cost2/b*_m*_c*.jsonl")):
    for line in open(f):
        r = json.loads(line)
        if r["kind"] == "rho": RHO[(r["bits"], r["curve"])] = r
        else: IC.append(r)
mean = st.mean
def rho_costs(r):
    n = r["n"]
    return dict(brent=mean(r["brent_ops"]), brent_walk=mean(r["brent_steps"]),
                dp1=mean(r["dp_single_ops"]) + r["dp_single_setup"],          # one DLP, setup included
                dp1_walk=mean(r["dp_single_ops"]),
                dpD=mean(r["dp_multi_ops"]) + r["dp_multi_setup"] / n)       # per DLP over D=n targets (Kuhn-Struik)
def ic_cost(r, D): return r["precomp_ops"] / D + r["desc_ops_mean"]
def ic_cost_net(r, D):   # minus size-independent fixed overhead (64 step points, verify mult), for exponent fits
    return max(1.0, (r["precomp_ops"] - r["step_cost"]) / D + r["desc_ops_mean"] - r["verify_ops_est"])

out = []
bits_all = sorted({b for b, _ in RHO})
# ---------- rho ----------
out.append("## Rho baselines (mean over curves; 100 targets per curve)\n")
out.append("| bits | curves | q (curve 0) | Brent, no neg | DP+neg, 1 DLP (incl setup) | DP+neg walk only | DP+neg multi-target, per DLP at D=100 | sqrt(pi q/2) |")
out.append("|---|---|---|---|---|---|---|---|")
RB = {}
for b in bits_all:
    cs = [rho_costs(RHO[k]) for k in RHO if k[0] == b]
    RB[b] = {key: mean(c[key] for c in cs) for key in cs[0]}
    r0 = RHO[(b, 0)] if (b, 0) in RHO else next(RHO[k] for k in RHO if k[0] == b)
    out.append(f"| {b} | {len(cs)} | {r0['q']} | {RB[b]['brent']:.0f} | {RB[b]['dp1']:.0f} | {RB[b]['dp1_walk']:.0f} | {RB[b]['dpD']:.0f} | {r0['sqrt_pi_q_over_2']:.0f} |")

def linfit(xs, ys):
    n = len(xs); mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx

def boot_slope(per_size, B=2000, seed=1):
    """per_size: {bits: [(log2q, log2cost) per curve]} -> slope, 95% CI by resampling curves within each size."""
    rng = random.Random(seed); bs = sorted(per_size)
    pt = lambda b, pts: (mean(x for x, _ in pts), math.log2(mean(2 ** y for _, y in pts)))
    xs, ys = zip(*[pt(b, per_size[b]) for b in bs]); s0 = linfit(xs, ys); sl = []
    for _ in range(B):
        xs, ys = zip(*[pt(b, [rng.choice(per_size[b]) for _ in per_size[b]]) for b in bs]); sl.append(linfit(xs, ys))
    sl.sort(); return s0, sl[int(.025 * B)], sl[int(.975 * B)]

MINB = 20
out.append(f"\n## Scaling exponents, cost ∝ q^e (bits ≥ {MINB}; curve-bootstrap 95% CI; fixed overhead subtracted for IC)\n")
out.append("| series | e (95% CI) | sizes |")
out.append("|---|---|---|")
for key, lab in (("brent", "rho Brent no-neg (incl. 64-point setup)"), ("brent_walk", "rho Brent no-neg walk only"), ("dp1_walk", "rho DP+neg walk"), ("dpD", "rho multi-target per DLP, D=100")):
    ps = defaultdict(list)
    for (b, cv), r in RHO.items():
        if b >= MINB: ps[b].append((math.log2(r["q"]), math.log2(rho_costs(r)[key])))
    if len(ps) >= 3:
        s, lo, hi = boot_slope(ps); out.append(f"| {lab} | {s:.3f} ({lo:.3f}–{hi:.3f}) | {min(ps)}–{max(ps)} |")
groups = defaultdict(list)
NCUR = defaultdict(int)
for r in IC: NCUR[r["bits"]] = max(NCUR[r["bits"]], r["curve"] + 1)
for r in IC: groups[(r["strat"], r["m"], r["c"], r["coset"])].append(r)
theory = {("MITM", 2): 1.0, ("MITM", 3): 2 / 3, ("MITM", 4): 3 / 4, ("MITM", 5): 4 / 5}
for (strat, m, c, cs), rs in sorted(groups.items()):
    for D in (1, 100):
        ps = defaultdict(list)
        for r in rs:
            if r["bits"] >= MINB: ps[r["bits"]].append((math.log2(r["q"]), math.log2(ic_cost_net(r, D))))
        if len(ps) < 3: continue
        s, lo, hi = boot_slope(ps)
        out.append(f"| IC {strat} m={m} c={c} coset={'folded' if cs else 'off'} D={D} | {s:.3f} ({lo:.3f}–{hi:.3f}) | {min(ps)}–{max(ps)} |")

# ---------- best per size ----------
out.append("\n## Best IC configuration per size (config must have run on every curve of that size; chosen by mean cost over curves)\n")
out.append("| bits | D | best config | IC ops/DLP | ÷ rho Brent | ÷ rho DP+neg (1 target) | ÷ rho multi-target DP+neg (D=100) | memory (table+list) | curves |")
out.append("|---|---|---|---|---|---|---|---|---|")
for b in bits_all:
    for D in (1, 100):
        best = None
        for key, rs in groups.items():
            rb = [r for r in rs if r["bits"] == b]
            if len({r["curve"] for r in rb}) < NCUR[b]: continue
            cst = mean(ic_cost(r, D) for r in rb)
            if best is None or cst < best[0]: best = (cst, key, rb)
        if not best: continue
        cst, (strat, m, c, cs), rb = best
        mem = mean(r["table_entries"] + r["list_entries"] for r in rb)
        out.append(f"| {b} | {D} | {strat} m={m} c={c} coset={'folded' if cs else 'off'} | {cst:.0f} | {cst/RB[b]['brent']:.2f}× | "
                   f"{cst/RB[b]['dp1']:.2f}× | {cst/RB[b]['dpD']:.2f}× | {mem:.0f} | {len(rb)} |")

# ---------- per m, best MITM per size (the scalable strategy) ----------
out.append("\n## Best MITM per m and size, D=1, ratio to single-target rho DP+neg (mean over curves)\n")
ms = sorted({r["m"] for r in IC})
out.append("| bits | " + " | ".join(f"m={m}" for m in ms) + " |")
out.append("|---|" + "---|" * len(ms))
for b in bits_all:
    row = []
    for m in ms:
        cands = []
        for (strat, mm, c, cs), rs in groups.items():
            rb = [r for r in rs if r["bits"] == b]
            if strat == "MITM" and mm == m and len({r["curve"] for r in rb}) == NCUR[b]: cands.append((mean(ic_cost(r, 1) for r in rb), c, cs))
        if cands:
            x = min(cands); row.append(f"{x[0]/RB[b]['dp1']:.1f}× (c={x[1]},{'f' if x[2] else '-'})")
        else: row.append("–")
    out.append(f"| {b} | " + " | ".join(row) + " |")

# ---------- folded coset gain (measured) ----------
out.append("\n## Folded-coset gain, measured: cost(coset off) / cost(folded), D=1, mean over curves\n")
out.append("| strat | m | c | " + " | ".join(str(b) for b in bits_all) + " |")
out.append("|---|---|---|" + "---|" * len(bits_all))
for (strat, m, c) in sorted({(k[0], k[1], k[2]) for k in groups}):
    row = []
    for b in bits_all:
        a0 = [r for r in groups.get((strat, m, c, 0), []) if r["bits"] == b]
        a1 = [r for r in groups.get((strat, m, c, 1), []) if r["bits"] == b]
        row.append(f"{mean(ic_cost(r,1) for r in a0)/mean(ic_cost(r,1) for r in a1):.1f}" if a0 and a1 else "–")
    out.append(f"| {strat} | {m} | {c} | " + " | ".join(row) + " |")

# ---------- checks ----------
tot = sum(r["nscal"] for r in IC); rec = sum(r["recovered"] for r in IC); wrong = sum(r["wrong_candidates"] for r in IC)
rrec = sum(r["n"] for r in RHO.values())
defs = sorted({(r["m"], r["coset"], r["rank_deficiency"]) for r in IC if r["rank_deficiency"]})
out.append(f"\n## Checks\n- IC: {len(IC)} configs, {rec}/{tot} scalars recovered, {wrong} wrong candidates (rejected by verification).")
out.append(f"- Rho: {len(RHO)} (size, curve) cells × 100 targets × 3 methods = {3*rrec} DLPs, all asserted correct.")
out.append(f"- Nonzero rank deficiencies (m, coset, dims): {defs}")
open("results_cost2/summary.md", "w").write("\n".join(out) + "\n")
print("\n".join(out))
