#!/usr/bin/env python3
"""Tables for the even-degree Smith census from even_census.jsonl.

Layout only: counts and Wilson intervals of recorded 0/1 outcomes; scorecard of the
PROTOCOL.md predictions E1..E5, E1b, E3' with its inputs. No measurement is made here.
"""
import json, math, sys, collections

src = sys.argv[1] if len(sys.argv) > 1 else "even_census.jsonl"
rows = [json.loads(l) for l in open(src) if l.strip()]
cfg = [r for r in rows if r["kind"] == "config"][0]
done = [r for r in rows if r["kind"] == "done"]
allc = [r for r in rows if r["kind"] == "curve"]
C = [r for r in allc if r.get("squarefree") and r.get("smith") is not None]


def wilson(k, n, z=1.96):
    if n == 0:
        return (float("nan"), float("nan"))
    ph = k / n
    d = 1 + z * z / n
    c = ph + z * z / (2 * n)
    h = z * math.sqrt(ph * (1 - ph) / n + z * z / (4 * n * n))
    return (max(0.0, (c - h) / d), min(1.0, (c + h) / d))


def fmt(k, n):
    lo, hi = wilson(k, n)
    return f"{k}/{n} = {k / n:.3f} [{lo:.3f}, {hi:.3f}]" if n else "-"


def ps(r):
    return bool(r["smith"]["full_success"])


def npass(cs):
    return sum(map(ps, cs))


out = []
w = out.append
w("# Smith's conditions on even-degree genus-3 models: tables\n")
w(f"Source `{src}`; config {json.dumps(cfg)}; done {json.dumps(done)}. Every figure `[pcc:unverified]`.\n")
w(f"Curves drawn {len(allc)}, squarefree with Smith analysis {len(C)}, non-squarefree {sum(1 for r in allc if not r.get('squarefree'))}.\n")
primes = sorted({r["p"] for r in C})
det = [r for r in C if r.get("order") is not None]

w("## T1. Per prime\n")
w("| p | curves | order det. | pass, all | admitted h<=8 | pass, admitted | prime order | pass, prime | type (8) | pass, type (8) |")
w("|--:|--:|--:|---|--:|---|--:|---|--:|---|")
for p in primes + ["pooled"]:
    cs = C if p == "pooled" else [r for r in C if r["p"] == p]
    adm = [r for r in cs if r["admitted"]]
    pr = [r for r in cs if r["prime_order"]]
    t8 = [r for r in cs if r["cycle_type_8"] == [8]]
    nd = sum(1 for r in cs if r.get("order") is not None)
    w(f"| {p} | {len(cs)} | {nd} | {fmt(npass(cs), len(cs))} | {len(adm)} | {fmt(npass(adm), len(adm))} | {len(pr)} | {fmt(npass(pr), len(pr))} | {len(t8)} | {fmt(npass(t8), len(t8))} |")

w("\n## T2. By k (irreducible factors of f), pooled\n")
w("| k | curves | pass, all | stable pairing | admitted | pass, admitted | prime order | odd #J |")
w("|--:|--:|---|--:|--:|---|--:|--:|")
byk = collections.defaultdict(list)
for r in C:
    byk[r["k"]].append(r)
for k in sorted(byk):
    cs = byk[k]
    adm = [r for r in cs if r["admitted"]]
    w(f"| {k} | {len(cs)} | {fmt(npass(cs), len(cs))} | {sum(r['stable_matching'] for r in cs)} | {len(adm)} | {fmt(npass(adm), len(adm))} | {sum(r['prime_order'] for r in cs)} | {sum(1 for r in cs if r.get('order') is not None and r['v2'] == 0)} |")

w("\n## T3. By Frobenius cycle type of the 8 Weierstrass points, pooled\n")
w("| cycle type | r2 | r2 brief | stable pairing | curves | pass | admitted | pass, admitted | prime order | v2 range |")
w("|---|--:|--:|---|--:|---|--:|---|--:|---|")
byct = collections.defaultdict(list)
for r in C:
    byct[tuple(r["cycle_type_8"])].append(r)
rate_t = {}
for ct in sorted(byct, key=lambda c: (-len(byct[c]), c)):
    cs = byct[ct]
    adm = [r for r in cs if r["admitted"]]
    rate_t[ct] = npass(cs) / len(cs)
    v2s = [r["v2"] for r in cs if r.get("order") is not None]
    w(f"| {list(ct)} | {cs[0]['r2']} | {cs[0]['r2_brief']} | {cs[0]['stable_matching']} | {len(cs)} | {fmt(npass(cs), len(cs))} | {len(adm)} | {fmt(npass(adm), len(adm))} | {sum(r['prime_order'] for r in cs)} | {min(v2s) if v2s else '-'}..{max(v2s) if v2s else '-'} |")

w("\n## T4. Leading coefficient square class and twist, pooled\n")
w("| lc square | curves | pass (curve) | pass (twist) |")
w("|---|--:|---|---|")
for s in (True, False):
    cs = [r for r in C if r["lc_square"] == s]
    w(f"| {s} | {len(cs)} | {fmt(npass(cs), len(cs))} | {fmt(sum(r['smith']['full_success_twist'] for r in cs), len(cs))} |")

dg = collections.Counter()
for r in C:
    for key in ("diag_fail", "x_agree", "x_disagree", "x_count_error"):
        dg[key] += r["smith"][key]
w(f"\nInstrument diagnostics, summed: {dict(dg)}; order errors: {sum(1 for r in C if r.get('order') is None)}.\n")

w("## Scorecard (PROTOCOL.md section 4)\n")
v1 = [r for r in det if r["v2"] < r["r2_brief"]]
w(f"- E1: v2 >= r2_brief on {len(det)} determined orders, violations {len(v1)}: {'held' if not v1 else 'FAILED'}")
v1b = [r for r in det if r["v2"] < r["r2"] or ((r["v2"] == 0) != (r["r2"] == 0))]
t8det = [r for r in det if r["cycle_type_8"] == [8]]
w(f"- E1b: v2 >= r2 and (v2 = 0 iff r2 = 0) on {len(det)} orders, violations {len(v1b)}; type-(8) curves with odd #J: {sum(1 for r in t8det if r['v2'] == 0)} of {len(t8det)}: {'held' if not v1b else 'FAILED'}")
t8 = byct.get((8,), [])
lo2, hi2 = wilson(npass(t8), len(t8))
w(f"- E2: type (8) {fmt(npass(t8), len(t8))}: {'held' if len(t8) and lo2 > 0 else 'FAILED'}")
pr = [r for r in C if r["prime_order"]]
share8 = sum(1 for r in pr if r["cycle_type_8"] == [8]) / max(1, len(pr))
pred3 = (npass(t8) / max(1, len(t8))) * share8
lo3, hi3 = wilson(npass(pr), len(pr))
e3 = npass(pr) > 0 and lo3 <= pred3 <= hi3
w(f"- E3: prime-order {fmt(npass(pr), len(pr))}; type-(8) share among prime-order {share8:.3f}; predicted rate {pred3:.4f}: {'held' if e3 else 'FAILED'} (positivity {'met' if npass(pr) > 0 else 'not met'}; product inside interval: {lo3 <= pred3 <= hi3})")
prct = collections.Counter(tuple(r["cycle_type_8"]) for r in pr)
e3p = npass(pr) == 0 and all(ct in ((1, 7), (3, 5)) for ct in prct)
w(f"- E3': prime-order passes {npass(pr)}, cycle types {dict((str(list(k)), v) for k, v in prct.items())}: {'held' if e3p else 'FAILED'}")
adm = [r for r in C if r["admitted"]]
la, ha = wilson(npass(adm), len(adm))
lu, hu = wilson(npass(C), len(C))
wt = collections.Counter(tuple(r["cycle_type_8"]) for r in adm)
mix = sum(wt[t] / len(adm) * rate_t[t] for t in wt) if adm else float("nan")
e4a = npass(adm) / max(1, len(adm)) < npass(C) / len(C) and ha < lu
e4b = la <= mix <= ha
w(f"- E4: admitted {fmt(npass(adm), len(adm))} vs all {fmt(npass(C), len(C))} (disjoint & below: {e4a}); cycle-type mixture {mix:.4f} inside admitted interval: {e4b}: {'held' if e4a and e4b else 'FAILED'}")
rate = npass(C) / len(C)
w(f"- E5: unconditioned rate {rate:.4f} [{lu:.4f}, {hu:.4f}] in [0.15, 0.22]: {'held' if 0.15 <= rate <= 0.22 else 'FAILED'}")
print("\n".join(out))
