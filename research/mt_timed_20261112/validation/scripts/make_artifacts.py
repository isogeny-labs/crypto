#!/usr/bin/env python3
"""VALIDATOR: write replay-certificate.json, validator-report.json and claim-report.json from the validator's evidence
files in V (recompute.json, results_check.json, replay_check.json, scalar_check.json, timing_reproduction.json,
price_model.json) and the validator's written judgement (scripts/findings.json, scripts/claim.json).
usage: make_artifacts.py <V> <frozen study dir> <nonce file>"""
import json, sys, os, hashlib

V, FZ, NF = sys.argv[1:4]
J = lambda f: json.load(open(os.path.join(V, f)))
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
rc, chk, rep, sc, tr, pm = (J(f) for f in ("recompute.json", "results_check.json", "replay_check.json", "scalar_check.json",
                                           "timing_reproduction.json", "price_model.json"))
findings = json.load(open(os.path.join(V, "scripts/findings.json")))
claim = json.load(open(os.path.join(V, "scripts/claim.json")))
nonce, nonce_t = open(NF).read().split()
VALIDATOR = "independent validator subagent (claude-opus-5-5), 2026-11-12; validator code in validation/scripts/, not the producer's"
VC = "544f8320631c9d71b7636d0c51ae031472457cf7"; PC = "a992ed6c807eee6fa713176412e6475e19db7946"
H = os.path.join(FZ, "results/holdout")
art = {f"research/mt_timed_20261112/results/holdout/{f}": sha(os.path.join(H, f)) for f in sorted(os.listdir(H))}
for f in ("PROTOCOL.md", "RESULTS.md", "run.py", "analyze.py", "mk_runs.py", "plot.py"):
    art[f"research/mt_timed_20261112/{f}"] = sha(os.path.join(FZ, f))
art["examples/mt_fieldops.rs"] = sha(os.path.join(FZ, "../../examples/mt_fieldops.rs"))
evid = {f: sha(os.path.join(V, f)) for f in ("recompute.json", "results_check.json", "replay_check.json", "scalar_check.json",
                                              "timing_reproduction.json", "price_model.json")}
runs_json = os.path.join(V, "../results/holdout/runs.json")
r1 = tr["analysis_round1"]; rounds = tr["analysis_rounds"]["rounds"]

# ------------------------------------------------------------------ replay certificate
all_rep = {x["key"]: x for x in rep["replay_timed_all"]["results"]}
prod_rep = {x["key"]: x["match"] for x in rep["replay_producer"]["results"]}
dump_keys = {x["key"]: x for x in sc["runs"]}
dump_fail = {x["key"]: not x["failures"] for x in sc["runs"]}
prod_keys = set()
for e in rc["per_configuration"]:
    for s in ("ic_key", "rho_key", "bsgs_key"):
        prod_keys.add((e["bits"], e["T"], e["idx"], e["ms_arm"][s][0], e["ms_arm"][s][1], e["ms_arm"][s][2]))
runs = []
for k in sorted(all_rep):
    x = all_rep[k]
    name = f"{x['arm']}/{x['pb']}/{x['T']}/{x['idx']}/43/{x['param']}" + (f"/{x['doff']}" if x["doff"] is not None else "")
    timed = (x["pb"], x["T"], x["idx"], x["arm"], x["param"], x["doff"]) in prod_keys
    runs.append([name, x["rows"], x["match"], prod_rep.get(k), dump_fail.get(k), timed or None])
cert = dict(
    schema="validator.replay-certificate/v1", status="verified", independent=True, scalars_verified=True, validator=VALIDATOR,
    validated_commit=VC, protocol_commit=PC,
    rows_commit="298863e5263e049567ddf42a229a3e847b4e2538 (rows' commit field: bench-record-only commit 8 s after the protocol commit; study and harness identical to a992ed6c80)",
    working_tree="study directory and examples/mt_fieldops.rs clean at 544f832063; the only dirty file is benchmarks/records/20261112T0422Z-mt-timed-ic-vs-rho.json (not part of the study, not validated); validated from git archive of 544f832063",
    head_during_validation="HEAD moved to 88ee570743 (benchrec metrics for this run: benchmarks/records/20261112T0422Z-mt-timed-ic-vs-rho.json only; study and harness unchanged); not validated",
    artifact_set=art,
    binary=dict(producer_path="target/release/examples/mt_fieldops", producer_sha256="77bb50922d5f22fc6614354f283869816fa43b8b0f023a578d87efeb409e52e4",
                rows_bin_sha256_all_equal=True,
                rebuild=dict(sha256="77bb50922d5f22fc6614354f283869816fa43b8b0f023a578d87efeb409e52e4", bit_identical=True,
                             command="CARGO_TARGET_DIR=<scratch>/repobuild/target cargo build --release --example mt_fieldops --offline (repository source, examples/mt_fieldops.rs sha256 a26f8802...0a4 = git show a992ed6c80)"),
                validator_standalone_build=dict(sha256=rep["replay_timed_all"]["bin_sha256"], source="git show a992ed6c80:examples/mt_fieldops.rs, standalone crate, rand =0.8.8, serde_json =1.0.151 (arbitrary_precision, raw_value), opt-level 3"),
                validator_dump_build=dict(sha256=rep["replay_dump"]["bin_sha256"], patch="scripts/patch_dump.py: adds only an MTFO_DUMP output of planted logs, targets and recovered logs after the verification pass"),
                toolchain="rustc 1.97.0 / cargo 1.97.0"),
    replay=dict(nonce=nonce, nonce_drawn=f"os.urandom(16) at {nonce_t}, before any sampling",
                holdout_all_keys_validator_build=rep["replay_timed_all"]["summary"],
                producer_binary_nonce_sample=rep["replay_producer"]["summary"],
                v5_rows_with_timed_binary=rep["replay_v5"]["summary"],
                dump_build_nonce_sample=rep["replay_dump"]["summary"],
                fields_compared="every field the binary prints except ms_arm and ms (I, M, S, H, L, N, ok, targets, every phase snapshot, every arm extra), against every repeat of the key",
                counters_identical_across_repeats=rc["checks"]["keys_with_counters_differing_across_repeats"] == 0,
                runs_json_exact_projection=rc["checks"]["runs_json_vs_runs_jsonl_field_mismatches"] == 0 and rc["checks"]["runs_json_source_sha256_matches"]),
    scalar_verification=dict(method="scripts/ec_check.py with scripts/ecv.py (validator's affine double-and-add, shares nothing with the harness): Q_t on curve, 0 <= log < n, log == planted, [log]G == Q_t; planted lists identical across runs of a configuration; instances: p, n prime, [n]G = O, cofactor 1 by Hasse",
                             **sc["summary"], harness_counted_check=dict(rows=rc["checks"]["rows_jsonl"], ok_ne_targets=rc["checks"]["ok_ne_targets"],
                                                                        targets_verified=rc["checks"]["targets_verified_total"])),
    timing_reproduction=dict(
        note="wall time does not replay exactly; this is a reproduction on the same host and the same bit-identical binary, pinned to CPU 2, sequential, with the producer's per-configuration keys (selected on the producer's medians, scored on new timings)",
        round1=dict(design="part A: 36 configurations x producer's fastest IC, rho and bsgs2 key; part B: one nonce-drawn configuration per cell x all 17 keys; 3 repeats, nonce-seeded shuffle per round; opcost before and after",
                    sampled_configurations=tr["round1_header"]["sampled_idx"], executions=r1["interference"]["executions"],
                    all_verified=r1["all_verified"], counter_mismatches=r1["counter_mismatches_vs_producer"],
                    cells=[dict(bits=c["bits"], T=c["T"], validator=[round(c["mine_median"], 4), round(c["mine_lo"], 4), round(c["mine_hi"], 4)], validator_decision=c["mine_verdict"],
                                producer=[round(c["prod_median"], 4), round(c["prod_lo"], 4), round(c["prod_hi"], 4)], producer_decision=c["prod_verdict"]) for c in r1["cells"]],
                    decisions_reproduced=sum(c["mine_verdict"] == c["prod_verdict"] for c in r1["cells"]),
                    independent_best_of_partB=[dict(cfg=b["cfg"], validator=round(b["mine_bestof_ratio"], 4), producer=round(b["prod_ratio"], 4)) for b in r1["partB"]],
                    absolute_time_ratio_median=r1["absolute_time"]["overall_median"]),
        round2=dict(design="added after round 1 (not pre-registered): 36 configurations x producer's fastest IC and rho key, 5 repeats",
                    executions=tr["analysis_rounds"]["round2_interference"]["executions"], all_verified=tr["analysis_rounds"]["round2_interference"]["all_verified"],
                    cells=[dict(bits=c["bits"], T=c["T"], validator=[round(c["median"], 4), round(c["lo"], 4), round(c["hi"], 4)], decision=c["verdict"]) for c in rounds["round2"]["cells"]],
                    decisions_reproduced=sum(c["verdict"] == c["prod_verdict"] for c in rounds["round2"]["cells"])),
        pooled=dict(repeats_per_key=8, cells=[dict(bits=c["bits"], T=c["T"], validator=[round(c["median"], 4), round(c["lo"], 4), round(c["hi"], 4)], decision=c["verdict"],
                                                   producer_decision=c["prod_verdict"], producer_fixed_key_decision=c["prod_fixed_key_verdict"]) for c in rounds["pooled"]["cells"]],
                    decisions_reproduced=sum(c["verdict"] == c["prod_verdict"] for c in rounds["pooled"]["cells"]), size_level=rounds["pooled"]["size_level"]),
        interference=dict(round1={k: v for k, v in r1["interference"].items() if k != "proc_stat_delta"},
                          round2={k: v for k, v in tr["analysis_rounds"]["round2_interference"].items() if k not in ("header", "footer")}),
        verdict="directions reproduced (2^24 faster, 2^32 slower, 2^28 near parity); all 9 decisions reproduced on the pooled 8-repeat rerun and every producer interval overlaps the validator's; the three edge cells (24 b T = 16384, 28 b T = 1024, 32 b T = 1024) were unranked in the 3-repeat round 1, as in the producer's own fixed-key control; the 28 b T = 1024 magnitude is 0.98x, not 0.93x"),
    runs_fields=["key arm/bits/T/curve/seed/param[/doff]", "rows", "validator_build_replay_match", "producer_binary_match (nonce sample)",
                 "all targets independently EC-verified (dump sample)", "re-timed by the validator (producer's selected keys)"],
    runs=runs)
json.dump(cert, open(os.path.join(V, "replay-certificate.json"), "w"), indent=1)

# ------------------------------------------------------------------ validator report
cells = {(c["bits"], c["T"]): c for c in rc["cells"]}
def head(name, claimed, recomputed, prec=None):
    d = dict(name=name, claimed=claimed, recomputed=recomputed)
    if prec is not None:
        d["rel_tol"] = (0.5 * 10 ** -prec) / abs(claimed) + 1e-12 if claimed else 1e-9
    return d
O = rc["opcost_ranges"]
headline = [
    head("executions", 1836, rc["checks"]["rows_jsonl"]), head("errors or unverified", 0, rc["checks"]["errors"] + rc["checks"]["ok_ne_targets"]),
    head("summary.json fields recomputed exactly", 272, chk["summary_fields"] - len(chk["summary_mismatches"])),
    head("ic/rho solve 2^24 min", 0.78, min(cells[(24, T)]["ic_rho_solve"]["median"] for T in (1024, 4096, 16384)), 2),
    head("ic/rho solve 2^24 max", 0.83, max(cells[(24, T)]["ic_rho_solve"]["median"] for T in (1024, 4096, 16384)), 2),
    head("cells faster at 2^24 (primary rule)", 3, sum(cells[(24, T)]["verdict_solve"] == "faster" for T in (1024, 4096, 16384))),
    head("ic/rho solve 2^28 T=1024", 0.93, cells[(28, 1024)]["ic_rho_solve"]["median"], 2),
    head("cells unranked at 2^28 T=4096,16384", 2, sum(cells[(28, T)]["verdict_solve"] == "unranked" for T in (4096, 16384))),
    head("ic/rho solve 2^32 min", 1.11, min(cells[(32, T)]["ic_rho_solve"]["median"] for T in (1024, 4096, 16384)), 2),
    head("ic/rho solve 2^32 max", 1.29, max(cells[(32, T)]["ic_rho_solve"]["median"] for T in (1024, 4096, 16384)), 2),
    head("cells slower at 2^32 (primary rule)", 3, sum(cells[(32, T)]["verdict_solve"] == "slower" for T in (1024, 4096, 16384))),
    head("fixed-key control min", 0.75, min(c["fixed_ic_rho_solve"]["median"] for c in rc["cells"]), 2),
    head("fixed-key control max", 1.28, max(c["fixed_ic_rho_solve"]["median"] for c in rc["cells"]), 2),
    head("A/A intervals containing 1", 9, sum(c["verdict_aa_contains_1"] for c in rc["cells"])),
    head("ic/bsgs min", 1.33, min(c["ic_bsgs_solve"]["median"] for c in rc["cells"]), 2),
    head("ic/bsgs max", 1.59, max(c["ic_bsgs_solve"]["median"] for c in rc["cells"]), 2),
    head("GAE ns min", 214, O["gae_ns"][0], 0), head("GAE ns max", 294, O["gae_ns"][1], 0),
    head("I/M min", 23, O["I_over_M"][0], 0), head("I/M max", 31, O["I_over_M"][1], 0),
    head("canonicalisation+hash GAE min", 0.005, O["native_gae"][0], 3), head("canonicalisation+hash GAE max", 0.009, O["native_gae"][1], 3),
    head("std lookup 2^10-2^16 GAE min", 0.04, O["std_lookup_2p10_2p16"][0], 2), head("std lookup 2^10-2^16 GAE max", 0.15, O["std_lookup_2p10_2p16"][1], 2),
    head("std lookup 2^22-2^24 GAE min", 0.27, O["std_lookup_2p22_2p24"][0], 2), head("std lookup 2^22-2^24 GAE max", 0.85, O["std_lookup_2p22_2p24"][1], 2),
    head("rhodpu kappa 0 fastest rho (configurations)", 29, rc["winners"]["rho"].get("rhodpu k=0.0")),
]
mism = [h["name"] for h in headline if abs(h["claimed"] - h["recomputed"]) > h.get("rel_tol", 1e-9) * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]
pooled = rounds["pooled"]
report = dict(
    schema="validator.report/v1", status="fail", independent=True,
    verdict=("headline as stated NOT validated. The measurement is validated: every counter replays, every recovered log verifies, the binary rebuilds bit for bit, "
             "and every headline number recomputes exactly from runs.json under the frozen rule; the validator's pinned rerun reproduces the directions and, pooled over 8 repeats, all 9 cell decisions. "
             "Two parts of the headline as stated are not supported by the data: the selection-bias control does not show 'the same pattern' (its decisions differ from the primary rule in 3 of 9 cells, unranked at 24 b T = 16384, 28 b T = 1024 and 32 b T = 1024; it is unranked in 5 of 9 cells overall, counting 28 b T = 4096 and 16384, which are unranked under both), "
             "and the 0.005-0.009 GAE 'native operation' price is for a canonicalisation plus a hash, two counted native operations (one is 0.002-0.005 GAE; a step index 0.002-0.006). "
             "The 28 b T = 1024 '0.93x faster' is fragile (unranked under the control and in each validator round alone; 0.98x pooled). The amended headline in accepted_results_headline is validated."),
    disposition="insufficient_evidence",
    validator=VALIDATOR, validated_commit=VC, protocol_commit=PC,
    claim_scope="solve-phase wall time (ms_arm) on one CPU of the session VM; icbest = fastest of ic5, ic5ks, ic5w, ic5rw (mu = 1, k = 4), rhobest = fastest of rho2xks x 4 kappa and rhodpu x 6, bsgsbest = fastest of bsgs2 x 3; key time = median of 3 interleaved repeats; cell = 4 configurations; decision faster/slower if the protocol bootstrap 95% bound of the median ratio is on one side of 1",
    scoped_dispositions={
        "measurement (binary provenance, counter replay, scalars, recompute)": "validated: binary rebuilt bit-identical; 612/612 keys (1836 rows) replay exactly on a validator build; 17-key producer-binary sample and 35 v5 rows match; 319,488 recovered logs of 54 runs verified with the validator's EC code; 272/272 summary.json fields recompute exactly",
        "2^24 faster and 2^32 slower (size level)": "validated: 12/12 configurations each side in the producer's data, 11/12 in validator round 1, 12/12 in round 2 and pooled; magnitudes 0.78-0.83 / 1.11-1.29 (producer) vs 0.77-0.84 / 1.11-1.22 (validator pooled)",
        "per-cell decisions under the primary rule": "reproduced on the pooled 8-repeat rerun (9/9); round 1 alone 6/9, round 2 alone 8/9; each decision is a 4-of-4 sign rule (T-F4)",
        "2^28 T = 1024 '0.93x faster'": "insufficient_evidence: unranked under the pre-registered fixed-key control, unranked in validator rounds 1 and 2 alone, 0.980 [0.959, 0.999] pooled",
        "selection-bias control 'same pattern 0.75-1.28'": "range validated; 'same pattern' not supported at the decision level (T-F1)",
        "A/A intervals contain 1 in all 9 cells": "validated",
        "IC 1.33-1.59x slower than BSGS": "validated (cell medians; per configuration 1.23-1.87; validator round 1 1.30-1.63)",
        "opcost prices": "GAE 214-294 ns, I/M 23-31, std lookup 0.04-0.15 (2^10-2^16) and 0.27-0.85 (2^22-2^24) validated and reproduced within noise; 'native op 0.005-0.009 GAE' mislabelled (T-F2)",
        "rhodpu fastest rho in 29/36": "validated (kappa 0; plus 1 at kappa 0.25; rho2xks 6, not 7 as RESULTS says)",
        "interpretation (h_native about 0 favours DP rho; cache misses at 2^32)": "labelled interpretation in RESULTS; lookup-price account quantitatively consistent at 2^32, not at 2^24; cache misses unmeasured (T-F7)"},
    raw_data=[dict(path="../results/holdout/runs.json", sha256=sha(runs_json))],
    supporting_data=[dict(path="../results/holdout/runs.jsonl", sha256=art["research/mt_timed_20261112/results/holdout/runs.jsonl"], note="full rows; replayed; in Git, not the artifact store (T-F13)"),
                     dict(path="../results/holdout/opcost.jsonl", sha256=art["research/mt_timed_20261112/results/holdout/opcost.jsonl"]),
                     *[dict(path=f, sha256=h) for f, h in evid.items()]],
    checks=dict(design=rc["checks"], results_check=dict(summary_fields=chk["summary_fields"], summary_mismatches=chk["summary_mismatches"], max_rel_diff=chk["max_rel_diff"],
                                                         quoted_failures=chk["quoted_failures"])),
    recompute=dict(method="scripts/recompute.py (validator's own code) from runs.jsonl/runs.json; protocol bootstrap (random.Random(1), B = 4000, order statistics 100 and 3899), the exact bootstrap over all 4^4 resamples and a numpy bootstrap (B = 20000, nonce-seeded): identical decisions in all 9 cells",
                   cells=[dict(bits=c["bits"], T=c["T"], ic_rho_solve=[c["ic_rho_solve"][k] for k in ("median", "lo", "hi")], decision=c["verdict_solve"],
                               ic_rho_all=[c["ic_rho_all"][k] for k in ("median", "lo", "hi")], ic_bsgs=c["ic_bsgs_solve"]["median"],
                               aa_rho=[c["aa_rho"][k] for k in ("median", "lo", "hi")], fixed=[c["fixed_ic_rho_solve"][k] for k in ("median", "lo", "hi")],
                               fixed_decision=c["verdict_fixed"], loo_median=c["loo_ic_rho_solve"]["median"],
                               loo_below1=[c["loo_ic_rho_solve"]["below1"], len(c["loo_ic_rho_solve"]["values"])]) for c in rc["cells"]],
                   size_level=rc["size_level"], global_aa=rc["global_aa"], winners=rc["winners"], opcost_ranges=rc["opcost_ranges"]),
    timing_reproduction=cert["timing_reproduction"],
    price_model=dict(note="validator diagnostic, not pre-registered (scripts/price_model.py)", per_cell=[{k: v for k, v in c.items() if k != "configs"} for c in pm["per_cell"]]),
    reporting_contract=dict(one_table_one_unit=True, ratio_reported=True, phases_separated="solve (ms_arm) and solve + shared verification (ms) reported separately",
                            reference_arm_priced="in wall time every arm's work is inside ms_arm (no unpriced counter); the section 5a band of the counted study is v5's",
                            null_A_A=True, cheaper_baseline="bsgs2 reported (IC 1.33-1.59x slower)", frozen_benchmark="v5 instances, seed 43",
                            external_fastest_implementation="not compared; the claim is scoped to this harness's implementation and says so",
                            isolation="not L2: one shared cloud VM, labelled diagnostic in PROTOCOL, RESULTS and the claim",
                            m83_ecc2k130="RESULTS Limits: says nothing about ECC2K-130 / m = 83"),
    headline=headline, headline_mismatches=mism,
    findings=findings,
    required_amendments=[
        "T-F1 (blocking for the headline as stated): replace 'Selection-bias control ...: 0.75-1.28x, the same pattern' with the control's decisions: faster at 2^24 T = 1024 and 4096, unranked at 2^24 T = 16384, unranked at all 2^28 cells, unranked at 2^32 T = 1024, slower at 2^32 T = 4096 and 16384 (medians 0.75-1.28x).",
        "T-F2 (blocking for the claim): state the native price as 'canonicalisation plus one hash of x (two counted native operations) 0.005-0.009 GAE; a step index 0.002-0.006 GAE'.",
        "T-F3/T-F4 (disclosure): say that each cell decision is a 4-of-4 sign rule (the n = 4 bootstrap interval equals the range), that the 2^28 T = 1024 'faster' is unranked under the fixed-key control and 0.98x in the validator's rerun, and that the validator's pooled rerun reproduces all 9 decisions at 0.77-0.84x (2^24) and 1.11-1.22x (2^32).",
        "T-F5 (claim): 'the 12 prime-order Weierstrass curves of the v5 study' instead of '12 fresh ... curves'.",
        "T-F6 (non-blocking prose): rho2xks wins 6 configurations, rhodpu kappa 0.25 wins 1; std lookup 2^18-2^20 is 0.11-0.42; the fast hasher is 1.2-6.1x cheaper (median 3.8); drop 'although it never won in v5's counted model' (v5: rhodpu wins 4 of 198 at h = 1).",
        "T-F7 (interpretation): keep 'lookups miss cache' labelled as an untested mechanism (no PMU); the measured lookup prices account for the 2^32 gap but not for the 2^24 advantage.",
        "T-F13/T-F14 (non-blocking): move runs.jsonl to the artifact store; preserve or account for the smoke run (W24-0, seed 43, T = 1024 is also a holdout configuration)."],
    accepted_results_headline=claim["proposed_wording"])
json.dump(report, open(os.path.join(V, "validator-report.json"), "w"), indent=1)

# ------------------------------------------------------------------ claim report
st_ = claim["statement_judged"]
clauses = [
    ("On one CPU of a 4-vCPU cloud VM (Intel Xeon @ 2.10 GHz; not an isolated lab node, so the seconds are diagnostic)", True,
     "rows host: 'Intel(R) Xeon(R) Processor @ 2.10GHz', nproc 4, pin 2; taskset -c 2 in every argv; not L2"),
    ("in this harness's implementation", True, "examples/mt_fieldops.rs at a992ed6c80; binary rebuilt bit-identical"),
    ("12 fresh prime-order Weierstrass curves of group order about 2^24, 2^28 and 2^32", False,
     "valid prime-order curves (n of 24-25, 28-29, 32-33 bits) but they are the v5 instances W{24,28,32}-{0..3}, already used in v5 (seeds 41, 42): not fresh (T-F5)"),
    ("T = 1024/4096/16384) x 17 run keys x 3 interleaved repeats, every target verified", True,
     "1836 rows = 36 x 17 x 3, order re-derived; ok = targets in every row; 319,488 logs re-verified with validator EC code"),
    ("0.78-0.83x ... at 2^24 (faster in all 3 cells)", True,
     "0.7777-0.8288, faster in 3/3 under the frozen rule (recomputed exactly); reproduced in validator round 2 and pooled (0.77-0.84); but the fixed-key control and validator round 1 put 2^24 T = 16384 at unranked, and each decision is a 4-of-4 sign rule (T-F1, T-F3, T-F4) - not disclosed"),
    ("0.93x at 2^28 with T = 1024", True,
     "0.9319, faster under the frozen rule; not robust: fixed-key control unranked (0.975), validator rounds 1 and 2 unranked, pooled 0.980 [0.959, 0.999] - not disclosed"),
    ("is unranked at 2^28 with T = 4096 and 16384", True, "1.053 [0.987, 1.101], 1.007 [0.954, 1.193]; reproduced"),
    ("takes 1.11-1.29x at 2^32 (slower in all 3 cells)", True,
     "1.1144-1.2885, slower in 3/3 (recomputed exactly); reproduced pooled 1.11-1.22 (slower 3/3); fixed-key control and validator round 1 unranked at T = 1024 - not disclosed"),
    ("it takes 1.33-1.59x the time of a paired BSGS", True, "cell medians 1.3265-1.5899"),
    ("one affine addition 214-294 ns, I/M 23-31", True, "214.4-293.8 ns; 22.97-30.58; padd chain is exactly I + 2M + S per step"),
    ("a native operation 0.005-0.009 GAE", False,
     "0.0049-0.0094 is canonicalisation + one hash of x = two counted native operations; per counted native operation 0.0025-0.0047, a step index 0.0024-0.0057 (T-F2)"),
    ("a hash-table lookup 0.04-0.85 GAE depending on table size", True, "std HashMap (SipHash) lookup at 2^10-2^24 entries: 0.0375-0.8528; independent-probe loop"),
]
creport = dict(
    schema="validator.report/v1", independent=True, validator=VALIDATOR, validated_commit=VC,
    scope="this one statement exactly as worded; nothing else in the study",
    statement_judged=st_, statement_sha256=hashlib.sha256(st_.encode()).hexdigest(),
    status="fail",
    verdict=("not endorsed as worded: every number in it recomputes from runs.json and opcost.jsonl, but 'fresh' curves is wrong (v5 curves), "
             "'a native operation 0.005-0.009 GAE' prices two counted native operations, and the statement omits that the pre-registered selection-bias control departs from the primary "
             "decision in 3 of 9 cells (unranked at one cell at each end and at 28 b T = 1024; unranked in 5 of 9 overall) and that 28 b T = 1024 is 0.98x in the validator's rerun. The proposed wording is endorsed."),
    disposition="insufficient_evidence",
    raw_data=[dict(path="../results/holdout/runs.json", sha256=sha(runs_json)),
              dict(path="../results/holdout/opcost.jsonl", sha256=art["research/mt_timed_20261112/results/holdout/opcost.jsonl"]),
              dict(path="timing_reproduction.json", sha256=evid["timing_reproduction.json"], note="validator's pinned rerun")],
    supporting=dict(replay_certificate="replay-certificate.json (status verified)", validator_report="validator-report.json (status fail: headline as stated)",
                    results_check="results_check.json", scalar_check="scalar_check.json"),
    clause_checks=[dict(clause=c, ok=ok, recomputed=r) for c, ok, r in clauses],
    headline=[h for h in headline if h["name"] not in ("summary.json fields recomputed exactly", "fixed-key control min", "fixed-key control max",
                                                        "A/A intervals containing 1", "rhodpu kappa 0 fastest rho (configurations)")],
    headline_mismatches=mism,
    notes=["Every number in the statement recomputes from the raw data at its quoted precision; the failures are wording (fresh; native-operation unit) and omitted robustness information.",
           "The proposed wording adds the 4-of-4 nature of each cell decision, the fixed-key control's unranked cells, and the validator's reproduction (pooled 8 repeats: all 9 decisions, 0.77-0.84x at 2^24, 1.11-1.22x at 2^32)."],
    proposed_wording=claim["proposed_wording"],
    proposed_wording_sha256=hashlib.sha256(claim["proposed_wording"].encode()).hexdigest())
json.dump(creport, open(os.path.join(V, "claim-report.json"), "w"), indent=1)
print("headline mismatches", mism)
for f in ("replay-certificate.json", "validator-report.json", "claim-report.json"):
    print(f, sha(os.path.join(V, f)))
