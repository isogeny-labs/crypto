"""Analysis fixed by PROTOCOL.md: per-round arm sums, geometric-mean ratios with a
95% percentile bootstrap over rounds (10,000 resamples, seed 20261114), A/A check."""
import json, math, random, statistics as st, pathlib, sys
here = pathlib.Path(__file__).resolve().parent
rows = here / "rows"
CELLS = [(26, 20), (26, 200), (26, 1000), (28, 200), (28, 1000), (28, 3000), (30, 200), (30, 1000), (30, 3000),
         (32, 200), (32, 1000), (32, 3000), (34, 200), (34, 1000), (34, 3000),
         (36, 200), (36, 1000), (36, 3000),
         (38, 200), (38, 1000), (38, 3000),
         (40, 200), (40, 1000), (40, 3000)]
CELLS = [c for c in CELLS if (rows / f"rho_b{c[0]}_T{c[1]}_r0.jsonl").exists()]
def load(arm, b, T, r):
    return [json.loads(l) for l in open(rows / f"{arm}_b{b}_T{T}_r{r}.jsonl")]
def boot(xs, seed=20261114, n=10000):
    rng = random.Random(seed); logs = [math.log(x) for x in xs]
    g = sorted(math.exp(sum(rng.choice(logs) for _ in logs) / len(logs)) for _ in range(n))
    return math.exp(sum(logs) / len(logs)), g[int(0.025 * n)], g[int(0.975 * n) - 1]
out = []
for b, T in CELLS:
    W = math.sqrt(2 * (2**b / 6) * T)
    rb, aa, sums, excluded, steps, giants = [], [], {"rho": [], "bsgs": [], "rho_aa": []}, [], [], []
    for r in range(6):
        d = {a: load(a, b, T, r) for a in ("rho", "bsgs", "rho_aa")}
        if not all(len(v) == 4 and all(x["ok"] == T for x in v) for v in d.values()):
            excluded.append(r); continue
        s = {"rho": sum(x["folded_multi_ms"] for x in d["rho"]), "rho_aa": sum(x["folded_multi_ms"] for x in d["rho_aa"]),
             "bsgs": sum(x["folded_bsgs_ms"] for x in d["bsgs"])}
        for k in s: sums[k].append(s[k])
        rb.append(s["rho"] / s["bsgs"]); aa.append(s["rho_aa"] / s["rho"])
        steps += [x["steps"] / W for x in d["rho"]]; giants += [x["giant_steps"] for x in d["bsgs"]]
    g, lo, hi = boot(rb); ga, alo, ahi = boot(aa)
    verdict = "rho faster" if hi < 1 else "BSGS faster" if lo > 1 else "unresolved"
    out.append(dict(bits=b, T=T, rounds=len(rb), excluded=excluded,
                    rho_ms=st.median(sums["rho"]) / 4, bsgs_ms=st.median(sums["bsgs"]) / 4,
                    rho_over_bsgs=g, lo=lo, hi=hi, aa=ga, aa_lo=alo, aa_hi=ahi, aa_ok=alo <= 1 <= ahi,
                    verdict=verdict if alo <= 1 <= ahi else verdict + " (A/A failed: unreliable)",
                    rho_steps_over_W=st.median(steps), bsgs_giant_steps=st.median(giants), bsgs_m=d["bsgs"][0]["m"]))
(here / "summary.json").write_text(json.dumps(out, indent=1))
print("| bits | T | rounds | rho ms/problem | BSGS ms/problem | rho/BSGS [95%] | A/A [95%] | verdict | rho steps/W |")
print("|---|---|---|---|---|---|---|---|---|")
for o in out:
    print(f"| {o['bits']} | {o['T']} | {o['rounds']} | {o['rho_ms']:.1f} | {o['bsgs_ms']:.1f} | {o['rho_over_bsgs']:.2f} [{o['lo']:.2f}, {o['hi']:.2f}] | "
          f"{o['aa']:.2f} [{o['aa_lo']:.2f}, {o['aa_hi']:.2f}] | {o['verdict']} | {o['rho_steps_over_W']:.2f} |")
