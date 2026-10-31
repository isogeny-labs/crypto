# Row 66 recompute: 20261015-ic-improve2-03-fallrow-gate. Reads committed files only. Run from repo root with python3 -I.
import json, random, math, os
D = "research/ic_improve_20261015/03-fallrow-gate/runs"
def jl(p):
    return [json.loads(x) for x in open(p) if x.strip()]
test = []
for f in ["test_pipe3.jsonl", "test_pipe2_barrel_small.jsonl", "test_barrel_large.jsonl"]:
    test += jl(os.path.join(D, f))
fit = {}
for r in jl(os.path.join(D, "fit_test.jsonl")):
    fit[(r["ratio"], r["cell"])] = r
cells = sorted(set(r["cell"] for r in test))
specs = {"I/G": ("i_cpu", "g_cpu"), "I/F": ("i_cpu", "f_cpu"), "I/LR": ("i_cpu", "lr_cpu"), "AA": ("i_cpu", "i2_cpu")}
def gm(xs): return math.exp(sum(math.log(x) for x in xs) / len(xs))
def boot(xs, n=10000, seed=1):
    rng = random.Random(seed); g = []
    for _ in range(n):
        s = [xs[rng.randrange(len(xs))] for _ in range(len(xs))]
        g.append(gm(s))
    g.sort(); return g[int(0.025 * n)], g[int(0.975 * n) - 1]
res = {}
for cell in cells:
    rows = [r for r in test if r["cell"] == cell]
    for name, (num, den) in specs.items():
        xs = [r[num] / r[den] for r in rows]
        g = gm(xs); lo, hi = boot(xs)
        res[(name, cell)] = (g, lo, hi, len(rows))
        stored = fit.get((name, cell))
        s = "stored gm=%s lo=%s hi=%s verdict=%s targets=%s" % (stored["gm"], stored["lo"], stored["hi"], stored["verdict"], stored["targets"]) if stored else "no stored row"
        print("%-6s %-18s n=%d gm=%.4f own95=[%.4f, %.4f] | %s" % (name, cell, len(rows), g, lo, hi, s))
# gate firing and verdict counts
print("test records:", len(test), "fires true:", sum(1 for r in test if r["fires"]), "gate_ok false:", sum(1 for r in test if not r["gate_ok"]))
print("fit_test failed_total sum:", sum(r["failed_total"] for r in fit.values()))
pipe = [c for c in cells if c.startswith("pipe")]
print("pipeline cells:", len(pipe), "stored I/G verdicts:", [fit[("I/G", c)]["verdict"] for c in pipe])
print("pipeline cells fires total:", sum(fit[("I/G", c)]["fires"] for c in pipe))
print("I/F pipeline gm min %.4f max %.4f" % (min(res[("I/F", c)][0] for c in pipe), max(res[("I/F", c)][0] for c in pipe)))
held = ["pipe-n19-l7-m3-a0", "pipe-n23-l8-m3-a0", "pipe-n29-l10-m3-a1", "pipe-n31-l10-m3-a0", "pipe-n23-l12-m2-a0", "pipe-n29-l15-m2-a1"]
print("I/LR gm on pipeline cells:", {c: round(res[("I/LR", c)][0], 4) for c in pipe})
print("I/LR gm on pipeline (13,4):", round(res[("I/LR", "pipe-n13-l4-m3-a0")][0], 4))
# train counts
tr = 0
for f in ["train_barrel.jsonl", "train_pipe2.jsonl", "train_pipe3.jsonl"]:
    tr += len(jl(os.path.join(D, f)))
print("test target records:", len(test), "train target records:", tr)
# end to end
e = jl(os.path.join(D, "e2e.jsonl"))
arms = sorted(set(r["arm"] for r in e))
print("e2e arms:", arms, "rows:", len(e))
n_cmp = 0; n_eq = 0; n_diff = 0
groups = {}
for r in e:
    groups.setdefault((r["n"], r["ell"], r["arm"]), []).append(r)
for key in sorted(set((k[0], k[1]) for k in groups)):
    a = groups.get(key + ("I",), []); b = groups.get(key + (arms[1] if arms[0] == "I" else arms[0],), [])
    ratios = []
    for x, y in zip(a, b):
        same = all(x[f] == y[f] for f in ["relations", "trials", "dispositions_hash", "reductions", "log_ok", "found", "refuted", "unknown", "d"])
        n_cmp += 1; n_eq += int(same); n_diff += int(not same)
        ratios.append(x["cpu_s"] / y["cpu_s"])
    print("cell (%d,%d): pairs=%d I/G cpu ratios=%s geomean=%.4f" % (key[0], key[1], len(ratios), [round(v, 4) for v in ratios], gm(ratios)))
print("e2e compared=%d equal=%d differ=%d" % (n_cmp, n_eq, n_diff))
print("e2e reductions per cell:", sorted(set((r["n"], r["ell"], r["reductions"]) for r in e)))
print("e2e log_ok all:", all(r["log_ok"] for r in e), "dispositions hashes:", sorted(set((r["n"], r["ell"], r["dispositions_hash"]) for r in e)))
