#!/usr/bin/env python3
"""Write the gate artifacts of VAL-20261115-276156 from the validator's own numbers.

Reads own-numbers.json (recompute.py), prune-model.json (prune_model.py), the validator's replay receipts
(replay/, replay-old/, fresh/), and only then the producer's claimed numbers (results.json, band/*.json,
summary.json, data/correctness.json) to compare. Writes validator-report.json (raw_data relative to this
directory), validator-report-benchmarks.json (identical but for raw_data paths, relative to
benchmarks/20261115-ksum-prune-tableb/), replay-certificate.json, and the two VALIDATION.json manifests.
usage: python3 -I mkreports.py <repo root>
"""
import glob, gzip, hashlib, json, os, sys

ROOT = os.path.abspath(sys.argv[1])
STUDY = os.path.join(ROOT, "research/ic_ksum_prune_20261115")
VAL = os.path.join(STUDY, "validator")
BENCH = os.path.join(ROOT, "benchmarks/20261115-ksum-prune-tableb")
VAL_ID = "VAL-20261115-276156"
ROWS = ["P20", "P22", "P24", "B15", "B18", "B21", "B24", "B27"]
WK = ["0", "0.1", "1"]
REL = 1e-9


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def rel(p, base):
    return os.path.relpath(p, base)


own = json.load(open(os.path.join(VAL, "own-numbers.json")))
model = json.load(open(os.path.join(VAL, "prune-model.json")))
negc = json.load(open(os.path.join(VAL, "negative-controls.json")))
nonce = dict(l.split("=", 1) for l in open(os.path.join(VAL, "nonce.txt")).read().splitlines())

# ---- replay receipts --------------------------------------------------------------------------
def receipts(d, base_sessions):
    out = []
    summ = {}
    for l in open(os.path.join(d, "summary.txt")).read().splitlines():
        n, rc, _ = l.split("\t"); summ[n] = int(rc.split("=")[1])
    for p in sorted(glob.glob(os.path.join(d, "*.verify.json.gz"))):
        n = os.path.basename(p)[:-len(".verify.json.gz")]
        r = json.load(gzip.open(p))
        out.append({"run": rel(os.path.join(base_sessions(n), n), ROOT), "session_id": r["session_id"],
                    "ok": r["ok"] and not r["problems"], "exit_code": summ[n],
                    "receipt": rel(p, ROOT), "receipt_sha256": sha(p),
                    "records": r["records"], "verified_records": r["verified_records"],
                    "replays": len(r["replays"]), "replays_reproduced": sum(x["reproduced"] for x in r["replays"]),
                    "auditor_binary_sha256": r["auditor_binary_sha256"]})
    return out


runs = receipts(os.path.join(VAL, "replay"), lambda n: os.path.join(STUDY, "sessions"))
old = receipts(os.path.join(VAL, "replay-old"),
               lambda n: os.path.join(ROOT, "research/ic_tableb_ksum_20261115/sessions" if n[0] == "P"
                                      else "research/ic_tableb_binary_ksum_20261115/sessions"))
fresh = {}
for n in ("FB15", "FP24", "FP22"):
    p = os.path.join(VAL, "fresh", n + ".verify.json.gz")
    r = json.load(gzip.open(p))
    F = own["fresh_sessions"][n]
    fresh[n] = {"dir": rel(os.path.join(VAL, "fresh", n), ROOT), "spec": rel(os.path.join(VAL, "fresh", n + ".spec.json"), ROOT),
                "session_id": r["session_id"], "target_seed": F["session"]["target_seed"],
                "measurement_seed": F["session"]["measurement_seed"], "targets_per_curve": 48, "rounds": 3, "warmup": 1,
                "arms": F["session"]["arms"], "git_commit": F["session"]["git_commit"], "git_dirty": F["session"]["git_dirty"],
                "binary_sha256": F["session"]["session_binary_sha256"],
                "verify": {"ok": r["ok"] and not r["problems"], "records": r["records"],
                           "replays_reproduced": sum(x["reproduced"] for x in r["replays"]), "replays": len(r["replays"]),
                           "receipt": rel(p, ROOT), "receipt_sha256": sha(p)},
                "scalar_certificate_failures": F["certificates"]["failures"],
                "correctness_disallowed_differences": {k: v["disallowed"] for k, v in F["correctness"].items()},
                "ratios": F["ratios"]}
assert all(x["ok"] and x["exit_code"] == 0 and x["replays"] == x["replays_reproduced"] for x in runs + old)
assert len(runs) == 24 and len(old) == 24

# ---- claimed numbers (producer files), read only now ------------------------------------------
res = json.load(open(os.path.join(STUDY, "results.json")))
summ = json.load(open(os.path.join(STUDY, "summary.json")))
pcorr = json.load(open(os.path.join(STUDY, "data/correctness.json")))
headline, mism, boot_dev = [], [], 0.0


def add(name, claimed, recomputed):
    headline.append({"name": name, "claimed": claimed, "recomputed": recomputed})
    if abs(claimed - recomputed) > REL * max(abs(claimed), abs(recomputed), 1e-300):
        mism.append(name)


verdict_cmp = {}
for row in res["rows"]:
    rid = row["id"]; R = own["rows"][rid]
    for wk in WK:
        W = row["weights"][wk]
        for arm, v in W["S"].items():
            add("%s/S/%s/w%s" % (rid, arm, wk), v, R["mean_S"][arm][wk])
        add("%s/rho-neg-aa_over_rho-neg/w%s" % (rid, wk), W["aa"]["ratio"], R["ratios"]["rho-neg-aa_over_rho-neg"][wk]["pooled"]["ratio"])
        for k in ("5", "4"):
            key = "ksum%s-p_over_ksum%s" % (k, k)
            c = W["ksum" + k]; o = R["ratios"][key][wk]
            add("%s/%s/w%s" % (rid, key, wk), c["ratio_p_over_u"]["ratio"], o["pooled"]["ratio"])
            add("%s/%s/w%s/ci95_linearised_t_lo" % (rid, key, wk), c["ratio_p_over_u"]["ci95"][0], o["pooled"]["lin_ci95"][0])
            add("%s/%s/w%s/ci95_linearised_t_hi" % (rid, key, wk), c["ratio_p_over_u"]["ci95"][1], o["pooled"]["lin_ci95"][1])
            for i, rep in enumerate(c["per_replicate"]):
                op = o["per_replicate"]["%s-r%d" % (rid, i + 1)]
                add("%s/%s/r%d/w%s" % (rid, key, i + 1, wk), rep["ratio"], op["ratio"])
                add("%s/%s/r%d/w%s/ci95_linearised_t_hi" % (rid, key, i + 1, wk), rep["ci95"][1], op["lin_ci95"][1])
                boot_dev = max(boot_dev, abs(rep["ci95_bootstrap"][1] - op["boot_ci95"][1]))
            boot_dev = max(boot_dev, abs(c["ratio_p_over_u"]["ci95_bootstrap"][1] - o["pooled"]["boot_ci95"][1]))
            add("%s/ksum%s/gap_to_rho_neg_unpruned/w%s" % (rid, k, wk), c["gap_to_rho_unpruned"], R["gap_to_rho_neg_k" + k][wk]["unpruned"])
            add("%s/ksum%s/gap_to_rho_neg_pruned/w%s" % (rid, k, wk), c["gap_to_rho_pruned"], R["gap_to_rho_neg_k" + k][wk]["pruned"])
    for k in ("5", "4"):
        add("%s/ksum%s/sibling_delta" % (rid, k), row["ksum" + k]["delta_sibling"], R["rule_k" + k]["sibling_delta"])
        o = R["rule_k" + k]
        # the producer's sibling-delta reading: 1 - max pooled upper > delta - 1
        pooled_reading = (1 - o["max_upper_pooled"]) > (o["sibling_delta"] - 1)
        verdict_cmp["%s/ksum%s" % (rid, k)] = {
            "claimed_holds": row["ksum" + k]["holds"], "own_holds_paired_aa_gate": o["holds_paired_aa_gate"],
            "claimed_holds_sibling_delta": row["ksum" + k]["holds_sibling_delta"],
            "own_sibling_delta_pooled_upper_reading": pooled_reading,
            "own_sibling_delta_replicate_inclusive_reading": o["holds_sibling_delta_gate"],
            "claimed_max_upper_pooled": row["ksum" + k]["max_upper_pooled"], "own_max_upper_pooled": o["max_upper_pooled"],
            "claimed_max_upper_replicate": row["ksum" + k]["max_upper_replicate"], "own_max_upper_replicate": o["max_upper_replicate"],
            "sibling_delta_margin_pooled": (1 - o["max_upper_pooled"]) - (o["sibling_delta"] - 1)}
    band = json.load(open(os.path.join(STUDY, "band", rid + ".json")))
    for br in band["rows"]:
        for i, wk in enumerate(WK):
            add("%s/band_mean_S/%s/w%s" % (rid, br["arm"], wk), br["mean_s"][i], R["mean_S"][br["arm"]][wk])

# claim ranges (task statement), from own numbers
def rng(f):
    v = [f(own["rows"][r]) for r in ROWS]
    return [min(v), max(v)]


ranges = {
    "ksum5_p_over_ksum5_w0": rng(lambda R: R["ratios"]["ksum5-p_over_ksum5"]["0"]["pooled"]["ratio"]),
    "ksum5_p_over_ksum5_w1": rng(lambda R: R["ratios"]["ksum5-p_over_ksum5"]["1"]["pooled"]["ratio"]),
    "ksum4_p_over_ksum4_w0": rng(lambda R: R["ratios"]["ksum4-p_over_ksum4"]["0"]["pooled"]["ratio"]),
    "max_upper_any_k5_pooled_or_replicate": max(max(own["rows"][r]["rule_k5"]["max_upper_pooled"], own["rows"][r]["rule_k5"]["max_upper_replicate"]) for r in ROWS),
    "max_upper_any_k4_pooled_or_replicate": max(max(own["rows"][r]["rule_k4"]["max_upper_pooled"], own["rows"][r]["rule_k4"]["max_upper_replicate"]) for r in ROWS),
    "gap_ksum5_unpruned_over_rho_neg_w0": rng(lambda R: R["gap_to_rho_neg_k5"]["0"]["unpruned"]),
    "gap_ksum5_pruned_over_rho_neg_w0": rng(lambda R: R["gap_to_rho_neg_k5"]["0"]["pruned"]),
}
claimed_ranges = {"ksum5_p_over_ksum5_w0": "0.83-0.94", "ksum5_p_over_ksum5_w1": "0.89-0.97",
                  "ksum4_p_over_ksum4_w0": "0.94-0.985", "upper_95": "< 1 everywhere",
                  "gap_unpruned_w0": "5.56-6.85", "gap_pruned_w0": "5.03-5.70"}
range_ok = (round(ranges["ksum5_p_over_ksum5_w0"][0], 2) == 0.83 and round(ranges["ksum5_p_over_ksum5_w0"][1], 2) == 0.94
            and round(ranges["ksum5_p_over_ksum5_w1"][0], 2) == 0.89 and round(ranges["ksum5_p_over_ksum5_w1"][1], 2) == 0.97
            and round(ranges["ksum4_p_over_ksum4_w0"][0], 2) == 0.94 and round(ranges["ksum4_p_over_ksum4_w0"][1], 3) == 0.985
            and ranges["max_upper_any_k5_pooled_or_replicate"] < 1 and ranges["max_upper_any_k4_pooled_or_replicate"] < 1
            and round(ranges["gap_ksum5_unpruned_over_rho_neg_w0"][0], 2) == 5.56 and round(ranges["gap_ksum5_unpruned_over_rho_neg_w0"][1], 2) == 6.85
            and round(ranges["gap_ksum5_pruned_over_rho_neg_w0"][0], 2) == 5.03 and round(ranges["gap_ksum5_pruned_over_rho_neg_w0"][1], 2) == 5.70)

# best k-sum arm before and after (the incumbent framing of the success criterion)
best = {}
for r in ROWS:
    S = own["rows"][r]["mean_S"]
    best[r] = {}
    for wk in WK:
        bu = min(("ksum4", "ksum5"), key=lambda a: S[a][wk]); bp = min(("ksum4-p", "ksum5-p"), key=lambda a: S[a][wk])
        best[r][wk] = {"best_unpruned": bu, "best_pruned": bp, "ratio_best_pruned_over_best_unpruned": S[bp][wk] / S[bu][wk],
                       "gap_best_unpruned_over_rho_neg": S[bu][wk] / S["rho-neg"][wk],
                       "gap_best_pruned_over_rho_neg": S[bp][wk] / S["rho-neg"][wk]}

not_best = [r for r in ROWS if best[r]["0"]["best_unpruned"] != "ksum5"]
bb = lambda wk, f: [min(best[r][wk][f] for r in ROWS), max(best[r][wk][f] for r in ROWS)]
best_text = ("the ksum5 headline compares against unpruned ksum5, which at w = 0 is not the cheapest unpruned k-sum arm on %d of 8 rows (%s: ksum4 is); "
             "cheapest-to-cheapest arm the gain is %.3f-%.3f at w = 0 and %.3f-%.3f at w = 1, and the cheapest k-sum arm's gap to rho-neg at w = 0 "
             "goes from %.2f-%.2f to %.2f-%.2f (checks.best_arm_framing)" % (
                 len(not_best), ", ".join(not_best), *bb("0", "ratio_best_pruned_over_best_unpruned"), *bb("1", "ratio_best_pruned_over_best_unpruned"),
                 *bb("0", "gap_best_unpruned_over_rho_neg"), *bb("0", "gap_best_pruned_over_rho_neg")))
aa_total = sum(s["records"] // 6 for s in own["sessions"].values())
aa_diff = sum(s["aa_pairs_differing"] for s in own["sessions"].values())

# ---- correctness and audits -------------------------------------------------------------------
corr_pairs = sum(v["pairs"] for v in own["correctness"].values())
corr_disallowed = {k: v["disallowed"] for k, v in own["correctness"].items() if v["disallowed"]}
corr_paths = sorted(set().union(*[set(v["differing_paths"]) for v in own["correctness"].values()]))
build = own["ksum_build_audit"]
build_ok = all(list(v["recorded_unpruned_setup_adds"]) == [str(v["formula_unpruned"])] and
               list(v["recorded_pruned_setup_adds"]) == [str(v["formula_pruned"])] and
               list(v["insert_attempts"]) == [str(v["expected_insert_attempts"])] for v in build.values())
dflt = own["default_off_vs_prebuild_siblings"]
dflt_paths = sorted(set().union(*[set(v["differing_paths"]) for v in dflt.values()]))
dflt_ok = set(dflt_paths) <= {".binary_sha256", ".env_class_id", ".record_id", ".seq", ".session_id"}
sess_ok = all(s["records_sha256_matches_session"] and s["record_seals_ok"] == s["records"] and s["binary_matches"]
              and s["all_records_bound_to_binary"] and s["started_after_registration"] and s["all_verified"]
              and s["spec_identical_to_registered"] and s["git_dirty"] is False and s["scalar_certificate_failures"] == 0
              for s in own["sessions"].values())
fresh_ok = all(f["verify"]["ok"] and f["scalar_certificate_failures"] == 0 and not any(f["correctness_disallowed_differences"].values())
               for f in fresh.values())
fresh_in_range = {n: {"ksum5_w0": f["ratios"]["ksum5-p_over_ksum5"]["0"]["ratio"], "ksum5_w1": f["ratios"]["ksum5-p_over_ksum5"]["1"]["ratio"],
                      "ksum4_w0": f["ratios"]["ksum4-p_over_ksum4"]["0"]["ratio"],
                      "max_upper": max(max(v["lin_ci95"][1], v["boot_ci95"][1]) for key in ("ksum5-p_over_ksum5", "ksum4-p_over_ksum4") for v in f["ratios"][key].values())}
                  for n, f in fresh.items()}
fresh_range_ok = all(0.825 <= v["ksum5_w0"] <= 0.945 and 0.885 <= v["ksum5_w1"] <= 0.975 and 0.935 <= v["ksum4_w0"] <= 0.9855 and v["max_upper"] < 1
                     for v in fresh_in_range.values())

checks_pass = (not mism and sess_ok and not corr_disallowed and corr_pairs == 9216 and build_ok and model["all_ok"]
               and dflt_ok and fresh_ok and fresh_range_ok and range_ok and own["certificates"]["failures"] == 0
               and summ["verdict"]["correctness_ok"] and pcorr["pairs_checked"] == corr_pairs and not pcorr["mismatches"]
               and all(v["claimed_holds"] == v["own_holds_paired_aa_gate"] for v in verdict_cmp.values())
               and negc["all_detected"])

# ---- rho band ---------------------------------------------------------------------------------
def band_of(r, ic_arm="ksum5-p"):
    S = own["rows"][r]["mean_S"]
    return {wk: {"ic": S[ic_arm][wk], "rho": S["rho-neg"][wk]} for wk in WK}


gae_band_all = {r: {"ksum5-p": band_of(r), "ksum5": band_of(r, "ksum5"), "ksum4-p": band_of(r, "ksum4-p"),
                    "best_pruned_arm_w0": best[r]["0"]["best_pruned"]} for r in ROWS}
head_row = "B27"
gb = band_of(head_row)
order = len({(v["ic"] > v["rho"]) - (v["ic"] < v["rho"]) for v in gb.values()}) == 1

# ---- reports ----------------------------------------------------------------------------------
raw = [os.path.join(STUDY, "sessions", "%s-r%d" % (r, i), "records.jsonl.gz") for r in ROWS for i in (1, 2, 3)]
raw += [os.path.join(STUDY, "specs", "%s-r%d.json" % (r, i)) for r in ROWS for i in (1, 2, 3)]
raw += [os.path.join(STUDY, x) for x in ("PROTOCOL.md", "BINARY.sha256", "REGISTRATION.sha256", "results.json", "summary.json")]
raw += [os.path.join(STUDY, "band", r + ".json") for r in ROWS]
code = {os.path.basename(p): sha(p) for p in sorted(glob.glob(os.path.join(VAL, "*.py")) + glob.glob(os.path.join(VAL, "*.sh")))}
code["own-numbers.json"] = sha(os.path.join(VAL, "own-numbers.json"))
code["prune-model.json"] = sha(os.path.join(VAL, "prune-model.json"))

validation_report = {
    "id": VAL_ID,
    "review_attestation": {
        "joints_owned": ["code audit of 6d6aa93b8f", "binary provenance", "replays", "independent recompute", "fresh sessions", "protocol adherence"],
        "sources_read": ["crypto-autoresearcher: docs/agent-runtime-core.md, agents/validator.md, AGENTS.md 'Review architecture'",
                         "crypto: AGENTS.md 'Standing workflow rules' (validation gate), docs/VALIDATION_GATE.md, scripts/validation_gate.py (evidence checks)",
                         "research/ic_ksum_prune_20261115: PROTOCOL.md, README.md, results.md, registered-at.txt, BINARY.sha256, REGISTRATION.sha256, run_all.sh, specs/*.json, sessions/*/{session,spec}.json and records (by script), analysis/correctness.py (registered and committed versions, diff), data/runlog.tsv; results.json, summary.json, data/correctness.json, band/*.json read only after own-numbers.json was written",
                         "git show 6d6aa93b8f (src), src/cryptanalysis/ic_boundary.rs (KSumTable build/extend), ic_framework/stages.rs (flag), ecbench/{band.rs, compare.rs, compare/pooled.rs, stats.rs, methods.rs PRICED_NATIVE, record.rs counts/seal, fbm.rs preimage}, ic_framework/solver_units.rs PINNABLE_NATIVE, docs/ic/calibration.json",
                         "template: research/ic_tableb_binary_ksum_20261115/validator/{VALIDATION.json, replay-certificate.json, validator-report.json (structure), own-numbers.json (keys), nonce.txt}, benchmarks/20261115-tableb-binary-ksum/VALIDATION.json",
                         "sibling sessions research/ic_tableb_ksum_20261115/sessions and research/ic_tableb_binary_ksum_20261115/sessions (records by script, for the default-off comparison and replays), research/ic_tableb_ksum_20261115/BINARY.sha256"],
        "not_read": ["research/ic_ksum_prune_20261115/analysis/evaluate.mjs, summary.mjs, plot.mjs, report.py (beyond the registration hash)"],
        "read_sibling_reports": "the earlier validator report of the sibling study ic_tableb_binary_ksum_20261115 was read as the structural template the task named; no other reviewer report of this study exists or was read",
        "verdict": "passed" if checks_pass else "failed"},
    "task_id": "TASK-unassigned (validation-gate request from the producing session, 2026-11-15)",
    "run_ids": sorted(s["session_id"] for s in own["sessions"].values()),
    "verdict": "passed" if checks_pass else "failed",
    "verdict_scope": ("the 24 sessions are admissible counted-cost evidence; the paired ratios, intervals, mean S, "
                      "gaps to rho-neg and the correctness identity recompute; the fresh sessions reproduce the effect. "
                      "Not an ECDLP claim, not a result against rho, not a promotion."),
    "limitations": [
        "validator is a subagent launched from the producing session (same Claude-Session id as the study commits): independent context, code, nonce and seeds, not an independent operator; no keyring signature",
        "no rebuild from source: free disk (2.7 GB) is below the 3.8 GB a separate release target needs; the registered binary at target/release/ecbench (sha256 equal to BINARY.sha256) was used, its identity shown by hash, mtime, embedded prune text and cross-binary agreement with two pre-prune binaries' records, not by a reproducible build",
        "replays are a re-execution by the same binary on the same host class (L0); not a second host class",
        "the unit test was run from a pre-existing release test binary (built 22:01:29Z from a tree containing the test) whose build was not reproduced",
        "the paired A/A gate is vacuous (rho-neg-aa is the same method with the same seeds, so its ratio is exactly 1 by construction); the effect is a deterministic per-pair count difference, so the evidence is the correctness identity and the closed-form count, not a noise gate",
        "scale: the saved adds are lower-level partials of the k-sum build; their share of the build is 36% at a = 7 (k = 5) and falls roughly as 1/a^2 (10% at a = 12, 3% at a = 20, 0.1% at a = 100); the gain is a constant-factor saving that vanishes at larger bases and does not change any exponent",
        best_text,
        "registration timing rests on producer-controlled git and session timestamps",
    ],
}

report = {
    "schema": "validation-gate.validator-report/v1",
    "status": "pass" if checks_pass else "fail",
    "independent": True,
    "study": "research/ic_ksum_prune_20261115",
    "validated_commit": "b7666dcd8f",
    "method": ("Independent recomputation in Python (stdlib) from the 24 committed records.jsonl.gz by validator/recompute.py, written "
               "from PROTOCOL.md, the record format and the documented formulas of band.rs (priced_gae(w) = G - C + w (U + n_C), C from "
               "docs/ic/calibration.json, U including counted table_insert_attempts) and compare.rs/stats.rs (ratio of totals over "
               "(workload, round) pairs; linearised t on (session, workload) clusters, log-scale interval; two-stage cluster bootstrap with the "
               "validator's own RNG), without reading analysis/*.mjs, analysis/correctness.py logic beyond its committed diff, summary.json or "
               "results.json until own-numbers.json was written. Correctness by a generic deep diff of every -p record against its twin "
               "(every round incl. warm-up). Scalar certificates by own prime-field and GF(2^n) arithmetic. Build adds against the closed form "
               "and an independent combinatorial model of the recursion (prune_model.py). Replays: ecbench verify --replay-all on all 24 "
               "sessions, on 24 pre-prune sibling sessions (default-off path), and on three fresh sessions with nonce-derived seeds (B15, P24, "
               "and nonce-chosen P22)."),
    "validator_code_sha256": code,
    "raw_data": None,  # filled per evidence directory below
    "headline": headline,
    "headline_mismatches": mism,
    "checks": {
        "binding": {"registration_commit": "fff219f63f", "results_commit": "b7666dcd8f",
                    "head_at_validation": "28aae8813a (study files, src, Cargo.lock identical to b7666dcd8f)",
                    "registered_at": "2026-11-15T22:11:38Z", "first_session_started_after_registration": True,
                    "registration_hashes": "sha256sum -c REGISTRATION.sha256: all OK except analysis/correctness.py",
                    "post_registration_edit": "analysis/correctness.py: input handling only (reads records.jsonl if present, else .gz); the comparison logic is byte-identical to the registered version (git diff fff219f63f b7666dcd8f)",
                    "post_hoc_files": "analysis/summary.mjs, plot.mjs, report.py written after the data, not registered; they copy numbers and decide no verdict (evaluate.mjs, which computes the verdict, is registered and unchanged)"},
        "sessions_integrity_ok": sess_ok,
        "sessions": {n: {k: s[k] for k in ("session_id", "records", "records_sha256", "records_sha256_matches_session", "record_seals_ok",
                                            "git_commit", "git_dirty", "binary_matches", "spec_identical_to_registered", "status_counts")}
                     for n, s in own["sessions"].items()},
        "code_audit_6d6aa93b8f": {
            "loop_bound": "extend() at depth d iterates abscissae ai < end, end = len - (k - d - 1) with prune (len otherwise); a partial ending at ai can be completed only if len - 1 - ai >= k - d - 1, i.e. ai < len - (k - d - 1): the bound skips exactly the partials that cannot reach k summands; at the last depth (d = k - 1) end = len, so no insert is skipped",
            "uncounted_ops": "every add is g.add(&mut self.build_ops, ..) and the oracle merges build_ops into oracle_setup (plugins.rs); pruning only removes calls, adds no group operation outside the counter",
            "default_off": "prune = false gives end = groups.len(), .take(len) is the identity: the iteration is unchanged; params.flag('prune') is false when the key is absent",
            "model_check": {"all_ok": model["all_ok"], "file": "prune-model.json"},
            "unit_test": {"name": "pruned_build_is_the_same_table_with_fewer_adds", "ran": "target/release/deps/crypto_lib-24e7e65220b9e040 (sha256 a0c8e3b0...), ok",
                          "meaningful": "asserts identical map, collisions, insert_attempts and entries, and adds bracketed by the closed forms from above (no slack) and below (64 for identity skips) for k = 3..7; the model shows a bound one step too aggressive loses inserts and one step too lenient exceeds the upper closed form, so both neighbours of the bound fail the test"}},
        "ksum_build_audit": {"all_records_equal_closed_form": build_ok, "rows": build},
        "correctness": {"pairs": corr_pairs, "disallowed_differences": corr_disallowed, "differing_paths_seen": corr_paths,
                        "max_abs_total_gae_diff_minus_saved_adds": max(v["max_abs_total_gae_diff_minus_saved_adds"] for v in own["correctness"].values()),
                        "producer_pairs_checked": pcorr["pairs_checked"], "producer_mismatches": pcorr["mismatches"]},
        "default_off_vs_prebuild_binaries": {"ok": dflt_ok, "differing_paths_seen": dflt_paths,
                                             "records_compared": sum(v["pairs"] for v in dflt.values()),
                                             "note": "this study's unpruned ksum4, ksum5 and rho-neg records equal, field by field, the records of the same spec, arm, round and target written by the pre-prune binaries bc96a44a (2476332d22) and 7d46e5ee (7b3c057e1c), except binary/env/session/record ids and seq",
                                             "sessions": dflt},
        "certificates": own["certificates"],
        "curves": own["curves"],
        "verdict_comparison": verdict_cmp,
        "bootstrap_upper_max_abs_dev_vs_producer": boot_dev,
        "claim_ranges": {"claimed": claimed_ranges, "recomputed": ranges, "ok": range_ok},
        "best_arm_framing": best,
        "fresh_sessions": {"ok": fresh_ok, "in_claimed_range": fresh_range_ok, "values": fresh_in_range},
        "prediction_check": {"rule": "R_0(ksum5) within 0.95 +/- 0.03", "rows_held": [r for r in ROWS if abs(own["rows"][r]["ratios"]["ksum5-p_over_ksum5"]["0"]["pooled"]["ratio"] - 0.95) <= 0.03],
                             "claimed_rows_held": summ["verdict"]["prediction_ksum5_r0_0p95_pm_0p03_held_rows"]},
        "noise_controls": {"paired_aa_gate": "vacuous: rho-neg-aa and rho-neg are the same method with the same seeds; %d of %d pairs (warm-up included) differ in cost, counters or recovered log" % (aa_diff, aa_total),
                           "what_carries_the_result": "per pair, -p differs from its twin only in oracle_setup adds, by exactly the closed-form dead-end count (constant per curve and k, zero variance); the interval reflects only the spread of the unpruned denominator over targets",
                           "decay_check": "the saving should shrink as the base grows (dead-end partials are lower-order); observed build share saved 36.3% (a = 7) > 20.0% (a = 9) > 15.5% (a = 10) > 12.4% (a = 11) > 10.1% (a = 12) at k = 5, and R_0(ksum5) rises from 0.83 (B15) to 0.94 (B27), as the closed form predicts"},
        "rho_reference": {"arm": "rho-neg (rho.negation), role reference, in every session on the same curve, targets and seeds",
                          "strongest_available": "random binary curves and the prime curves here have automorphisms_available = 2 (negation only)",
                          "gae_band_all_rows": gae_band_all},
        "negative_controls": negc,
        "discrepancies": [
            "B15/ksum4 sibling-delta gate (secondary, conservative reading): claimed true; holds by %.1e under the producer's pooled-upper reading, fails under a replicate-inclusive reading (PROTOCOL.md's '1 - max_w upper(R_w)' does not say which); the registered verdict (paired A/A gate) is unaffected" % verdict_cmp["B15/ksum4"]["sibling_delta_margin_pooled"],
            "results.json gives 'pairs': 144 for the pooled ratios; 144 is the number of (session, workload) clusters; each pooled ratio is over 432 (workload, round) pairs; labelling only, the ratios and intervals recompute",
            "the commit message of 6d6aa93b8f says the pruned adds are about 10-12% of the build at m = 5 and 3% at m = 4; on these bases they are 10.1-36.3% (m = 5, a = 7..12) and 2.8-17.7% (m = 4, a = 7..16), closed form; disclosed by the producer",
            "bootstrap upper ends differ from the producer's by up to %.1e (different RNG, same procedure); the registered verdicts (every upper end below 1) have a margin of at least %.1e, and every sibling-delta verdict except B15/ksum4 a margin of at least %.1e" % (
                boot_dev, min(1 - max(v["own_max_upper_pooled"], v["own_max_upper_replicate"]) for v in verdict_cmp.values()),
                min(abs(v["sibling_delta_margin_pooled"]) for k, v in verdict_cmp.items() if k != "B15/ksum4")),
            "fresh validator sessions record git_dirty = true (untracked files of a concurrent study and of this validator directory); they are validator sessions, not study evidence",
        ],
        "accounting": "not a relabel: the removed work is g.add calls of the unpruned build that produced partials with no insert below them (prune_model.py: saved adds = dead-end adds exactly); pricing, native counters and every non-setup count are unchanged",
    },
    "validation_report": validation_report,
}


def write_report(path, base):
    rep = dict(report)
    rep["raw_data"] = [{"path": rel(p, base), "sha256": sha(p)} for p in raw]
    rep["evidence_dir"] = rel(base, ROOT)
    json.dump(rep, open(path, "w"), indent=1, sort_keys=False)
    return sha(path)


rep_val = write_report(os.path.join(VAL, "validator-report.json"), VAL)
rep_bench = write_report(os.path.join(VAL, "validator-report-benchmarks.json"), BENCH)

cert = {
    "schema": "ecbench.replay-certificate/v1",
    "status": "verified" if all(x["ok"] for x in runs) else "failed",
    "independent": True,
    "scalars_verified": own["certificates"]["failures"] == 0 and own["certificates"]["records_checked"] == 27648,
    "paths_relative_to": "repository root",
    "study": "research/ic_ksum_prune_20261115",
    "method": ("ecbench verify --replay-all --exit-code on copies of each of the 24 committed sessions (records.jsonl.gz gunzipped into a "
               "scratch directory; committed files untouched), under the shared CPU lock; every measured deterministic run re-executed and "
               "compared with its record. The same on 24 sibling sessions recorded by two pre-prune binaries (default-off path). Every "
               "recovered log of every record re-checked with the validator's own arithmetic: target on the curve, [recovered]G == target. "
               "Three fresh sessions on nonce-derived seeds run and replayed the same way."),
    "nonce": {"git_ls_remote_origin_main": nonce["ls_remote"], "utc": nonce["utc"], "sha256": nonce["nonce"],
              "drawn_after": "results commit b7666dcd8f (2026-11-15T22:26:27Z)",
              "rule": "see mkfresh.py: FB15 and FP24 seeds from nonce hex slices; third row [P20,P22,B18,B21,B24,B27][int(n[56:64],16) mod 6] = P22, seeds from sha256(n + ':third'); every committed session was replayed, so no replay sample was drawn"},
    "build": {"rebuilt": False,
              "binary": "target/release/ecbench", "binary_sha256": "6c60b4c931294bf770039a5b47800e39c424eb2e7f4876b426cd084f7c364d77",
              "equals_BINARY_sha256": True, "mtime": "2026-11-15T22:10:55Z (before registration 22:11:38Z)",
              "source_identity": "git diff 3a9e85e644 HEAD -- src Cargo.lock Cargo.toml: empty; working tree src clean",
              "embedded_text": "the binary contains the prune parameter description added by 6d6aa93b8f",
              "why_not_rebuilt": "2.7 GB free on the shared disk against 3.8 GB for a separate release target directory; rebuilding in place would overwrite the binary a concurrent study uses",
              "behavioural_identity": "24 pre-prune sibling sessions (binaries bc96a44a from 2476332d22, 7d46e5ee from 7b3c057e1c) replay with this binary: 29,808 replays identical; and 13,824 of this study's unpruned/rho records equal the siblings' field by field",
              "toolchain_present": "cargo 1.97.0, rustc 1.97.0"},
    "independence": {"independent_of_producer": ["validator context; did not read analysis/*.mjs or the producer's statistics before own-numbers.json",
                                                  "own pricing, pairing, interval, bootstrap, deep-diff and curve-arithmetic code",
                                                  "nonce and fresh seeds drawn after publication"],
                     "not_independent": ["same binary (not rebuilt)", "same host class", "same Claude-Session as the producer"]},
    "scalar_certificates": own["certificates"],
    "runs": runs,
    "default_off_replays": old,
    "fresh_sessions": fresh,
}
json.dump(cert, open(os.path.join(VAL, "replay-certificate.json"), "w"), indent=1)
cert_sha = sha(os.path.join(VAL, "replay-certificate.json"))


def manifest(base, rep_name):
    return {
        "schema": "validation-gate/v1",
        "note": ("Counted-cost measurement (L0) of ksum prune=1 against prune=0 on Table B's eight curves (study research/ic_ksum_prune_20261115, "
                 "validated by %s). The rho reference is the matched negation-map rho arm every session runs beside every arm. gae_band is the "
                 "B27 row (ic = ksum5-p, the cheapest k-sum arm there); every row's band is in the validator report "
                 "(checks.rho_reference.gae_band_all_rows). [pcc:unverified]" % VAL_ID),
        "replay_certificate": {"path": rel(os.path.join(VAL, "replay-certificate.json"), base), "sha256": cert_sha},
        "validator_report": {"path": rel(os.path.join(VAL, rep_name), base), "sha256": sha(os.path.join(VAL, rep_name))},
        "rho_reference": {
            "kind": "negation",
            "protocol": "matched rho.negation reference arm (role reference) in every session, same curve, targets and algorithm seeds as every k-sum arm; curves with 2 usable automorphisms, so negation is the strongest symmetry available",
            "unit": "ecbench.gae (counted group operations, S = GAE/sqrt(r)); native work priced at w in {0, 0.1, 1} GAE per operation (AGENTS.md section 5a)",
            "binary_or_process": "one ecbench binary, sha256 6c60b4c931294bf770039a5b47800e39c424eb2e7f4876b426cd084f7c364d77 (built at 3a9e85e644); every arm of a session runs in it",
            "matches_protocol": True, "strongest_available": True, "same_unit": True, "same_binary_or_process": True,
            "band_source": "own-numbers.json rows.B27.mean_S (equal to research/ic_ksum_prune_20261115/band/B27.json to 1e-9)",
            "gae_band": gb,
            "order_holds_across_band": order,
        },
    }


json.dump(manifest(VAL, "validator-report.json"), open(os.path.join(VAL, "VALIDATION.json"), "w"), indent=1, sort_keys=True)
os.makedirs(BENCH, exist_ok=True)
json.dump(manifest(BENCH, "validator-report-benchmarks.json"), open(os.path.join(BENCH, "VALIDATION.json"), "w"), indent=1, sort_keys=True)
print("status", report["status"], "headline", len(headline), "mismatches", len(mism), "boot_dev %.2e" % boot_dev,
      "sess", sess_ok, "corr", corr_pairs, bool(corr_disallowed), "build", build_ok, "model", model["all_ok"], "dflt", dflt_ok,
      "fresh", fresh_ok, fresh_range_ok, "ranges", range_ok)
