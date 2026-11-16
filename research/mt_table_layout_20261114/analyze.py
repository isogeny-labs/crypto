#!/usr/bin/env python3
"""Gates of PROTOCOL.md on results/holdout/runs.jsonl (written before the holdout data was read).
Key time = median of its repeats; per configuration a ratio; cell value = median over the cell's 4 configurations;
a cell decision is a 4-of-4 sign rule (all configurations on one side of the threshold)."""
import json, os, statistics, sys, collections
D = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(D, "results", "holdout", "runs.jsonl"))]
errs = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
cfg = collections.defaultdict(lambda: collections.defaultdict(list))   # (pb,T,idx) -> key -> rows
for r in rows:
    cfg[(r["pb"], r["T"], r["idx"])][r["key"]].append(r)
IC = ["A", "Bs", "Bf1", "Bf8", "B"]
# correctness: counted totals of every ic5b key equal key A's (per configuration, every repeat)
bad = []
for c, kk in cfg.items():
    ref = {tuple(r[x] for x in "IMSHLN") for r in kk["A"]}
    if len(ref) != 1: bad.append((c, "A not deterministic"))
    for k in IC:
        for r in kk[k]:
            if {tuple(r[x] for x in "IMSHLN")} != ref: bad.append((c, k))
print(f"rows {len(rows)}; errors/unverified {len(errs)}; correctness mismatches {len(bad)}")
if errs or bad:
    print("CORRECTNESS FAIL: reading stops", errs[:3], bad[:5]); sys.exit(1)
def med(rs, f): return statistics.median(r[f] for r in rs)
out = {}
print("\n| bits | T | B/A targets | Bs/A | Bf1/A | Bf8/A | B/A arm | B/rho arm | A/rho arm |")
print("|---|---|---|---|---|---|---|---|---|")
for pb in (24, 28, 32):
    for T in (1024, 4096, 16384):
        cs = [cfg[(pb, T, i)] for i in range(4) if (pb, T, i) in cfg]
        def ratios(num, den, f):
            return [med(c[num], f) / med(c[den], f) for c in cs]
        def rho_ratios(num):
            return [med(c[num], "ms_arm") / min(med(v, "ms_arm") for k, v in c.items() if k.startswith("rho")) for c in cs]
        cell = dict(
            B_A_t=ratios("B", "A", "ms_targets"), Bs_A_t=ratios("Bs", "A", "ms_targets"),
            Bf1_A_t=ratios("Bf1", "A", "ms_targets"), Bf8_A_t=ratios("Bf8", "A", "ms_targets"),
            B_A_arm=ratios("B", "A", "ms_arm"), B_rho=rho_ratios("B"), A_rho=rho_ratios("A"))
        out[f"{pb}:{T}"] = {k: dict(values=v, median=statistics.median(v)) for k, v in cell.items()}
        m = lambda k: statistics.median(cell[k])
        print(f"| {pb} | {T} | {m('B_A_t'):.3f} | {m('Bs_A_t'):.3f} | {m('Bf1_A_t'):.3f} | {m('Bf8_A_t'):.3f} | "
              f"{m('B_A_arm'):.3f} | {m('B_rho'):.3f} | {m('A_rho'):.3f} |")
def all_le(v, th): return all(x <= th for x in v)
def all_ge(v, th): return all(x >= th for x in v)
p = [out[f"32:{T}"]["B_A_t"]["values"] for T in (4096, 16384)]
primary = all(all_le(v, 0.8) for v in p)
null = all(all_ge(v, 0.95) for v in p)
sec_cells = [all_le(out[f"32:{T}"]["B_rho"]["values"], 1.0) for T in (1024, 4096, 16384)]
secondary = sum(sec_cells) >= 2
print(f"\nPRIMARY (B/A targets <= 0.8 at 2^32, T = 4096 and 16384, 4 of 4 each): {'MET' if primary else 'not met'}")
print(f"NULL (B/A targets >= 0.95 at 2^32, both cells, 4 of 4): {'MET' if null else 'not met'}")
print(f"SECONDARY (B/rho arm <= 1 at 2^32 in >= 2 of 3 T cells, 4 of 4 each): {'MET' if secondary else 'not met'} {sec_cells}")
json.dump(dict(cells=out, primary=primary, null=null, secondary=secondary, secondary_cells=sec_cells),
          open(os.path.join(D, "summary.json"), "w"), indent=1)
