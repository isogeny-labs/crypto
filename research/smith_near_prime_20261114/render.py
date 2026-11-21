#!/usr/bin/env python3
"""Tables for the near-prime Smith census from near_prime.jsonl.

Layout plus counts and Wilson intervals of recorded 0/1 outcomes; no measurement is made
here. Scorecard of PROTOCOL.md predictions S1..S6 is printed with its inputs.
"""
import json, math, sys, collections

src = sys.argv[1] if len(sys.argv) > 1 else "near_prime.jsonl"
rows = [json.loads(l) for l in open(src) if l.strip()]
cfg = [r for r in rows if r["kind"] == "config"][0]
done = [r for r in rows if r["kind"] == "done"]
C = [r for r in rows if r["kind"] == "curve" and r.get("squarefree") and r.get("smith") is not None]


def wilson(k, n, z=1.96):
    if n == 0:
        return (float("nan"), float("nan"))
    ph = k / n
    d = 1 + z * z / n
    c = ph + z * z / (2 * n)
    h = z * math.sqrt(ph * (1 - ph) / n + z * z / (4 * n * n))
    return ((c - h) / d, (c + h) / d)


def fmt(k, n):
    lo, hi = wilson(k, n)
    return f"{k}/{n} = {k / n:.3f} [{lo:.3f}, {hi:.3f}]" if n else "-"


def passed(r):
    return bool(r["smith"]["full_success"])


out = []
w = out.append
w("# Smith's conditions on near-prime genus-3 Jacobians: tables\n")
w(f"Source `{src}`; config {json.dumps(cfg)}; done {json.dumps(done)}. Every figure `[pcc:unverified]`.\n")
primes = sorted({r["p"] for r in C})
w("## T1. Per prime: all curves, admitted (h <= 8), admitted (h <= 2)\n")
w("| p | curves | order determined | pass, all | admitted h<=8 | pass, admitted | admitted h<=2 | pass, h<=2 |")
w("|--:|--:|--:|---|--:|---|--:|---|")
tot = collections.Counter()
for p in primes:
    cs = [r for r in C if r["p"] == p]
    det = [r for r in cs if r["order"] is not None]
    adm = [r for r in cs if r["admitted"]]
    adm2 = [r for r in cs if r["admitted_h2"]]
    pa, pad, pad2 = sum(map(passed, cs)), sum(map(passed, adm)), sum(map(passed, adm2))
    tot.update({"n": len(cs), "pass": pa, "adm": len(adm), "pass_adm": pad, "adm2": len(adm2), "pass_adm2": pad2, "det": len(det)})
    w(f"| {p} | {len(cs)} | {len(det)} | {fmt(pa, len(cs))} | {len(adm)} | {fmt(pad, len(adm))} | {len(adm2)} | {fmt(pad2, len(adm2))} |")
w(f"| pooled | {tot['n']} | {tot['det']} | {fmt(tot['pass'], tot['n'])} | {tot['adm']} | {fmt(tot['pass_adm'], tot['adm'])} | {tot['adm2']} | {fmt(tot['pass_adm2'], tot['adm2'])} |")

w("\n## T2. Pass rate by k (irreducible factors of f), pooled over primes\n")
w("| k | 2-rank k-1 | curves | pass, all | with stable pairing | admitted h<=8 | pass, admitted | not admitted | pass, not admitted |")
w("|--:|--:|--:|---|--:|--:|---|--:|---|")
byk = collections.defaultdict(list)
for r in C:
    byk[r["k"]].append(r)
rate_k = {}
for k in sorted(byk):
    cs = byk[k]
    adm = [r for r in cs if r["admitted"]]
    non = [r for r in cs if not r["admitted"] and r["order"] is not None]
    pa = sum(map(passed, cs))
    rate_k[k] = pa / len(cs)
    w(f"| {k} | {k - 1} | {len(cs)} | {fmt(pa, len(cs))} | {sum(r['stable_matching'] for r in cs)} | {len(adm)} | {fmt(sum(map(passed, adm)), len(adm))} | {len(non)} | {fmt(sum(map(passed, non)), len(non))} |")

w("\n## T3. Pass rate by Frobenius cycle type on the 8 Weierstrass points, pooled\n")
w("| cycle type | stable pairing | curves | pass | admitted | pass, admitted |")
w("|---|---|--:|---|--:|---|")
byct = collections.defaultdict(list)
for r in C:
    byct[tuple(r["cycle_type_8"])].append(r)
for ct in sorted(byct, key=lambda c: (-len(byct[c]), c)):
    cs = byct[ct]
    adm = [r for r in cs if r["admitted"]]
    w(f"| {list(ct)} | {cs[0]['stable_matching']} | {len(cs)} | {fmt(sum(map(passed, cs)), len(cs))} | {len(adm)} | {fmt(sum(map(passed, adm)), len(adm))} |")

# scorecard
w("\n## Scorecard (PROTOCOL.md section 3)\n")
k1 = byk.get(1, [])
s1 = sum(map(passed, k1))
w(f"- S1: k = 1 curves {len(k1)}, passes {s1}: {'held' if s1 == 0 else 'FAILED'}")
lo_all, hi_all = wilson(tot["pass"], tot["n"])
lo_adm, hi_adm = wilson(tot["pass_adm"], tot["adm"])
s2 = tot["pass_adm"] / max(1, tot["adm"]) < tot["pass"] / max(1, tot["n"]) and hi_adm < lo_all
w(f"- S2: all {fmt(tot['pass'], tot['n'])}; admitted {fmt(tot['pass_adm'], tot['adm'])}: {'held' if s2 else 'FAILED'} (disjoint intervals required)")
adm_all = [r for r in C if r["admitted"]]
wk = collections.Counter(r["k"] for r in adm_all)
pred = sum(wk[k] / len(adm_all) * rate_k[k] for k in wk) if adm_all else float("nan")
s3 = lo_adm <= pred <= hi_adm
w(f"- S3: by-k mixture prediction {pred:.4f} against admitted interval [{lo_adm:.4f}, {hi_adm:.4f}]: {'held' if s3 else 'FAILED'}; admitted k-distribution {dict(sorted(wk.items()))}")
s4_rows = []
s4 = True
for k in sorted(byk):
    adm = [r for r in byk[k] if r["admitted"]]
    non = [r for r in byk[k] if not r["admitted"] and r["order"] is not None]
    if len(adm) >= 40 and len(non) >= 40:
        a = wilson(sum(map(passed, adm)), len(adm))
        b = wilson(sum(map(passed, non)), len(non))
        overlap = not (a[1] < b[0] or b[1] < a[0])
        s4 &= overlap
        s4_rows.append(f"k={k}: admitted [{a[0]:.3f}, {a[1]:.3f}] vs not [{b[0]:.3f}, {b[1]:.3f}] {'overlap' if overlap else 'DISJOINT'}")
w(f"- S4: {'held' if s4 else 'FAILED'}; " + ("; ".join(s4_rows) if s4_rows else "no k with 40 and 40 (vacuous)"))
viol = [r for r in C if r["order"] is not None and r["v2"] < r["k"] - 1]
w(f"- S5: v2(#J) >= k - 1 on {sum(1 for r in C if r['order'] is not None)} determined orders, violations {len(viol)}: {'held' if not viol else 'FAILED'}")
# S6: central 95% binomial range for n = 28 at the admitted rate
rate = tot["pass_adm"] / max(1, tot["adm"])
n = 28
pmf = [math.comb(n, i) * rate ** i * (1 - rate) ** (n - i) for i in range(n + 1)]
cdf = 0.0
lo6 = hi6 = None
for i, q in enumerate(pmf):
    cdf += q
    if lo6 is None and cdf > 0.025:
        lo6 = i
    if hi6 is None and cdf >= 0.975:
        hi6 = i
w(f"- S6: first run saw 1/28; at the admitted rate {rate:.3f} the central 95% range for n = 28 is [{lo6}, {hi6}]: {'held' if lo6 <= 1 <= hi6 else 'FAILED'}")
open("tables.md", "w").write("\n".join(out) + "\n")
print("\n".join(out[-8:]))
