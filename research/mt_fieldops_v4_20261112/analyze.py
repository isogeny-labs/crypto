#!/usr/bin/env python3
"""v4 analysis (rules in PROTOCOL.md).  cost/target GAE = [(cI*I + M + cS*S)/(cI + 2 + cS) + h*(H + L)]/T, cS = 1.
rhobest = cheapest of the 15 retention rho runs (rho2xks x 3 kappa, rhodp x 12) per configuration and (cI, h);
icbest = cheaper of ic5 and ic5ks; bsgs2 at its best mf."""
import json, sys, random, statistics as st, collections
CI = [3, 10, 30]; HP = [0, 0.1, 1]; CS = 1.0
RHOS = ["rho2xks", "rhodp"]; ICS = ["ic5", "ic5ks"]; CANDS = ["icbest", "ic5", "ic5ks"]; REFS = ["rhobest", "rho2xks", "rhodp", "bsgs2"]
def cost(r, ci, h): return ((ci * r["I"] + r["M"] + CS * r["S"]) / (ci + 2 + CS) + h * (r["H"] + r["L"])) / r["targets"]
def boot(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    meds = sorted(st.median(rnd.choices(xs, k=n)) for _ in range(B)); return meds[int(.025 * B)], meds[int(.975 * B) - 1]
def load(path):
    rows = [json.loads(l) for l in open(path)]
    return rows, [r for r in rows if "error" in r or r["ok"] != r["targets"]]
def tune(path):
    rows, bad = load(path); print("rows", len(rows), "bad", len(bad))
    cfg = collections.defaultdict(dict)
    for r in rows: cfg[(r["pb"], r["T"], r["seed"])][(r["param"], r["k"])] = cost(r, 10, 0.1)
    keys = set(k for c in cfg.values() for k in c)
    score = {k: st.mean(c[k] / min(c.values()) for c in cfg.values()) for k in keys}
    for k, v in sorted(score.items(), key=lambda kv: kv[1]): print(f"mu={k[0]} k={k[1]}  mean rel cost {v:.4f}")
    best = min(score, key=score.get); print("BEST", best); return best
def holdout(path, out=sys.stdout):
    rows, bad = load(path); print("rows", len(rows), "bad (error/unverified)", len(bad), file=out)
    cfg = collections.defaultdict(lambda: collections.defaultdict(list))
    for r in rows: cfg[(r["pb"], r["T"], r["idx"], r["seed"])][r["arm"]].append(r)
    cells = sorted({k[:2] for k in cfg}); S = []
    def best(k, a, ci, h):
        rs = ([r for x in RHOS for r in cfg[k][x]] if a == "rhobest" else
              [r for x in ICS for r in cfg[k][x]] if a == "icbest" else cfg[k][a])
        return min(cost(r, ci, h) for r in rs)
    for ci in CI:
        for cand in CANDS:
            refs = [a for a in REFS if a != cand]
            print(f"\n## {cand}, I = {ci} M (S = M)", file=out)
            print("| bits | T | h | " + cand + " GAE/t | " + " | ".join(refs) + " | " + " | ".join(f"{cand}/{a} [95%]" for a in refs) + " | cheaper in |", file=out)
            print("|" + "---|" * (5 + 2 * len(refs)), file=out)
            for pb, T in cells:
                ks = [k for k in cfg if k[:2] == (pb, T)]
                for h in HP:
                    c = {a: [best(k, a, ci, h) for k in ks] for a in [cand] + refs}
                    line = [str(pb), str(T), str(h), f"{st.median(c[cand]):.1f}"] + [f"{st.median(c[a]):.1f}" for a in refs]; wins = []
                    for a in refs:
                        rat = [x / y for x, y in zip(c[cand], c[a])]; lo, hi = boot(rat)
                        line.append(f"{st.median(rat):.3f} [{lo:.3f}, {hi:.3f}]"); wins.append(f"{sum(x < 1 for x in rat)}/{len(rat)}")
                        S.append(dict(cand=cand, cI=ci, bits=pb, T=T, h=h, vs=a, median=st.median(rat), lo=lo, hi=hi,
                                      wins=sum(x < 1 for x in rat), n=len(rat), ic=st.median(c[cand]), ref=st.median(c[a])))
                    line.append(" / ".join(wins)); print("| " + " | ".join(line) + " |", file=out)
    print("\n## Section 5a ranking (cheaper: 95% upper bound < 1 at h = 0, 0.1, 1; dearer: lower bound > 1 at all three)", file=out)
    for cand in CANDS:
        for a in [x for x in REFS if x != cand]:
            for ci in CI:
                res = []
                for pb, T in cells:
                    ss = [s for s in S if s["cand"] == cand and s["cI"] == ci and s["vs"] == a and s["bits"] == pb and s["T"] == T]
                    res.append(f"{pb}b/T{T}: " + ("cheaper" if all(s["hi"] < 1 for s in ss) else ("dearer" if all(s["lo"] > 1 for s in ss) else "unranked")))
                print(f"- {cand} vs {a}, I = {ci} M: " + "; ".join(res), file=out)
    return S
if __name__ == "__main__":
    if sys.argv[1] == "tune": tune(sys.argv[2])
    else:
        s = holdout(sys.argv[2]); json.dump(s, open(sys.argv[3], "w"), indent=1) if len(sys.argv) > 3 else None
