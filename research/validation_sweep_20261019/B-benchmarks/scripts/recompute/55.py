import json, math, statistics, collections, os
RUN = "/home/user/crypto/research/ic_improve3_20261015/02-harvest-gate/runs"
def rows(*names):
    out = []
    for n in names:
        out += [json.loads(l) for l in open(os.path.join(RUN, n)) if l.strip()]
    return out
T = rows("test_a.jsonl", "test_b.jsonl")
print("test rows", len(T))
bycell = collections.defaultdict(list)
for x in T:
    bycell[(x["cell"], x["n"], x["ell"], x["m"])].append(x)
for k in sorted(bycell, key=lambda k: (k[3], k[1], k[2])):
    v = bycell[k]
    cpu = [x["g_cpu"] / x["c_cpu"] for x in v]
    wop = [x["g_word_ops"] / x["c_word_ops"] for x in v]
    print("cell (%d,%d) m%d" % (k[1], k[2], k[3]), "targets", len(v), "CPU G0/C geomean", round(math.exp(statistics.fmean(map(math.log, cpu))), 4), "word ops G0/C geomean", round(math.exp(statistics.fmean(map(math.log, wop))), 4))
# root-set identity: every arm's root hash equals i's, all test rows
arms = ["g", "c", "r", "i2"]
n_cmp = 0; n_eq = 0
for x in T:
    n_cmp += 1
    if all(x.get(a + "_root_hash") == x["i_root_hash"] for a in arms):
        n_eq += 1
print("root-set identity test rows: compared", n_cmp, "equal", n_eq, "differ", n_cmp - n_eq, "gate_ok all", all(x.get("gate_ok", True) for x in T))
print("same_decomposition field present", sum(1 for x in T if "same_decomposition" in x), "all True", all(x.get("same_decomposition", True) for x in T))
# end to end: arms '0', 'gpv' (G0), 'unset' (C)
E = rows("e2e.jsonl")
by = collections.defaultdict(list)
for x in E:
    by[(x["n"], x["ell"], x["m"], x["arm"])].append(x)
for n, l in [(23, 12), (29, 15), (31, 16)]:
    g = by[(n, l, 2, "gpv")]; c = by[(n, l, 2, "unset")]
    pr = [a["cpu_s"] / b["cpu_s"] for a, b in zip(g, c)]
    print("e2e (%d,%d) pairs", len(pr), "pair ratios G0/C (file order)", [round(x, 4) for x in pr],
          "mean of pair ratios", round(statistics.fmean(pr), 4), "geomean", round(math.exp(statistics.fmean(map(math.log, pr))), 4),
          "ratio of means", round(statistics.fmean(a["cpu_s"] for a in g) / statistics.fmean(b["cpu_s"] for b in c), 4),
          "log_ok", all(x["log_ok"] for x in g + c), "identical dispositions", len(set(x["dispositions_hash"] for x in g + c)) == 1)
# unit tests
for f in ["unit_tests_default.log", "unit_tests_gpv.log", "unit_tests_optout.log"]:
    txt = open(os.path.join(RUN, f)).read()
    print(f, [l.strip()[:90] for l in txt.splitlines() if "test result" in l])
# fit_test.jsonl rule: ratio G0/C restricted to targets where C fires (--fires c:rank=331)
FT = [json.loads(l) for l in open(os.path.join(RUN, "fit_test.jsonl")) if l.strip()]
ref = {x["cell"]: x for x in FT if x.get("ratio") == "G0/C" and str(x.get("cell", "")).endswith("/bench")}
print("fit_test bench cells", len(ref))
for cell_key in sorted(bycell, key=lambda k: (k[3], k[1], k[2])):
    lab = cell_key[0] + "/bench"
    v = bycell[cell_key]
    fired = [x for x in v if x["c_harvests"] != x["g_harvests"] or x["c_harvested_rows"] != x["g_harvested_rows"]]
    fired_rank = [x for x in v if x["root_rank"] >= 331 and (x["gens"] / x["occ_vars"]) >= 0.9607]
    def gm(z):
        return round(math.exp(statistics.fmean(math.log(x["g_cpu"] / x["c_cpu"]) for x in z)), 4) if z else None
    f = ref.get(lab, {})
    print(lab, "all", len(v), gm(v), "| harvest-differs", len(fired), gm(fired), "| rank>=331&gpv>=0.9607", len(fired_rank), gm(fired_rank), "| study gm", f.get("gm"), "targets", f.get("targets"), "CI", f.get("lo"), f.get("hi"))
for a in ["g", "c", "r", "i2"]:
    print("root hash equal to i for arm", a, sum(1 for x in T if x.get(a + "_root_hash") == x["i_root_hash"]), "of", len(T), "| roots equal count", sum(1 for x in T if x.get(a + "_roots") == x["i_roots"]))
print("gate_ok count True", sum(1 for x in T if x.get("gate_ok")), "of", len(T))
print("sample hashes", T[0]["i_root_hash"], T[0]["g_root_hash"], T[0]["c_root_hash"], T[0].get("r_root_hash"))
