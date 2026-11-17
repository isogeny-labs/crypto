#!/usr/bin/env python3
"""VALIDATOR: write the compact evidence files of validation/ from the scratch outputs (no recomputation here).
usage: compact_evidence.py <scratch v5val dir> <validation dir> <frozen study dir>"""
import json, sys, os, hashlib, collections
W, V, FZ = sys.argv[1:4]
J = lambda f: json.load(open(os.path.join(W, f)))
def dump(name, obj): json.dump(obj, open(os.path.join(V, name), "w"), separators=(",", ":"))
rows = [json.loads(l) for l in open(os.path.join(FZ, "results/holdout/rows.jsonl"))]
ra = J("replay_all.json")
# native work: per-row instrumented tallies
F = ["row", "arm", "pb", "T", "idx", "seed", "param", "doff", "match", "H", "N", "xc_hashmap_calls", "xc_ret_probe", "xc_hidx",
     "xc_canon", "xc_idx", "xc_is_dp", "xc_wstep", "xc_restart_rng", "xc_rhodpu_setup_set"]
nw = []; chk = collections.Counter()
for x in sorted(ra["results"], key=lambda x: x["row"]):
    r = rows[x["row"]]; c = x["xc"]
    nw.append([x["row"], r["arm"], r["pb"], r["T"], r["idx"], r["seed"], r["param"], r.get("doff"), x["match"], r["H"], r["N"]] + c[:9])
    a = r["arm"]
    if a in ("ic5ks", "ic5rw"):
        # one f.hash() covers ret.get + table.get; on a retained hit the table probe is skipped, so H - xc0 = retained-hit targets
        chk["ic5ks/ic5rw: 0 <= H - hashmap calls <= targets"] += 0 <= r["H"] - c[0] <= r["targets"]
        chk["ic5ks/ic5rw: H <= hashmap calls incl. ret probes"] += r["H"] <= c[0] + c[1]
    else:
        chk["other arms: H == hashmap calls"] += c[0] == r["H"]
    expN = c[2] if a == "rho2xks" else 2 * c[3] if a == "rhodpu" else c[6] if a in ("ic5w", "ic5rw") else 0
    chk["N == expected native count"] += expN == r["N"]
    chk["no_native_outside_declared"] += (a in ("ic5", "ic5ks", "bsgs2") and r["N"] == 0 and c[2] == c[3] == c[6] == 0) or a not in ("ic5", "ic5ks", "bsgs2")
tot = collections.defaultdict(lambda: collections.Counter())
for v in nw:
    t = tot[v[1]]; t["runs"] += 1; t["H"] += v[9]; t["N"] += v[10]
    for i, k in enumerate(F[11:]): t[k] += v[11 + i]
dump("native_work.json", dict(schema="validator.native-work/v1",
     source="scripts/replay.py all + scripts/patch_instr.py (instrumented build; adds no counted operation)",
     summary=ra["summary"], identity_checks=dict(chk), rows=len(nw), totals_by_arm={k: dict(v) for k, v in tot.items()}, fields=F, data=nw))
# replay certificate inputs, scalar check, others are referenced by make_artifacts
sc = J("scalar_check.json"); dump("scalar_check.json", sc)
dump("instances_check.json", J("instances_check.json"))
rc = J("recompute.json")
dump("recompute.json", {k: v for k, v in rc.items() if not k.startswith("summary_")} |
     {"note": "summary_5a and summary_v4 (972 entries each) recomputed by scripts/recompute.py equal results/holdout/summary.json and summary_v4rule.json exactly (compare_5a, compare_v4); they are not duplicated here"})
dump("results_check.json", J("results_check.json"))
se = J("sensitivity.json"); dump("sensitivity.json", se)
sr = []
for f in ("sens_kappa.json", "sens_rho2x.json"):
    d = J(f)
    for o in d["rows"]:
        sr.append([o["arm"], o["pb"], o["T"], o["idx"], o["seed"], o["param"], o["targets"], o["ok"], o["I"], o["M"], o["S"], o["H"], o["L"], o["N"]])
dump("sens_runs.json", dict(schema="validator.sens-runs/v1", note="validator sensitivity panels, not pre-registered: rho2xks kappa 0.75/1.0 and rho2x (no retention) kappa 0.5/1.0, all 198 holdout configurations, pristine build of the frozen harness (scripts/run_sens.py)",
     binaries={f: J(f)["binary_sha256"] for f in ("sens_kappa.json", "sens_rho2x.json")},
     fields=["arm", "pb", "T", "idx", "seed", "param", "targets", "ok", "I", "M", "S", "H", "L", "N"], rows=sr,
     errors=0, unverified=sum(1 for r in sr if r[7] != r[6])))
dump("v4_crosscheck.json", J("v4cross.json"))
dump("claim_check.json", J("claim_check.json"))
print("ok", dict(chk), len(nw))
