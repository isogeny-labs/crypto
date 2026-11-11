#!/usr/bin/env python3
"""Assemble replay-certificate.json and validator-report.json for Study A (v3) and Study B (rho-start fix)
from the validator's own outputs in out/.  Writes only into <study>/validation/ of the repository.
usage: make_artifacts.py <V scratch dir> <repo>"""
import json, sys, os, hashlib, collections
V, REPO = sys.argv[1:3]
O = f"{V}/out"; FR = f"{V}/frozen/research"
def L(p): return [json.loads(l) for l in open(p)]
def J(p): return json.load(open(p))
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
HEAD = "70d0b40bc365c1269dfc3c1251153cbc44cc589e"
WHO = "independent validator subagent (claude-opus-5-5), 2026-11-11; code in the validator scratchpad (copied to validation/scripts/), not the producer's"
TOOL = "rustc 1.97.0 / cargo 1.97.0; rand 0.8.8, rand_chacha 0.3.1, rand_core 0.6.4, serde_json 1.0.151 (arbitrary_precision, raw_value); Python 3, numpy"
nonce = J(f"{O}/nonce.json")["nonce"]
src_blob = "1f1b86a1581f61576996d300f9367f5cec91cc47269b48dc18dfa658e0ad2dd2"

def replay_stats(path):
    rs = L(path)
    return dict(rows=len(rs), mismatching=sum(bool(r["mismatch"]) or r["rc"] != 0 for r in rs),
                ok_ne_targets=sum(r.get("ok") != r.get("targets") for r in rs),
                phase_last_ne_total=sum(not r.get("phase_last_eq_total") for r in rs),
                non_monotone=sum(not r.get("phase_monotone") for r in rs)), {r["i"]: r for r in rs}

RUNS_FIELDS = ["run (arm/bits/T/idx/seed/param)", "row index in rows.jsonl", "replay exact", "sha256[:16] of the replayed output without ms",
               "replayed by: i = instrumented source build (all rows), p = pristine source build (nonce sample), b = producer binary (nonce sample)",
               "scalar: v:<ok>/<targets> independently verified by the validator's EC code, c:<ok>/<targets> the replayed binary's counted check"]
def runs_list(rows, dbg, src, prod, ec):
    out = []
    for i, r in enumerate(rows):
        d = dbg[i]; e = ec.get(i)
        by = "i" + ("p" if i in src else "") + ("b" if i in prod else "")
        sc = f"v:{e['targets_checked'] - e['failures']}/{e['targets_checked']}" if e is not None else f"c:{r['ok']}/{r['targets']}"
        out.append([f"{r['arm']}/{r['pb']}/{r['T']}/{r['idx']}/{r['seed']}/{r['param']}", i, (not d["mismatch"]) and d["rc"] == 0, d["out_sha256"][:16], by, sc])
    return out


def dump_cert(cert, path):
    """indent the header, one run per line (keeps the file small and diffable)"""
    head = {k: v for k, v in cert.items() if k != "runs"}
    txt = json.dumps(head, indent=1)[:-2] + ',\n "runs": [\n' + ",\n".join("  " + json.dumps(r) for r in cert["runs"]) + "\n ]\n}\n"
    json.loads(txt); open(path, "w").write(txt)

# ---------------- Study A ----------------
A = f"{FR}/mt_fieldops_v3_20261111"
rowsA = L(f"{A}/results/holdout/rows.jsonl")
dA_stats, dA = replay_stats(f"{O}/dbg_v3_holdout.jsonl")
sA_stats, sA = replay_stats(f"{O}/src_v3_sample.jsonl")
pA_stats, pA = replay_stats(f"{O}/prod_v3_sample.jsonl")
tA_stats, _ = replay_stats(f"{O}/dbg_v3_tune.jsonl")
ecA = J(f"{O}/eccheck_v3.json"); ecA_by = {r["i"]: r for r in ecA["results"]}
neg = J(f"{O}/negctl_v3.json")
recA = J(f"{O}/recompute_v3.json"); rcA = J(f"{O}/results_check_v3.json"); tune = J(f"{O}/tune_check.json")
inst = J(f"{O}/instances_v3.json"); fresh = J(f"{O}/freshness.json")
sensA = J(f"{O}/sens_v3.json"); theory = J(f"{O}/theory.json")
certA = dict(schema="validator.replay-certificate/v1", status="verified", independent=True, scalars_verified=True,
    validator=WHO, validated_commit=HEAD, rows_commit="e821c5fbaf36ed36bbad1e9bf65d3f8ec9431a73",
    artifact_set={"results/holdout/rows.jsonl": sha(f"{A}/results/holdout/rows.jsonl"), "results/holdout/runs.json": sha(f"{A}/results/holdout/runs.json"),
                  "results/holdout/summary.json": sha(f"{A}/results/holdout/summary.json"), "results/tune/rows.jsonl": sha(f"{A}/results/tune/rows.jsonl"),
                  "frozen_ic5.json": sha(f"{A}/frozen_ic5.json"), "instances/MANIFEST.json": sha(f"{A}/instances/MANIFEST.json"),
                  "examples/mt_fieldops.rs (sha256 of the file at 70d0b40bc3 = 75dbc70b36)": src_blob},
    binary=dict(producer_path="target/release/examples/mt_fieldops", producer_sha256="773fb2ca930fa0c5a96bfa98e5c71a042d201b36e0a677ea28f92d1befcebfd6",
                rows_bin_sha256_all_equal=len({r["bin_sha256"] for r in rowsA}) == 1,
                pristine_source_build=dict(toolchain=TOOL, sha256=sha(f"{V}/vbuild/target/release/mt_fieldops"), source="git show 70d0b40bc3:examples/mt_fieldops.rs, standalone crate"),
                instrumented_build=dict(sha256=sha(f"{V}/dbgbuild/target/release/mt_fieldops"), patch="validation/scripts/patch_dbg.py (adds no counted operation)")),
    replay=dict(nonce=nonce, holdout_all_rows_instrumented=dA_stats, holdout_nonce_sample_pristine_build=sA_stats,
                holdout_nonce_sample_producer_binary=pA_stats, tune_all_rows_instrumented=tA_stats,
                fields_compared="every field the binary emits except ms: arm, n, p, targets, seed, param, ok, I, M, S, H, L, every phase snapshot, every arm extra",
                instances="frozen git-archive copy of 70d0b40bc3 (blob-identical to the recorded argv paths)",
                runs_json_exact_projection_of_rows_jsonl=recA["runs_json_exact_projection"]),
    scalar_verification=dict(method="instrumented build dumps (t, planted x_t, recovered log, Q_t); validator Python (projective Montgomery ladder, Fermat inversion; nothing shared with the harness) checks Q_t on the curve, 0 <= log < n, log == planted x_t, [log]G == Q_t; planted lists identical across arms of a configuration",
                             runs_independently_verified=ecA["runs"], runs_passing=ecA["runs_passing"], targets_independently_verified=ecA["targets"],
                             failures=ecA["failures"], per_arm_runs=ecA["per_arm"], cells=ecA["cells"],
                             configs_with_conflicting_planting=ecA["configs_with_conflicting_planting"],
                             sample_rule="per (arm, bits, T) one run (ic5: three), drawn with random.Random('v3val:' + nonce)",
                             negative_control=dict(arms=len(neg), all_caught=all(x["binary_caught"] and x["validator_caught"] for x in neg)),
                             remaining_runs="ok == targets from the binary's own counted check, reproduced exactly by replay; not independently re-derived"),
    runs_fields=RUNS_FIELDS, runs=runs_list(rowsA, dA, sA, pA, ecA_by))
dump_cert(certA, f"{V}/artifacts_A_cert.json")
print("A cert written", dA_stats, sA_stats, pA_stats, tA_stats, ecA["runs"], ecA["targets"], ecA["failures"])

# ---------------- Study B ----------------
B = f"{FR}/mt_rho_start_fix_20261111"
rowsB = L(f"{B}/results/merged.jsonl")
dB_stats, dB = replay_stats(f"{O}/dbg_b_merged.jsonl")
sB_stats, sB = replay_stats(f"{O}/src_b_sample.jsonl")
ecB = J(f"{O}/eccheck_b.json"); ecB_by = {r["i"]: r for r in ecB["results"]}
certB = dict(schema="validator.replay-certificate/v1", status="verified", independent=True, scalars_verified=True,
    validator=WHO, validated_commit=HEAD, rows_commit={"rho-family rows (merged lines 529-2112 = rows.jsonl)": "e821c5fbaf36ed36bbad1e9bf65d3f8ec9431a73",
                                                     "ic4 and bsgs2 rows (merged lines 1-528, from the v2 holdout)": "6b0ac6f192859544a30dc1422ccfa12e37776e6b"},
    artifact_set={"results/merged.jsonl": sha(f"{B}/results/merged.jsonl"), "results/rows.jsonl": sha(f"{B}/results/rows.jsonl"),
                  "results/runs.json": sha(f"{B}/results/runs.json"), "results/summary.json": sha(f"{B}/results/summary.json"),
                  "../mt_fieldops_v2_20261111/results/holdout/rows.jsonl (artifact store, local copy)": sha(f"{FR}/mt_fieldops_v2_20261111/results/holdout/rows.jsonl"),
                  "examples/mt_fieldops.rs (75dbc70b36 = 70d0b40bc3)": src_blob},
    binary=certA["binary"],
    replay=dict(nonce=nonce, merged_all_rows_instrumented=dB_stats, merged_nonce_sample_pristine_build=sB_stats,
                note="the 528 v2 ic4/bsgs2 rows (v2 binary bbcc68cc) replay exactly with the fixed source: their code is unchanged",
                fields_compared=certA["replay"]["fields_compared"],
                runs_json_exact_projection_of_merged_jsonl=J(f"{O}/recompute_b.json")["runs_json_exact_projection"]),
    scalar_verification=dict(method=certA["scalar_verification"]["method"], runs_independently_verified=ecB["runs"], runs_passing=ecB["runs_passing"],
                             targets_independently_verified=ecB["targets"], failures=ecB["failures"], per_arm_runs=ecB["per_arm"], cells=ecB["cells"],
                             configs_with_conflicting_planting=ecB["configs_with_conflicting_planting"],
                             sample_rule="per (arm, bits, T) one run, drawn with random.Random('v3val:' + nonce)",
                             negative_control="shared with Study A (same binary and checker)",
                             remaining_runs=certA["scalar_verification"]["remaining_runs"]),
    runs_fields=RUNS_FIELDS, runs=runs_list(rowsB, dB, sB, {}, ecB_by))
dump_cert(certB, f"{V}/artifacts_B_cert.json")
print("B cert written", dB_stats, sB_stats, ecB["runs"], ecB["targets"], ecB["failures"])

# ---------------- validator reports ----------------
def headline_from(summary_path, mine, pick):
    S = J(summary_path); H = []
    for s in S:
        key = (s.get("cand", "ic4"), s["vs"], s["cI"], s["bits"], s["T"], s["h"])
        if not pick(key): continue
        m = mine[key]
        H.append(dict(name=f"median {key[0]}/{key[1]} cI={key[2]} {key[3]}b T={key[4]} h={key[5]}", claimed=s["median"], recomputed=m["median"], rel_tol=1e-9))
        H.append(dict(name=f"95% hi {key[0]}/{key[1]} cI={key[2]} {key[3]}b T={key[4]} h={key[5]}", claimed=s["hi"], recomputed=m["hi"], rel_tol=1e-9))
    return H
mineA = {(e["cand"], e["vs"], e["cI"], e["bits"], e["T"], e["h"]): e for e in recA["entries"]}
fA = J(f"{V}/findings_A.json")
repA = dict(schema="validator.report/v1", status=fA["status"], independent=True, verdict=fA["verdict"], disposition=fA["disposition"],
    validator=WHO, validated_commit=HEAD, protocol_commit="f71b42cc4fb95adf303e154e01747274d239d31e (amendment A1 in 75dbc70b36)",
    claim_scope=fA["claim_scope"], scoped_dispositions=fA["scoped_dispositions"],
    raw_data=[dict(path="../results/holdout/runs.json", sha256=sha(f"{A}/results/holdout/runs.json"))],
    supporting_data=[dict(path="../results/holdout/rows.jsonl", sha256=sha(f"{A}/results/holdout/rows.jsonl"), note="full rows with phases and argv; replayed; in Git, not the artifact store (finding)"),
                     dict(path="../results/holdout/summary.json", sha256=sha(f"{A}/results/holdout/summary.json")),
                     dict(path="../results/tune/rows.jsonl", sha256=sha(f"{A}/results/tune/rows.jsonl")),
                     dict(path="replay-certificate.json", sha256=None)],
    checks=fA["checks"],
    recompute=dict(method="own script (validation/scripts/recompute.py) from runs.json: cost/target = [(cI I + M + S)/(cI + 3) + h (H + L)]/T; per configuration min over the 12 rho runs (rhobest) or 3 bsgs2 runs; median of 22 per-configuration ratios; re-implemented frozen CI procedure (B = 4000, Python random seed 1) and an independent numpy bootstrap (B = 20000, seeds 7 and 2026)",
                   summary_compare=recA["summary_compare"], decision_counts=recA["decision_counts"],
                   frozen_vs_numpy_decision_disagreements=len(recA["frozen_vs_numpy_decision_disagreements"]),
                   rhobest_winner_counts=recA["rhobest_winner_counts"], results_md_table_checks=rcA["table_checks"],
                   results_md_table_mismatches=len(rcA["table_mismatches"]), noverify_mismatches=len(rcA["noverify_mismatches"]),
                   noverify_ranges=rcA["noverify_ranges"], verify_gae_per_target_cI10=rcA["verify_gae_per_target_cI10"],
                   ic5_struct_32b_T1024=rcA["ic5_struct_32b_T1024"], ic5_phase_share_cI10_h01=rcA["ic5_phase_share_cI10_h01"],
                   tune=tune, instances={k: v for k, v in inst.items() if k != "results"}, freshness=dict(full_abp_match_elsewhere=fresh["full_match"], same_p_and_n_elsewhere=fresh["same_p_and_n_elsewhere"])),
    sensitivity=dict(readings={k: {kk: vv for kk, vv in v.items() if kk != "entries"} for k, v in sensA["readings"].items()},
                     rho_overhead_min_median_max=sensA["rho_overhead_min_median_max"], dp_estimate_ic5_over_ksdp=J(f"{O}/dp_estimate.json"),
                     first_order_models=theory, ks_runs=fA.get("ks_runs")),
    headline=headline_from(f"{A}/results/holdout/summary.json", mineA, lambda k: k[2] == 10 and k[0] == "ic5" and k[1] in ("rhobest", "ic4", "bsgs2")),
    findings=fA["findings"], required_amendments=fA["required_amendments"])
fB = J(f"{V}/findings_B.json"); recB = J(f"{O}/recompute_b.json"); rcB = J(f"{O}/results_check_b.json")
mineB = {(e["cand"], e["vs"], e["cI"], e["bits"], e["T"], e["h"]): e for e in recB["entries"]}
repB = dict(schema="validator.report/v1", status=fB["status"], independent=True, verdict=fB["verdict"], disposition=fB["disposition"],
    validator=WHO, validated_commit=HEAD, protocol_commit="75dbc70b36805ee9f136d462a871beb5970b9353",
    claim_scope=fB["claim_scope"], scoped_dispositions=fB["scoped_dispositions"],
    raw_data=[dict(path="../results/runs.json", sha256=sha(f"{B}/results/runs.json"))],
    supporting_data=[dict(path="../results/merged.jsonl", sha256=sha(f"{B}/results/merged.jsonl")), dict(path="../results/rows.jsonl", sha256=sha(f"{B}/results/rows.jsonl")),
                     dict(path="../results/summary.json", sha256=sha(f"{B}/results/summary.json"))],
    checks=fB["checks"],
    recompute=dict(method=repA["recompute"]["method"] + "; v2 analysis rules (IC4 vs rhobest, each rho arm, bsgs2)", summary_compare=recB["summary_compare"],
                   decision_counts=recB["decision_counts"], frozen_vs_numpy_decision_disagreements=len(recB["frozen_vs_numpy_decision_disagreements"]),
                   rhobest_winner_counts=recB["rhobest_winner_counts"], results_check={k: v for k, v in rcB.items() if k != "table"}, results_table=rcB["table"]),
    sensitivity=fB.get("sensitivity"),
    headline=headline_from(f"{B}/results/summary.json", mineB, lambda k: k[1] == "rhobest"),
    findings=fB["findings"], required_amendments=fB["required_amendments"])
json.dump(repA, open(f"{V}/artifacts_A_report.json", "w"), indent=1)
json.dump(repB, open(f"{V}/artifacts_B_report.json", "w"), indent=1)
print("reports assembled; headline A", len(repA["headline"]), "B", len(repB["headline"]))

# ---------------- write into the repository validation/ directories ----------------
for study, cert, rep in (("mt_fieldops_v3_20261111", certA, repA), ("mt_rho_start_fix_20261111", certB, repB)):
    vd = f"{REPO}/research/{study}/validation"
    dump_cert(cert, f"{vd}/replay-certificate.json")
    for item in rep["supporting_data"]:
        if item["path"] == "replay-certificate.json": item["sha256"] = sha(f"{vd}/replay-certificate.json")
    if not any(item["path"] == "replay-certificate.json" for item in rep["supporting_data"]):
        rep["supporting_data"].append(dict(path="replay-certificate.json", sha256=sha(f"{vd}/replay-certificate.json")))
    for extra, note in (("ks_runs.json", "validator retention-arm runs (sensitivity; scratch build of scripts/patch_ks.py)"),
                        ("rho_instrumentation.json", "per rho-family row: restart costs, phase-2 lookups and repeats (instrumented replay)")):
        rep["supporting_data"].append(dict(path=extra, sha256=sha(f"{vd}/{extra}"), note=note))
    json.dump(rep, open(f"{vd}/validator-report.json", "w"), indent=1)
    print("wrote", vd)
