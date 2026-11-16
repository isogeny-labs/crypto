#!/usr/bin/env python3
"""Option 3, check 1: measured trials per target against the coverage model, from the v2 holdout rows.
Random-log table (ic4): a trial hits with probability 2K/n (K = x-classes in the table, d = determined fraction
of factor-base logs, a hit is usable with probability ~ d^2): E[trials] = n / (2 K d^2).
Tiled table (bsgs2): baby x-classes cover |k| <= Mx; paired giants cover disjoint windows of 2(2Mx+1):
E[trials] = 1 + n / (2 (2 Mx + 1)) (uniform over the windows, half of the covering count)."""
import json, statistics as st, collections, sys
rows = [json.loads(l) for l in open(sys.argv[1])]
out = collections.defaultdict(lambda: collections.defaultdict(list))
for r in rows:
    if r["arm"] == "ic4":
        d = r["determined"] / r["m"]
        out[(r["pb"], r["T"])]["ic4"].append((r["trials"] / r["targets"]) / (r["n"] / (2 * r["table"] * d * d)))
    if r["arm"] == "bsgs2":
        out[(r["pb"], r["T"])]["bsgs2"].append((r["trials"] / r["targets"]) / (1 + r["n"] / (2 * (2 * r["mx"] + 1))))
print("| bits | T | ic4 measured / model n/(2Kd^2) | bsgs2 measured / model 1 + n/(2(2Mx+1)) |")
print("|---|---|---|---|")
for k in sorted(out):
    print(f"| {k[0]} | {k[1]} | {st.median(out[k]['ic4']):.3f} (n={len(out[k]['ic4'])}) | {st.median(out[k]['bsgs2']):.3f} (n={len(out[k]['bsgs2'])}) |")
