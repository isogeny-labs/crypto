# Row 65 recompute: 20261015-cyclotomic-falsifier. Reads committed files only. Run from repo root with python3 -I.
import json, random, statistics, os
R = "research/cyclotomic_falsifier_20261015"
def jl(p):
    with open(p) as f:
        return [json.loads(x) for x in f if x.strip()]
out = {}
# yield tests
tests = []
for f in ["yield/yield-a0-n23.json", "yield/yield-a1-n23.json"]:
    y = json.load(open(os.path.join(R, f)))
    out.setdefault("critical_p", set()).add(y["critical_p"])
    for t in y["tests"]:
        t["_file"] = f
        tests.append(t)
cyc = [t for t in tests if t["base"].startswith("cyclotomic") and t["direction"] != "at"]
eff = [t["effect_vs_median"] for t in cyc]
print("yield cyclotomic non-at tests:", len(cyc))
print("effect min %.6f (%s %s w=%s k=%s %s) max %.6f" % (min(eff), min(cyc, key=lambda t: t["effect_vs_median"])["base"], min(cyc, key=lambda t: t["effect_vs_median"])["metric"], min(cyc, key=lambda t: t["effect_vs_median"])["w_cell"], min(cyc, key=lambda t: t["effect_vs_median"])["k"], "", max(eff)))
print("min p over cyclotomic tests:", min(t["p_two_sided"] for t in cyc))
pc = [t for t in tests if t["base"] == "positive_lambda_progression" and t["direction"] != "at"]
print("positive control non-at tests:", len(pc), "p values:", sorted(set(t["p_two_sided"] for t in pc)), "all significant:", all(t["significant"] for t in pc))
print("positive control at-tests (K=1, identical base) p:", sorted(set(t["p_two_sided"] for t in tests if t["base"] == "positive_lambda_progression" and t["direction"] == "at")))
print("critical_p values:", sorted(out["critical_p"]))

# cost: paired S from records (warmup excluded), pairs keyed by (target_seed,target_index,round)
def pairs_for(sess, a_arm, b_arm):
    recs = jl(os.path.join(R, "sessions", sess, "records.jsonl"))
    idx = {}
    for r in recs:
        if r["warmup"]:
            continue
        key = (r["workload"]["target_seed"], r["workload"]["target_index"], r["round"])
        idx.setdefault(r["arm"], {})[key + (r["workload"]["curve"]["slug"],)] = r["cost"]["s"]
    keys = sorted(set(idx[a_arm]) & set(idx[b_arm]))
    return [(k, idx[a_arm][k], idx[b_arm][k]) for k in keys], idx
def cluster_boot(pr, n=4000, seed=20261015):
    # cluster = workload (target_seed,target_index); ratio of pooled means
    by = {}
    for k, a, b in pr:
        by.setdefault(k[:2], []).append((a, b))
    cl = list(by.values())
    rng = random.Random(seed)
    rs = []
    for _ in range(n):
        sa = sb = 0.0; m = 0
        for _ in range(len(cl)):
            for a, b in cl[rng.randrange(len(cl))]:
                sa += a; sb += b; m += 1
        rs.append(sb / sa)
    rs.sort()
    return rs[int(0.025 * n)], rs[int(0.975 * n) - 1]
def ratio_report(sess, a, b, label):
    pr, _ = pairs_for(sess, a, b)
    ma = statistics.fmean(x[1] for x in pr); mb = statistics.fmean(x[2] for x in pr)
    lo, hi = cluster_boot(pr)
    print("%s: %s/%s pairs=%d mean_a=%.6f mean_b=%.6f ratio_b_over_a=%.6f cluster95=[%.4f, %.4f]" % (label, b, a, len(pr), ma, mb, mb / ma, lo, hi))
ratio_report("a1-n23", "ic-cos-c30", "ic-cyc-w2", "K1 30cols")
ratio_report("a0-n23", "ic-cos-c17", "ic-cyc-w2", "K0 17cols")
ratio_report("a1-n23", "ic-cos-c1", "ic-cyc-w1", "K1 1col")
ratio_report("a0-n23", "ic-cos-c3", "ic-cyc-w3-c3", "K0 3cols")
# IC arms vs rho-frob, ratio of mean S
rr = []
for sess in ["a0-n23", "a1-n23"]:
    _, idx = pairs_for(sess, "rho-frob", "rho-frob")
    recs = jl(os.path.join(R, "sessions", sess, "records.jsonl"))
    arms = {}
    for r in recs:
        if not r["warmup"]:
            arms.setdefault(r["arm"], []).append(r["cost"]["s"])
    rho = statistics.fmean(arms["rho-frob"])
    for arm, v in sorted(arms.items()):
        if arm.startswith("ic-"):
            rr.append((sess, arm, round(statistics.fmean(v) / rho, 4)))
print("IC/rho-frob mean-S ratios:", rr)
print("IC/rho min %.4f max %.4f" % (min(x[2] for x in rr), max(x[2] for x in rr)))
# replays and verification
ver = 0; tot = 0; ident = 0; rep = 0
for sess in ["a0-n23", "a1-n23"]:
    recs = jl(os.path.join(R, "sessions", sess, "records.jsonl"))
    ver += sum(1 for r in recs if r["outcome"]["status"] == "verified"); tot += len(recs)
for f in ["audits/audit-a0-n23.json", "audits/audit-a1-n23.json"]:
    a = json.load(open(os.path.join(R, f)))
    rep += len(a["replays"]); ident += sum(1 for x in a["replays"] if x["reproduced"] and x["detail"] == "identical")
print("records verified %d/%d; replays identical %d/%d" % (ver, tot, ident, rep))
