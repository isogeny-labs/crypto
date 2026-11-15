#!/usr/bin/env python3
"""Gates of PROTOCOL.md on results/holdout/runs.jsonl (written before the holdout data was read).
Key time = median of its repeats of ms_arm; per configuration ratios; cell = median of 4 configurations; a cell
decision is a 4-of-4 sign rule."""
import json, os, statistics, sys, collections
D = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(D, "results", "holdout", "runs.jsonl"))]
errs = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
cfg = collections.defaultdict(lambda: collections.defaultdict(list))
for r in rows:
    cfg[(r["pb"], r["T"], r["idx"])][r["key"]].append(r)
bad = []
for c, kk in cfg.items():
    for k, rs in kk.items():
        if k.startswith("rhodpup:"):
            ref = {tuple(r[x] for x in "IMSHLN") for r in kk["rhodpu:" + k.split(":", 1)[1]]}
            got = {tuple(r[x] for x in "IMSHLN") for r in rs}
            if got != ref or len(ref) != 1: bad.append((c, k))
print(f"rows {len(rows)}; errors/unverified {len(errs)}; rhodpup/rhodpu count mismatches {len(bad)}")
if errs or bad:
    print("CORRECTNESS FAIL: reading stops", errs[:3], bad[:5]); sys.exit(1)
def t(rs): return statistics.median(r["ms_arm"] for r in rs)
TUNED = ("rho2xksp:", "rhodpup:"); UNTUNED = ("rho2xks:", "rhodpu:")
out = {}
print("\n| bits | T | IC-B / best rho | IC-B / best untuned rho | tuned / untuned rho | IC-A / best rho | best rho family (configs) |")
print("|---|---|---|---|---|---|---|")
for pb in (24, 28, 32):
    for T in (1024, 4096, 16384):
        cs = [cfg[(pb, T, i)] for i in range(4) if (pb, T, i) in cfg]
        def best(c, pref): return min(t(v) for k, v in c.items() if k.startswith(pref))
        def fam(c):
            k = min((k for k in c if k.startswith(TUNED + UNTUNED)), key=lambda k: t(c[k])); return k.split(":")[0]
        cell = dict(
            icb_best=[t(c["IC-B"]) / best(c, TUNED + UNTUNED) for c in cs],
            icb_untuned=[t(c["IC-B"]) / best(c, UNTUNED) for c in cs],
            tuned_untuned=[best(c, TUNED) / best(c, UNTUNED) for c in cs],
            ica_best=[t(c["IC-A"]) / best(c, TUNED + UNTUNED) for c in cs])
        fams = collections.Counter(fam(c) for c in cs)
        out[f"{pb}:{T}"] = {k: dict(values=v, median=statistics.median(v)) for k, v in cell.items()}
        out[f"{pb}:{T}"]["best_family"] = dict(fams)
        m = lambda k: statistics.median(cell[k])
        print(f"| {pb} | {T} | {m('icb_best'):.3f} | {m('icb_untuned'):.3f} | {m('tuned_untuned'):.3f} | {m('ica_best'):.3f} | {dict(fams)} |")
v32 = [out[f"32:{T}"]["icb_best"]["values"] for T in (1024, 4096, 16384)]
ahead = sum(all(x <= 1 for x in v) for v in v32) >= 2
behind = all(all(x >= 1 for x in v) for v in v32)
verdict = "IC ahead" if ahead else "IC behind" if behind else "mixed"
print(f"\nPRIMARY (IC-B / best rho at 2^32): {verdict}")
tun32 = [all(x < 1 for x in out[f'32:{T}']['tuned_untuned']['values']) for T in (1024, 4096, 16384)]
print(f"SECONDARY: tuned rho faster than untuned rho at 2^32 (4 of 4) per T cell: {tun32}")
json.dump(dict(cells=out, primary=verdict, tuned_faster_32=tun32), open(os.path.join(D, "summary.json"), "w"), indent=1)
