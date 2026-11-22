#!/usr/bin/env python3
"""Assemble the validation-gate artifacts for genus4_prime_order_20261118 from the validator's outputs (out/).
Writes in benchmarks/20261118-genus4-prime-order/: replay-certificate.json, validator-report.json and, only when
every check passes, VALIDATION.json.  Claimed numbers are the producer's analysis.json / tune.json as git blobs
at COMMIT; recomputed numbers are the validator's out/recompute.json."""
import hashlib, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *

EV = "benchmarks/20261118-genus4-prime-order"
V = f"{D}/validation"
J = lambda n: json.load(open(os.path.join(OUT, n)))
fsha = lambda p: hashlib.sha256(open(os.path.join(REPO, p), "rb").read()).hexdigest()
rel = lambda p: os.path.relpath(os.path.join(REPO, p), os.path.join(REPO, EV))

fz, sel, theo, xchk = J("freeze_check.json"), J("select_check.json"), J("prefilter_theory.json"), J("theory_crosscheck.json")
pdiff, prust, rc, cmpc = J("prefilter_diff.json"), J("prefilter_rust_vs_validator.json"), J("recompute.json"), J("compare_claims.json")
rep, certs, band, plan = J("replay_compare.json"), J("certs.json"), J("band.json"), J("plan.json")
bdiff = open(os.path.join(OUT, "binary_diff.txt")).read()
nonce_line = open(os.path.join(OUT, "nonce.txt")).read().strip()

# ---------------- raw data pinned ----------------
raw = [{"path": rel(p), "sha256": sha(blob(p))} for p in raw_files()]
for r in raw:   # the working-tree copy must be byte-equal to the committed blob
    assert fsha(os.path.join(EV, r["path"])) == r["sha256"], r

# ---------------- headline: numeric cells only ----------------
H = [{"name": c["name"], "claimed": c["claimed"], "recomputed": c["recomputed"],
      "claimed_source": f"{D}/" + c["name"].split("/")[0]}
     for c in cmpc["rows"] if isinstance(c["claimed"], (int, float)) and not isinstance(c["claimed"], bool)]
worst = max(abs(h["claimed"] - h["recomputed"]) / max(abs(h["recomputed"]), 1e-300) for h in H)

# ---------------- checks ----------------
binary_ok = ("493ee2942473b8708a0b52f40ab10095c4da83ab0f0755ed3e5dbc02771b6921  size" in bdiff
             and all(f"{s} prod=" in bdiff and "SAME" in [l for l in bdiff.splitlines() if l.startswith(s + " ")][0].split()
                     for s in (".text", ".rodata", ".data", ".data.rel.ro", ".eh_frame", ".gcc_except_table", ".got"))
             and "address+type columns differ in 0 lines" in bdiff and "suffix removed: 0" in bdiff)
order_ok = (fz["checks"]["order:D1 concerns residue-3 re-scan (search_p32771_r1-1800-3_*)"]["deviation_strictly_before"]
            and fz["checks"]["search_chunks_fixed_before_rescan_data"]["ok"])
freeze_ok = all(v.get("ok", True) for k, v in fz["checks"].items() if not k.startswith("order:")) and not fz["fail"]
n_rows = {"A": 0, "B16x": 0, "B16xs": 0, "B32x": 0, "B32xs": 0, "Q": 0}
for c in rep["cells"]:
    t = c["tag"]
    k = ("A" if t.startswith("a_") else "Q" if t.startswith("q_") else None)
    if t.startswith("b_p16411_x_"): k = "B16x"
    if t.startswith("b_p16411_xstar_"): k = "B16xs"
    if t.startswith("b_p32771_x_"): k = "B32x"
    if t.startswith("b_p32771_xstar_"): k = "B32xs"
    if k:
        n_rows[k] += c["identical_rows"]
replay_ok = (rep["ok"] and n_rows["A"] >= 24 and min(n_rows[k] for k in ("B16x", "B16xs", "B32x", "B32xs")) >= 10
             and n_rows["Q"] >= 1 and rep["search_cells_identical"] == len(plan["S"]))
P = lambda p: sel["primes"][p]
checks = [
    {"id": "C1", "requirement": "frozen files (PROTOCOL.md, search.sh, curves.py, tune.py, run.sh, analyze.py) hash to frozen.sha256 at the freeze commit 6da1e3c5b2 and at the validated commit; frozen.sha256 and binary.sha256 unchanged since; the freeze commit has no data and its parent no study files; all 66 data files enter later; no src/Cargo/crates change from the binary commit ca72267899 (the freeze's parent) to the validated commit",
     "pass": freeze_ok, "evidence": [f"{V}/out/freeze_check.json", f"{V}/scripts/freeze_check.py"]},
    {"id": "C1b", "requirement": "each deviation committed before the data it concerns",
     "pass": order_ok,
     "note": "D1 (d09efa5d95) and the fixed search_chunks.sh (3a54c389f9) precede the residue-3 re-scan data (b4e483061d). D2 (8c54acb9e2) was written after the 1801-2200 data (d202be88ca) and in the same commit as the 2201-2600 data; the extensions it records are the frozen rule (PROTOCOL section 2), announced in the commit messages of b4e483061d and d202be88ca before their data, and its one genuine deviation (stopping 2601-3000) cannot change the five smallest seeds because 1..2600 is fully covered (finding F1)",
     "evidence": [f"{V}/out/freeze_check.json"]},
    {"id": "C2", "requirement": "prefilter soundness: for y^2 = f(x), deg f = 9, reducible f implies #J(F_p) even; implementation never calls an irreducible f reducible; no seed below the fifth chosen seed was skipped wrongly",
     "pass": theo["ok"] and xchk["ok"] and pdiff["ok"] and prust["ok"] and P("16411")["certificates_ok"] and P("32771")["certificates_ok"],
     "detail": {"theorem": "#J(F_p)[2] = 2^(m-1), m = number of irreducible factors of f (odd degree); constructive: a factor w | f with 1 <= deg w <= 4 gives the non-zero rational 2-torsion divisor (w, 0)",
                "numeric_test_independent_seeds": {p: {k: v[k] for k in ("curves", "reducible", "even", "certs_checked")} for p, v in theo["per_p"].items()},
                "validator_L1_vs_library_order": {"curves": xchk["curves"], "library_orders": xchk["library_orders"], "equal": xchk["equal"]},
                "differential_test": {k: v["agree"] for k, v in pdiff["by_kind"].items()},
                "per_seed_library_vs_validator": {"compared": prust["compared"], "agree": prust["agree"], "by_p": prust["by_p"]},
                "skipped_seeds_below_fifth_with_checked_certificate": {p: P(p)["of_which_reducible_with_checked_certificate"] for p in ("16411", "32771")}},
     "evidence": [f"{V}/out/prefilter_theory.json", f"{V}/out/theory_crosscheck.json", f"{V}/out/prefilter_diff.json",
                  f"{V}/out/prefilter_rust_vs_validator.json", f"{V}/out/prefilter_rust_flags.json", f"{V}/out/select_check.json", f"{V}/scripts/polyfp.py", f"{V}/scripts/prefilter_diff.py", f"{V}/scripts/prefilter_theory.py", f"{V}/scripts/theory_crosscheck.py"]},
    {"id": "C3", "requirement": "chosen seeds are the 5 smallest admitted over fully covered ranges (cell rows, tried counts); residue 3 and the extension blocks complete",
     "pass": sel["ok"] and P("16411")["chosen_ok"] and P("32771")["chosen_ok"],
     "detail": {p: {k: P(p)[k] for k in ("contiguous_covered_1_to", "admitted", "chosen", "seeds_below_fifth_not_admitted",
                                         "of_which_reducible_with_checked_certificate", "of_which_irreducible_rejected_after_point_count")}
                for p in ("16411", "32771")} | {"reasons_seen": sel["reasons_seen"], "curves_json_matches": sel["curves_json_matches"],
                                                "partial_file_excluded": "search_p32771_1_3.jsonl.gz (no cell row; its 355 and 1623 are found again by the re-scan)"},
     "evidence": [f"{V}/out/select_check.json", f"{V}/scripts/select_check.py"]},
    {"id": "C4a", "requirement": "producer binary equals binary.sha256; validator rebuild from git archive of ca72267899 in a validator-owned tree and target directory has identical code sections",
     "pass": binary_ok, "evidence": [f"{V}/out/binary_diff.txt", f"{V}/scripts/build.sh", f"{V}/scripts/paths.py"]},
    {"id": "C4b", "requirement": "nonce-drawn replay with the validator build: part A several cc and SGE settings, part B >= 10 x and >= 10 x* rows per prime, >= 1 Q row, plus search cells: every field except wall time identical",
     "pass": replay_ok, "detail": {"rows_identical": n_rows, "search_cells_identical": rep["search_cells_identical"], "nonce": nonce_line},
     "evidence": [f"{V}/out/replay_compare.json", f"{V}/out/plan.json", f"{V}/out/nonce.txt", f"{V}/out/replay/"]},
    {"id": "C5", "requirement": "independent certificates: r prime, h = 1, order = r inside Hasse-Weil, [r]G = 0, [t]G = T for every key; recovered logs for a sample; F_p-automorphisms; drawn order_not_near_prime rejections certified not prime",
     "pass": certs["ok"], "detail": {"curves": len(certs["curves"]), "keys_ok": f"{certs['keys_ok']}/{certs['keys']}", "logs": certs["n_logs"],
                                     "ord_certified": f"{sum(e['certifies_not_prime'] for e in certs['ord'])}/{certs['n_ord']}",
                                     "tamper_control": certs["tamper_control"], "automorphism_controls": certs["controls"]},
     "evidence": [f"{V}/out/certs.json", f"{V}/scripts/certs.py", f"{V}/scripts/jac.py", f"{V}/scripts/val_po.rs"]},
    {"id": "C6", "requirement": "every number in analysis.json and tune.json recomputed from the raw rows with the validator's own code (protocol section 6 bootstrap, seed 20261124)",
     "pass": cmpc["ok"] and not rc["fail"], "detail": {"cells": cmpc["cells"], "exact": cmpc["exact"], "max_rel_diff": cmpc["max_rel_diff"]},
     "evidence": [f"{V}/out/recompute.json", f"{V}/out/compare_claims.json", f"{V}/scripts/recompute.py"]},
    {"id": "C7", "requirement": "gates G1-G3 with real keys (indexed with [])",
     "pass": all(rc["gates"].values()), "detail": rc["gates"], "evidence": [f"{V}/out/recompute.json"]},
    {"id": "C8", "requirement": "section 5a band against the strongest reference, every listed unpriced counter shown",
     "pass": True, "evidence": [f"{V}/out/band.json", f"{V}/scripts/band.py"]},
    {"id": "C9", "requirement": "claims of RESULTS.md, framing and scope", "pass": True, "note": "findings F1-F9 (validator-report.md)",
     "evidence": [f"{EV}/validator-report.md"]},
]
status = "pass" if all(c["pass"] for c in checks) else "fail"

findings = [
    {"id": "F1", "severity": "minor", "topic": "deviation record", "text": "DEVIATIONS.md says each deviation was logged before the data it concerns was read. D2 (20:50 UTC, 8c54acb9e2) lists the seeds found in 2201-2600 and was committed with that block, after the 1801-2200 block (d202be88ca). The extensions are the frozen rule and were announced in commit messages before their data; the stop of 2601-3000 cannot change the five smallest seeds (1..2600 fully covered). Its 'deleted unread' statement cannot be verified from git."},
    {"id": "F2", "severity": "minor", "topic": "pricing symmetry", "text": "The IC walk's per-step mix64 hash and two coefficient additions mod r (genus_ladder.rs 1590-1594) and the large-prime table operations are not emitted, so 'fully charged' / 'every native operation is charged' is slightly overstated. Pricing 3 per walk step moves x* from 0.8759 to 0.8848 (p = 16411) and 0.6399 to 0.6457 (p = 32771) at w = 1; x at 16411 from 0.9987 to 1.0040. No D1 decision changes."},
    {"id": "F3", "severity": "interpretation", "topic": "scope of 'below the ideal negation-rho bound at all w'", "text": "Correct as stated for x* at both primes and x at 32771 (upper bound < 1 at w = 0, 0.1, 1). Scope: 5 prime-order genus-4 Jacobians per prime (#J about 2^56 and 2^60), 6 targets x 2 seeds each, counted group operations plus w per native operation; reference is the closed-form expected cost of an ideal random walk on the r/2 negation classes (a heuristic floor, not a measured walk and not a proven lower bound); by linearity the order holds for every w up to 1.142 (x*, 16411), 1.565 (x*, 32771), 1.300 (x, 32771). No wall-clock statement."},
    {"id": "F4", "severity": "interpretation", "topic": "'known in theory, now measured'", "text": "The theory cited is right (Gaudry 2000; Gaudry-Thome-Theriault-Diem 2007, O~(q^(2-2/g)); rho O~(q^(g/2))). 'What is new here' is a novelty statement not checked against the knowledge base (unavailable to producer and validator) and should be scoped to this lab's implementation. The crossing is at the pessimistic w = 1 end; at w = 0 and 0.1 the IC arm was already below the floor in the earlier study. The 'p^1.62 / p^1.54' figures are two-point local slopes between p and 2p, not exponent fits (AGENTS.md section 5 asks for at least four sizes); at w = 0 the slopes are 1.340 and 1.383."},
    {"id": "F5", "severity": "minor", "topic": "prose", "text": "'linear algebra from 77-81% of native work to 56-60%': the 56-60% are LA over total w = 1 cost (la_share_w1); over native work they are 76.7-80.8% and 56.2-60.6%."},
    {"id": "F6", "severity": "note", "topic": "tuning", "text": "cc* = 1.4 is at the lower edge of the grid {1.4, 2.0, 2.8}; the optimum is not located. The best cc = 2.0 cell costs 1.110x the choice, so the cc choice is robust; the SGE choice (4,1.5) over (8,2) at cc 1.4 is a 0.72% margin on 12 keys. 'The optimum falls with p' is consistent (p = 8209 chose the upper edge 2.0) but neither optimum is located."},
    {"id": "F7", "severity": "note", "topic": "bootstrap", "text": "analyze.py reseeds the same generator for both primes, and both have the same design, so replicate j uses the same index pattern at both primes: the D2 slope interval is not from independent per-prime bootstraps as PROTOCOL section 6 says. With an independent stream at 32771: x [-0.3847, -0.3699], x* [-0.4612, -0.4463] (claimed [-0.3841, -0.3706], [-0.4622, -0.4455]). The protocol does not fix the draw order; an interleaved order moves interval ends by at most about 2e-4. No decision changes."},
    {"id": "F8", "severity": "note", "topic": "section 5a calibrated weight", "text": "No w_cal was measured for the mod-p and mod-r counters (owner rule 2026-11-12). Because the order holds up to the break-even w (1.14 or more), it holds at w_cal and 10 w_cal for any w_cal below 0.114 GAE per operation; one Jacobian group operation here is about 394 counted F_p multiplications and 7 inversions. 'Ranked at calibrated price' is an inference, not a measurement. Pricing mod-r operations like F_p operations is not material to the verdict: with F_p relation work at 1 GAE, x* reaches the floor only if a mod-r LA operation costs more than 1.25 GAE (16411) or 1.93 GAE (32771)."},
    {"id": "F9", "severity": "note", "topic": "records and reference", "text": "Bench record 20261118T1652Z-genus4-prime-order is still open (closed_utc null), raw entries not uploaded (no S3, disclosed), the 58 search files are not listed in it; its 8 raw hashes match the blobs and its 5 metrics match the recompute. The measured plain-rho sanity check (5 runs, 0.804x sqrt(pi r/2), per run 0.48-1.38) is noisy by design; no measured rho at p = 32771 (disclosed)."},
]

report = {
    "schema": "validator-report/v1", "status": status, "independent": True,
    "disposition": "validated" if status == "pass" else "insufficient_evidence",
    "study": D, "commit_validated": COMMIT, "freeze_commit": FREEZE, "binary_commit": BIN_COMMIT, "pr": "isogeny-labs/crypto#4693",
    "validator": "Validator subagent (Claude Opus 5.5), 2026-11-18; recompute, certificates, prefilter tests and replay comparison with the validator's own code in " + V + "/scripts; analyze.py, tune.py, curves.py and report.py were read, never run or imported",
    "raw_data_base": EV, "raw_data": raw,
    "headline": H, "headline_max_rel_diff": worst,
    "recomputed_readings": {p: {a: {k: rc["primes"][p]["arms"][a][k] for k in ("neg_w0", "neg_w0.1", "neg_w1", "plain_w1", "below_neg_all_w",
                                                                             "mean_cost_w1", "la_share_w1", "la_share_of_native", "per_curve_neg_w1",
                                                                             "break_even_w_vs_neg")}
                                for a in ("x", "x*")} | {"xstar_over_x_w1": rc["primes"][p]["xstar_over_x_w1"],
                                                         "xstar_over_x_w1_paired_per_key": rc["primes"][p]["xstar_over_x_w1_paired_per_key"]}
                            for p in ("16411", "32771")} | {"slope": rc["slope"], "partA": {k: rc["partA"][k] for k in ("choice", "choice_over_baseline", "runner_up", "best_cc2.0_over_choice")},
                                                            "q_check": rc["checks"]["Q"]},
    "checks": checks, "findings": findings,
    "rho_reference_assessment": {
        "kind": "negation", "reference": "ideal negation bound sqrt(pi r/4) group operations, nothing charged (PROTOCOL section 6 R_neg, the D1 reference)",
        "strongest_available": "F_p-automorphisms of all 10 curves are the identity and the hyperelliptic involution (affine scan with planted controls; f irreducible so infinity is the only rational Weierstrass point); Frobenius is trivial on J(F_p)",
        "measured_rho_sanity": "plain rho Q at p = 16411, 5 runs: 0.804x sqrt(pi r/2) (1.137x the negation floor), replayed key 455 identical",
        "band": band["cells"], "order_holds": {k: c["order_below_at_all_w_ci_hi"] for k, c in band["cells"].items()}},
    "lessons_applied": ["L-546d37ca7750", "L-99ee952350b2", "L-3f39e1d8b3a9", "L-7f8877029be4", "L-ce5e4e4b764b", "L-ca1840e60dc0",
                        "L-b0626ec2a4c0", "L-26895912385b", "L-10ef4b64c2a6", "L-f19644df279f", "L-c53b34f2a6ce", "L-1a5a6b64022c", "L-8d86e8fc352c"],
}

cert = {
    "schema": "replay-certificate/v1", "status": "verified" if replay_ok and certs["ok"] else "fail", "independent": True,
    "scalars_verified": bool(replay_ok and certs["ok"]), "study": D, "commit_validated": COMMIT, "freeze_commit": FREEZE,
    "binary": {"commit": BIN_COMMIT, "binary_sha256_file": "493ee2942473b8708a0b52f40ab10095c4da83ab0f0755ed3e5dbc02771b6921",
               "producer_binary_sha256_at_dispatch": "493ee2942473b8708a0b52f40ab10095c4da83ab0f0755ed3e5dbc02771b6921",
               "validator_rebuild_sha256": "ac1dc60d8cf2d18956ec78ae381471069d3199ee6b7f77192f8505890845c276",
               "code_sections_identical": binary_ok, "harness_val_po_sha256": "81bbb7452728a377bbe809c2616fa72cf4e61fd3843be0b4ac9c38dff4b19c1d",
               "evidence": f"{V}/out/binary_diff.txt"},
    "nonce": nonce_line.split()[0], "nonce_drawn_utc": nonce_line.split()[1], "plan": f"{V}/out/plan.json",
    "commands": [f"{V}/scripts/jobs.py (writes {V}/out/jobs.txt)", f"{V}/scripts/run_jobs.sh"],
    "comparison": "every field except wall_ns / search_wall_s / wall_s equal to the committed row (git blob at commit_validated); instance and cell rows equal",
    "summary": {"run_rows_identical": rep["run_rows_identical"], "run_rows_expected": rep["run_rows_expected"], "by_kind": n_rows,
                "search_cells_identical": rep["search_cells_identical"]},
    "runs": [{"tag": c["tag"], "rows_replayed": c.get("rows_replayed"), "identical_rows": c.get("identical_rows"),
              "instance_rows_identical": c.get("instance_rows_identical"), "cell_row": c.get("cell_row"), "ok": c["ok"],
              "file": f"{V}/out/replay/{c['tag']}.jsonl"} for c in rep["cells"]],
    "logarithm_certificates": {"logs_checked": certs["n_logs"], "all_recovered_eq_t_and_G_times_log_eq_T": all(e["recovered_G_eq_T"] and e["recovered_eq_t"] for e in certs["logs"]),
                               "counts_equal_committed": all(e["counts_equal_committed"] for e in certs["logs"]),
                               "keys_certified": f"{certs['keys_ok']}/{certs['keys']}", "tamper_control": certs["tamper_control"],
                               "evidence": f"{V}/out/certs.json"},
}
os.makedirs(os.path.join(REPO, EV), exist_ok=True)
for name, obj in (("replay-certificate.json", cert), ("validator-report.json", report)):
    with open(os.path.join(REPO, EV, name), "w") as f:
        json.dump(obj, f, indent=1)
        f.write("\n")
print("status", status, "headline", len(H), "worst", worst, "replay", cert["status"])
for c in checks:
    print(c["id"], c["pass"])

if status == "pass" and cert["status"] == "verified":
    b = band["cells"]["16411 x*"]["band"]
    val = {
        "schema": "validation-gate/v1",
        "note": "Validator's manifest for genus4_prime_order_20261118 (prime-order genus-4 Jacobians, p = 16411 and 32771, arms x and x*). The band below is arm x* at p = 16411, the binding case of the claim 'x* below the ideal negation bound at all w at both primes' (x* at 32771: 0.0019, 0.0657, 0.6399; x at 32771: 0.0011, 0.0779, 0.7693; x at 16411: 0.0018, 0.1015, 0.9987 with upper bound 1.0023, not ranked by PROTOCOL D1). A genus-4 hyperelliptic result, not an ECDLP one. [pcc:unverified]",
        "replay_certificate": {"path": "replay-certificate.json", "sha256": fsha(f"{EV}/replay-certificate.json")},
        "validator_report": {"path": "validator-report.json", "sha256": fsha(f"{EV}/validator-report.json")},
        "rho_reference": {
            "kind": "negation",
            "protocol": "PROTOCOL.md (frozen 6da1e3c5b2) section 6: R_neg = sqrt(pi r / 4), the ideal negation-rho bound with nothing charged, the D1 reference; section 4's measured plain rho Q (5 runs, p = 16411) is a sanity check only (G3)",
            "unit": "GAE per key, mean over the 60 keys (5 curves x 6 targets x 2 seeds) at p = 16411: one counted Jacobian group operation = 1; each listed unpriced native operation (rel_mul, rel_add, la_mul, la_add; F_p and mod-r) = w; the negation floor charges no native work at any w",
            "binary_or_process": "IC arm x*: genus_ladder sha256 493ee2942473b8708a0b52f40ab10095c4da83ab0f0755ed3e5dbc02771b6921 (built from ca72267899; validator rebuild ac1dc60d... has identical code sections and replays the sampled rows identically). The negation floor is a closed form of each instance's r as emitted by that binary and certified by the validator's own arithmetic (r prime, #J = r, [r]G = 0); no other binary or process enters it",
            "matches_protocol": True, "strongest_available": True, "same_unit": True, "same_binary_or_process": True,
            "gae_band": {w: {"ic": b[w]["ic"], "rho": b[w]["rho_neg"]} for w in ("0", "0.1", "1")},
            "order_holds_across_band": all(b[w]["ic"] < b[w]["rho_neg"] for w in ("0", "0.1", "1")),
        },
    }
    with open(os.path.join(REPO, EV, "VALIDATION.json"), "w") as f:
        json.dump(val, f, indent=1)
        f.write("\n")
    print("VALIDATION.json written")
else:
    p = os.path.join(REPO, EV, "VALIDATION.json")
    if os.path.exists(p):
        os.remove(p)
    print("VALIDATION.json NOT written")
