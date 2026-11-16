import json, statistics as st
D = "research/f4_f5_hash_20261012/records/"
c = json.load(open(D + "comparison.json"))
print("systems", c["systems"], "paired_rounds", c["paired_rounds"], "exact_outputs_and_counts", c["exact_outputs_and_counts"])
for a in c["arms"]:
    b = st.median(a["baseline_ir"]); k = st.median(a["candidate_ir"])
    red = 100 * (1 - k / b)
    print(a["arm"], "median baseline", b, "median candidate", k, "reduction% recomputed", round(red, 4),
          "stored", round(a["instruction_reduction_percent"], 4), "stored CI", [round(x, 4) for x in a["instruction_reduction_ci95_percent"]],
          "median_log_ratio", round(a["median_log_ratio"], 6), "log_threshold", round(a["log_threshold"], 6), "verdict", a["verdict"])
# identical-output check: baseline (primary-a, a/bench) vs candidate (primary-b, b/bench)
def load(p):
    return [json.loads(l) for l in open(D + p + "/records.jsonl")]
A = load("primary-a"); Bc = load("primary-b")
def key(r): return (r["arm"], r["case"], r["seed"], r["repeat"])
ma = {key(r): r for r in A}; mb = {key(r): r for r in Bc}
print("keys a", len(ma), "keys b", len(mb), "common", len(set(ma) & set(mb)))
n_c = n_e = n_d = 0; files = set(); diffs = []
def fields(r):
    out = {}
    for f in ("output_digest", "criterion_digest", "input_digest", "rank", "pruned", "word_ops", "lower_rows", "pruned_by_part"):
        if f in r: out[f] = r[f]
    rep = r.get("report")
    if rep:
        for f in ("reduce_word_ops", "criterion_word_ops", "rank", "rows_pruned", "criterion_rows", "cols", "rows_built"):
            if f in rep: out["report." + f] = rep[f]
    return out
for k in sorted(set(ma) & set(mb)):
    fa, fb = fields(ma[k]), fields(mb[k])
    for f in sorted(set(fa) & set(fb)):
        n_c += 1
        if fa[f] == fb[f]: n_e += 1
        else: n_d += 1; diffs.append((k, f, fa[f], fb[f]))
print("n_compared", n_c, "n_equal", n_e, "n_differ", n_d, "first diffs", diffs[:5])
print("fields compared per arm:", {arm: sorted({f for k in ma if k[0]==arm for f in fields(ma[k])}) for arm in ("f4","f5","criterion")})
