#!/usr/bin/env python3
"""VALIDATOR: assemble replay-certificate.json, validator-report.json and claim-report.json from the evidence files
in validation/ (written by compact_evidence.py) and the scratch replay outputs.
usage: make_artifacts.py <scratch v5val dir> <validation dir> <frozen study dir> <repo> <frozen pristine main.rs> <pristine bin> <instr bin>"""
import json, sys, os, hashlib, collections
W, V, FZ, REPO, SRC, PB, IB = sys.argv[1:8]
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
J = lambda d, f: json.load(open(os.path.join(d, f)))
NONCE = open(os.path.join(W, "NONCE")).read().split()[0]
COMMIT = "2729e252e2e66196cd48bb1a3b58cb3dc1cc8cf0"; PROTO = "5aab996e554e9b1212c86bbe88d136c1c68ed547"
VALIDATOR = "independent validator subagent (claude-opus-5-5), 2026-11-12; validator code in validation/scripts/, not the producer's"
rows = [json.loads(l) for l in open(os.path.join(FZ, "results/holdout/rows.jsonl"))]
ra, rs, rp, rd = (J(W, f) for f in ("replay_all.json", "replay_sample.json", "replay_producer.json", "replay_dump.json"))
sc = J(V, "scalar_check.json"); nw = J(V, "native_work.json"); rc = J(V, "recompute.json"); rk = J(V, "results_check.json")
se = J(V, "sensitivity.json"); ic = J(V, "instances_check.json"); v4 = J(V, "v4_crosscheck.json"); cc = J(V, "claim_check.json")
art = {f"research/mt_fieldops_v5_20261112/{p}": sha(os.path.join(FZ, p)) for p in
       ["results/holdout/rows.jsonl", "results/holdout/runs.json", "results/holdout/summary.json", "results/holdout/summary_v4rule.json",
        "results/holdout/analysis.txt", "results/holdout/analysis_v4rule.txt", "results/holdout/noverify.txt", "PROTOCOL.md", "RESULTS.md",
        "run.py", "analyze.py", "noverify.py", "mk_runs.py", "plot.py", "instances/MANIFEST.json"]}
art["examples/mt_fieldops.rs"] = sha(SRC)
def rsum(x):
    s = x["summary"]; res = x["results"]
    return dict(n=s["rows_replayed"], mismatching=s["mismatching"], errors=s["errors"], ok_ne_targets=s["ok_ne_targets"], binary_sha256=s["binary_sha256"],
                per_arm=s["arms"], per_bits=s["bits"], per_arm_param=dict(collections.Counter(f"{r['arm']}:{r['param']}:{r['doff']}" for r in res)))
m_all = {x["row"]: x["match"] for x in ra["results"]}; m_s = {x["row"]: x["match"] for x in rs["results"]}
m_p = {x["row"]: x["match"] for x in rp["results"]}
ecok = {r["row"]: (not r["failures"] and r["targets_eq_row"]) for r in sc["runs"]}
runs = []
for i, r in enumerate(rows):
    key = f"{r['arm']}/{r['pb']}/{r['T']}/{r['idx']}/{r['seed']}/{r['param']}" + (f"/{r['doff']}" if r.get("doff") is not None else "")
    runs.append([key, i, m_all.get(i), m_s.get(i), m_p.get(i), ecok.get(i)])
all_ok = (rsum(ra)["mismatching"] == 0 and rsum(ra)["n"] == len(rows) and rsum(rs)["mismatching"] == 0 and rsum(rp)["mismatching"] == 0
          and rsum(rd)["mismatching"] == 0 and sc["summary"]["all_pass"] and rsum(ra)["ok_ne_targets"] == 0)
cert = dict(schema="validator.replay-certificate/v1", status="verified" if all_ok else "failed", independent=True,
    scalars_verified=bool(sc["summary"]["all_pass"]), validator=VALIDATOR, validated_commit=COMMIT,
    rows_commit="59e4e892b3e0d66a5c17ec26de71f071910ef902 (rows' commit field: bench-record-only commit after the protocol commit; study files and harness identical to the protocol commit " + PROTO + ")",
    head_during_validation="HEAD moved to 1f85ca3678 (bench metrics and leaderboard builder only; study and harness unchanged); validated from git archive of " + COMMIT,
    artifact_set=art,
    binary=dict(producer_path="target/release/examples/mt_fieldops", producer_sha256=rp["summary"]["binary_sha256"],
                rows_bin_sha256_all_equal=len({r["bin_sha256"] for r in rows}) == 1 and rows[0]["bin_sha256"] == rp["summary"]["binary_sha256"],
                pristine_source_build=dict(sha256=sha(PB), source=f"git show {PROTO[:10]}:examples/mt_fieldops.rs (sha256 {sha(SRC)}), standalone crate",
                                           toolchain="rustc 1.97.0 / cargo 1.97.0; rand 0.8.8 (rand_chacha 0.3.1, rand_core 0.6.4), serde_json 1.0.151 (arbitrary_precision, raw_value), release profile; Python 3 + numpy 2.5.3"),
                instrumented_build=dict(sha256=sha(IB), patch="validation/scripts/patch_instr.py (adds no counted operation: a separate atomic tally of HashMap calls, step-index, canonicalisation, hash, DP-test, W-step and restart-RNG calls, and MTFO_DUMP target dumps)")),
    replay=dict(nonce=NONCE, nonce_drawn="os.urandom(16) at 2026-11-12T03:31:42Z, before any sampling", rows_sha256=art["research/mt_fieldops_v5_20261112/results/holdout/rows.jsonl"],
                holdout_all_rows_instrumented=rsum(ra), holdout_nonce_sample_pristine_build=rsum(rs), holdout_nonce_sample_producer_binary=rsum(rp),
                holdout_dump_configs_instrumented=rsum(rd),
                sampling="random.Random(nonce + ':' + mode): pristine = 1 row per (arm, param, doff) x bits (51 rows, 17 at 32 bits); producer = 1 row per (arm, param, doff) (17 rows); dump = one (curve, seed) per (bits, T) cell, all 17 runs (153 rows)",
                fields_compared="every field the binary prints except ms: I, M, S, H, L, N, ok, targets, n, p, every phase snapshot and every arm extra; rows may carry only the runner's meta keys besides",
                instances="argv instance paths mapped to the frozen copy (git archive); every instance hashes to MANIFEST.json",
                runs_json_exact_projection_of_rows_jsonl=True),
    scalar_verification=dict(method="scripts/ecverify.py with scripts/ec.py (validator's Jacobian double-and-add, shares nothing with the harness): Q_t on curve, 0 <= log < n, log == planted x_t, [log]G == Q_t; planted lists identical across the 17 runs of each configuration",
                             **sc["summary"]),
    runs_fields=["key arm/bits/T/curve/seed/param[/doff]", "row", "instrumented_replay_match", "pristine_build_match (nonce sample)", "producer_binary_match (nonce sample)", "all targets independently verified (dump configs)"],
    runs=runs)
json.dump(cert, open(os.path.join(V, "replay-certificate.json"), "w"), indent=1)
# ---------------- validator report
r5 = rc["ranges_5a"]; r4 = rc["ranges_v4"]
def H(name, claimed, rec, ndp=None):
    d = dict(name=name, claimed=claimed, recomputed=rec)
    if ndp is not None: d["rel_tol"] = max(0.5 * 10 ** -ndp / abs(claimed), 1e-9) * 1.0000001
    return d
headline = [H("rows", 3366, rc["rows"]), H("unverified rows", 0, rc["unverified_rows"]),
            H("summary.json entries recomputed exactly", 972, rc["compare_5a"]["entries"] if rc["compare_5a"]["mismatching_fields"] == 0 else -1),
            H("summary_v4rule.json entries recomputed exactly", 972, rc["compare_v4"]["entries"] if rc["compare_v4"]["mismatching_fields"] == 0 else -1),
            H("icbest vs rhobest unranked (of 27), section 5a rule", 27, rc["decisions_5a"]["icbest vs rhobest"].get("unranked", 0)),
            H("ic5rw vs rhobest unranked (of 27)", 27, rc["decisions_5a"]["ic5rw vs rhobest"].get("unranked", 0)),
            H("icbest vs rhobest dearer (of 27), v4 rule", 27, rc["decisions_v4"]["icbest vs rhobest"].get("dearer", 0)),
            H("icbest vs bsgs2 dearer (of 27)", 27, rc["decisions_5a"]["icbest vs bsgs2"].get("dearer", 0))]
for c in rk["all"]:
    if c["ok"] and c["decimals"] > 0: headline.append(H("RESULTS/noverify: " + c["name"], c["claimed"], c["recomputed"], c["decimals"]))
    elif c["ok"]: headline.append(H("RESULTS: " + c["name"], c["claimed"], c["recomputed"]))
hl_mis = [h for h in headline if abs(h["claimed"] - h["recomputed"]) > h.get("rel_tol", 1e-9) * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]
assert not hl_mis, hl_mis
SV = {k: dict(decisions=v.get("decisions"), I10M_median_range=v.get("I10M_median_range"), h1_cells_hi_below_1_of_27=v.get("h1_cells_hi_below_1_of_27"),
              h0_cells_lo_above_1_of_27=v.get("h0_cells_lo_above_1_of_27"), rho_winner_I10M_h1=(v.get("rho_winner_I10M") or {}).get("1"))
      for k, v in se.items() if isinstance(v, dict) and "decisions" in v}
FIND = json.load(open(os.path.join(W, "findings.json")))
rep = dict(schema="validator.report/v1", status=FIND["status"], independent=True, verdict=FIND["verdict"], disposition=FIND["disposition"],
    validator=VALIDATOR, validated_commit=COMMIT, protocol_commit=PROTO,
    claim_scope=FIND["claim_scope"], scoped_dispositions=FIND["scoped_dispositions"],
    raw_data=[dict(path="../results/holdout/runs.json", sha256=art["research/mt_fieldops_v5_20261112/results/holdout/runs.json"])],
    supporting_data=[dict(path="../results/holdout/rows.jsonl", sha256=art["research/mt_fieldops_v5_20261112/results/holdout/rows.jsonl"], note="full rows with phases and argv; replayed; in Git, not the artifact store (V5-F7)")] +
        [dict(path=f, sha256=sha(os.path.join(V, f)), note=n) for f, n in (
            ("native_work.json", "instrumented replay of every row: HashMap calls, step-index, canonicalisation, hash, DP-test, W-step and restart-RNG tallies; identity checks against H and N"),
            ("scalar_check.json", "validator EC verification of every target of 153 dumped runs"),
            ("instances_check.json", "instance validity, regeneration and freshness"),
            ("recompute.json", "scripts/recompute.py: decisions, ranges, winners, noverify, comparison with summary.json and summary_v4rule.json"),
            ("results_check.json", "every number of RESULTS.md's table and ranges and of noverify.txt against the recompute"),
            ("sensitivity.json", "scripts/sens_analysis.py: N conventions, kappa above the grid, rho2x, modelled non-negation DP rho, cross-fit, two-probe IC count"),
            ("sens_runs.json", "validator sensitivity panels (792 runs, all targets verified by the harness)"),
            ("v4_crosscheck.json", "v5 binary on v4 rows and on the v4 validator's sensitivity rows: I, M, S, H, L, ok unchanged; N vs the v4 validator's instrumented counts"),
            ("claim_check.json", "clause-by-clause check of the claim statement"))],
    checks=FIND["checks"],
    recompute=dict(method="scripts/recompute.py (validator's own code) from runs.json; the protocol's bootstrap (random.Random(1).choices, B = 4000, order statistics 100 and 3899) and an independent numpy bootstrap (B = 20000, nonce-seeded)",
                   summary_5a=rc["compare_5a"], summary_v4rule=rc["compare_v4"], decisions_5a=rc["decisions_5a"], decisions_v4=rc["decisions_v4"],
                   numpy_bootstrap_decision_disagreements=rc["decision_draw_dependence_5a"] + rc["decision_draw_dependence_v4"],
                   ranges_5a=r5, ranges_v4=r4, winners_I10M_5a=rc["winners_I10M_5a"], noverify_I10M=rc["noverify_I10M"], noverify_ranges=rc["noverify_ranges"],
                   verify_gae_per_target_I10M=rc["verify_gae_per_target_I10M"], phase_violations=rc["phase_violations"],
                   results_md_and_noverify_checks=rk["checks"], results_md_mismatches=rk["mismatching"]),
    accounting=dict(native_work_identity_checks=nw["identity_checks"], rows=nw["rows"], totals_by_arm=nw["totals_by_arm"], v4_crosscheck=v4["summary"]),
    sensitivity=dict(method="scripts/sens_analysis.py; protocol decision rule and bootstrap; not pre-registered", variants=SV),
    headline=headline, findings=FIND["findings"], required_amendments=FIND["required_amendments"], accepted_results_headline=FIND["accepted_results_headline"])
json.dump(rep, open(os.path.join(V, "validator-report.json"), "w"), indent=1)
# ---------------- claim report
CL = json.load(open(os.path.join(W, "claim.json")))
claim = dict(schema="validator.report/v1", independent=True, validator=VALIDATOR, validated_commit=COMMIT,
    scope="this one statement exactly as worded; nothing else in the study", statement_judged=cc["statement"], statement_sha256=cc["statement_sha256"],
    status=CL["status"], verdict=CL["verdict"],
    raw_data=[dict(path="../results/holdout/runs.json", sha256=art["research/mt_fieldops_v5_20261112/results/holdout/runs.json"]),
              dict(path="claim_check.json", sha256=sha(os.path.join(V, "claim_check.json")), note="validator clause checks and sensitivity")],
    supporting=dict(rows_jsonl_sha256=art["research/mt_fieldops_v5_20261112/results/holdout/rows.jsonl"], replay_certificate="replay-certificate.json (status " + cert["status"] + ")",
                    validator_report="validator-report.json (status " + rep["status"] + ")", instances_check="instances_check.json"),
    clause_checks=cc["clause_checks"], headline=cc["headline"], headline_mismatches=cc["headline_mismatches"], notes=CL["notes"],
    proposed_wording=CL.get("proposed_wording"))
json.dump(claim, open(os.path.join(V, "claim-report.json"), "w"), indent=1)
print("cert", cert["status"], "report", rep["status"], "claim", claim["status"], "headline items", len(headline))
