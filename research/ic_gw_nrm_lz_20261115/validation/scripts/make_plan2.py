#!/usr/bin/env python3
"""Plan 2 (validator): (a) small-q sweep where first-coordinate-0 points and multi-attempt units occur:
q in {223, 499, 991, 2011, 4051, 8191, 16363, 32719} x seeds 1-5, REF, BOTH (CAND NRM=1 LZ_REUSE=1), ORACLE;
(b) negative controls: mut_lzwy, mut_lzvv, mut_nrm with NRM=1 LZ_REUSE=1 at (262111, 1) and (524287, 5).
Columns as plan.tsv. usage: make_plan2.py > plan2.tsv"""
rows = []
for q in [223, 499, 991, 2011, 4051, 8191, 16363, 32719]:
    for s in range(1, 6):
        rows += [("small", "REF", "ref", "-", q, s), ("small", "BOTH", "cand", "NRM=1,LZ_REUSE=1", q, s),
                 ("small", "ORACLE", "oracle", "NRM=1,LZ_REUSE=1", q, s)]
for q, s in [(262111, 1), (524287, 5)]:
    for m in ("mut_lzwy", "mut_lzvv", "mut_nrm"):
        rows.append(("neg", m.upper().replace("_", ""), m, "NRM=1,LZ_REUSE=1", q, s))
for i, r in enumerate(rows, 101):
    print(i, *r, "1.7", sep="\t")
