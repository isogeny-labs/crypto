"""Summarise raw/*.jsonl of the prefix-cached root probe into summary.json, a table and a figure."""
import glob, json, re, sys
rows = []
for f in sorted(glob.glob("raw/stage_*.jsonl")):
    tag = f[len("raw/stage_"):-len(".jsonl")]
    stage = [json.loads(l) for l in open(f) if l.startswith("{")]
    root = [json.loads(l) for l in open(f"raw/root_{tag}.jsonl") if l.startswith("{")]
    # the probe prints shapes as Rust `Some((rows, columns, rank))`
    fix = lambda l: re.sub(r"Some\(\((\d+), (\d+), (\d+)\)\)", r"[\1,\2,\3]", l)
    shape = [json.loads(fix(l)) for l in open(f"raw/shape_{tag}.jsonl") if l.startswith("{")]
    s = [r for r in stage if r.get("summary")][0]
    per = [r for r in stage if not r.get("summary")]
    sc = {r["k"]: r for r in per if r["arm"] == "scratch"}
    pf = {r["k"]: r for r in per if r["arm"] == "prefix"}
    assert all((sc[k]["found"], sc[k]["splits"], sc[k]["reductions"]) == (pf[k]["found"], pf[k]["splits"], pf[k]["reductions"]) for k in sc)
    rows.append(dict(
        cell=s["cell"], targets=s["targets"],
        prefix_generators=shape[0]["prefix"], generators=shape[0]["generators"],
        prefix_same_all_targets=all(x["same_as_first"] >= x["prefix"] for x in shape),
        root_rows_full=shape[0]["full"], root_rows_prefix=shape[0]["prefix_shape"],
        root_scratch_xors=sum(r["scratch_xors"] for r in root), root_prefix_xors_incl_once=sum(r["prefix_xors"] for r in root),
        root_share_of_stage=sum(r["scratch_root_elim"] for r in root)/len(root) / (s["scratch_word_xors"]/s["targets"]),
        all_rowspaces_equal=all(r["same_row_space"] for r in root),
        stage_scratch=s["scratch_word_xors"], stage_prefix=s["prefix_word_xors"], once=s["prefix_once_total"],
        ratio_incl_once=s["ratio_incl_once"], ratio_excl_once=s["ratio_excl_once"],
        cpu_scratch_ms=s["scratch_cpu_ms"], cpu_prefix_ms=s["prefix_cpu_ms"]))
json.dump(rows, open("summary.json", "w"), indent=1)
for r in rows:
    print(f"| {r['cell']} | {r['targets']} | {r['root_rows_prefix'][0]} / {r['root_rows_full'][0]} | {100*r['root_share_of_stage']:.0f}% | {r['ratio_incl_once']:.3f} | {r['ratio_excl_once']:.3f} | {r['cpu_scratch_ms']:.0f} / {r['cpu_prefix_ms']:.0f} |")
import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(7, 3.2))
names = [r["cell"] for r in rows]
ax.barh(names, [r["ratio_incl_once"] for r in rows], color="#4a7bb7", label="incl. one-time cache build")
ax.scatter([r["ratio_excl_once"] for r in rows], names, color="#d9822b", zorder=3, label="excl. one-time build")
ax.axvline(1.0, color="k", lw=0.8)
ax.set_xlim(0.95, 1.05); ax.set_xlabel("scratch ÷ prefix-cached (stage word XORs; >1 = gain)")
ax.legend(fontsize=7, loc="lower right"); ax.set_title("Prefix-cached inherited-F4 root: whole-stage effect", fontsize=9)
fig.tight_layout(); fig.savefig("figures/ratio_by_cell.svg"); fig.savefig("figures/ratio_by_cell.pdf"); fig.savefig("figures/ratio_by_cell.png", dpi=150)
