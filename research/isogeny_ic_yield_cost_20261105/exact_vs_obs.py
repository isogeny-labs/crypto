import json, glob, math, re, csv
from scipy import stats
exact = {}
for f in glob.glob("results_fixedK/exact_m*.txt"):
    for line in open(f):
        p = line.rstrip("\n").split("\t")
        exact[(int(p[0][2:]), p[1])] = float(p[4].split("=")[1])
bits = {}
for f in glob.glob("results_fixedK/shard_*/bits.json"): bits.update(json.load(open(f)))
# per-scalar unbiased yield (r-1)/(n-1) from the full runs
rows = list(csv.DictReader(open("results_fixedK/per_scalar.csv")))
out = ["| m | curve | exact P = \|T∩⟨G⟩\|/q | fixed-length bits: p̂ [Wilson 95%] | z vs exact | stopping-corrected mean (r−1)/(n−1) [95% CI] | naive r/n |", "|---|---|---|---|---|---|---|"]
for m in (2, 3, 4):
    ks, ns, chi_obs_exact = [], [], 0.0
    for key, bl in sorted(bits.items()):
        mm, name, mode = key.split("|")
        if int(mm) != m or mode != "fixedK": continue
        k = n = 0
        for b in bl:
            b = b[:-1] if len(b) < 300 else b    # truncated list ends on the stopping success -> drop it
            k += sum(b); n += len(b)
        P = exact[(m, name)]; ph = k/n
        z = (ph - P) / math.sqrt(P*(1-P)/n); chi_obs_exact += z*z
        zc = 1.96; d = 1 + zc*zc/n; c = (ph + zc*zc/(2*n))/d; w = zc*math.sqrt(ph*(1-ph)/n + zc*zc/(4*n*n))/d
        R = [r for r in rows if int(r["m"]) == m and r["curve"] == name and r["fbmode"] == "fixedK"]
        u = [(int(r["relations"])-1)/(int(r["trials"])-1) for r in R]; mu = sum(u)/len(u)
        sd = math.sqrt(sum((x-mu)**2 for x in u)/(len(u)-1)); h = stats.t.ppf(.975, len(u)-1)*sd/math.sqrt(len(u))
        nv = sum(float(r["yield_"]) for r in R)/len(R)
        out.append(f"| {m} | {name} | {P:.4f} | {ph:.4f} [{c-w:.4f}, {c+w:.4f}] (n={n}) | {z:+.2f} | {mu:.4f} [{mu-h:.4f}, {mu+h:.4f}] | {nv:.4f} |")
        ks.append(k); ns.append(n)
    tab = [ks, [n-k for k, n in zip(ks, ns)]]
    chi, pv, dof, _ = stats.chi2_contingency(tab)
    out.append(f"| {m} | **homogeneity across 9 curves** | | χ²={chi:.1f}, dof={dof}, p={pv:.3f} | Σz²={chi_obs_exact:.1f} vs χ²(9): p={1-stats.chi2.cdf(chi_obs_exact, 9):.3f} | | |")
open("results_fixedK/exact_vs_observed.md", "w").write("\n".join(out) + "\n"); print("\n".join(out))
