#!/usr/bin/env python3
"""Item 09 assessment: frozen decision rule applied to the committed -r2 eval records (cell 17,4,3) and controls."""
import json, collections, random, math, sys
RAW = "raw/"
def load(fs):
    rs = []
    for f in fs: rs += [json.loads(l) for l in open(RAW + f)]
    return rs
# P3 nstd == solutions on sat outcomes
for f in ["eval-p3-8-3-3-test-r2","eval-p3-8-3-3-planted-r2","eval-p3-8-2-4-test-r2","eval-p3-8-2-4-planted-r2"]:
    rs = load([f + ".jsonl"]); n = [r for r in rs if r['outcome']=='sat']
    print("P3", f, "sat runs", len(n), "nstd!=solutions", sum(r['nstd']!=r['solutions'] for r in n))
rs = load(["eval-17-4-3-test-00-16-r2.jsonl", "eval-17-4-3-test-16-32-r2.jsonl"])
by = collections.defaultdict(dict)
for r in rs: by[r['idx']][r['arm']] = r
idxs = sorted(by)
print("targets", len(idxs), "outcomes", collections.Counter((r['arm'], r['outcome']) for r in rs if r['outcome'] != 'unsat'))
print("incumbent outcomes", collections.Counter(by[i]['a']['outcome'] for i in idxs), "checks false", sum(not r['check'] for r in rs))
usable = [i for i in idxs if by[i]['a']['outcome'] == 'unsat']
arms = ['b50','b25','c','d','p','r50','r25']
def cost(r, w): return r['touches'] + w * r['policy_ops']
def ratio(sel, num, den, w):
    # sum cost_num / sum cost_den
    return sum(cost(by[i][num], w) for i in sel) / sum(cost(by[i][den], w) for i in sel)
def rrows(sel, x): return sum(by[i]['a']['rows'] for i in sel) / sum(by[i][x]['rows'] for i in sel)
def boot(num, den, w, seed=1):
    rng = random.Random(seed); out = []
    for _ in range(2000):
        s = [rng.choice(usable) for _ in usable]
        out.append(ratio(s, num, den, w))
    out.sort(); return out[49], out[1949]
res = {}
print("arm w ratio_vs_a [95%] median rows_ratio  censored(cap_touch)")
for x in arms:
    cens = sum(by[i][x]['outcome'] == 'cap_touch' for i in usable)
    for w in (0, 0.1, 1):
        rt = ratio(usable, 'a', x, w); lo, hi = boot('a', x, w)
        med = sorted(cost(by[i]['a'], w) / cost(by[i][x], w) for i in usable)
        med = (med[15] + med[16]) / 2
        res[(x, w)] = (rt, lo, hi, med)
        print(f"{x:4s} w={w:<4} {rt:6.3f} [{lo:.3f},{hi:.3f}] med {med:.3f} rows {rrows(usable, x):.3f} cens {cens}")
print("matched null: sum cost_null / sum cost_learned")
for x, nl in (('b50','r50'),('b25','r25'),('c','p'),('d','r50')):
    for w in (0, 0.1, 1):
        rt = ratio(usable, nl, x, w); lo, hi = boot(nl, x, w)
        print(f"{nl}/{x} w={w:<4} {rt:6.3f} [{lo:.3f},{hi:.3f}]")
print("basis hash agree where both complete:", all(by[i][x]['basis_hash'] == by[i]['a']['basis_hash'] for i in usable for x in arms if by[i][x]['outcome'] == 'unsat'))
print("wall_ms sums:", {x: sum(by[i][x]['wall_ms'] for i in usable) for x in ['a'] + arms})
print("touches sums:", {x: sum(by[i][x]['touches'] for i in usable) for x in ['a'] + arms})
print("per-target b50 ratio min/max:", min(by[i]['a']['touches'] / by[i]['b50']['touches'] for i in usable), max(by[i]['a']['touches'] / by[i]['b50']['touches'] for i in usable))
o = load(["eval-17-4-3-test-00-03-o-r2.jsonl"])
for r in o:
    a = by[r['idx']]['a']; print("o idx", r['idx'], "touches", r['touches'], "a", a['touches'], "ratio %.3f" % (a['touches'] / r['touches']), "rows ratio %.3f" % (a['rows'] / r['rows']), "outcome", r['outcome'], "label_xors", r['label_xors'])
print("o sum ratio", sum(by[r['idx']]['a']['touches'] for r in o) / sum(r['touches'] for r in o))
print("a vs b50 on same 3 targets", sum(by[i]['a']['touches'] for i in range(3)) / sum(by[i]['b50']['touches'] for i in range(3)))
# offline cost: train label_xors in cell 17 first-run
tr = load(["train-17-4-3-00-16.jsonl", "train-17-4-3-16-32.jsonl"])
print("train targets", len(tr), "label_xors sum", sum(r['label_xors'] for r in tr), "touches sum", sum(r['touches'] for r in tr))
sav = sum(by[i]['a']['touches'] - by[i]['b50']['touches'] for i in usable) / len(usable)
print("mean saving b50 per target", sav)
# figure data (recorded/derived values only)
out = {"cell": "17-4-3", "n_usable": len(usable), "arms": {}}
for x in arms:
    rt, lo, hi, med = res[(x, 0)]
    out["arms"][x] = {"ratio_w0": rt, "lo": lo, "hi": hi, "median": med, "rows_ratio": rrows(usable, x),
                      "ratio_w1": res[(x, 1)][0], "lo_w1": res[(x, 1)][1], "hi_w1": res[(x, 1)][2]}
out["o_ratio_3targets"] = sum(by[r['idx']]['a']['touches'] for r in o) / sum(r['touches'] for r in o)
out["cell13_train"] = [{"idx": r["idx"], "touches": r["touches"], "outcome": r["outcome"]} for r in load(["train-13-3-4-00-16-r2.jsonl"])]
json.dump(out, open("figures/cell17.json", "w"), indent=1)
