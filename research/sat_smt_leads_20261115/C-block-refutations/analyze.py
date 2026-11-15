#!/usr/bin/env python3
"""raw/*.jsonl.gz -> results.json, tables.md, figures.  Usage: analyze.py [raw files]
With explicit raw files nothing is written except the tables on stdout."""
import sys, os, json, glob, gzip, math, random
from collections import defaultdict
import numpy as np

H = os.path.dirname(os.path.abspath(__file__))
ARGS = sys.argv[1:]
files = ARGS or sorted(glob.glob(os.path.join(H, "raw", "*.jsonl.gz")))
rows = [json.loads(x) for f in files for x in gzip.open(f, "rt")]
WRITE = not ARGS
out_lines = []


def P(*a):
    s = " ".join(str(x) for x in a)
    out_lines.append(s)
    print(s)


mean = lambda v: float(np.mean(v)) if len(v) else float("nan")
NB = 2000


def boot(vals, f=np.mean, seed=1, nb=NB):
    """percentile bootstrap over the entries of vals (one per target)."""
    v = np.asarray(vals, dtype=float)
    rng = np.random.default_rng(seed)
    idx = rng.integers(0, len(v), size=(nb, len(v)))
    st = np.array([f(v[i]) for i in idx])
    return float(np.percentile(st, 2.5)), float(np.percentile(st, 97.5))


def slope(xs, ys):
    xs, ys = np.asarray(xs, float), np.asarray(ys, float)
    if len(xs) < 2:
        return float("nan")
    return float(((xs - xs.mean()) * (ys - ys.mean())).sum() / ((xs - xs.mean()) ** 2).sum())


status = defaultdict(int)
for r in rows:
    status[(r["l"], r["part"], r["status"])] += 1
P("## Row counts by (l, part, status)\n")
for k in sorted(status):
    P("- l=%d %s %s: %d" % (k[0], k[1], k[2], status[k]))

Ls = sorted(set(r["l"] for r in rows))
ok = [r for r in rows if r["status"] == "OK"]
cls = {(r["l"], r["idx"]): r["cls"] for r in ok if r["part"] == "leafclass"}
dfs = {}
for r in ok:
    if r["part"] in ("sweep", "rand", "planted", "explore"):
        dfs.setdefault((r["part"], r["l"], r["idx"], r["budget"], r["rep"], r.get("variant", "")), r)  # first OK row wins
RES = {"protocol": "PROTOCOL.md", "tag": "[pcc:unverified]", "sizes": {str(l): {} for l in Ls}}


# ------------------------------------------------------------------ per-DFS census
def popcount(x):
    return bin(x).count("1")


def census(r, ref0=None, deep=False):
    """Per-DFS census of one non-decomposable run."""
    l, nodes, order = r["l"], r["nodes"], r["order"]
    N = 1 << l
    c = cls.get((l, r["idx"]))
    co = None
    if c is not None:  # class of each leaf in order-space
        co = np.zeros(N, dtype=np.int8)
        for li in range(N):
            x = 0
            for i in range(l):
                if (li >> i) & 1:
                    x |= 1 << order[i]
            co[li] = int(c[x])
    cov = np.zeros(N, bool)
    cov_by = {3: np.zeros(N, bool), 4: np.zeros(N, bool), "l2": np.zeros(N, bool)}
    covlift = np.zeros(N, bool)
    covown = np.zeros(N, bool)   # under an event refuted by its own call
    covzero = np.zeros(N, bool)  # under an event refuted by its own call for 0 conflicts
    ev = []
    maxj = 0
    prof = [0.0] * (l + 1)
    marg = defaultdict(list)
    core_small = [0, 0]
    leaf_core_small = [0, 0]
    for nd in nodes:
        k, pre, res, cf, core, il, ic = nd
        j = l - k
        prof[j] += cf
        if res == 0:
            marg[j].append(cf)
            if k >= 1:
                tgt = leaf_core_small if j == 0 else core_small
                tgt[1] += 1
                tgt[0] += popcount(core) < k
        if j < 2:
            continue
        idx = pre + (np.arange(1 << j) << k)
        cost = il + ic
        if cost < (1 << j):
            cov[idx] = True
            maxj = max(maxj, j)
            if j >= 3:
                cov_by[3][idx] = True
            if j >= 4:
                cov_by[4][idx] = True
            if j >= l - 2:
                cov_by["l2"][idx] = True
            if res == 0:
                covown[idx] = True
                if cf == 0:
                    covzero[idx] = True
            e = {"j": j, "k": k, "pre": pre, "res": res, "cf": cf, "cost": cost, "ic": ic, "core": core}
            if co is not None:
                cc = co[idx]
                e["all0"] = bool((cc == 0).all())
                e["all01"] = bool((cc <= 1).all())
                e["nlift"] = int((cc != 0).sum())
            ev.append(e)
        if co is not None and cost < int((co[idx] != 0).sum()):
            covlift[idx] = True
    d = {"rho": (r["leaves"] + r["conflicts"]) / N, "conf": r["conflicts"] / N,
         "rhop": (r["calls"] + r["conflicts"]) / N, "leaves": r["leaves"], "calls": r["calls"],
         "cov": cov.mean(), "cov3": cov_by[3].mean(), "cov4": cov_by[4].mean(), "covl2": cov_by["l2"].mean(),
         "covlift": covlift.mean(), "covown": covown.mean(), "covzero": covzero.mean(), "maxj": maxj,
         "nev": len(ev), "prof": [p / N for p in prof], "time": r["time"],
         "core_small": core_small, "leaf_core_small": leaf_core_small,
         "marg": {j: v for j, v in marg.items()}, "ev": ev}
    # maximal events and the conflict-only comparison with the budget-0 DFS of the same target and rep
    if ref0 is not None:
        m0 = {(nd[0], nd[1]): nd[6] for nd in ref0["nodes"]}
        evs = sorted(ev, key=lambda e: e["k"])
        top = []
        for e in evs:
            if not any(t["k"] <= e["k"] and (e["pre"] & ((1 << t["k"]) - 1)) == t["pre"] and t is not e for t in top):
                top.append(e)
        d["top_conf"] = sum(e["ic"] for e in top)
        d["top_conf0"] = sum(m0[(e["k"], e["pre"])] for e in top)
        d["top_leaves"] = sum(1 << e["j"] for e in top)
    if deep:
        # was the event's cube already inside the final-conflict cube of an earlier refutation?
        seen = []
        pre_ref = [0, 0, 0, 0]  # own-call events: pre-refuted / total ; of which 0-conflict: pre-refuted / total
        for nd in nodes:
            k, pre, res, cf, core, il, ic = nd
            if res != 0:
                continue
            j = l - k
            if j >= 2 and cf < (1 << j):
                hit = any(cm < (1 << k) and (pre & cm) == vm for cm, vm in seen)
                pre_ref[1] += 1
                pre_ref[0] += hit
                if cf == 0:
                    pre_ref[3] += 1
                    pre_ref[2] += hit
            seen.append((core, pre & core))
        d["pre_ref"] = pre_ref
    return d


CEN = {}
for key, r in dfs.items():
    part, l, idx, b, rep, var = key
    if part in ("sweep", "rand", "explore") and not r["decomposable"]:
        ref0 = dfs.get((part if part != "explore" else "sweep", l, idx, 0, rep, ""))
        if part == "explore":
            ref0 = None
        CEN[key] = census(r, ref0, deep=(part == "sweep" and rep == 0 and b in (1, 3, 10)))


def cell(part, l, b, rep, var=""):
    ks = sorted(k for k in CEN if k[0] == part and k[1] == l and k[3] == b and k[4] == rep and k[5] == var)
    return [CEN[k] for k in ks]


def fmt_ci(v, f="%.3f"):
    lo, hi = boot(v)
    return (f + " (" + f + ".." + f + ")") % (mean(v), lo, hi)


# ------------------------------------------------------------------ (2) census tables
P("\n## (2) Census: share of leaves under events (j >= 2, inclusive leaves + conflicts < 2^j), non-decomposable targets [pcc:unverified]\n")
P("Mean over targets; 95 % bootstrap interval over targets for the primary series (budget 3, rep 0).\n")
allb = sorted(set(k[3] for k in CEN if k[0] == "sweep"))
censb = [0, 1, 2, 3, 5, 10]
for rep in (0, 1):
    P("\nrep %d (%s):\n" % (rep, "canonical numbering" if rep == 0 else "renumbered"))
    P("| budget | " + " | ".join("l=%d" % l for l in Ls) + " |")
    P("|---|" + "---|" * len(Ls))
    for b in censb:
        cs = []
        for l in Ls:
            q = cell("sweep", l, b, rep)
            if not q:
                cs.append("not run")
                continue
            v = [d["cov"] for d in q]
            cs.append((fmt_ci(v) if (b == 3 and rep == 0) else "%.3f" % mean(v)) + " [%d]" % len(q))
        P("| %d | " % b + " | ".join(cs) + " |")
prim = {}
for l in Ls:
    q = cell("sweep", l, 3, 0)
    if q:
        v = [d["cov"] for d in q]
        lo, hi = boot(v)
        prim[l] = (mean(v), lo, hi, len(q))
        RES["sizes"][str(l)]["census_primary_cov_b3_rep0"] = {"mean": mean(v), "ci95": [lo, hi], "targets": len(q)}
lmax = max(Ls)
dec2 = "undecided"
if lmax in prim and len(prim) >= 3:
    seq = [prim[l][0] for l in sorted(prim)]
    nondecr = all(seq[i + 1] >= seq[i] for i in range(len(seq) - 3, len(seq) - 1)) if len(seq) >= 3 else False
    strict_dec = all(seq[i + 1] < seq[i] for i in range(len(seq) - 1))
    if prim[lmax][0] < 0.02 or strict_dec:
        dec2 = "no signal"
    elif prim[lmax][0] >= 0.10 and nondecr:
        dec2 = "signal"
P("\nDecision (2) by the rule: **%s** (primary series %s)" % (dec2, ", ".join("l=%d: %.3f" % (l, prim[l][0]) for l in sorted(prim))))
RES["decision_2"] = {"decision": dec2, "primary": {str(l): prim[l][0] for l in prim}}

P("\n### What the events are (rep 0)\n")
P("| l | budget | rho | conflicts/2^l | rho' | leaves visited | cov | cov, events j>=3 | j>=4 | j>=l-2 | largest event j | cov lift-adjusted | cov under own-call refutations | cov under 0-conflict refutations | events per target |")
P("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for l in Ls:
    for b in censb:
        q = cell("sweep", l, b, 0)
        if not q:
            continue
        g = lambda f: mean([d[f] for d in q])
        P("| %d | %d | %.3f | %.3f | %.3f | %.1f | %.3f | %.3f | %.3f | %.3f | %d | %.3f | %.3f | %.3f | %.1f |" % (
            l, b, g("rho"), g("conf"), g("rhop"), g("leaves"), g("cov"), g("cov3"), g("cov4"), g("covl2"),
            max(d["maxj"] for d in q), g("covlift"), g("covown"), g("covzero"), g("nev")))
        RES["sizes"][str(l)].setdefault("census_rep0", {})[str(b)] = {
            f: g(f) for f in ("rho", "conf", "rhop", "leaves", "cov", "cov3", "cov4", "covl2", "covlift", "covown", "covzero", "nev")}
        RES["sizes"][str(l)]["census_rep0"][str(b)]["max_event_j"] = max(d["maxj"] for d in q)

P("\n### Event dimension histogram (rep 0, all targets pooled): events / share of them refuted by their own call / of those, 0 conflicts\n")
for l in Ls:
    for b in (1, 3, 10):
        q = cell("sweep", l, b, 0)
        if not q:
            continue
        h = defaultdict(lambda: [0, 0, 0])
        for d in q:
            for e in d["ev"]:
                h[e["j"]][0] += 1
                h[e["j"]][1] += e["res"] == 0
                h[e["j"]][2] += e["res"] == 0 and e["cf"] == 0
        P("l=%d b=%d: " % (l, b) + "; ".join("j=%d: %d/%d/%d" % (j, *h[j]) for j in sorted(h)))

P("\n### Conflict-only comparison on maximal events: conflicts spent on the event subcubes / conflicts the budget-0 DFS (same target, same rep) spent on the same subcubes\n")
P("| l | budget | leaves under maximal events | event conflicts | budget-0 conflicts on the same subcubes | ratio | event conflicts per leaf |")
P("|---|---|---|---|---|---|---|")
for l in Ls:
    for b in (1, 2, 3, 5, 10):
        q = [d for d in cell("sweep", l, b, 0) if "top_conf" in d]
        if not q:
            continue
        a, c, lv = sum(d["top_conf"] for d in q), sum(d["top_conf0"] for d in q), sum(d["top_leaves"] for d in q)
        P("| %d | %d | %d | %d | %d | %.3f | %.3f |" % (l, b, lv, a, c, a / max(c, 1), a / max(lv, 1)))
        RES["sizes"][str(l)]["census_rep0"][str(b)]["event_conf_vs_b0"] = a / max(c, 1)
        RES["sizes"][str(l)]["census_rep0"][str(b)]["event_conf_per_leaf"] = a / max(lv, 1)

P("\n### Trivial-cause check (rep 0): share of events whose leaves are all class 0 (x1 does not lift) / all class 0 or 1 (linear (s,p) stage inconsistent for both u), against the base rate over all aligned subcubes with the same j mix\n")
P("| l | budget | events | all class 0 | base rate | all class 0 or 1 | base rate | mean share of class-0 leaves in events | in all leaves |")
P("|---|---|---|---|---|---|---|---|---|")
base_cache = {}


def base_rates(l, idx):
    if (l, idx) not in base_cache:
        c = np.array([int(x) for x in cls[(l, idx)]], dtype=np.int8)
        d = {}
        for j in range(2, l + 1):
            blk = c.reshape(1 << j, 1 << (l - j))  # leaf index = pre + (m << k): rows m, columns pre
            d[j] = (float((blk == 0).all(axis=0).mean()), float((blk <= 1).all(axis=0).mean()))
        base_cache[(l, idx)] = d
    return base_cache[(l, idx)]


for l in Ls:
    for b in (1, 3, 10):
        ks = sorted(k for k in CEN if k[0] == "sweep" and k[1] == l and k[3] == b and k[4] == 0)
        n = a0 = a01 = 0
        br0 = br01 = 0.0
        sh0 = []
        for k in ks:
            br = base_rates(l, k[2])
            for e in CEN[k]["ev"]:
                n += 1
                a0 += e["all0"]
                a01 += e["all01"]
                br0 += br[e["j"]][0]
                br01 += br[e["j"]][1]
                sh0.append(1 - e["nlift"] / (1 << e["j"]))
        if not n:
            continue
        allc0 = mean([cls[(l, k[2])].count("0") / (1 << l) for k in ks])
        P("| %d | %d | %d | %.3f | %.3f | %.3f | %.3f | %.3f | %.3f |" % (l, b, n, a0 / n, br0 / n, a01 / n, br01 / n, mean(sh0), allc0))
        RES["sizes"][str(l)]["census_rep0"][str(b)]["trivial"] = {
            "events": n, "all_class0": a0 / n, "all_class0_base": br0 / n, "all_class01": a01 / n,
            "all_class01_base": br01 / n, "class0_leaf_share_in_events": mean(sh0), "class0_leaf_share": allc0}
P("\nLeaf classes over all targets: " + "; ".join("l=%d: class 0 %.3f, class 1 %.3f, class 2 %.3f" % (
    l, *[mean([cls[k].count(ch) / (1 << l) for k in cls if k[0] == l]) for ch in "012"]) for l in Ls))

P("\n### Assumption-conflict size and attribution (rep 0)\n")
P("| l | budget | own-call refutations at depth k >= 1, j >= 1 | share with final conflict on fewer than k coordinates | leaf refutations | share on fewer than l coordinates | own-call events (j >= 2, cf < 2^j) | share already inside an earlier final-conflict cube | 0-conflict events | share already inside |")
P("|---|---|---|---|---|---|---|---|---|---|")
for l in Ls:
    for b in (0, 1, 3, 10):
        q = cell("sweep", l, b, 0)
        if not q:
            continue
        cs = [sum(d["core_small"][i] for d in q) for i in (0, 1)]
        ls_ = [sum(d["leaf_core_small"][i] for d in q) for i in (0, 1)]
        pr = [sum(d.get("pre_ref", [0, 0, 0, 0])[i] for d in q) for i in range(4)]
        P("| %d | %d | %d | %.3f | %d | %.3f | %d | %s | %d | %s |" % (
            l, b, cs[1], cs[0] / max(cs[1], 1), ls_[1], ls_[0] / max(ls_[1], 1), pr[1],
            "%.3f" % (pr[0] / pr[1]) if pr[1] else "n/a", pr[3], "%.3f" % (pr[2] / pr[3]) if pr[3] else "n/a"))
        RES["sizes"][str(l)].setdefault("cores_rep0", {})[str(b)] = {
            "internal_refutations": cs[1], "share_core_lt_depth": cs[0] / max(cs[1], 1),
            "leaf_refutations": ls_[1], "leaf_share_core_lt_l": ls_[0] / max(ls_[1], 1),
            "own_call_events": pr[1], "share_pre_refuted": pr[0] / pr[1] if pr[1] else None,
            "zero_conflict_events": pr[3], "zero_share_pre_refuted": pr[2] / pr[3] if pr[3] else None}

P("\n### Final assumption conflict of own-call events (rep 0): mean depth k, mean number of x1-coordinate literals in the conflict; the same for 0-conflict events\n")
for l in Ls:
    for b in (1, 3, 10):
        q = cell("sweep", l, b, 0)
        e1 = [(e["k"], popcount(e["core"])) for d in q for e in d["ev"] if e["res"] == 0]
        e0 = [(e["k"], popcount(e["core"])) for d in q for e in d["ev"] if e["res"] == 0 and e["cf"] == 0]
        if e1:
            P("l=%d b=%d: all own-call events n=%d, k %.2f, literals %.2f; 0-conflict n=%d, k %.2f, literals %.2f; 0-conflict events with all k literals: %d" % (
                l, b, len(e1), mean([x[0] for x in e1]), mean([x[1] for x in e1]), len(e0),
                mean([x[0] for x in e0]) if e0 else float("nan"), mean([x[1] for x in e0]) if e0 else float("nan"),
                sum(1 for x in e0 if x[1] == x[0])))
            RES["sizes"][str(l)].setdefault("event_cores_rep0", {})[str(b)] = {
                "n": len(e1), "mean_k": mean([x[0] for x in e1]), "mean_literals": mean([x[1] for x in e1]),
                "n_zero": len(e0), "zero_mean_k": mean([x[0] for x in e0]) if e0 else None,
                "zero_mean_literals": mean([x[1] for x in e0]) if e0 else None}

P("\n### Is the final assumption conflict a merge of cubes the DFS has already finished? (rep 0, own-call refutations at depth k >= 1, leaves included)\n")
P("A conflict that omits a coordinate where the node has value 1 only re-covers the already finished 0-branch; one that omits a coordinate where the node has value 0 refutes leaves the DFS has not reached yet ('ahead').\n")
P("| l | budget | refutations | conflict = exactly the coordinates where the prefix is 0 | conflict omits a 0-coordinate (refutes ahead) | leaves refuted ahead / 2^l per target (upper bound, overlaps counted) |")
P("|---|---|---|---|---|---|")
for l in Ls:
    for b in (0, 1, 3, 10):
        ks = sorted(k for k in dfs if k[0] == "sweep" and k[1] == l and k[3] == b and k[4] == 0 and not dfs[k]["decomposable"])
        n = ex = ah = 0
        vol = 0.0
        for k_ in ks:
            for nd in dfs[k_]["nodes"]:
                k, pre, res, cf, core = nd[:5]
                if res != 0 or k < 1:
                    continue
                zeros = ((1 << k) - 1) & ~pre
                n += 1
                ex += core == zeros
                om = zeros & ~core
                if om:
                    ah += 1
                    vol += 2.0 ** (l - popcount(core)) - 2.0 ** (l - popcount(core | om))
        if n:
            P("| %d | %d | %d | %.3f | %.4f | %.4f |" % (l, b, n, ex / n, ah / n, vol / 2 ** l / max(len(ks), 1)))
            RES["sizes"][str(l)].setdefault("core_merge_rep0", {})[str(b)] = {"refutations": n, "exact_zero_set": ex / n, "ahead": ah / n,
                                                                             "ahead_leaves_per_2l": vol / 2 ** l / max(len(ks), 1)}

P("\n### 0-conflict leaf refutations in the budget-0 scan (rep 0): share of leaves, share of them class 0 (x1 does not lift), share already inside the final-conflict cube of an earlier leaf; conflicts per leaf by class\n")
for l in Ls:
    ks = sorted(k for k in dfs if k[0] == "sweep" and k[1] == l and k[3] == 0 and k[4] == 0 and not dfs[k]["decomposable"])
    n = z = z0 = zin = 0
    cc = {0: [0, 0], 1: [0, 0], 2: [0, 0]}
    for k_ in ks:
        c = cls[(l, k_[2])]
        cubes = []
        for nd in dfs[k_]["nodes"]:
            if nd[0] != l:
                continue
            pre, cf, core = nd[1], nd[3], nd[4]
            n += 1
            ci = int(c[pre])
            cc[ci][0] += 1
            cc[ci][1] += cf
            if cf == 0:
                z += 1
                z0 += ci == 0
                zin += any((pre & cm) == vm for cm, vm in cubes)
            cubes.append((core, pre & core))
    if n:
        P("l=%d: 0-conflict leaves %.3f of leaves; class 0: %.3f of them; inside an earlier cube: %.3f of them; conflicts per leaf class 0 / 1 / 2: %.2f / %.2f / %.2f" % (
            l, z / n, z0 / max(z, 1), zin / max(z, 1), *[cc[i][1] / max(cc[i][0], 1) for i in (0, 1, 2)]))
        RES["sizes"][str(l)]["b0_zero_conflict_leaves_rep0"] = {"share": z / n, "class0_share": z0 / max(z, 1), "inside_earlier_cube": zin / max(z, 1),
                                                               "conflicts_per_leaf_by_class": [cc[i][1] / max(cc[i][0], 1) for i in (0, 1, 2)]}

P("\n### Class composition of the leaves under events against conflicts (rep 0, budget 3): for dimension-2 own-call events, events by number of class-0 leaves (0..4) and mean conflicts\n")
for l in Ls:
    h = defaultdict(list)
    for d in cell("sweep", l, 3, 0):
        for e in d["ev"]:
            if e["j"] == 2 and e["res"] == 0 and "nlift" in e:
                h[4 - e["nlift"]].append(e["cf"])
    if h:
        tot = sum(len(v) for v in h.values())
        P("l=%d: " % l + "; ".join("%d class-0 leaves: %.3f of events, %.2f conflicts" % (c0, len(h[c0]) / tot, mean(h[c0])) for c0 in sorted(h)))

P("\n### Where the conflicts are spent: own-call conflicts / 2^l by node dimension j (rep 0), last column leaves / 2^l\n")
for l in Ls:
    for b in (0, 1, 3, 10):
        q = cell("sweep", l, b, 0)
        if q:
            pf = np.mean([d["prof"] for d in q], axis=0)
            P("l=%d b=%d: " % (l, b) + " ".join("j%d:%.2f" % (j, pf[j]) for j in range(l + 1)) + "  leaves %.2f" % mean([d["leaves"] / 2 ** l for d in q]))

P("\n### Warm marginal cost of own-call refutations by j in the DFS (rep 0, pooled; selection by the budget): n, mean conflicts, share below 2^j, share 0\n")
for l in Ls:
    for b in (3, 10):
        q = cell("sweep", l, b, 0)
        if not q:
            continue
        m = defaultdict(list)
        for d in q:
            for j, v in d["marg"].items():
                m[j] += v
        P("l=%d b=%d: " % (l, b) + "; ".join("j=%d: %d, %.2f, %.2f, %.2f" % (
            j, len(m[j]), mean(m[j]), mean([x < 2 ** j for x in m[j]]), mean([x == 0 for x in m[j]])) for j in sorted(m)))

P("\n### Attempts at dimension 2 by child position (rep 0): first child = called right after its parent's failed attempt; n attempts, share refuted by own call, mean conflicts of the call\n")
for l in Ls:
    for b in (1, 3, 10):
        seg = []
        for child in (0, 1):
            n = suc = cfs = 0
            for k, r in dfs.items():
                if k[0] == "sweep" and k[1] == l and k[3] == b and k[4] == 0 and not r["decomposable"]:
                    for nd in r["nodes"]:
                        if l - nd[0] == 2 and ((nd[1] >> (nd[0] - 1)) & 1) == child:
                            n += 1
                            suc += nd[2] == 0
                            cfs += nd[3]
            if n:
                seg.append("%s child: %d, %.3f, %.2f" % (("first", "second")[child], n, suc / n, cfs / n))
                RES["sizes"][str(l)].setdefault("j2_attempts_rep0", {})["b%d_child%d" % (b, child)] = {"n": n, "refuted": suc / n, "mean_cf": cfs / n}
        if seg:
            P("l=%d b=%d: " % (l, b) + "; ".join(seg))

P("\n### Control: random coordinate order (rep 0)\n")
P("| l | budget | rho | cov | cov j>=3 | largest event j |")
P("|---|---|---|---|---|---|")
for l in Ls:
    for b in (0, 3):
        ks = [k for k in CEN if k[0] == "rand" and k[1] == l and k[3] == b]
        if ks:
            q = [CEN[k] for k in ks]
            P("| %d | %d | %.3f | %.3f | %.3f | %d |" % (l, b, mean([d["rho"] for d in q]), mean([d["cov"] for d in q]),
                                                    mean([d["cov3"] for d in q]), max(d["maxj"] for d in q)))
            RES["sizes"][str(l)].setdefault("rand_order", {})[str(b)] = {"rho": mean([d["rho"] for d in q]), "cov": mean([d["cov"] for d in q])}

# ------------------------------------------------------------------ (4) budgets
P("\n## (4) Budget sweep, non-decomposable targets: rho = (leaves + conflicts)/2^l, [conflicts/2^l], {rho'} [pcc:unverified]\n")
for rep in (0, 1):
    P("\nrep %d:\n" % rep)
    P("| budget | " + " | ".join("l=%d" % l for l in Ls) + " |")
    P("|---|" + "---|" * len(Ls))
    for b in allb:
        cs = []
        for l in Ls:
            q = cell("sweep", l, b, rep)
            cs.append("%.3f [%.3f] {%.3f} (%d)" % (mean([d["rho"] for d in q]), mean([d["conf"] for d in q]),
                                                 mean([d["rhop"] for d in q]), len(q)) if q else "not run")
            if q:
                RES["sizes"][str(l)].setdefault("budget_rep%d" % rep, {})[str(b)] = {
                    "rho": mean([d["rho"] for d in q]), "conf": mean([d["conf"] for d in q]),
                    "rhop": mean([d["rhop"] for d in q]), "targets": len(q),
                    "time_s": mean([d["time"] for d in q])}
        P("| %d | " % b + " | ".join(cs) + " |")


def vec(l, b, rep, f):
    ks = sorted(k for k in CEN if k[0] == "sweep" and k[1] == l and k[3] == b and k[4] == rep)
    return {k[2]: CEN[k][f] for k in ks}


P("\n### Budget 3 against budget 0 and the A/A interval\n")
P("| l | unit | E_0 | E_1 | E_2 | E_3 | A/A 95 % interval | A/A spread: budget 0 pairs / budget 3 pairs (max abs d) | all E_1..E_3 below the interval |")
P("|---|---|---|---|---|---|---|---|---|")
outside = {}
for l in Ls:
    for f in ("rho", "conf", "rhop"):
        V = {(b, r): vec(l, b, r, f) for b in (0, 3) for r in (0, 1, 2, 3)}
        if any(len(V[k]) == 0 for k in V):
            continue
        ids = sorted(set.intersection(*[set(v) for v in V.values()]))
        A = {k: np.array([V[k][i] for i in ids]) for k in V}
        E = [float((A[(3, r)] - A[(0, r)]).mean()) for r in range(4)]
        rng = np.random.default_rng(l * 100 + len(f))
        pairs = [(r, s) for r in (1, 2, 3) for s in (1, 2, 3) if r != s]
        st = []
        for _ in range(4000):
            ix = rng.integers(0, len(ids), len(ids))
            b = (0, 3)[rng.integers(0, 2)]
            r, s = pairs[rng.integers(0, len(pairs))]
            st.append(float((A[(b, r)][ix] - A[(b, s)][ix]).mean()))
        lo, hi = float(np.percentile(st, 2.5)), float(np.percentile(st, 97.5))
        sp = [max(abs(float((A[(b, r)] - A[(b, s)]).mean())) for r, s in pairs) for b in (0, 3)]
        outb = all(e < lo for e in E[1:])
        outa = all(e > hi for e in E[1:])
        if f == "rho":
            outside[l] = outb
        Eci = [boot(A[(3, r)] - A[(0, r)], seed=r + 5) for r in range(4)]
        P("| %d | %s | %s | %.4f..%.4f | %.4f / %.4f | %s |" % (
            l, {"rho": "rho", "conf": "conflicts/2^l", "rhop": "rho'"}[f],
            " | ".join("%+.4f (%+.3f..%+.3f)" % (E[r], *Eci[r]) for r in range(4)), lo, hi, sp[0], sp[1],
            "yes" if outb else ("no (all above)" if outa else "no")))
        RES["sizes"][str(l)].setdefault("b3_vs_b0", {})[f] = {"E": E, "E_ci95": Eci, "AA_ci95": [lo, hi],
                                                             "AA_max_abs_d_b0": sp[0], "AA_max_abs_d_b3": sp[1],
                                                             "all_below": outb, "all_above": outa, "targets": len(ids)}
n_out = sum(1 for l in (6, 7, 8) if outside.get(l))
dec4 = "budget-3 edge real" if n_out >= 2 else "budget-3 edge not shown above the A/A floor"
P("\nDecision (4) by the rule (unit rho): **%s** (outside at %d of l = 6, 7, 8: %s; l = 9 reported: %s)" % (
    dec4, n_out, {l: outside.get(l) for l in (6, 7, 8)}, outside.get(9)))
RES["decision_4"] = {"decision": dec4, "outside": {str(l): outside.get(l) for l in Ls}}

P("\n### Every budget against budget 0, same rep: mean paired difference of rho (95 % bootstrap over targets), rep 0 / rep 1\n")
for l in Ls:
    line = []
    for b in allb:
        if b == 0:
            continue
        seg = []
        for rep in (0, 1):
            a, c = vec(l, b, rep, "rho"), vec(l, 0, rep, "rho")
            ids = sorted(set(a) & set(c))
            if ids:
                dd = np.array([a[i] - c[i] for i in ids])
                seg.append("%+.3f (%+.3f..%+.3f)" % (dd.mean(), *boot(dd, seed=b)))
        if seg:
            line.append("b=%d: " % b + " / ".join(seg))
    P("l=%d: " % l + "; ".join(line))

P("\n### Planted (decomposable) targets, rep 0, reported separately: rho, leaves visited, conflicts\n")
for l in Ls:
    seg = []
    for b in (0, 3, 10):
        q = [r for k, r in dfs.items() if k[0] == "planted" and k[1] == l and k[3] == b]
        if q:
            seg.append("b=%d: %.3f, %.1f, %.1f (%d targets)" % (b, mean([(r["leaves"] + r["conflicts"]) / 2 ** l for r in q]),
                                                              mean([r["leaves"] for r in q]), mean([r["conflicts"] for r in q]), len(q)))
            RES["sizes"][str(l)].setdefault("planted", {})[str(b)] = {"rho": mean([(r["leaves"] + r["conflicts"]) / 2 ** l for r in q]), "targets": len(q)}
    P("l=%d: " % l + "; ".join(seg))

# ------------------------------------------------------------------ (3) alpha
P("\n## (3) Alpha: conflicts to refute one subcube of dimension j [pcc:unverified]\n")
al = [r for r in rows if r["part"] == "alpha"]
ALPHA = {}
for arm in ("cold", "warm32", "warmq"):
    P("\n%s: mean conflicts (multiple of 2^j; censored or skipped samples / samples)\n" % arm)
    P("| l | " + " | ".join("j=%d" % j for j in range(7)) + " | alpha, j >= 2 (95 % bootstrap) [j used] | alpha, j >= 1 | targets |")
    P("|---|" + "---|" * 10)
    for l in Ls:
        q = [r for r in al if r["l"] == l and r["arm"] == arm]
        if not q:
            continue
        tids = sorted(set(r["idx"] for r in q))
        byj = defaultdict(lambda: defaultdict(list))
        cens = defaultdict(int)
        tot = defaultdict(int)
        for r in q:
            if r["status"] == "NOT_IN_PROTOCOL":
                continue
            tot[r["j"]] += 1
            if r["status"] != "OK" or r["censored"]:
                cens[r["j"]] += 1
            if r["status"] == "OK":
                byj[r["j"]][r["idx"]].append(r["conflicts"])
        cellsj = []
        for j in range(7):
            if not tot[j]:
                cellsj.append("-")
                continue
            allv = [x for v in byj[j].values() for x in v]
            m = mean(allv) if allv else float("nan")
            cellsj.append("%s%.1f (%.2fx; %d/%d)" % (">= " if cens[j] else "", m, m / 2 ** j, cens[j], tot[j]))

        def fit(jmin, ids):
            js = [j for j in sorted(tot) if j >= jmin and tot[j] and cens[j] == 0]
            if len(js) < 3:
                return float("nan"), js
            ys = []
            for j in js:
                vals = [x for i in ids for x in byj[j].get(i, [])]
                ys.append(math.log2(max(float(np.mean(vals)) if vals else 0.0, 1 / 16)))
            return slope(js, ys), js

        a2, js2 = fit(2, tids)
        a1, js1 = fit(1, tids)
        rng = random.Random(l * 7 + len(arm))
        bs2, bs1 = [], []
        if not math.isnan(a2) or not math.isnan(a1):
            for _ in range(NB):
                ids = [rng.choice(tids) for _ in tids]
                bs2.append(fit(2, ids)[0])
                bs1.append(fit(1, ids)[0])
        ci = lambda bs: (float(np.percentile(bs, 2.5)), float(np.percentile(bs, 97.5))) if bs and not math.isnan(bs[0]) else (float("nan"),) * 2
        c2, c1 = ci(bs2), ci(bs1)
        ALPHA[(arm, l)] = (a2, c2, js2, a1, c1, js1)
        P("| %d | " % l + " | ".join(cellsj) + " | %.2f (%.2f..%.2f) %s | %.2f (%.2f..%.2f) | %d |" % (a2, c2[0], c2[1], js2, a1, c1[0], c1[1], len(tids)))
        RES["sizes"][str(l)].setdefault("alpha", {})[arm] = {
            "alpha_j_ge_2": a2, "ci95_j_ge_2": list(c2), "j_used": js2, "alpha_j_ge_1": a1, "ci95_j_ge_1": list(c1),
            "mean_conflicts_by_j": {str(j): mean([x for v in byj[j].values() for x in v]) for j in sorted(byj)},
            "censored_by_j": {str(j): cens[j] for j in sorted(tot)}, "samples_by_j": {str(j): tot[j] for j in sorted(tot)},
            "targets": len(tids)}
        if arm != "cold":
            wc = [r["warm_conflicts"] for r in q if r["status"] == "OK"]
            RES["sizes"][str(l)]["alpha"][arm]["mean_warmup_conflicts"] = mean(wc)
ub = {arm: ALPHA.get((arm, lmax), (float("nan"), (float("nan"),) * 2))[1][1] for arm in ("warm32", "warmq")}
dec3 = "sub-scan exponent" if all((not math.isnan(u)) and u < 0.9 for u in ub.values()) else "no sub-scan exponent"
P("\nDecision (3) by the rule: **%s** (upper bounds of the warm alpha interval at l = %d: %s)" % (
    dec3, lmax, ", ".join("%s %.2f" % (a, u) for a, u in ub.items())))
RES["decision_3"] = {"decision": dec3, "warm_alpha_upper_at_lmax": ub}

# ------------------------------------------------------------------ (5) exploratory
ex = sorted(set((k[5], k[3]) for k in CEN if k[0] == "explore"))
if ex:
    P("\n## (5) Exploratory arm (no rule) [pcc:unverified]\n")
    P("| variant | budget | rep | " + " | ".join("l=%d rho [conflicts/2^l] {rho'} (targets; leaves visited; share of dimension-2 attempts refuted)" % l for l in Ls) + " |")
    P("|---|---|---|" + "---|" * len(Ls))
    refs = [("reference: budget-0 arm", 0), ("reference: budget-3 arm", 3)]
    for var, b in [(v, bb) for v, bb in ex] + refs:
        for rep in (0, 1):
            cs = []
            for l in Ls:
                part = "sweep" if var.startswith("reference") else "explore"
                vv = "" if part == "sweep" else var
                q = [CEN[k] for k in CEN if k[0] == part and k[1] == l and k[3] == b and k[4] == rep and k[5] == vv]
                if not q:
                    cs.append("not run")
                    continue
                n2 = s2 = 0
                for k, r in dfs.items():
                    if k[0] == part and k[1] == l and k[3] == b and k[4] == rep and k[5] == vv and not r["decomposable"]:
                        for nd in r["nodes"]:
                            if l - nd[0] == 2 and (part == "sweep" or var != "coreskip"):
                                n2 += 1
                                s2 += nd[2] == 0
                cs.append("%.3f [%.3f] {%.3f} (%d; %.1f; %s)" % (mean([d["rho"] for d in q]), mean([d["conf"] for d in q]),
                          mean([d["rhop"] for d in q]), len(q), mean([d["leaves"] for d in q]), "%.3f" % (s2 / n2) if n2 and b else "n/a"))
                if part == "explore":
                    RES["sizes"][str(l)].setdefault("explore", {})["%s_rep%d" % (var, rep)] = {
                        "rho": mean([d["rho"] for d in q]), "conf": mean([d["conf"] for d in q]),
                        "rhop": mean([d["rhop"] for d in q]), "leaves": mean([d["leaves"] for d in q]),
                        "j2_refuted": (s2 / n2) if n2 and b else None, "targets": len(q),
                        "rho_ci95": list(boot([d["rho"] for d in q]))}
            P("| %s | %d | %d | " % (var, b, rep) + " | ".join(cs) + " |")

if WRITE:
    RES["row_counts"] = {"%d/%s/%s" % k: v for k, v in status.items()}
    json.dump(RES, open(os.path.join(H, "results.json"), "w"), indent=1, default=float)
    open(os.path.join(H, "tables.md"), "w").write("\n".join(out_lines) + "\n")

    # -------------------------------------------------------------- figures
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    INK, MUTED, SURF = "#0b0b0b", "#52514e", "#fcfcfb"
    COL = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4"]
    plt.rcParams.update({"font.size": 10, "axes.edgecolor": MUTED, "axes.labelcolor": INK, "xtick.color": MUTED,
                         "ytick.color": MUTED, "axes.spines.top": False, "axes.spines.right": False,
                         "figure.facecolor": SURF, "axes.facecolor": SURF, "savefig.facecolor": SURF,
                         "svg.fonttype": "none"})

    def finish(fig, name):
        fig.tight_layout()
        fig.savefig(os.path.join(H, name + ".svg"))
        fig.savefig(os.path.join(H, name + ".png"), dpi=160)
        plt.close(fig)

    fig, axs = plt.subplots(1, 2, figsize=(11, 5.6))
    ax = axs[0]
    series = [(1, "budget 1", "o"), (3, "budget 3 (primary)", "s"), (10, "budget 10", "^")]
    for i, (b, lab, mk) in enumerate(series):
        xs = [l for l in Ls if cell("sweep", l, b, 0)]
        ys = [mean([d["cov"] for d in cell("sweep", l, b, 0)]) for l in xs]
        ax.plot(xs, ys, color=COL[i], lw=2, marker=mk, ms=7, label=lab)
        if b == 3:
            ci = [boot([d["cov"] for d in cell("sweep", l, b, 0)]) for l in xs]
            ax.vlines(xs, [c[0] for c in ci], [c[1] for c in ci], color=COL[i], lw=1.5)
        ax.annotate("%.2f" % ys[-1], (xs[-1], ys[-1]), textcoords="offset points", xytext=(6, 0), color=INK, fontsize=9, va="center")
    xs = [l for l in Ls if cell("sweep", l, 3, 0)]
    ax.plot(xs, [mean([d["cov3"] for d in cell("sweep", l, 3, 0)]) for l in xs], color=COL[3], lw=2, ls="--", marker="D", ms=6,
            label="budget 3, events of dimension >= 3 only")
    ax.plot(xs, [mean([d["covlift"] for d in cell("sweep", l, 3, 0)]) for l in xs], color=COL[4], lw=2, ls=":", marker="v", ms=6,
            label="budget 3, lift-adjusted events")
    ax.axhline(0.10, color=MUTED, lw=0.8, ls=(0, (4, 4)))
    ax.text(Ls[0] + 0.12, 0.065, "rule: 10 %", color=MUTED, fontsize=8)
    ax.set_xticks(Ls)
    ax.set_ylim(0, 1.02)
    ax.set_xlabel("l (dimension of V)")
    ax.set_ylabel("share of leaves under an event")
    ax.set_title("Leaves under cheap subcube events (j >= 2, cost < 2^j)", fontsize=10, color=INK, loc="left")
    ax.grid(axis="y", color="#e4e3df", lw=0.6)
    ax.legend(frameon=False, fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2)
    ax = axs[1]
    for i, (b, lab, mk) in enumerate([(0, "budget 0 (the scan done by CMS)", "o"), (3, "budget 3", "s"), (10, "budget 10", "^")]):
        xs = [l for l in Ls if cell("sweep", l, b, 0)]
        ys = [mean([d["rho"] for d in cell("sweep", l, b, 0)]) for l in xs]
        yc = [mean([d["conf"] for d in cell("sweep", l, b, 0)]) for l in xs]
        ax.plot(xs, ys, color=COL[i], lw=2, marker=mk, ms=7, label=lab + ": (leaves + conflicts) / 2^l")
        ax.plot(xs, yc, color=COL[i], lw=1.5, ls="--", marker=mk, ms=5, mfc=SURF, label=lab + ": conflicts / 2^l")
    ax.axhline(1.0, color=INK, lw=1)
    ax.text(Ls[0], 0.72, "linear 2+1 scan = 1", color=INK, fontsize=8)
    ax.set_xticks(Ls)
    ax.set_ylim(0, None)
    ax.set_xlabel("l (dimension of V)")
    ax.set_ylabel("cost at the root / 2^l")
    ax.set_title("Root total of the same runs", fontsize=10, color=INK, loc="left")
    ax.grid(axis="y", color="#e4e3df", lw=0.6)
    ax.legend(frameon=False, fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=1)
    fig.suptitle("Lead C pilot, 24 non-decomposable targets per size, rep 0; unverified pilot numbers", fontsize=9, color=MUTED, x=0.01, ha="left")
    finish(fig, "fig_covered_fraction_vs_l")

    fig, ax = plt.subplots(figsize=(6.4, 4.2))
    off = {"cold": -0.08, "warm32": 0.0, "warmq": 0.08}
    labs = {"cold": "cold (fresh solver), fit over j = 1..3 (j >= 4 censored)", "warm32": "warm, 32 leaves outside first, j >= 2",
            "warmq": "warm, 2^(l-2) leaves outside first, j >= 2"}
    for i, arm in enumerate(("cold", "warm32", "warmq")):
        pick = (lambda t: (t[3], t[4])) if arm == "cold" else (lambda t: (t[0], t[1]))
        pts = [(l, pick(ALPHA[(arm, l)])) for l in Ls if (arm, l) in ALPHA and not math.isnan(pick(ALPHA[(arm, l)])[0])]
        if not pts:
            continue
        xs = [l + off[arm] for l, _ in pts]
        ys = [a[0] for _, a in pts]
        ax.plot(xs, ys, color=COL[i], lw=2, marker="os^"[i], ms=7, label=labs[arm])
        ax.vlines(xs, [a[1][0] for _, a in pts], [a[1][1] for _, a in pts], color=COL[i], lw=1.5)
        ax.annotate("%.2f" % ys[-1], (xs[-1], ys[-1]), textcoords="offset points", xytext=(7, 0), color=INK, fontsize=9, va="center")
    ax.axhline(1.0, color=INK, lw=1)
    ax.text(Ls[0] - 0.1, 1.02, "alpha = 1: cost doubles with each free coordinate (the scan)", color=INK, fontsize=8)
    ax.axhline(0.9, color=MUTED, lw=0.8, ls=(0, (4, 4)))
    ax.text(Ls[0] - 0.1, 0.78, "rule: warm upper bound < 0.9", color=MUTED, fontsize=8)
    ax.set_xticks(Ls)
    ax.set_xlabel("l (dimension of V)")
    ax.set_ylabel("alpha: slope of log2(mean conflicts) in j")
    ax.set_title("Exponent of subcube refutation cost (95 % bootstrap over targets)", fontsize=10, color=INK, loc="left")
    ax.grid(axis="y", color="#e4e3df", lw=0.6)
    ax.set_ylim(0, None)
    ax.legend(frameon=False, fontsize=8, loc="lower right")
    finish(fig, "fig_alpha_vs_l")
