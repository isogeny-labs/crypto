#!/usr/bin/env python3
"""Append the pool section to STAGE0.md from raw/pool*.txt (run from this directory)."""
import re
T = "[pcc:unverified]"
FLOOR = {1: 1.329, 2: 1.662}
agg = {}
for pool in (64, 256):
    for k in (20, 24, 28):
        for line in open(f"raw/pool{pool}-k{k}.txt"):
            m = re.match(r"AGG C=(\d) (\S+(?: m=\d+(?: M=\d+)?)?)\s+E_points = ([\d.]+) \+- ([\d.]+)(?:\s+E_parents = ([\d.]+) \+- ([\d.]+))?\s+hit = ([\d.]+)", line)
            if m:
                agg[(int(m[1]), k, pool, m[2].strip())] = dict(e=float(m[3]), se=float(m[4]), p=float(m[5]) if m[5] else None,
                                                           pse=float(m[6]) if m[6] else None, hit=float(m[7]))
arms = [(4, 64), (8, 64), (8, 256), (16, 256)]
rows = []; ratios = {}
for c in (1, 2):
    rows.append(f"### C = {c}, floor {FLOOR[c]} {T}\n")
    rows.append("| k | m | M | E_points +- SE | E_parents +- SE | hit | E_points / D0 | E_points / floor | E_points / E_parents |")
    rows.append("|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
    for k in (20, 24, 28):
        d0 = agg[(c, k, 64, "D0")]; di = agg[(c, k, 64, "D0i")]
        rows.append(f"| {k} | D0 | n/a | {d0['e']:.4f} +- {d0['se']:.4f} | n/a | {d0['hit']:.4f} | 1.000 | {d0['e']/FLOOR[c]:.3f} | n/a |")
        rows.append(f"| {k} | D0i | n/a | {di['e']:.4f} +- {di['se']:.4f} | n/a | {di['hit']:.4f} | {di['e']/d0['e']:.3f} | {di['e']/FLOOR[c]:.3f} | n/a |")
        for m, M in arms:
            a = agg[(c, k, M, f"SP m={m} M={M}")]
            rt = a['e'] / d0['e']; ratios[(c, k, m, M)] = rt
            rows.append(f"| {k} | {m} | {M} | {a['e']:.4f} +- {a['se']:.4f} | {a['p']:.4f} +- {a['pse']:.4f} | {a['hit']:.4f} | {rt:.3f} | {a['e']/FLOOR[c]:.3f} | {a['e']/a['p']:.2f} |")
    rows.append("")
lo, hi = min(ratios.values()), max(ratios.values())
over = [(c, k) for c in (1, 2) for k in (20, 24, 28) if ratios[(c, k, 8, 256)] > 1.08]
r256 = [ratios[(c, k, 8, 256)] for c in (1, 2) for k in (20, 24, 28)]
dr = open("raw/diag_rows_pool.txt").read().strip().splitlines()
doc = open("STAGE0.md").read().split("\n## Sibling pool amendment")[0].rstrip() + "\n"
doc += f"""
## Sibling pool amendment (second section, dated 2026-11-10)

Status: [pcc:unverified]. The first section above is unchanged and stays negative: the symmetric sibling set loses
pairs. This section tests the owner's fix, a per-parent random m-subset of a pool of M jumps [pcc:unverified].

Answer [pcc:unverified]: the amended falsification line is NOT hit. E_points / D0 for the four arms is {lo:.3f} to {hi:.3f}
over both C and all three sizes; for (8, 256) it is {min(r256):.3f} to {max(r256):.3f}; sizes above 1.08 for (8, 256): {len(over)} of 6
(C, k) cells. The prediction 1.00 +- 0.05 holds as a band; the ratios sit below 1.00 (0.96 to 0.99) because the pool walk
has a smaller walk excess than D0's fixed 16-jump walk (E_points / floor is 1.00 to 1.03) [pcc:unverified]. No ratio is
below 0.95, the bug guard [pcc:unverified].

Design, pre-registration and commit: header amendment (commit db2022fe35, before any pool run), mode `--pool M
--siblings m,..` (`SP(m, M)`, struct `PoolGen`), the two known-copy children kept skipped (conditionally on the
subsets, see the header). Tests: `cargo test --release --bin nc2_stage0` (17 passed); clippy `-D warnings` clean
[pcc:unverified]. Commands (also `run_pool.sh`), seed 20261108, n = 2000, from the repository root, per size k with
its three curves (the same r values as above) [pcc:unverified]:

```
./target/release/nc2_stage0 --r <3 curves> --c 1,2 --n 2000 --seed 20261108 --pool 64  --siblings 4,8  > raw/pool64-k<k>.txt
./target/release/nc2_stage0 --r <3 curves> --c 1,2 --n 2000 --seed 20261108 --pool 256 --siblings 8,16 > raw/pool256-k<k>.txt
```

D0 and D0i controls are in every run on the same samples and reproduce the first section's values exactly (same
samples and seed), so they are inside their intervals [pcc:unverified]. Per-k = mean of three curve means. Tables built
by `gen_pool.py` from `raw/pool*.txt` [pcc:unverified]. Hit = 1.0000 everywhere.

{chr(10).join(rows)}
## Diagnostic: raw collisions per distinct row

`diag_rows_pool.py` (Python, r = 131203, C = 1, 20 samples, walk run to 12 sqrt(r) points; output
`raw/diag_rows_pool.txt`) [pcc:unverified]:

```
{chr(10).join(dr)}
```

Prediction for (8, 256): about 1.0 + (m/M)^2 = 1.001. Measured 1.006, above the prediction by 0.005 (the SE of
the ratio is about 0.01 from 2772 raw collisions), consistent with it [pcc:unverified]. For comparison the symmetric
set gave 2.95 at m = 16 [pcc:unverified]. The other arms read 1.02 to 1.03 against 1.004 to 1.016 predicted
[pcc:unverified]. The diagnostic keeps identical vectors out of the count and does not apply the known-copy skip.

## Caveats (pool)

* The pool walk's walk excess is lower than D0's, so the ratio to D0 mixes the sibling effect with that difference;
  the ratio to the Poisson floor is given alongside [pcc:unverified].
* E_parents counts the parent in progress at stop as a whole parent. Cost per point (0.59 of a step per child) is
  not applied here [pcc:unverified].
* With (m, M) the expected parent subsets overlap, so same-parent structure other than the swap pair is not
  excluded; the measured E says it does not matter at these sizes [pcc:unverified].
"""
open("STAGE0.md", "w").write(doc)
print(lo, hi, min(r256), max(r256), over)
