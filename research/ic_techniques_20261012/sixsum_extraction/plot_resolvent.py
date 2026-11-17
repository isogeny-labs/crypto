"""Plots recorded values only (raw_resolvent_20261018/*.json); computes nothing new
beyond the ratios printed in the report."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
R = os.path.join(H, "raw_resolvent_20261018")
sp = json.load(open(f"{R}/n41_k10_k20_split.json"))
rs = json.load(open(f"{R}/n41_k10_k20_resolvent.json"))
sc = rs["solver_costs"][0]
def cell(d, K, kern):
    return next(c for c in d["cells"] if c["columns"] == K and c["kernel"].startswith(kern))
fig, ax = plt.subplots(1, 3, figsize=(13, 4.2))
a = ax[0]
names = ["S3\n(loop)", "S3\n(table)", "S4 roots\nsplit", "S4 roots\nresolvent"]
vals = [sc["ns_s3_crate"], sc["ns_s3_table"], sc["ns_roots_split"], sc["ns_roots_resolvent"]]
b = a.bar(names, vals, color=["#888", "#bbb", "#c44", "#27a"])
a.set_yscale("log"); a.set_ylabel("ns per solve (best of 7)")
for r, v in zip(b, vals): a.text(r.get_x()+r.get_width()/2, v*1.1, f"{v:.0f}", ha="center", fontsize=8)
a.set_title("n=41 per-solve wall (3000 quartics)", fontsize=10)
Ks = [10, 20]; w = 0.27
a = ax[1]
for i, (lab, col, d, kern) in enumerate([("4-sum (S3 loop)", "#888", rs, "4sum"), ("B2 split (control)", "#c44", sp, "6sum_b2"), ("B2 resolvent", "#27a", rs, "6sum_b2")]):
    v = [cell(d, K, kern)["wall_s_per_relation"] for K in Ks]
    bb = a.bar([k+(i-1)*w for k in range(2)], v, w, label=lab, color=col)
    for r, y in zip(bb, v): a.text(r.get_x()+r.get_width()/2, y*1.08, f"{y:.3g}", ha="center", fontsize=8)
a.set_xticks(range(2)); a.set_xticklabels([f"K={K}" for K in Ks]); a.set_yscale("log")
a.set_ylabel("wall s per verified relation"); a.legend(fontsize=8); a.set_title("n=41 wall per relation (32 rel/cell)", fontsize=10)
a = ax[2]
for i, (lab, col, d, kern, key) in enumerate([("4-sum", "#888", rs, "4sum", None), ("B2 split", "#c44", sp, "6sum_b2", "s3_equivalents_per_relation_wall"), ("B2 resolvent", "#27a", rs, "6sum_b2", "s3_equivalents_per_relation_wall")]):
    v = [(cell(d, K, kern)[key] if key else cell(d, K, kern)["counters"]["s3_solves"]/cell(d, K, kern)["relations"]) for K in Ks]
    bb = a.bar([k+(i-1)*w for k in range(2)], v, w, color=col, label=lab)
    for r, y in zip(bb, v): a.text(r.get_x()+r.get_width()/2, y*1.08, f"{y:.2e}", ha="center", fontsize=7)
a.set_xticks(range(2)); a.set_xticklabels([f"K={K}" for K in Ks]); a.set_yscale("log")
a.set_ylabel("S3-equivalents per relation (wall-priced)"); a.legend(fontsize=8); a.set_title("n=41 S3-equivalents per relation", fontsize=10)
fig.suptitle("Kernel B/B2 quartic root finder, n=41 Koblitz K_0 (seed 20261012, shared host). Stage diagnostic; sources: raw_resolvent_20261018/n41_k10_k20_{split,resolvent}.json", fontsize=8)
fig.tight_layout(rect=(0, 0, 1, 0.94))
fig.savefig(f"{H}/sixsum_s4_resolvent_n41.svg"); fig.savefig(f"{H}/sixsum_s4_resolvent_n41.png", dpi=150)
