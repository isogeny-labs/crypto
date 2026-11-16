#!/usr/bin/env python3
"""Analysis for the field-op-counted multi-target panel (rules in PROTOCOL.md).
cost per target in GAE = [(cI*I + M + cS*S)/(cI + 2 + cS) + h*(H + L)] / T, cS = 1.
Generic arms take their best parameter per configuration at each (cI, h) (favours them)."""
import json, sys, random, statistics as st, collections
CI = [3, 10, 30]; HP = [0, 0.1, 1]; CS = 1.0
def cost(r, ci, h): return ((ci * r["I"] + r["M"] + CS * r["S"]) / (ci + 2 + CS) + h * (r["H"] + r["L"])) / r["targets"]
def boot(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    meds = sorted(st.median(rnd.choices(xs, k=n)) for _ in range(B)); return meds[int(.025 * B)], meds[int(.975 * B) - 1]
def load(path):
    rows = [json.loads(l) for l in open(path)]
    bad = [r for r in rows if "error" in r or r["ok"] != r["targets"]]
    return rows, bad
def tune(path):
    rows, bad = load(path); print("rows", len(rows), "bad", len(bad))
    by = collections.defaultdict(list)
    for r in rows: by[(r["param"], r["g"])].append(r)
    # objective (frozen): sum over tune configs of cost at cI = 10, h = 0.1, relative to the per-config best
    cfg = collections.defaultdict(dict)
    for r in rows: cfg[(r["pb"], r["T"], r["seed"])][(r["param"], r["g"])] = cost(r, 10, 0.1)
    score = {k: st.mean(c[k] / min(c.values()) for c in cfg.values()) for k in by}
    for k, v in sorted(score.items(), key=lambda kv: kv[1]): print(f"mu={k[0]} g={k[1]}  mean rel cost {v:.4f}")
    best = min(score, key=score.get); print("BEST", best); return best
def holdout(path, out=sys.stdout):
    rows, bad = load(path); print("rows", len(rows), "bad (error/unverified)", len(bad), file=out)
    cfg = collections.defaultdict(lambda: collections.defaultdict(list))
    for r in rows: cfg[(r["pb"], r["T"], r["idx"], r["seed"])][r["arm"]].append(r)
    cells = sorted({(k[0], k[1]) for k in cfg})
    summary = []
    for ci in CI:
        print(f"\n## I = {ci} M (S = M); GAE = I + 2M + S", file=out)
        print("| bits | T | h | IC GAE/target | rho | rho2 | bsgs2 | IC/rho [95%] | IC/rho2 [95%] | IC/bsgs2 [95%] | IC cheaper vs rho / rho2 / bsgs2 |", file=out)
        print("|---|---|---|---|---|---|---|---|---|---|---|", file=out)
        for pb, T in cells:
            for h in HP:
                ks = [k for k in cfg if k[:2] == (pb, T)]
                c = {a: [] for a in ["ic4", "rho", "rho2", "bsgs2"]}
                for k in ks:
                    for a in c: c[a].append(min(cost(r, ci, h) for r in cfg[k][a]))
                line = [str(pb), str(T), str(h), f"{st.median(c['ic4']):.1f}"] + [f"{st.median(c[a]):.1f}" for a in ["rho", "rho2", "bsgs2"]]
                wins = []
                for a in ["rho", "rho2", "bsgs2"]:
                    rat = [x / y for x, y in zip(c["ic4"], c[a])]; lo, hi = boot(rat)
                    line.append(f"{st.median(rat):.3f} [{lo:.3f}, {hi:.3f}]"); wins.append(f"{sum(x < 1 for x in rat)}/{len(rat)}")
                    summary.append(dict(cI=ci, bits=pb, T=T, h=h, vs=a, median=st.median(rat), lo=lo, hi=hi, wins=sum(x < 1 for x in rat), n=len(rat), ic=st.median(c["ic4"]), ref=st.median(c[a])))
                line.append(" / ".join(wins)); print("| " + " | ".join(line) + " |", file=out)
    # Section 5a ranking: IC ranked below arm X in a cell (at a given cI) only if hi < 1 at every h
    print("\n## Section 5a ranking (IC cheaper with the 95% upper bound < 1 at h = 0, 0.1 and 1)", file=out)
    for ci in CI:
        for a in ["rho", "rho2", "bsgs2"]:
            res = []
            for pb, T in cells:
                ss = [s for s in summary if s["cI"] == ci and s["vs"] == a and s["bits"] == pb and s["T"] == T]
                res.append(f"{pb}b/T{T}: " + ("IC cheaper" if all(s["hi"] < 1 for s in ss) else ("IC dearer" if all(s["lo"] > 1 for s in ss) else "unranked")))
            print(f"- I = {ci} M, vs {a}: " + "; ".join(res), file=out)
    return summary
if __name__ == "__main__":
    if sys.argv[1] == "tune": tune(sys.argv[2])
    else:
        s = holdout(sys.argv[2]); json.dump(s, open(sys.argv[3], "w"), indent=1) if len(sys.argv) > 3 else None
