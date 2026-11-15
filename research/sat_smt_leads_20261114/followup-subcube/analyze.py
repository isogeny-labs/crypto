#!/usr/bin/env python3
"""Merge raw/*.jsonl -> results.json and print the tables (markdown). Usage: analyze.py [raw files]"""
import sys, os, json, glob, math
from collections import defaultdict
H = os.path.dirname(os.path.abspath(__file__))
files = sys.argv[1:] or sorted(glob.glob(os.path.join(H, "raw", "*.jsonl")))
rows = [json.loads(x) for f in files for x in open(f)]
if not sys.argv[1:]:
    json.dump(rows, open(os.path.join(H, "results.json"), "w"))
mean = lambda v: sum(v) / len(v) if v else float("nan")


def slope(xs, ys):
    if len(xs) < 2:
        return float("nan")
    mx, my = mean(xs), mean(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


Ls = sorted(set(r["l"] for r in rows))
ok = [r for r in rows if r["status"] == "OK"]
print("rows", len(rows), "OK", len(ok), "NOT_RUN", sum(r["status"] == "NOT_RUN" for r in rows),
      "SKIPPED_AFTER_CENSOR", sum(r["status"] == "SKIPPED_AFTER_CENSOR" for r in rows))
nr = defaultdict(int)
for r in rows:
    if r["status"] == "NOT_RUN":
        nr[(r["l"], r["part"], r["arm"])] += 1
for k in sorted(nr):
    print("  NOT_RUN", k, nr[k])

print("\n## Part A: conflicts per leaf (non-decomposable targets)\n")
arms = ["A-plain-inc", "A-plain-cold", "A-dual-inc", "A-dual-cold"]
print("| l | n | " + " | ".join(a + " mean (max; zero-conflict share; censored leaves; targets)" for a in arms) + " |")
print("|---|---|" + "---|" * len(arms))
pa = {}
for l in Ls:
    cells = []
    for a in arms:
        rs = [r for r in ok if r["part"] == "A" and r["arm"] == a and r["l"] == l]
        if not rs:
            cells.append("not run")
            continue
        lv = sum(r["leaves"] for r in rs)
        m = sum(r["conflicts"] for r in rs) / lv
        pa[(a, l)] = m
        cells.append("%.2f (%d; %.2f; %d; %d)" % (m, max(r["max"] for r in rs),
                     sum(r["zero_conflict_leaves"] for r in rs) / lv, sum(r["censored"] for r in rs), len(rs)))
    n = [r["n"] for r in rows if r["l"] == l][0]
    print("| %d | %d | " % (l, n) + " | ".join(cells) + " |")
for a in arms:
    xs = [l for l in Ls if (a, l) in pa and pa[(a, l)] > 0]
    print("slope of log2(conflicts/leaf) in l, %s: %.2f bits per l over l = %s" % (a, slope(xs, [math.log2(pa[(a, l)]) for l in xs]), xs))
rs = [r for r in ok if r["arm"] == "E" and r["variant"] == "std" and not r["decomposable"]]
for l in Ls:
    q = [r for r in rs if r["l"] == l]
    if q:
        print("engine l=%d: leaves/target %.1f, sub-nodes per leaf %.2f, row XORs per leaf %.0f, pruned above depth l: %d" % (
            l, mean([r["leaves"] for r in q]), sum(r["sub_nodes"] for r in q) / max(1, sum(r["leaves"] for r in q)),
            sum(r["xors"] for r in q) / max(1, sum(r["leaves"] for r in q)), sum(sum(r["prune_depth"][:-1]) for r in q)))


def bname(b):
    return ("B=%d" % b[1]) if b[0] == "abs" else ("%g*2^j" % b[1])


def table(dec, variant, keep, title):
    print("\n## Part B: %s\n" % title)
    print("| budget | " + " | ".join("l=%d rho (rho'; leaves; conflicts; targets)" % l for l in Ls) + " | trend bits/l |")
    print("|---|" + "---|" * (len(Ls) + 1))
    res = {}
    buds = sorted(set(tuple(r["budget"]) for r in ok if r["part"] == "B" and r["arm"] != "E"))
    for b in buds:
        cells, xs, ys = [], [], []
        for l in Ls:
            q = [r for r in ok if r["part"] == "B" and r["arm"] != "E" and tuple(r["budget"]) == b and r["l"] == l
                 and r["decomposable"] == dec and r["variant"] == variant and r["keep"] == keep]
            if not q:
                cells.append("not run")
                continue
            rho = mean([(r["leaves"] + r["conflicts"]) / 2 ** l for r in q])
            rho2 = mean([(r["calls"] + r["conflicts"]) / 2 ** l for r in q])
            res[(b, l)] = (rho, len(q))
            xs.append(l)
            ys.append(math.log2(rho))
            cells.append("%.2f (%.2f; %.1f; %.0f; %d)" % (rho, rho2, mean([r["leaves"] for r in q]), mean([r["conflicts"] for r in q]), len(q)))
        print("| %s | " % bname(b) + " | ".join(cells) + " | %+.2f |" % slope(xs, ys))
    return res


K = table(False, "std", True, "keep learnt clauses, std order, non-decomposable")
D = table(False, "std", False, "discard (fresh solver per node), std order, non-decomposable")
table(False, "rand", True, "control: random coordinate order, keep, non-decomposable")
table(False, "alt", True, "control: second basis of V, keep, non-decomposable")
table(True, "std", True, "decomposable (planted) targets, keep, std order")

print("\n## rho relative to the solver's own scan (keep, B=0)\n")
for b in sorted(set(k[0] for k in K)):
    print(bname(b), " ".join("l=%d: %.2f" % (l, K[(b, l)][0] / K[(("abs", 0), l)][0]) for l in Ls if (b, l) in K))

print("\n## prune-depth histograms (keep, std, non-decomposable; mean nodes pruned per target at depth k = 0..l)\n")
for l in Ls:
    for b in sorted(set(k[0] for k in K)):
        q = [r for r in ok if r["part"] == "B" and r["arm"] == "keep" and tuple(r["budget"]) == b and r["l"] == l
             and not r["decomposable"] and r["variant"] == "std"]
        if q:
            h = [mean([r["prune_depth"][k] for r in q]) for k in range(l + 1)]
            cov = sum(h[k] * 2 ** (l - k) for k in range(l)) / 2 ** l
            print("l=%d %-9s %s  share of leaves covered by prunes above depth l: %.2f" % (l, bname(b), " ".join("%.1f" % x for x in h), cov))

print("\n## engine E (linearisation at internal nodes)\n")
for v in ("std", "rand", "alt"):
    for l in Ls:
        q = [r for r in ok if r["arm"] == "E" and r["variant"] == v and r["l"] == l and not r["decomposable"]]
        if q:
            print("E %s l=%d: nodes %.1f leaves %.1f (leaves/2^l %.2f) pruned at depth<l by fixpoint: %d, pruned on arrival at depth l: n/a; fix calls %.0f row XORs %.0f; targets %d" % (
                v, l, mean([r["nodes"] for r in q]), mean([r["leaves"] for r in q]), mean([r["leaves"] for r in q]) / 2 ** l,
                sum(sum(r["prune_depth"][:-1]) for r in q), mean([r["fix"] for r in q]), mean([r["xors"] for r in q]), len(q)))

print("\n## subcube refutation cost vs dimension j (fresh solver, cap 20000)\n")
alphas = {}
for l in Ls:
    q = [r for r in rows if r["part"] == "alpha" and r["l"] == l]
    line, xs, ys = [], [], []
    for j in range(l + 1):
        qq = [r for r in q if r["j"] == j]
        o = [r for r in qq if r["status"] == "OK"]
        if not o:
            line.append("j=%d: not run" % j)
            continue
        cens = sum(r["censored"] for r in o) + sum(r["status"] == "SKIPPED_AFTER_CENSOR" for r in qq)
        m = mean([r["conflicts"] for r in o])
        line.append("j=%d: %.0f (x%.2f of 2^j; cens %d/%d)" % (j, m, m / 2 ** j, cens, len(qq)))
        if j >= 2 and cens == 0 and len(o) == len(qq) and m > 0:
            xs.append(j)
            ys.append(math.log2(m))
    a = slope(xs, ys) if len(xs) >= 3 else float("nan")
    alphas[l] = a
    print("l=%d: " % l + "; ".join(line))
    print("   alpha(l=%d) = %.2f over j = %s" % (l, a, xs))

print("\n## decision inputs\n")
lmax = max(Ls)
allr = {("keep",) + k: v for k, v in K.items()}
allr.update({("discard",) + k: v for k, v in D.items()})
atl = {k: v for k, v in allr.items() if k[2] == lmax}
print("largest l:", lmax, "min rho over CMS std arms:", min(v[0] for v in atl.values()), "arms:", len(atl))
print("arms with rho < 1 at largest l:", [(k[0], bname(k[1]), round(v[0], 3)) for k, v in atl.items() if v[0] < 1])
la = max([l for l in Ls if not math.isnan(alphas[l])] or [0])
print("alpha at largest l that has it (l=%d): %s" % (la, alphas.get(la)))
