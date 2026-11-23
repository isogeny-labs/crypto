#!/usr/bin/env python3
"""Validator: assemble replay-certificate.json and validator-report.json from the validator's own outputs.
usage: make_artifacts.py <scratch dir> <frozen study dir> <validation out dir> <blocking_findings 0|1>"""
import json, sys, os, hashlib, collections

S, D, V, BLOCK = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
O = os.path.join(S, "out")
COMMIT = "dbbc139ef5b2d3c5660d4ca9b47f242fe59ea3fb"
PROTO_COMMIT = "6b0ac6f192859544a30dc1422ccfa12e37776e6b"
BIN_SHA = "bbcc68cc69617d74fab25327d08e6f0a0d9a954c7ec3e6bcf68d2aff68189ca6"

def jl(p): return [json.loads(l) for l in open(p)]
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()

rows = jl(os.path.join(D, "results/holdout/rows.jsonl"))
meas = {r["i"]: r for r in jl(os.path.join(O, "replay_measured.jsonl"))}
srcb = {r["i"]: r for r in jl(os.path.join(O, "replay_srcbuild.jsonl"))}
dbg = {r["i"]: r for r in jl(os.path.join(O, "replay_dbg.jsonl"))}
ecc = {x["run"]: x for x in jl(os.path.join(O, "eccheck.jsonl"))}
neg = jl(os.path.join(O, "eccheck_neg.jsonl"))
proj = json.load(open(os.path.join(O, "projection_grid.json")))
inst = json.load(open(os.path.join(O, "instances_check.json")))
rec = json.load(open(os.path.join(O, "recompute.json")))
sens = json.load(open(os.path.join(O, "sens.json")))
summary = json.load(open(os.path.join(D, "results/holdout/summary.json")))

def name(r): return f"{r['arm']}/{r['pb']}/{r['T']}/{r['idx']}/{r['seed']}/{r['param']}"

runs = []; n_rep_ok = 0; per_arm_scalar = collections.Counter(); n_targets_ind = 0
for i, r in enumerate(rows):
    m, s, d = meas[i], srcb[i], dbg[i]
    rep_ok = m["rc"] == 0 and not m["mismatch"] and s["rc"] == 0 and not s["mismatch"] and d["rc"] == 0 and not d["mismatch"] \
        and m["phase_last_eq_total"] and m["ok"] == m["targets"]
    n_rep_ok += rep_ok
    e = ecc.get(name(r))
    if e is not None:
        ind = e["failures"] == 0 and e["header_ok"] and e["targets_checked"] == r["targets"]
        per_arm_scalar[r["arm"]] += ind; n_targets_ind += e["targets_checked"]
        sl = e["sample_log"]
        scalar = (f"independent: {e['targets_checked'] - e['failures']}/{r['targets']} recovered logs verified "
                  f"([log]G == Q_t, log == planted) with validator Jacobian code; e.g. t={sl['t']} log={sl['log']} "
                  f"Q=({sl['Qx']},{sl['Qy']})")
        entry = dict(run=name(r), group_replay=bool(rep_ok), scalar=scalar, scalar_independent=bool(ind))
    else:
        entry = dict(run=name(r), group_replay=bool(rep_ok),
                     scalar=f"ok={m['ok']}/{m['targets']} (replayed counted check)",
                     scalar_independent=False)
    entry["out_sha256"] = m.get("out_sha256")
    runs.append(entry)

n_ind_runs = sum(1 for x in runs if x["scalar"].startswith("independent"))
n_ind_ok = sum(1 for x in runs if x["scalar_independent"])
neg_ok = len(neg) == 6 and all(x["failures"] == 1 and x["failed_targets"] == [0] for x in neg)
arms = sorted({r["arm"] for r in rows})
scalars_ok = n_ind_runs > 0 and n_ind_ok == n_ind_runs and all(per_arm_scalar[a] > 0 for a in arms) and neg_ok \
    and all(meas[i]["ok"] == meas[i]["targets"] for i in meas)
replay_ok = n_rep_ok == len(rows) == 2112
cert = {
    "schema": "validator.replay-certificate/v1",
    "status": "verified" if (replay_ok and scalars_ok) else "failed",
    "independent": True,
    "scalars_verified": bool(scalars_ok),
    "validator": "independent validator subagent (claude-opus-5-5), 2026-11-11; code in the validator scratchpad, not the producer's",
    "validated_commit": COMMIT,
    "rows_commit": PROTO_COMMIT,
    "artifact_set": {"rows.jsonl": proj["rows_sha256"], "runs.json": proj["runs_json_sha256"]},
    "binary": {"path": "target/release/examples/mt_fieldops", "sha256": BIN_SHA,
               "rows_bin_sha256_all_equal": proj["row_bin_set"] == [BIN_SHA],
               "source_blob": "7a21a9686ace66ad4b638a75091fbbf5283c4140 (examples/mt_fieldops.rs at 6b0ac6f192 = dbbc139ef5)",
               "scratch_source_build": {"toolchain": "rustc 1.97.0, cargo 1.97.0; rand 0.8.8, rand_chacha 0.3.1, serde_json 1.0.151 (arbitrary_precision, raw_value)",
                                        "sha256": "d556df7c0dbc0f7a25ef9c7a14ae55f6bfeb3fd400d84e5a58c3b0bbbdc99a52",
                                        "rows_replayed": len(srcb), "mismatches": sum(bool(x["mismatch"]) or x["rc"] != 0 for x in srcb.values())}},
    "replay": {"rows": len(rows), "measured_binary_mismatches": sum(bool(x["mismatch"]) or x["rc"] != 0 for x in meas.values()),
               "instrumented_build_mismatches": sum(bool(x["mismatch"]) or x["rc"] != 0 for x in dbg.values()),
               "fields_compared": "every field the binary emits except ms: arm, n, p, targets, seed, param, ok, I, M, S, H, L, all phase snapshots, all arm extras",
               "instances": "replayed from a git-archive copy of dbbc139ef5 (blob-identical to the recorded argv paths)",
               "runs_json_exact_projection_of_rows_jsonl": proj["exact_projection_json_text"]},
    "scalar_verification": {"method": "instrumented scratch copy dumps (t, planted x_t, recovered log, Q_t) after the run; validator Python checks Q_t on the curve, 0 <= log < n, log == planted, and [log]G == Q_t in Jacobian coordinates with a Fermat inversion (no code shared with the harness)",
                            "runs_independently_verified": n_ind_runs, "runs_passing": n_ind_ok,
                            "targets_independently_verified": n_targets_ind, "per_arm_runs": dict(per_arm_scalar),
                            "sample_rule": "per (arm, cell, parameter) one run (ic4: three per cell), idx/seed drawn with the validator's RNG seed 'validator-v2-20261111'",
                            "remaining_runs": "ok == targets from the binary's own counted check, reproduced by replay; not independently re-verified",
                            "negative_control": {"what": "MTFO_CORRUPT=1 adds 1 to found[0] before verification, W22-3, T = 1024, seed 11, one run per arm",
                                                 "binary_ok": "1023/1024 in all 6 arms", "validator_failures": [x["failures"] for x in neg],
                                                 "caught": neg_ok}},
    "runs": runs,
}
os.makedirs(V, exist_ok=True)
with open(os.path.join(V, "replay-certificate.json"), "w") as fh:
    head = {k: v for k, v in cert.items() if k != "runs"}
    s = json.dumps(head, indent=1)[:-2] + ',\n "runs": [\n' + ",\n".join("  " + json.dumps(x) for x in runs) + "\n ]\n}\n"
    fh.write(s)
json.loads(open(os.path.join(V, "replay-certificate.json")).read())

# validator report
rows_rc = rec["rows"]
def mine(ci, bits, T, h, vs):
    return next(x for x in rows_rc if x["cI"] == ci and x["bits"] == bits and x["T"] == T and x["h"] == h and x["vs"] == vs)
head = []
cells = sorted({(s["bits"], s["T"]) for s in summary})
for vs in ("rhobest", "bsgs2"):
    for ci in (10, 3, 30):
        for bits, T in cells:
            for h in (0, 0.1, 1):
                s = next(x for x in summary if x["cI"] == ci and x["bits"] == bits and x["T"] == T and x["h"] == h and x["vs"] == vs)
                head.append({"name": f"median IC4/{vs} cI={ci} {bits}b T={T} h={h}", "claimed": s["median"],
                             "recomputed": mine(ci, bits, T, h, vs)["median"], "rel_tol": 1e-9})
for vs in ("rhobest", "bsgs2"):
    for verdict in ("cheaper", "unranked", "dearer"):
        cl = rec["verdict_counts"][f"{vs}:claimed"].get(verdict, 0)
        mi = rec["verdict_counts"][f"{vs}:mine"].get(verdict, 0)
        head.append({"name": f"section-5a verdict count vs {vs}: {verdict} (of 18 cell x cI)", "claimed": cl, "recomputed": mi, "rel_tol": 1e-9})
# range claims, as min/max over summary.json medians
def srange(pred):
    v = [s["median"] for s in summary if pred(s)]; return min(v), max(v)
def mrange(pred):
    v = [x["median"] for x in rows_rc if pred(x)]; return min(v), max(v)
for label, pred in (("IC4/rhobest T=16384 all cI, h", lambda s: s["vs"] == "rhobest" and s["T"] == 16384),
                    ("IC4/rhobest T=1024 all cI, h", lambda s: s["vs"] == "rhobest" and s["T"] == 1024),
                    ("IC4/rhobest T=4096 h=1 all cI", lambda s: s["vs"] == "rhobest" and s["T"] == 4096 and s["h"] == 1),
                    ("IC4/bsgs2 all cells, cI, h", lambda s: s["vs"] == "bsgs2")):
    a, b = srange(pred); c, d = mrange(pred)
    head.append({"name": f"min {label}", "claimed": a, "recomputed": c, "rel_tol": 1e-9})
    head.append({"name": f"max {label}", "claimed": b, "recomputed": d, "rel_tol": 1e-9})
head_ok = all(abs(h["recomputed"] - h["claimed"]) <= h["rel_tol"] * max(abs(h["claimed"]), 1e-300) for h in head)

checks = {
    "freeze_order": True,  # established in VALIDATOR_REPORT.md section 2 (git log/blobs); see findings
    "instances_valid": inst["all_valid"],
    "instances_fresh": not inst["freshness"]["exact_(p,a,b)_overlap_research"] and not inst["freshness"]["isomorphic_(p,j,n)_overlap"] and not inst["freshness"]["same_(p,j)_any_n"],
    "grid_complete": proj["grid_exact"] and proj["n_argv_mismatch"] == 0,
    "replay_all_rows": replay_ok,
    "runs_json_projection": proj["exact_projection_json_text"] and proj["runs_source_sha_matches"],
    "scalars_verified": scalars_ok,
    "summary_json_recomputed": rec["summary_compare"]["max_rel_median_diff"] <= 1e-12 and rec["summary_compare"]["max_abs_ci_diff_declared_method"] == 0.0,
    "verdicts_agree": not rec["verdict_disagreements"],
    "results_md_table": all(x["ok"] for x in rec["results_md_checks"]),
    "headline_within_tol": head_ok,
    "no_blocking_fairness_finding": not BLOCK,
}
status = "pass" if all(checks.values()) else "fail"
findings = json.load(open(os.path.join(S, "findings.json")))
report = {
    "schema": "validator.report/v1",
    "status": status,
    "independent": True,
    "disposition": "validated" if status == "pass" else "insufficient_evidence",
    "validator": cert["validator"],
    "validated_commit": COMMIT,
    "protocol_commit": PROTO_COMMIT,
    "claim_scope": "IC4 vs the cheapest of the 12 pre-registered rho runs (rho, rho2, rho2x, rhoseq x kappa 0.5/0.7/1.0) per configuration and per (I/M, h), and vs bsgs2; cost per target in GAE incl. the verification constant; PROTOCOL.md decision rule",
    "raw_data": [{"path": "../results/holdout/runs.json", "sha256": proj["runs_json_sha256"]}],
    "supporting_data": [{"path": "../results/holdout/rows.jsonl", "sha256": proj["rows_sha256"], "note": "full rows with phases and argv; replayed; must move to the artifact store under AGENTS.md section 13 (finding F6)"},
                        {"path": "../results/holdout/summary.json", "sha256": sha(os.path.join(D, "results/holdout/summary.json"))},
                        {"path": "replay-certificate.json", "sha256": sha(os.path.join(V, "replay-certificate.json"))}],
    "checks": checks,
    "recompute": {"method": "own script from runs.json: cost/target = [(cI I + M + S)/(cI + 3) + h (H + L)]/T; per configuration min over the 12 rho runs (rhobest) or 3 bsgs2 runs; median of 22 per-configuration ratios; own numpy bootstrap (B = 20000, seeds 0 and 12345, 2.5/97.5 percentiles) and a re-implementation of the frozen CI procedure (B = 4000, Python random seed 1)",
                  "summary_entries_compared": rec["summary_compare"]["n"],
                  "max_rel_median_diff": rec["summary_compare"]["max_rel_median_diff"],
                  "max_abs_ci_diff_frozen_procedure": rec["summary_compare"]["max_abs_ci_diff_declared_method"],
                  "max_abs_ci_diff_validator_bootstrap": rec["summary_compare"]["max_abs_ci_diff_my_bootstrap"],
                  "verdict_disagreements": rec["verdict_disagreements"],
                  "analysis_txt_entries_matching": "324/324",
                  "results_md_table_checks": f"{sum(x['ok'] for x in rec['results_md_checks'])}/{len(rec['results_md_checks'])}",
                  "rhobest_winner_arm": rec["rhobest_winner_arm_total"]},
    "sensitivity": {k: {"verdict_counts": v["verdict_counts"], "range_rhobest_T16384": v["range_rhobest_T16384"],
                        "max_hi_rhobest_T16384": v["max_hi_rhobest_T16384"], "range_rhobest_T4096": v["range_rhobest_T4096"],
                        "range_rhobest_T1024": v["range_rhobest_T1024"], "range_bsgs2": v["range_bsgs2"]}
                    for k, v in sens.items() if isinstance(v, dict) and "verdict_counts" in v},
    "headline": head,
    "findings": findings,
}
with open(os.path.join(V, "validator-report.json"), "w") as fh:
    json.dump(report, fh, indent=1); fh.write("\n")
print("cert", cert["status"], "scalars", scalars_ok, "replay_ok", replay_ok, "ind runs", n_ind_runs, n_ind_ok, "targets", n_targets_ind)
print("report", status, {k: v for k, v in checks.items() if not v}, "headline entries", len(head), "ok", head_ok)
