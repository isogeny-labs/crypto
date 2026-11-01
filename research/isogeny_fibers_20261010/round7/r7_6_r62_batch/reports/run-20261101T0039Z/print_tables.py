"""Print the Markdown tables of RESULTS.md from the recorded JSON (formatting
only; every figure is read from the native example's or ecbench's output)."""
import glob, json, math, pathlib

study = pathlib.Path(__file__).resolve().parent.parent.parent
runs = study / "runs"
sess = study / "sessions" / "pooled-vs-rho-l0" / "comparisons"
f2 = lambda x, d=2: f"{x:.{d}f}"
ci = lambda x, d=2: f"{x['est']:.{d}f} [{x['lo']:.{d}f}, {x['hi']:.{d}f}]"

print("### batch: per run header")
rows = []
for f in sorted(glob.glob(str(runs / "batch_r*_sym*_v1.json"))):
    d = json.load(open(f)); a = d["args"]
    rb = int(a[a.index("--rbits") + 1]); sym = int(a[a.index("--sym") + 1])
    rows.append((rb, sym, d))
print("| run | p | r (= 2^) | h | class curves | edges by l (2,3,5,7) | edges dropped, no dual (5,7) | trials | verified logs |")
print("|:--|--:|--:|--:|--:|:--|:--|--:|--:|")
for rb, sym, d in rows:
    b, c = d["base_curve"], d["class"]; e = c["edges_by_ell"]
    print(f"| r{rb} sym{sym} | {b['p']} | {b['rbits_actual']:.2f} | {b['h']} | {c['nodes']} | {e['2']}, {e['3']}, {e['5']}, {e['7']} | {c['edges_without_dual_by_ell'][2]}, {c['edges_without_dual_by_ell'][3]} | {len(d['trials'])} | {d['verified_logs']} |")
print()
for rb, sym, d in rows:
    print(f"### r{rb} sym{sym} (r = 2^{d['base_curve']['rbits_actual']:.2f}, {len(d['trials'])} trials)")
    print("| L = C | pooled/separate | gain | gain/sqrt(L) | pooled/separate over ideal KS | B/C |")
    print("|--:|:--|:--|:--|:--|:--|")
    for s in d["summary"]:
        print(f"| {s['L']} | {ci(s['ratio_pooled_over_separate'],4)} | {ci(s['gain_separate_over_pooled'])} | {ci(s['gain_over_sqrt_L'],3)} | {ci(s['ratio_over_ks_prediction'],3)} | {ci(s['ratio_B_over_C'],3)} |")
    e = d["exponents_over_L"]
    print(f"\nexponents over L=2..256: gain {ci(e['gain_ln_on_ln_L'],3)}; pooled cost {ci(e['pooled_cost_ln_on_ln_L'],3)}; separate cost {ci(e['separate_cost_ln_on_ln_L'],3)}\n")
print("### exponents table")
print("| run | r (= 2^) | gain exponent in L (pred. 0.5; ideal KS 0.489) | pooled-cost exponent (0.5) | separate-cost exponent (1) |")
print("|:--|--:|:--|:--|:--|")
for rb, sym, d in rows:
    e = d["exponents_over_L"]
    print(f"| r{rb} sym{sym} | {d['base_curve']['rbits_actual']:.2f} | {ci(e['gain_ln_on_ln_L'],3)} | {ci(e['pooled_cost_ln_on_ln_L'],3)} | {ci(e['separate_cost_ln_on_ln_L'],3)} |")
print("\n### transport priced in (L = 256)")
print("| run | transport per target (M, I) | pooled+transport / separate at I = 3M, 10M, 30M |")
print("|:--|:--|:--|")
for rb, sym, d in rows:
    s = d["summary"][-1]
    t = " ; ".join(f"{x['ratio']['est']:.4f} [{x['ratio']['lo']:.4f}, {x['ratio']['hi']:.4f}]" for x in s["ratio_with_transport_priced"])
    print(f"| r{rb} sym{sym} | {s['transport_M_per_target']:.1f} M, {s['transport_I_per_target']:.1f} I | {t} |")
print("\n### B/C summary")
n = ex = eq = 0
for rb, sym, d in rows:
    for s in d["summary"]:
        n += 1; ex += s["B_over_C_interval_excludes_1"]; eq += s["B_over_C_within_0.95_1.05"]
print(f"cells (run, L): {n}; B/C interval excludes 1: {ex}; interval inside [0.95, 1.05]: {eq}")
for rb, sym, d in rows:
    for s in d["summary"]:
        if s["B_over_C_interval_excludes_1"]:
            print(f"  excludes 1: r{rb} sym{sym} L={s['L']}: {ci(s['ratio_B_over_C'],3)}")

print("\n### ecbench")
print("| L | ecbench ratio pooled/rho.negation, all curves | pooled/separate = ratio/L | gain = L/ratio | per curve (r = 2^19.1, 24.0, 28.0, 31.7) gain |")
print("|--:|:--|:--|:--|:--|")
for L in (4, 16, 64, 256):
    d = json.load(open(sess / f"rho-neg__pooled-L{L}.json")); o = d["ops"]
    est, (lo, hi), (blo, bhi) = o["ratio_b_over_a"], o["ci95"], o["ci95_bootstrap"]
    per = " ; ".join(f"{L/c['ratio_b_over_a']:.2f}" for c in d["curves"])
    print(f"| {L} | {est:.3f} [{blo:.3f}, {bhi:.3f}] (cluster bootstrap; t-interval [{lo:.3f}, {hi:.3f}]) | {est/L:.4f} [{blo/L:.4f}, {bhi/L:.4f}] | {L/est:.2f} [{L/bhi:.2f}, {L/blo:.2f}] | {per} |")
d = json.load(open(sess / "rho-neg__pooled-L256.json"))
print("mean S: rho-neg", f2(d["a"]["mean_s"], 3), "pooled-L256", f2(d["b"]["mean_s"], 3))
for L in (4, 16, 64, 256):
    d = json.load(open(sess / f"rho-neg__pooled-L{L}.json"))
    print(L, "mean S pooled", f2(d["b"]["mean_s"], 3), "unpriced", d["b"]["unpriced_counts"], "band order", d["ops"]["band"]["order"])
acc = {}
for line in open(sess.parent / "records.jsonl"):
    r = json.loads(line)
    if r["warmup"]:
        continue
    acc.setdefault((r["workload"]["curve"]["slug"], r["arm"]), []).append(r["cost"]["s"])
print("per-curve mean S (descriptive, from records.jsonl)")
for k in sorted(acc):
    if k[1] in ("rho-neg", "pooled-L256"):
        print(" ", k[0], k[1], len(acc[k]), f2(sum(acc[k]) / len(acc[k]), 3))

fitf = runs / "scaling_fit_v1.json"
if fitf.exists():
    fit = json.load(open(fitf))
    print("\n### scaling fits"); print(fit["sizes"], "ks exponent", fit["ks_ideal_gain_exponent_L2_to_256"])
    print("| subset | cost | classes | sizes | exponent [95%] | contains 0.5 | contains 1 |")
    print("|:--|:--|--:|:--|:--|:--|:--|")
    for x in fit["fits"]:
        print(f"| {x['subset']} | {x['cost']} | {x['classes']} | {x['size_min']}..{x['size_max']} | {x['exponent']:.3f} [{x['lo']:.3f}, {x['hi']:.3f}] | {x['sqrt_in_interval']} | {x['one_in_interval']} |")
    print("\n| size bin | classes | mean size | mean total/pair | total/C mean (min..max) | total/sqrt(C) mean |")
    print("|:--|--:|--:|--:|:--|--:|")
    for b in fit["size_bins"]:
        print(f"| {b['size_from']}..{b['size_to']-1} | {b['classes']} | {b['mean_size']:.0f} | {b['mean_total_cost']:.0f} | {b['mean_cost_over_size']:.2f} ({b['min_cost_over_size']:.2f}..{b['max_cost_over_size']:.2f}) | {b['mean_cost_over_sqrt_size']:.1f} |")
    sc = [json.load(open(f)) for f in sorted(glob.glob(str(runs / "scaling_v[0-9].json")))]
    cl = [c for d in sc for c in d["classes"]]
    print("\nclasses:", len(cl), " pairs:", sum(len(c["pairs"]) for c in cl), " all routes validated forward (asserted in-run)")
    fr = [c["mean_search"] / (4 * c["component_size"]) for c in cl if c["component_size"] >= 100]
    print("search enumerations / (4 C), classes >= 100 curves: mean", f2(sum(fr)/len(fr), 3), "min", f2(min(fr), 3), "max", f2(max(fr), 3), "n", len(fr))
    for d in sc:
        print(d["args"], d["sampling"])
